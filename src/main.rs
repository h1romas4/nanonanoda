use std::i16;
use std::path::{Path, PathBuf};
use std::str::FromStr;

use clap::{Parser, ValueEnum};
use nanonanoda::{Chip, interleaved_to_mono};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input WAV file
    input: String,

    /// Output format: wav or vgm
    #[arg(short, long, value_enum, default_value_t = Format::Wav)]
    format: Format,

    /// Output file (optional). If omitted, a default name is derived from input.
    #[arg(short, long)]
    output: Option<PathBuf>,

    /// Window size for analysis/synthesis
    #[arg(short = 'w', long = "window-size", default_value_t = 512)]
    window_size: usize,

    /// Output sample rate (Hz) for synthesis and written file
    #[arg(short = 'r', long = "output-sample-rate", default_value_t = 44100)]
    output_sample_rate: usize,

    /// Chip specifications. Can be given multiple times. Syntax: name[:count[:voices]]
    /// Examples: --chip ymf262:1:18 --chip ym2203:2:3
    #[arg(long = "chip")]
    chip: Vec<ChipSpecArg>,
}

#[derive(ValueEnum, Clone, Debug)]
enum Format {
    Wav,
    Vgm,
}

#[derive(Debug, Clone)]
struct ChipSpecArg {
    chip: Chip,
    count: usize,
    voices: usize,
}

impl FromStr for ChipSpecArg {
    type Err = String;

    // Syntax: name[:count[:voices]] e.g. "ymf262:1:18" or "ym2203:2:3" or "ymf262".
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(':').collect();
        if parts.is_empty() {
            return Err("empty chip spec".into());
        }
        let name = parts[0].to_lowercase();
        let chip = match name.as_str() {
            "ymf262" => Chip::YMF262Opl3,
            "ym2203" => Chip::YM2203,
            other => return Err(format!("unknown chip '{}'.", other)),
        };
        let count = if parts.len() >= 2 {
            parts[1]
                .parse::<usize>()
                .map_err(|e| format!("invalid count: {}", e))?
        } else {
            1
        };
        let voices = if parts.len() >= 3 {
            parts[2]
                .parse::<usize>()
                .map_err(|e| format!("invalid voices: {}", e))?
        } else {
            3 // default voices per-instance
        };
        Ok(ChipSpecArg {
            chip,
            count,
            voices,
        })
    }
}

fn read_wav_to_mono_f32(path: &str) -> Result<(Vec<f32>, usize), Box<dyn std::error::Error>> {
    let mut reader = hound::WavReader::open(path)?;
    let spec = reader.spec();
    let sample_rate = spec.sample_rate as usize;

    let out: Vec<f32>;

    match (spec.sample_format, spec.bits_per_sample) {
        (hound::SampleFormat::Int, 16) => {
            let samples: Vec<i16> = reader.samples::<i16>().map(|s| s.unwrap_or(0)).collect();
            out = interleaved_to_mono(&samples, spec.channels as usize);
        }
        (hound::SampleFormat::Int, 24) | (hound::SampleFormat::Int, 32) => {
            let samples_i32: Vec<i32> = reader.samples::<i32>().map(|s| s.unwrap_or(0)).collect();
            out = interleaved_to_mono(&samples_i32, spec.channels as usize);
        }
        (hound::SampleFormat::Float, 32) => {
            let samples_f32: Vec<f32> = reader.samples::<f32>().map(|s| s.unwrap_or(0.0)).collect();
            out = interleaved_to_mono(&samples_f32, spec.channels as usize);
        }
        _ => {
            return Err(format!(
                "Unsupported WAV format: {:?} {} bits",
                spec.sample_format, spec.bits_per_sample
            )
            .into());
        }
    }

    Ok((out, sample_rate))
}

fn write_mono_f32_wav(
    path: &Path,
    samples: &[f32],
    sample_rate: usize,
) -> Result<(), Box<dyn std::error::Error>> {
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: sample_rate as u32,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    let mut writer = hound::WavWriter::create(path, spec)?;
    for &s in samples {
        let s_clamped = s.max(-1.0).min(1.0);
        let sample_i16 = (s_clamped * (i16::MAX as f32)) as i16;
        writer.write_sample(sample_i16)?;
    }
    writer.finalize()?;
    Ok(())
}

fn generate_wav_file(
    input: &str,
    output: Option<PathBuf>,
    window_size: usize,
    output_sample_rate: usize,
    chip_instances: &[(Chip, usize)],
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Reading input WAV: {}", input);

    let (buf, sample_rate) = match read_wav_to_mono_f32(input) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("failed to read WAV {}: {:?}", input, e);
            return Err(e);
        }
    };

    let resynth = nanonanoda::process_samples_resynth_multi(
        &buf,
        sample_rate,
        window_size,
        output_sample_rate,
        chip_instances,
    )?;

    let out_path = if let Some(p) = output {
        p
    } else {
        Path::new(input).with_file_name(format!(
            "{}_resynth.wav",
            Path::new(input)
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("out")
        ))
    };

    println!("Resynth out path: {:?}", out_path);
    write_mono_f32_wav(&out_path, &resynth, output_sample_rate)?;
    println!("Wrote resynth WAV for {}", input);

    Ok(())
}

fn generate_vgm_file(
    input: &str,
    output: Option<PathBuf>,
    window_size: usize,
    chip_instances: &[(Chip, usize)],
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Reading input WAV: {}", input);

    let (buf, sample_rate) = match read_wav_to_mono_f32(input) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("failed to read WAV {}: {:?}", input, e);
            return Err(e);
        }
    };

    let mut vgm = nanonanoda::process_samples_resynth_multi_to_vgm(
        &buf,
        sample_rate,
        window_size,
        0x16, // max_tl
        chip_instances,
    )?;

    let track_name = Path::new(input)
        .file_stem()
        .and_then(|s| s.to_str())
        .map(|s| s.to_string());
    let gd3 = nanonanoda::vgm::Gd3 {
        track_name_en: track_name,
        track_name_jp: None,
        game_name_en: None,
        game_name_jp: None,
        system_name_en: None,
        system_name_jp: None,
        author_name_en: None,
        author_name_jp: None,
        release_date: None,
        creator: Some("nanonanoda".to_string()),
        notes: None,
    };
    vgm.gd3 = Some(gd3);

    let out_path = if let Some(p) = output {
        p
    } else {
        Path::new(input).with_file_name(format!(
            "{}_resynth_ym.vgm",
            Path::new(input)
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("out")
        ))
    };

    let bytes = vgm.to_bytes();
    std::fs::write(&out_path, &bytes)?;
    println!("Wrote VGM to {:?}", out_path);

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let mut chip_instances: Vec<(Chip, usize)> = Vec::new();
    if args.chip.is_empty() {
        // default: one ymf262 18 voices, two ym2203 3 voices
        chip_instances.push((Chip::YMF262Opl3, 18));
        chip_instances.push((Chip::YM2203, 3));
        chip_instances.push((Chip::YM2203, 3));
    } else {
        for spec in args.chip.into_iter() {
            for _ in 0..spec.count {
                chip_instances.push((spec.chip, spec.voices));
            }
        }
    }

    match args.format {
        Format::Wav => generate_wav_file(
            &args.input,
            args.output,
            args.window_size,
            args.output_sample_rate,
            &chip_instances,
        ),
        Format::Vgm => generate_vgm_file(
            &args.input,
            args.output,
            args.window_size,
            &chip_instances,
        ),
    }
}
