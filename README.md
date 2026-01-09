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

- Install [wasmtime](https://github.com/bytecodealliance/wasmtime) or [Wasmer](https://github.com/wasmerio/wasmer):

Linux/macOS:

```sh
# Wasmer
curl https://get.wasmer.io -sSfL | sh
# wasmtime
curl https://wasmtime.dev/install.sh -sSf | bash
```

Windows:

```powershell
# Wasmer
winget install --id=Wasmer.Wasmer -e
# wasmtime
winget install --id=BytecodeAlliance.Wasmtime -e
```

```bash
$ wasmer -V
wasmer 6.1.0
$ wasmtime -V
wasmtime 40.0.0 (0807b003e 2025-12-22)
```

- Run example (map host `./assets/voice` to guest `/` inside WASI):

```sh
# Wasmer
# `--mapdir <GUEST_DIR:HOST_DIR>` maps a host directory into the WASI module.
wasmer run --mapdir /:./assets/voice nanonanoda.wasm -- --format vgm /nanonanoda.wav
# wasmtime
# `--dir <HOST_DIR[::GUEST_DIR]>` maps a host directory into the WASI module.
wasmtime run --dir ./assets/voice::/ nanonanoda.wasm --format vgm /nanonanoda.wav
```

## Build and test

Rust and Cargo are required.

Linux/macOS:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Windows: [rustup](https://rustup.rs/)

Build and run tests:

```sh
cargo build
cargo test
```

## Performance

Sample Wave (`3:14` sec):

```
$ ls -laF assets/voice/01.wav
-rw-rw-r-- 1 hiromasa hiromasa 34362990  1月  9 16:20 assets/voice/01.wav
```

Native:

```bash
$ time target/release/nanonanoda --format wav assets/voice/01.wav
Reading input WAV: assets/voice/01.wav
Resynth out path: "assets/voice/01_resynth.wav"
Wrote resynth WAV for assets/voice/01.wav

real    0m5.013s
user    0m4.924s
sys     0m0.089s
```

Wasmer:

```bash
$ time time wasmer run --mapdir /:./assets/voice target/wasm32-wasip1/release/nanonanoda.wasm /01.wav
⠁ Compiling to WebAssembly                                                                                                                          Reading input WAV: /01.wav
Resynth out path: "/01_resynth.wav"
Wrote resynth WAV for /01.wav

real    0m6.331s
user    0m6.173s
sys     0m0.210s
```

Wasmer (AOT optimize): 

```bash
$ time wasmer run --llvm --enable-pass-params-opt --mapdir /:./assets/voice target/wasm32-wasip1/release/nanonanoda.wasm -- --format wav /01.wav
⠁ Compiling to WebAssembly                                                                                                                          Reading input WAV: /01.wav
Resynth out path: "/01_resynth.wav"
Wrote resynth WAV for /01.wav

real    0m5.611s
user    0m5.437s
sys     0m0.176s
```

wasmtime:

```bash
$ time wasmtime run --dir ./assets/voice::/ target/wasm32-wasip1/release/nanonanoda.wasm --format wav /01.wav
Reading input WAV: /01.wav
Resynth out path: "/01_resynth.wav"
Wrote resynth WAV for /01.wav

real    0m6.521s
user    0m6.416s
sys     0m0.101s
```
