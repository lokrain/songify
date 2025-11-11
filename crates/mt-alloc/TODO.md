# TODO — mt-alloc

## Tempo map & timing

- [ ] Expose bi-directional conversions (beats→samples, grid snapping) on `TempoMap`; today `sample_to_beats_x1000` is one-way.
- [ ] Incorporate meter and swing events into conversion helpers instead of only storing them.
- [ ] Add deterministic interpolation for sub-sample beat queries (avoid integer truncation when BPM is not divisible by 60).
- [ ] Provide `TempoMap::from_iter` and sorting safeguards to reject unsorted inputs instead of relying on debug assertions.
- [ ] Implement bounded lookups for large catalogues (binary search instead of linear scans over tempo events).

## MIDI & note utilities

- [ ] Teach `MidiNormalizer` to honour sustain pedal (CC 64) and overlapping voice stealing scenarios.
- [ ] Support configurable velocity curves and off-velocity handling in `MidiNormalizer`.
- [ ] Extend `NoteStore` with range and track filters plus iterators optimised for chord detectors.
- [ ] Add optional deduplication / merge heuristics for repeated short notes emitted by audio analysis.

## Queues & buffers

- [ ] Benchmark and validate `SpscQueue` behaviour under cache pressure; add configurable capacity growth policies.
- [ ] Provide `try_push_slice` / `drain_slice` helpers on `EventRing` to minimise per-event overhead.
- [ ] Add `FeatureBuffer` views for strided (channel-major) data to reduce copies in spectral features.
- [ ] Validate lock-free invariants with Loom tests or Miri to guard against ordering bugs.

## Testing & QA

- [ ] Create exhaustive property tests for `TempoMap` covering tempo ramps, metre jumps, and numerical stability at 192 kHz.
- [ ] Add integration tests that feed raw MIDI sequences into `MidiNormalizer` and compare against golden `NoteEvent` outputs.
- [ ] Run `#[cfg(miri)]` memory safety checks for ring buffers and queues.
- [ ] Provide criterion benchmarks for tempo conversions, queue throughput, and feature-buffer fill performance.

## Tooling & docs

- [ ] Document expected invariants for each public type (e.g., capacity must be power-of-two for `SpscQueue`).
- [ ] Publish example snippets demonstrating how higher crates should wire `TempoMap` + `NoteStore` together.
- [ ] Consider enabling `serde` feature-gated support for serialising timeline caches for debugging.
