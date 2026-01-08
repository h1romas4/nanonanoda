use crate::fnumber::{
    Chip, ChipSpec, FNumber, FNumberError, YM2203Spec, YMF262SpecOpl3, find_and_tune_fnumber,
    generate_12edo_fnum_table,
};
use crate::pcm::{Peak, analyze_pcm_peaks, synthesize_sines};
use crate::vgm::VgmBuilder;
use crate::ym::{
    init_ym2203, init_ym2203_channel_and_op, init_ymf262, init_ymf262_channel_and_op, ym2203_keyon,
    ymf262_keyon,
};

/// Extracted spectral feature representing a chip `FNumber` and the detected magnitude.
///
/// This struct pairs a tuned `FNumber` (chip-specific frequency descriptor)
/// with the measured magnitude from spectral analysis. It is produced by
/// `map_samples_to_fnums` and consumed by `synth_from_spectral_features`.
#[derive(Debug, Clone)]
pub struct SpectralFeature {
    pub fnumber: FNumber,
    pub magnitude: f64,
}

/// Analyze a mono sample window and map dominant spectral peaks to
/// chip-specific F-numbers.
///
/// Generic over a `ChipSpec` implementation (`YMF262Spec`, `YM2203Spec`, etc.).
/// - `samples`: mono PCM samples (floating-point, length typically equals analysis window)
/// - `sample_rate`: sampling rate of `samples` in Hz
/// - `max_voices`: maximum number of peaks/voices to return
/// - `table`: precomputed 12-EDO F-number table produced by `generate_12edo_fnum_table`.
///
/// Returns a vector of `SpectralFeature` containing tuned `FNumber`s and their magnitudes,
/// or `FNumberError` if table lookup/tuning fails.
pub fn map_samples_to_fnums<C: crate::fnumber::ChipSpec>(
    samples: &[f32],
    sample_rate: usize,
    max_voices: usize,
    table: &[[Option<crate::fnumber::FNumberEntry>; 12]; 8],
) -> Result<Vec<SpectralFeature>, FNumberError> {
    let peaks = analyze_pcm_peaks(samples, sample_rate, max_voices);

    if peaks.is_empty() {
        return Ok(Vec::new());
    }

    let mut out: Vec<SpectralFeature> = Vec::new();

    let mclk = C::default_master_clock();
    for peak in peaks.into_iter().take(max_voices) {
        if let Ok(fnum) = find_and_tune_fnumber::<C>(table, peak.freq_hz, mclk) {
            out.push(SpectralFeature {
                fnumber: fnum,
                magnitude: peak.magnitude,
            });
        }
    }

    Ok(out)
}

/// Assign detected spectral peaks to chip instances.
///
/// For each peak this function selects at most one target instance from
/// `chip_instances`. Candidates are instances that still have remaining
/// voice slots; the chosen instance is the one whose tuned `FNumber`
/// produces the smallest tuning error (in cents). The assigned feature
/// (tuned `FNumber` + original magnitude) is appended to that instance's
/// output list and its remaining voice count is decremented.
///
/// The function supports `YMF262Opl3` and `YM2203` chips via the provided
/// per-chip F-number tables. The returned `Vec<Vec<SpectralFeature>>` has
/// the same length and ordering as `chip_instances` so callers can map
/// features back to instances directly.
fn assign_peaks_to_chip_instances(
    peaks: &[Peak],
    _input_sample_rate: usize,
    chip_instances: &[(Chip, usize)],
    fnum_table_ymf262opl3: &[[Option<crate::fnumber::FNumberEntry>; 12]; 8],
    fnum_table_ym2203: &[[Option<crate::fnumber::FNumberEntry>; 12]; 8],
) -> Result<Vec<Vec<SpectralFeature>>, FNumberError> {
    let total_instances = chip_instances.len();
    let mut remaining: Vec<usize> = chip_instances.iter().map(|(_, v)| *v).collect();
    let mut out: Vec<Vec<SpectralFeature>> = vec![Vec::new(); total_instances];

    for peak in peaks.iter() {
        let mut best: Option<(usize, SpectralFeature)> = None;
        for (idx, (chip, _voices)) in chip_instances.iter().enumerate() {
            if remaining[idx] == 0 {
                continue;
            }
            match chip {
                Chip::YMF262Opl3 => {
                    if let Ok(fnum) = find_and_tune_fnumber::<YMF262SpecOpl3>(
                        fnum_table_ymf262opl3,
                        peak.freq_hz,
                        YMF262SpecOpl3::default_master_clock(),
                    ) {
                        let feat = SpectralFeature {
                            fnumber: fnum,
                            magnitude: peak.magnitude,
                        };
                        let err = fnum.error_cents;
                        if best.is_none() || err < best.as_ref().unwrap().1.fnumber.error_cents {
                            best = Some((idx, feat));
                        }
                    }
                }
                Chip::YM2203 => {
                    if let Ok(fnum) = find_and_tune_fnumber::<YM2203Spec>(
                        fnum_table_ym2203,
                        peak.freq_hz,
                        YM2203Spec::default_master_clock(),
                    ) {
                        let feat = SpectralFeature {
                            fnumber: fnum,
                            magnitude: peak.magnitude,
                        };
                        let err = fnum.error_cents;
                        if best.is_none() || err < best.as_ref().unwrap().1.fnumber.error_cents {
                            best = Some((idx, feat));
                        }
                    }
                }
            }
        }

        if let Some((idx, feat)) = best {
            remaining[idx] = remaining[idx].saturating_sub(1);
            out[idx].push(feat);
        }

        // stop early if all assigned
        if remaining.iter().all(|&r| r == 0) {
            break;
        }
    }

    Ok(out)
}

/// Synthesize a mono PCM buffer from a set of `SpectralFeature` entries.
///
/// For each `SpectralFeature`, the function uses the `actual_freq_hz` field
/// from the `FNumber` as the target frequency and preserves the measured
/// magnitude. The peaks are converted to `Peak` structures and summed
/// using `synthesize_sines` to produce `sample_count` samples at `sample_rate` Hz.
///
/// Note: this is a lightweight/simplified chip simulation. It approximates
/// chip output by synthesizing sinusoids at the tuned frequencies and
/// magnitudes from `SpectralFeature` and does not model register-level
/// behavior, envelope/PCM intricacies, or other internal chip details.
pub fn synth_from_spectral_features(
    features: &[SpectralFeature],
    sample_rate: usize,
    sample_count: usize,
) -> Result<Vec<f32>, FNumberError> {
    if features.is_empty() || sample_rate == 0 || sample_count == 0 {
        return Ok(vec![0.0f32; sample_count]);
    }

    let mut peaks: Vec<Peak> = Vec::with_capacity(features.len());

    for feat in features {
        let fnum = feat.fnumber;
        let freq = fnum.actual_freq_hz;
        let mag = feat.magnitude;
        let mag_db = if mag <= 0.0 {
            -200.0
        } else {
            20.0 * mag.log10()
        };
        peaks.push(Peak {
            freq_hz: freq,
            magnitude: mag,
            magnitude_db: mag_db,
            bin: 0,
        });
    }

    let buf = synthesize_sines(&peaks, sample_rate, sample_count);
    Ok(buf)
}

/// Process an entire PCM buffer in fixed-size windows, analyze spectral
/// content per window for multiple chip instances, and resynthesize audio.
///
/// This function coordinates per-window analysis (using `map_samples_to_fnums`)
/// across the provided `chip_instances` and synthesizes output windows at
/// `output_sample_rate`. It precomputes per-chip 12-EDO tables and preserves
/// the time duration of each input window by scaling the synthesized sample
/// count according to the input/output sample rates.
///
/// - `samples`: input mono PCM buffer (f32)
/// - `input_sample_rate`: sample rate of `samples` in Hz
/// - `window_size`: analysis window size in samples
/// - `output_sample_rate`: desired sample rate for synthesized output
/// - `chip_instances`: list of `(Chip, voices)` tuples describing which chips
///   to emulate and how many voices to allocate per instance
///
/// Returns the synthesized mono buffer or an error message if analysis
/// or synthesis fails.
pub fn process_samples_resynth_multi(
    samples: &[f32],
    input_sample_rate: usize,
    window_size: usize,
    output_sample_rate: usize,
    chip_instances: &[(Chip, usize)],
) -> Result<Vec<f32>, String> {
    if window_size == 0 {
        return Err("window_size must be > 0".to_string());
    }

    let fnum_table_ymf262opl3 =
        generate_12edo_fnum_table::<YMF262SpecOpl3>(YMF262SpecOpl3::default_master_clock())
            .map_err(|e| format!("table gen 262 error: {:?}", e))?;
    let fnum_table_ym2203 =
        generate_12edo_fnum_table::<YM2203Spec>(YM2203Spec::default_master_clock())
            .map_err(|e| format!("table gen 2203 error: {:?}", e))?;

    let total_samples = samples.len();
    let mut out: Vec<f32> = Vec::with_capacity(total_samples);

    let mut offset = 0usize;
    while offset < total_samples {
        let end = (offset + window_size).min(total_samples);

        let mut window: Vec<f32> = samples[offset..end].to_vec();
        if window.len() < window_size {
            window.resize(window_size, 0.0);
        }

        // analyze peaks once per window and assign them to chip instances
        let total_voices_needed: usize = chip_instances.iter().map(|(_, v)| *v).sum();
        let peaks = analyze_pcm_peaks(&window, input_sample_rate, total_voices_needed.max(1));
        let per_instance_feats = assign_peaks_to_chip_instances(
            &peaks,
            input_sample_rate,
            chip_instances,
            &fnum_table_ymf262opl3,
            &fnum_table_ym2203,
        )
        .map_err(|e| format!("FNumber mapping error: {:?}", e))?;
        let mut all_features: Vec<SpectralFeature> = Vec::new();
        for mut v in per_instance_feats.into_iter() {
            all_features.append(&mut v);
        }

        let input_count = end - offset;
        let mut output_count = ((input_count as f64) * (output_sample_rate as f64)
            / (input_sample_rate as f64))
            .round() as usize;
        if output_count == 0 {
            output_count = 1;
        }

        let synth = synth_from_spectral_features(&all_features, output_sample_rate, output_count)
            .map_err(|e| format!("synthesis error: {:?}", e))?;
        out.extend_from_slice(&synth[..]);

        offset += window_size;
    }

    Ok(out)
}

// helper: map FFT magnitude to TL (0 = loud, larger value = quieter)
// `max_tl` is the maximum TL value to use (e.g. 0x24). note: 0x00 == loudest.
pub fn mag_to_tl(mag: f64, max_tl: u8) -> u8 {
    // For non-finite or silence, return full attenuation (0x3f)
    if !mag.is_finite() || mag <= 0.0 {
        return 0x3f;
    }
    let mag_db = 20.0 * mag.log10();
    let db_min = -60.0;
    let db_max = 0.0;
    let t = ((mag_db - db_min) / (db_max - db_min)).clamp(0.0, 1.0);
    // Map t==1.0 -> max_tl (loud), t==0.0 -> 0x3f (silent)
    let range = (0x3f_i32 - max_tl as i32) as f64;
    let tl_f = (1.0 - t) * range + (max_tl as f64);
    tl_f.round().clamp(max_tl as f64, 0x3f as f64) as u8
}

/// Similar to `process_samples_resynth_multi`, but instead of synthesizing
/// audio it builds a `VgmDocument` that reproduces the analysis using chip
/// register writes. For each analysis window this function:
/// - maps spectral peaks to per-chip FNumbers
/// - programs operator parameters and frequencies for assigned channels
/// - issues key-on writes for each active voice
/// - inserts a `WaitSamples` corresponding to the synthesized window length
///
/// Returns a built `VgmDocument` on success.
pub fn process_samples_resynth_multi_to_vgm(
    samples: &[f32],
    input_sample_rate: usize,
    window_size: usize,
    max_tl: u8,
    chip_instances: &[(Chip, usize)],
) -> Result<crate::vgm::VgmDocument, String> {
    if window_size == 0 {
        return Err("window_size must be > 0".to_string());
    }
    // VGM sample rate
    let output_sample_rate = 44100;

    let fnum_table_ymf262opl3 =
        generate_12edo_fnum_table::<YMF262SpecOpl3>(YMF262SpecOpl3::default_master_clock())
            .map_err(|e| format!("table gen 262 error: {:?}", e))?;
    let fnum_table_ym2203 =
        generate_12edo_fnum_table::<YM2203Spec>(YM2203Spec::default_master_clock())
            .map_err(|e| format!("table gen 2203 error: {:?}", e))?;

    let total_samples = samples.len();
    let mut builder = VgmBuilder::new();
    builder.set_sample_rate(output_sample_rate as u32);

    let mut seen_ymf262 = false;
    let mut seen_ym2203 = false;
    let ym2203_instances = chip_instances
        .iter()
        .filter(|(c, _)| matches!(c, Chip::YM2203))
        .count();
    for (chip, _voices) in chip_instances.iter() {
        match chip {
            Chip::YMF262Opl3 if !seen_ymf262 => {
                builder.add_chip_clock(
                    crate::vgm::VgmChip::Ymf262,
                    YMF262SpecOpl3::default_master_clock() as u32,
                );
                seen_ymf262 = true;
            }
            Chip::YM2203 if !seen_ym2203 => {
                builder.add_chip_clock(
                    crate::vgm::VgmChip::Ym2203,
                    YM2203Spec::default_master_clock() as u32,
                );
                seen_ym2203 = true;
            }
            _ => {}
        }
    }

    if ym2203_instances >= 2 {
        builder.enable_dual_chip(crate::vgm::VgmChip::Ym2203);
    }

    if seen_ymf262 {
        init_ymf262(&mut builder);
        let base_262 = find_and_tune_fnumber::<YMF262SpecOpl3>(
            &fnum_table_ymf262opl3,
            440.0,
            YMF262SpecOpl3::default_master_clock(),
        )
        .map_err(|e| format!("fnum tune error 262: {:?}", e))?;
        for ch in 0u8..18u8 {
            init_ymf262_channel_and_op(
                &mut builder,
                ch,
                base_262.f_num as u16,
                base_262.block,
                max_tl,
            );
        }
    }
    if seen_ym2203 {
        init_ym2203(&mut builder, 0);
        let chip_count = if ym2203_instances >= 2 {
            ym2203_instances
        } else {
            1usize
        };
        let base_2203 = find_and_tune_fnumber::<YM2203Spec>(
            &fnum_table_ym2203,
            440.0,
            YM2203Spec::default_master_clock(),
        )
        .map_err(|e| format!("fnum tune error 2203: {:?}", e))?;
        for port in 0..chip_count {
            for ch in 0u8..3u8 {
                init_ym2203_channel_and_op(
                    &mut builder,
                    port as u8,
                    ch,
                    base_2203.f_num as u16,
                    base_2203.block,
                    max_tl,
                );
            }
        }
    }

    let mut offset = 0usize;
    while offset < total_samples {
        let end = (offset + window_size).min(total_samples);

        let mut window: Vec<f32> = samples[offset..end].to_vec();
        if window.len() < window_size {
            window.resize(window_size, 0.0);
        }

        let total_voices_needed: usize = chip_instances.iter().map(|(_, v)| *v).sum();
        let peaks = analyze_pcm_peaks(&window, input_sample_rate, total_voices_needed.max(1));
        let per_instance_feats = assign_peaks_to_chip_instances(
            &peaks,
            input_sample_rate,
            chip_instances,
            &fnum_table_ymf262opl3,
            &fnum_table_ym2203,
        )
        .map_err(|e| format!("FNumber mapping error: {:?}", e))?;

        for (idx, (chip, _voices)) in chip_instances.iter().enumerate() {
            let feats = &per_instance_feats[idx];
            if feats.is_empty() {
                continue;
            }

            match chip {
                Chip::YMF262Opl3 => {
                    let max_ch = 18usize;
                    for (i, feat) in feats.iter().enumerate() {
                        let ch_idx = (i % max_ch) as u8;
                        let fnum = feat.fnumber;
                        let fnum_val = fnum.f_num as u16;
                        let block_val = fnum.block;
                        let tl = mag_to_tl(feat.magnitude, max_tl);
                        ymf262_keyon(&mut builder, ch_idx, fnum_val, block_val, tl);
                    }
                }
                Chip::YM2203 => {
                    let port_num = chip_instances[..=idx]
                        .iter()
                        .filter(|(c, _)| matches!(c, Chip::YM2203))
                        .count()
                        - 1;
                    for (i, feat) in feats.iter().enumerate() {
                        let ch = (i % 3) as u8; // YM2203 channels per chip = 3
                        let fnum = feat.fnumber;
                        let fnum_val = fnum.f_num as u16;
                        let block_val = fnum.block;
                        let tl = mag_to_tl(feat.magnitude, max_tl);
                        ym2203_keyon(&mut builder, port_num as u8, ch, fnum_val, block_val, tl);
                    }
                }
            }
        }

        let input_count = end - offset;
        let mut output_count = ((input_count as f64) * (output_sample_rate as f64)
            / (input_sample_rate as f64))
            .round() as usize;
        if output_count == 0 {
            output_count = 1;
        }
        builder.wait_samples(output_count as u32);

        offset += window_size;
    }

    builder.end();
    Ok(builder.build())
}
