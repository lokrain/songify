# TODO — mt-signal-core

## Sample & frame utilities

- [ ] Support additional sample formats (u16, planar f32) with clamped conversions.
- [ ] Provide SIMD-accelerated `as_f32_slice` path under optional feature.
- [ ] Add frame-level arithmetic operations (mixing, dot products) to reduce boilerplate in analyzers.
- [ ] Document thread-safety guarantees for sample conversions.

## Windowing & spectra

- [ ] Implement additional window types (Kaiser, Nuttall) and expose parameterised constructors.
- [ ] Cache window coefficients for reuse across calls to avoid recomputation.
- [ ] Extend `SpectrumView` with helper for cumulative energy / centroid calculations.
- [ ] Validate bin-frequency mapping against FFT length corner cases (odd sizes, zero bins).

## Testing & benchmarks

- [ ] Add golden tests comparing window tables against SciPy outputs.
- [ ] Provide property tests ensuring `Sample::scale` stays within representable bounds.
- [ ] Bench sample/frame transforms at 48 kHz / 96 kHz to detect regressions.
- [ ] Run Miri on core traits to validate no UB with generic implementations.

## Documentation & examples

- [ ] Write developer guide illustrating how analysis crates should consume frames and windows.
- [ ] Publish example demonstrating STFT pipeline using only mt-signal-core building blocks.
- [ ] Explain numeric precision expectations (e.g., 1e-6 tolerances) in README.
- [ ] Add doc comments to `Complex32` explaining magnitude computations and edge handling.
