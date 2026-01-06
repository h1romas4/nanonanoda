use ::nanonanoda::nanonanoda::{mag_to_tl, map_samples_to_fnums, synth_from_spectral_features};
use ::nanonanoda::pcm::{Peak, analyze_pcm_peaks, synthesize_sines};
use ::nanonanoda::{
    Chip, ChipSpec, YM2203Spec, YMF262SpecOpl3, generate_12edo_fnum_table,
    process_samples_resynth_multi,
};

fn generate_test_sine(freq: f64, sample_rate: usize, sample_count: usize, mag: f64) -> Vec<f32> {
    let peak = Peak {
        freq_hz: freq,
        magnitude: mag,
        magnitude_db: if mag <= 0.0 {
            -200.0
        } else {
            20.0 * mag.log10()
        },
        bin: 0,
    };
    synthesize_sines(&[peak], sample_rate, sample_count)
}

#[test]
fn test_process_samples_resynth_multi_44100() {
    let sample_rate = 44100usize;
    let samples = vec![0.0f32; 44100];
    let window_size = 1024usize;
    let chip_instances = vec![(Chip::YMF262Opl3, 6usize)];

    let out = process_samples_resynth_multi(
        &samples,
        sample_rate,
        window_size,
        sample_rate,
        &chip_instances,
    )
    .expect("process_samples_resynth_multi failed");

    // Basic sanity checks: returned buffer is non-empty and has reasonable length
    assert!(!out.is_empty(), "output buffer is empty");
    assert!(out.len() >= 1, "output length should be at least 1");
}

#[test]
fn test_map_samples_to_fnums_single_tone() {
    let sample_rate = 48000usize;
    let window = 4096usize;
    let freq = 1500.0_f64;

    let buf = generate_test_sine(freq, sample_rate, window, 1.0);

    let fnum_table_ymf262 =
        generate_12edo_fnum_table::<YMF262SpecOpl3>(YMF262SpecOpl3::default_master_clock())
            .expect("table gen 262");
    let fnum_table_ym2203 =
        generate_12edo_fnum_table::<YM2203Spec>(YM2203Spec::default_master_clock())
            .expect("table gen 2203");

    // YMF262
    let features_262 =
        map_samples_to_fnums::<YMF262SpecOpl3>(&buf, sample_rate, 4, &fnum_table_ymf262)
            .expect("mapping failed 262");
    assert!(!features_262.is_empty(), "no features returned for 262");
    let f = &features_262[0];
    assert!(f.magnitude > 0.0, "magnitude is zero or negative");
    assert!(f.fnumber.actual_freq_hz.is_finite());
    assert!(f.fnumber.error_cents.is_finite());
    assert!(
        f.fnumber.error_cents < 200.0,
        "error too large: {} cents",
        f.fnumber.error_cents
    );

    // YM2203
    let features_2203 =
        map_samples_to_fnums::<YM2203Spec>(&buf, sample_rate, 4, &fnum_table_ym2203)
            .expect("mapping failed 2203");
    assert!(!features_2203.is_empty(), "no features returned for 2203");
}

#[test]
fn test_synth_from_spectral_features_roundtrip() {
    let sample_rate = 48000usize;
    let window = 4096usize;
    let freq = 1000.0_f64;

    let src = generate_test_sine(freq, sample_rate, window, 1.0);

    let fnum_table_ymf262 =
        generate_12edo_fnum_table::<YMF262SpecOpl3>(YMF262SpecOpl3::default_master_clock())
            .expect("table gen 262");
    let fnum_table_ym2203 =
        generate_12edo_fnum_table::<YM2203Spec>(YM2203Spec::default_master_clock())
            .expect("table gen 2203");

    // YMF262 roundtrip
    let features = map_samples_to_fnums::<YMF262SpecOpl3>(&src, sample_rate, 4, &fnum_table_ymf262)
        .expect("mapping failed 262");
    assert!(!features.is_empty());
    let synth = synth_from_spectral_features(&features, sample_rate, window).expect("synth failed");
    assert_eq!(synth.len(), window);
    let peaks = analyze_pcm_peaks(&synth, sample_rate, 4);
    assert!(!peaks.is_empty(), "no peaks in synthesized buffer");
    let top = &peaks[0];
    let bin_width = (sample_rate as f64) / (window as f64);
    let diff = (top.freq_hz - freq).abs();
    assert!(
        diff <= bin_width * 1.5,
        "peak freq mismatch: {} Hz (bin {})",
        diff,
        bin_width
    );

    // YM2203 roundtrip
    let features = map_samples_to_fnums::<YM2203Spec>(&src, sample_rate, 4, &fnum_table_ym2203)
        .expect("mapping failed 2203");
    assert!(!features.is_empty());
    let synth = synth_from_spectral_features(&features, sample_rate, window).expect("synth failed");
    assert_eq!(synth.len(), window);
    let peaks = analyze_pcm_peaks(&synth, sample_rate, 4);
    assert!(!peaks.is_empty(), "no peaks in synthesized buffer");
    let top = &peaks[0];
    let diff = (top.freq_hz - freq).abs();
    assert!(
        diff <= bin_width * 1.5,
        "peak freq mismatch: {} Hz (bin {})",
        diff,
        bin_width
    );
}

#[test]
fn test_multi_tone_varied_magnitudes() {
    let sample_rate = 48000usize;
    let window = 4096usize;

    let peaks = vec![
        Peak {
            freq_hz: 220.0,
            magnitude: 1.0,
            magnitude_db: 20.0 * 1.0_f64.log10(),
            bin: 0,
        },
        Peak {
            freq_hz: 440.0,
            magnitude: 0.6,
            magnitude_db: 20.0 * 0.6_f64.log10(),
            bin: 0,
        },
        Peak {
            freq_hz: 880.0,
            magnitude: 0.3,
            magnitude_db: 20.0 * 0.3_f64.log10(),
            bin: 0,
        },
    ];

    let src = synthesize_sines(&peaks, sample_rate, window);

    let fnum_table_ymf262 =
        generate_12edo_fnum_table::<YMF262SpecOpl3>(YMF262SpecOpl3::default_master_clock())
            .expect("table gen 262");
    let fnum_table_ym2203 =
        generate_12edo_fnum_table::<YM2203Spec>(YM2203Spec::default_master_clock())
            .expect("table gen 2203");

    // YMF262
    let features = map_samples_to_fnums::<YMF262SpecOpl3>(&src, sample_rate, 6, &fnum_table_ymf262)
        .expect("mapping failed 262");
    assert!(
        features.len() >= 3,
        "expected at least 3 features, got {}",
        features.len()
    );
    let synth = synth_from_spectral_features(&features, sample_rate, window).expect("synth failed");
    assert_eq!(synth.len(), window);
    let peaks_out = analyze_pcm_peaks(&synth, sample_rate, 6);
    assert!(
        peaks_out.len() >= 3,
        "expected at least 3 peaks in synthesized output"
    );
    let bin_width = (sample_rate as f64) / (window as f64);
    let targets = [220.0_f64, 440.0_f64, 880.0_f64];
    for i in 0..3usize {
        let detected = peaks_out.get(i).expect("missing peak");
        let diff = (detected.freq_hz - targets[i]).abs();
        assert!(diff <= bin_width * 2.0, "peak {} mismatch: {} Hz", i, diff);
    }

    // YM2203
    let features = map_samples_to_fnums::<YM2203Spec>(&src, sample_rate, 6, &fnum_table_ym2203)
        .expect("mapping failed 2203");
    assert!(
        features.len() >= 3,
        "expected at least 3 features, got {}",
        features.len()
    );
    let synth = synth_from_spectral_features(&features, sample_rate, window).expect("synth failed");
    assert_eq!(synth.len(), window);
    let peaks_out = analyze_pcm_peaks(&synth, sample_rate, 6);
    assert!(
        peaks_out.len() >= 3,
        "expected at least 3 peaks in synthesized output (2203)"
    );
    for i in 0..3usize {
        let detected = peaks_out.get(i).expect("missing peak");
        let diff = (detected.freq_hz - targets[i]).abs();
        assert!(
            diff <= bin_width * 2.0,
            "peak {} mismatch: {} Hz (2203)",
            i,
            diff
        );
    }
}

#[test]
fn test_mag_to_tl_mapping() {
    let max_tl: u8 = 0x24;

    assert_eq!(mag_to_tl(0.0, max_tl), 0x3f);
    assert_eq!(mag_to_tl(f64::NAN, max_tl), 0x3f);

    assert_eq!(mag_to_tl(1.0, max_tl), max_tl);

    let mag_neg60 = 10f64.powf(-60.0 / 20.0);
    assert_eq!(mag_to_tl(mag_neg60, max_tl), 0x3f);

    let mag_neg30 = 10f64.powf(-30.0 / 20.0);
    let tl_mid = mag_to_tl(mag_neg30, max_tl);
    assert_eq!(tl_mid, 0x32);
}
