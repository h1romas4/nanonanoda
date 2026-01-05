use rustfft::{FftPlanner, num_complex::Complex};
use std::f32::consts::PI;

/// A detected spectral peak.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Peak {
    /// Frequency in Hz
    pub freq_hz: f64,
    /// Linear magnitude (sqrt(re^2+im^2))
    pub magnitude: f64,
    /// Magnitude in dB (20*log10), approximate; -inf mapped to -200.0
    pub magnitude_db: f64,
    /// FFT bin index
    pub bin: usize,
}

/// Analyze PCM samples and return up to `max_peaks` dominant spectral peaks.
///
/// - `samples`: mono PCM samples (f32). If the slice length is not a power
///   of two, the input is zero-padded to the next power of two.
/// - `sample_rate`: sampling rate in Hz.
/// - `max_peaks`: maximum number of peaks to return (0 => empty vec).
pub fn analyze_pcm_peaks(samples: &[f32], sample_rate: usize, max_peaks: usize) -> Vec<Peak> {
    if samples.is_empty() || max_peaks == 0 || sample_rate == 0 {
        return Vec::new();
    }

    let len = samples.len();
    let fft_size = len.next_power_of_two();

    let mut buffer: Vec<Complex<f32>> = vec![Complex::new(0.0, 0.0); fft_size];
    for idx in 0..len {
        // Hann window
        let win = 0.5 * (1.0 - (2.0 * PI * (idx as f32) / ((len - 1) as f32)).cos());
        buffer[idx].re = samples[idx] * win;
    }

    let mut planner = FftPlanner::<f32>::new();
    let fft = planner.plan_fft_forward(fft_size);
    fft.process(&mut buffer);

    let half = fft_size / 2;
    let mut mags: Vec<f64> = Vec::with_capacity(half);
    for bin_idx in 0..half {
        let comp = buffer[bin_idx];
        let mag = ((comp.re as f64).powi(2) + (comp.im as f64).powi(2)).sqrt();
        mags.push(mag);
    }

    let mut candidates: Vec<(usize, f64)> = Vec::new();
    for bin_idx in 1..(mags.len() - 1) {
        let mag = mags[bin_idx];
        if mag > mags[bin_idx - 1] && mag > mags[bin_idx + 1] {
            candidates.push((bin_idx, mag));
        }
    }

    candidates.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    let take_n = candidates.len().min(max_peaks);

    let mut peaks: Vec<Peak> = Vec::with_capacity(take_n);
    for &(bin, mag) in candidates.iter().take(take_n) {
        let freq = (bin as f64) * (sample_rate as f64) / (fft_size as f64);
        let mag_db = if mag <= 0.0 {
            -200.0
        } else {
            20.0 * mag.log10()
        };
        peaks.push(Peak {
            freq_hz: freq,
            magnitude: mag,
            magnitude_db: mag_db,
            bin,
        });
    }

    peaks
}

/// Synthesize a mono buffer of `sample_count` samples at `sample_rate` Hz
/// by summing sinusoids for each provided `peaks` entry.
///
/// The per-peak magnitude is taken from `Peak::magnitude` and normalized
/// relative to the strongest peak to form amplitude weights. The final
/// output is scaled to avoid clipping (max absolute value <= 0.95).
pub fn synthesize_sines(peaks: &[Peak], sample_rate: usize, sample_count: usize) -> Vec<f32> {
    let mut out = vec![0.0f32; sample_count];
    if peaks.is_empty() || sample_rate == 0 || sample_count == 0 {
        return out;
    }

    let max_mag = peaks.iter().map(|p| p.magnitude).fold(0.0_f64, f64::max);
    let max_mag = if max_mag.is_finite() && max_mag > 0.0 {
        max_mag
    } else {
        1.0
    };

    let mut comps: Vec<(f64, f64)> = Vec::with_capacity(peaks.len()); // (omega, amp)
    for peak in peaks {
        let omega = 2.0 * std::f64::consts::PI * peak.freq_hz / (sample_rate as f64);
        let amp = (peak.magnitude / max_mag) as f64;
        comps.push((omega, amp));
    }

    for sample_idx in 0..sample_count {
        let t = sample_idx as f64;
        let mut sum = 0.0f64;
        for (omega, amp) in &comps {
            sum += amp * (omega * t).sin();
        }
        out[sample_idx] = sum as f32;
    }

    let max_abs = out.iter().fold(0.0f32, |m, &v| m.max(v.abs()));
    if max_abs > 0.0 {
        let scale = 0.95_f32 / max_abs;
        if scale < 1.0 {
            for val in &mut out {
                *val *= scale;
            }
        }
    }

    // TODO: adjust overall
    for val in &mut out {
        *val *= 0.2_f32;
    }

    out
}

/// Trait to convert various sample integer/float types to normalized f32.
pub trait SampleToF32 {
    fn to_f32_normalized(self) -> f32;
}

impl SampleToF32 for i16 {
    fn to_f32_normalized(self) -> f32 {
        (self as f32) / (i16::MAX as f32)
    }
}

impl SampleToF32 for i32 {
    fn to_f32_normalized(self) -> f32 {
        (self as f32) / (i32::MAX as f32)
    }
}

impl SampleToF32 for f32 {
    fn to_f32_normalized(self) -> f32 {
        self
    }
}

/// Convert interleaved multi-channel samples into a mono `Vec<f32>`.
///
/// - `samples`: interleaved samples (frame0_ch0, frame0_ch1, ..., frame1_ch0, ...)
/// - `channels`: number of channels per frame
pub fn interleaved_to_mono<S: SampleToF32 + Copy>(samples: &[S], channels: usize) -> Vec<f32> {
    if channels == 0 {
        return Vec::new();
    }
    if channels == 1 {
        return samples.iter().map(|&s| s.to_f32_normalized()).collect();
    }
    if samples.is_empty() {
        return Vec::new();
    }
    let frames = samples.len() / channels;
    let mut out = Vec::with_capacity(frames);
    for frame_idx in 0..frames {
        let mut acc = 0.0f32;
        for ch_idx in 0..channels {
            acc += samples[frame_idx * channels + ch_idx].to_f32_normalized();
        }
        out.push(acc / (channels as f32));
    }
    out
}
