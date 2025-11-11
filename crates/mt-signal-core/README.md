# mt-signal-core

Deterministic, `no_std`-capable signal primitives for the Music Theory & Analytics engine.

Provides:

- `Sample` trait + implementations for `f32`, `f64`, `i16`, `i32`
- `Frame<S, N>` / `MonoFrame` / `StereoFrame` for fixed-size channel groups
- `WindowKind` + `window_value` + `fill_window` for in-place windowing
- `Complex32` and `SpectrumView` trait for FFT/spectrum integration (no FFT code)

Design:

- No heap allocations.
- No I/O, logging, threading, or randomness.
- `#![forbid(unsafe_code)]`, Clippy pedantic.
- Stable contracts consumed by `mt-analysis` and `mt-engine`.

mt-analysis implements STFT/FFT and feature extraction using these contracts.
