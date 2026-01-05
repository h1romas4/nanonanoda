# nanonanoda

nanonanoda is a small VGM/OPL3/OPN resynthesis tool.

It analyzes PCM audio, extracts spectral peaks, maps those peaks to FM-chip F-numbers (YMF262/OPL3 and YM2203), and emits VGM files that reproduce the analysis using register writes. It also includes lightweight synthesis helpers for testing and comparison.

Note:

- These default settings were created for [NanoDrive7](https://github.com/Fujix1/NanoDrive7).
- This repository is a work in progress (WIP) and may contain bugs.

## Requirements

Rust and Cargo are required.

Build and run tests:

```sh
cargo build
cargo test
```

## Basic usage

Use the CLI to generate either a VGM or a re-synthesized WAV from an input WAV file.

Generate a VGM (defaults to one YMF262 18-voice instance and two YM2203 3-voice instances):

```sh
cargo run --release -- --format vgm path/to/input.wav
```

Generate a WAV by re-synthesizing into PCM:

```sh
cargo run --release -- --format wav path/to/input.wav
```

Example: to specify chip instances and voice counts use the `--chip` flag; the following runs with one YM2203 instance with 3 voices:

```sh
cargo run --release -- --format vgm --chip ym2203:1:3 path/to/input.wav
```

Options:

```sh
cargo run --release -- --help
Usage: nanonanoda [OPTIONS] <INPUT>

Arguments:
  <INPUT>  Input WAV file

Options:
  -f, --format <FORMAT>
          Output format: wav or vgm [default: wav] [possible values: wav, vgm]
  -o, --output <OUTPUT>
          Output file (optional). If omitted, a default name is derived from input
  -w, --window-size <WINDOW_SIZE>
          Window size for analysis/synthesis [default: 512]
  -r, --output-sample-rate <OUTPUT_SAMPLE_RATE>
          Output sample rate (Hz) for synthesis and written file [default: 44100]
      --chip <CHIP>
          Chip specifications. Can be given multiple times. Syntax: name[:count[:voices]] Examples: --chip ymf262:1:18 --chip ym2203:2:3
  -h, --help
          Print help
  -V, --version
          Print version
```

## Development notes

- TL mapping: `src/nanonanoda.rs::mag_to_tl` converts spectral magnitude to the chip TL range. Unit tests for this mapping exist in `tests/nanonanoda.rs`.

If you'd like additional features (for example, a CLI option to set `max_tl`, voice-tracking across analysis windows, or finer TL mapping curves), tell me which one and I can implement it.
