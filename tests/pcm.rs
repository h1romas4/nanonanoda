use nanonanoda::pcm::{Peak, analyze_pcm_peaks, synthesize_sines};

#[test]
fn test_analyze_single_tone() {
    let sample_rate = 44_100usize;
    let window = 4096usize;
    let freq = 1000.0f64;

    fn generate_sine(freq_hz: f64, sample_rate: usize, len: usize) -> Vec<f32> {
        let mut v = vec![0.0f32; len];
        for i in 0..len {
            let t = i as f64 / (sample_rate as f64);
            v[i] = (2.0 * std::f64::consts::PI * freq_hz * t).sin() as f32;
        }
        v
    }

    let samples = generate_sine(freq, sample_rate, window);
    let peaks = analyze_pcm_peaks(&samples, sample_rate, 5);

    assert!(!peaks.is_empty(), "no peaks found");
    let p = peaks[0];
    let resolution = (sample_rate as f64) / (window as f64);
    // Accept within one bin width
    assert!(
        (p.freq_hz - freq).abs() <= resolution,
        "peak freq {:?} not near {} (res={})",
        p.freq_hz,
        freq,
        resolution
    );
    assert!(p.magnitude > 0.0, "peak magnitude non-positive");
}

#[test]
fn test_synthesize_and_analyze() {
    let sample_rate = 48_000usize;
    let window = 4096usize;
    let target = 1500.0f64;

    let peak = Peak {
        freq_hz: target,
        magnitude: 1.0,
        magnitude_db: 0.0,
        bin: 0,
    };
    let buf = synthesize_sines(&[peak], sample_rate, window);

    let peaks = analyze_pcm_peaks(&buf, sample_rate, 3);
    assert!(!peaks.is_empty(), "no peaks found in synthesized buffer");
    let p = peaks[0];
    let resolution = (sample_rate as f64) / (window as f64);
    assert!(
        (p.freq_hz - target).abs() <= resolution,
        "synth peak {:?} not near {}",
        p.freq_hz,
        target
    );
}

#[test]
fn test_synthesize_multi_peaks() {
    let sample_rate = 48_000usize;
    let window = 4096usize;
    let targets = [220.0f64, 440.0f64, 880.0f64];

    let peaks: Vec<Peak> = targets
        .iter()
        .map(|&f| Peak {
            freq_hz: f,
            magnitude: 1.0,
            magnitude_db: 0.0,
            bin: 0,
        })
        .collect();

    let buf = synthesize_sines(&peaks, sample_rate, window);
    let detected = analyze_pcm_peaks(&buf, sample_rate, 16);
    assert!(!detected.is_empty(), "no peaks detected");

    let resolution = (sample_rate as f64) / (window as f64);
    for &t in &targets {
        let found = detected.iter().any(|p| (p.freq_hz - t).abs() <= resolution);
        assert!(
            found,
            "target {} Hz not found among detected peaks: {:?}",
            t,
            detected.iter().map(|p| p.freq_hz).collect::<Vec<_>>()
        );
    }
}

#[test]
fn test_synthesize_multi_peaks_low_magnitude() {
    let sample_rate = 48_000usize;
    let window = 4096usize;
    let targets = [220.0f64, 440.0f64, 880.0f64];

    // Lower magnitudes for secondary peaks
    let mags = [1.0f64, 0.3f64, 0.1f64];

    let peaks: Vec<Peak> = targets
        .iter()
        .zip(mags.iter())
        .map(|(&f, &m)| Peak {
            freq_hz: f,
            magnitude: m,
            magnitude_db: if m > 0.0 { 20.0 * m.log10() } else { -200.0 },
            bin: 0,
        })
        .collect();

    let buf = synthesize_sines(&peaks, sample_rate, window);
    let detected = analyze_pcm_peaks(&buf, sample_rate, 32);
    assert!(!detected.is_empty(), "no peaks detected");

    let resolution = (sample_rate as f64) / (window as f64);
    for &t in &targets {
        let found = detected.iter().any(|p| (p.freq_hz - t).abs() <= resolution);
        assert!(
            found,
            "target {} Hz not found among detected peaks: {:?}",
            t,
            detected.iter().map(|p| p.freq_hz).collect::<Vec<_>>()
        );
    }
}
