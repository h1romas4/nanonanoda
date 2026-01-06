# nanonanoda

![](https://github.com/h1romas4/nanonanoda/workflows/Release/badge.svg) ![](https://github.com/h1romas4/nanonanoda/workflows/Build/badge.svg)

nanonanoda is a small VGM/OPL3/OPN resynthesis tool.

It analyzes PCM audio, extracts spectral peaks, maps those peaks to FM-chip F-numbers (YMF262/OPL3 and YM2203), and emits VGM files that reproduce the analysis using register writes. It also includes lightweight synthesis helpers for testing and comparison.

Note:

- These default settings were created for [NanoDrive7](https://github.com/Fujix1/NanoDrive7).
- This repository is a work in progress (WIP) and may contain bugs.

[![](https://github.com/h1romas4/nanonanoda/raw/main/assets/image/nanonanoda-02.jpg)](https://youtu.be/BBktoIkhfDk)

## Usage

Use the CLI to generate either a VGM or a re-synthesized WAV from an input WAV file.

Generate a VGM (defaults to one YMF262 18-voice instance and two YM2203 3-voice instances):

```sh
${nanonanoda} --format vgm path/to/input.wav
```

Generate a WAV by re-synthesizing into PCM:

```sh
${nanonanoda} --format wav path/to/input.wav
```

Example: to specify chip instances and voice counts use the `--chip` flag; the following runs with one YM2203 instance with 3 voices:

```sh
${nanonanoda} --format vgm --chip ym2203:1:3 path/to/input.wav
```

Options:

```sh
${nanonanoda} --help
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

`${nanonanoda}` refers to the native binary. To run the program from source using Cargo, use `cargo run --release --` followed by the CLI arguments. Example:

```sh
cargo run --release -- --format vgm path/to/input.wav
```

Or run the built binary directly:

```sh
target/release/nanonanoda --format vgm path/to/input.wav
```

## Usage - Running the .wasm release

- Download the latest `.wasm` release artifact:

```sh
curl -L -o nanonanoda.wasm https://github.com/h1romas4/nanonanoda/releases/latest/download/nanonanoda.wasm
```

- Install [wasmtime](https://github.com/bytecodealliance/wasmtime) (recommended):

Linux/macOS:

```sh
curl https://wasmtime.dev/install.sh -sSf | bash
```

Windows:

```powershell
winget install --id=BytecodeAlliance.Wasmtime -e
```

```bash
wasmtime -V
wasmtime 40.0.0 (0807b003e 2025-12-22)
```

- Run example (map host `./assets/voice` to guest `/` inside WASI):

```sh
# Use host ./assets/voice/nanonanoda.wav as /nanonanoda.wav inside the WASI module
wasmtime run --dir ./assets/voice::/ nanonanoda.wasm --format vgm /nanonanoda.wav
```

Note: `--dir <HOST_DIR[::GUEST_DIR]>` maps a host directory into the WASI module.

## Build and test

Rust and Cargo are required.

Build and run tests:

```sh
cargo build
cargo test
```

## Development notes

- TL mapping: `src/nanonanoda.rs::mag_to_tl` converts spectral magnitude to the chip TL range. Unit tests for this mapping exist in `tests/nanonanoda.rs`.

If you'd like additional features (for example, a CLI option to set `max_tl`, voice-tracking across analysis windows, or finer TL mapping curves), tell me which one and I can implement it.
