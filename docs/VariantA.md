# Songify

Here is the full workspace tree plus precise responsibility for every file.

Use this as the canonical Rust app layout for your Music Theory & Analytics engine.

```text
mt-workspace/
  Cargo.toml
  rust-toolchain.toml
  Justfile
  README.md
  TODO.md
  .cargo/
    config.toml

  mt-core/
    Cargo.toml
    README.md
    TODO.md
    src/
      lib.rs
      error.rs
      time.rs
      pitch.rs
      interval.rs
      scale.rs
      chord_kind.rs
      chord.rs
      key.rs
      midi.rs
      events/
        mod.rs
        tempo.rs
        meter.rs
        swing.rs
        note.rs
        chord_event.rs
        key_event.rs
        segment.rs
      traits.rs
    tests/
      pitch_test.rs
      interval_test.rs
      chord_kind_test.rs
      chord_test.rs
      key_test.rs
      time_test.rs
      events_test.rs

  mt-signal-core/
    Cargo.toml
    README.md
    TODO.md
    src/
      lib.rs
      sample.rs
      frame.rs
      window.rs
      spectrum.rs
      traits.rs
    tests/
      sample_test.rs
      frame_test.rs
      window_test.rs

  mt-alloc/
    Cargo.toml
    README.md
    TODO.md
    src/
      lib.rs
      alloc_utils.rs
      tempo_map.rs
      midi_normalizer.rs
      note_store.rs
      event_ring.rs
      spsc_queue.rs
      feature_buffer.rs
    tests/
      tempo_map_test.rs
      midi_normalizer_test.rs
      note_store_test.rs
      event_ring_test.rs
      spsc_queue_test.rs

  mt-analysis/
    Cargo.toml
    README.md
    TODO.md
    src/
      lib.rs
      config.rs
      traits.rs
      midi_note_detector.rs
      audio_note_detector.rs
      chord_detector/
        mod.rs
        rule_based.rs
        template_matching.rs
      key_detector/
        mod.rs
        histogram.rs
      tempo_meter_detector.rs
      swing_detector.rs
      segmenter/
        mod.rs
        energy_segmenter.rs
        harmonic_segmenter.rs
      postprocess.rs
      confidence.rs
    tests/
      midi_note_detector_test.rs
      audio_note_detector_test.rs
      chord_detector_test.rs
      key_detector_test.rs
      tempo_meter_detector_test.rs
      swing_detector_test.rs
      segmenter_test.rs

  mt-engine/
    Cargo.toml
    README.md
    TODO.md
    src/
      lib.rs
      types.rs
      config.rs
      engine_rt.rs
      engine_session.rs
      pipeline.rs
      event_bus.rs
      snapshot.rs
      validate.rs
      logging.rs
    tests/
      engine_smoke_test.rs
      rt_safety_test.rs
      pipeline_contract_test.rs

  mt-ffi/
    Cargo.toml
    README.md
    TODO.md
    src/
      lib.rs
      version.rs
      error.rs
      types.rs
      engine_handle.rs
      api.rs
    tests/
      ffi_smoke_test.rs
      ffi_versioning_test.rs

  mt-cli/
    Cargo.toml
    README.md
    TODO.md
    src/
      main.rs
      commands/
        mod.rs
        analyze.rs
        dump_timeline.rs
        benchmark.rs
        validate.rs
    tests/
      cli_smoke_test.rs
      analyze_golden_test.rs

  mt-semantic/         # optional, from your earlier choice
    Cargo.toml
    README.md
    TODO.md
    src/
      lib.rs
      motif.rs
      voice_leading.rs
      functional_harmony.rs
      graph.rs
    tests/
      motif_test.rs
      voice_leading_test.rs
      functional_harmony_test.rs
```

Now file-by-file responsibilities.

---

## Root

**`Cargo.toml`**
Workspace definition. Declares members, shared profiles, resolver, features shared across crates.

**`rust-toolchain.toml`**
Pins Rust toolchain channel/version used by all crates.

**`.cargo/config.toml`**
Global compiler flags, `-D warnings`, `-D clippy::pedantic` etc, plus target-specific config.

**`Justfile`**
Standard tasks: format, clippy, test, lint, coverage, release build.

**`README.md`**
Top-level architecture and usage summary.

**`TODO.md`**
Cross-cutting JIRA-style tasks referencing crates/files.

---

## `mt-core` (`no_std`)

Pure theory and event primitives. No heap, no I/O.

**`src/lib.rs`**
Crate root.
Enables `#![no_std]`, feature flags, re-exports all public types and modules.

**`src/error.rs`**
`TheoryError` and small error enums for invalid pitches, intervals, keys, chords, timelines. No `std::error::Error`.

**`src/time.rs`**
`SampleTime`, `MusicalTime`, `TimeStamp`.
Constants and pure helpers for comparison, arithmetic, but no tempo mapping (that is in `mt-alloc`).

**`src/pitch.rs`**
`PitchClass`, `MidiPitch`, conversions, validation, spelling helpers using enums/ids, not strings.

**`src/interval.rs`**
Interval representations (semitones, diatonic steps). Validation and operations.

**`src/scale.rs`**
Scale definitions as interval patterns over `PitchClass`. Major, minor, modes; extensible via features.

**`src/chord_kind.rs`**
`ChordKindId` enum and static lookup tables mapping kinds to interval sets, tensions bitmasks. No allocations at runtime.

**`src/chord.rs`**
Core chord representation using `ChordKindId`, root, bass, inversion, tension bits. Validation and equality.

**`src/key.rs`**
`Key` and `KeyMode`. Only contains structure and small helpers; no detection logic.

**`src/midi.rs`**
`RawMidiEvent` enum. Packing/unpacking helpers. No stateful parsing.

**`src/events/mod.rs`**
Re-exports of all event types: tempo, meter, swing, note, chord, key, segment.

**`src/events/tempo.rs`**
`TempoEvent` (fixed-point bpm, ppq). No tempo map.

**`src/events/meter.rs`**
`MeterEvent` (time signature change).

**`src/events/swing.rs`**
`SwingEvent` (swing ratio).

**`src/events/note.rs`**
`NoteId`, `TrackId`, `NoteSpan` (normalized note with onset/duration and flags).

**`src/events/chord_event.rs`**
`ChordEvent` with start/end `SampleTime`, `ChordEventCore`, confidence.

**`src/events/key_event.rs`**
`KeyEvent` with `Key`, position, confidence, local/global marker.

**`src/events/segment.rs`**
`SegmentEvent` with start/end, `SegmentKind`, label id.

**`src/traits.rs`**
Core shared traits:

* `TimeStamped`
* `HasConfidence`
* Small marker traits reused across crates.
  All object-safe and `no_std`.

**`tests/*.rs`**
Unit tests for each primitive:

* correct ranges, arithmetic, equality,
* serialization (if enabled),
* no floating-point surprises for fixed-point fields.

---

## `mt-signal-core` (`no_std`)

Audio primitives and math only.

**`src/lib.rs`**
Crate root, re-exports modules.

**`src/sample.rs`**
Scalar sample types and helpers: conversions, clamping, trait bounds.

**`src/frame.rs`**
Fixed-size audio frames (mono/stereo/N) with index-safe access. No heap.

**`src/window.rs`**
Deterministic generators for Hann, Hamming, Blackman, etc. Return fixed or borrowed slices.

**`src/spectrum.rs`**
Interfaces for magnitude/phase views; no FFT implementation, only traits and helpers.

**`src/traits.rs`**
Traits for STFT-like usage and feature extraction contracts that `mt-analysis` can rely on.

**`tests/*`**
Validate window correctness, frame behavior, invariants.

---

## `mt-alloc` (`no_std + alloc`)

Heap-based utilities and runtime structures.

**`src/lib.rs`**
Re-exports submodules. Keeps layering: depends only on `mt-core` and `mt-signal-core`.

**`src/alloc_utils.rs`**
Common patterns for `Vec`, `Box`, slice wrappers, growth strategies. No logging or OS.

**`src/tempo_map.rs`**
Pure tempo map:

* Stores `TempoEvent`, `MeterEvent`, `SwingEvent`.
* Provides conversions SampleTime ↔ MusicalTime.
* Handles tempo changes deterministically.

**`src/midi_normalizer.rs`**
State machine:

* Consumes `RawMidiEvent` sequences with timestamps.
* Emits `NoteSpan`s using `NoteId` allocation.
* Handles overlaps, pedaling, missing note-offs robustly.

**`src/note_store.rs`**
Efficient storage for `NoteSpan`s:

* Sorted by time.
* Query ranges for analysis and engine snapshots.

**`src/event_ring.rs`**
Lock-free-friendly ring buffer for `EngineEvent` / analysis outputs.

**`src/spsc_queue.rs`**
Single-producer single-consumer queue for RT ↔ non-RT communication. No `std::sync`.

**`src/feature_buffer.rs`**
Buffers for intermediate numeric features (chroma, onset strengths, etc.) using `Vec`/`Box`.

**`tests/*`**
Cover tempo mapping correctness, normalization scenarios, queue correctness, no allocation on RT path usage patterns.

---

## `mt-analysis` (`std`)

Algorithms and pluggable strategies.

**`src/lib.rs`**
Re-exports detectors and config. No engine orchestration.

**`src/config.rs`**
`AnalysisConfig`:

* tunings, min confidence, window sizes, latencies,
* selects strategy implementations.

**`src/traits.rs`**
Core analysis traits:

* `NoteDetector`
* `ChordDetector`
* `KeyDetector`
* `TempoMeterDetector`
* `SwingDetector`
* `Segmenter`
  All pure and testable.

**`src/midi_note_detector.rs`**
Implementation of `NoteDetector` for normalized MIDI/NoteSpan input; mainly pass-through/cleanup.

**`src/audio_note_detector.rs`**
Implementation of `NoteDetector` for audio:

* Uses `mt-signal-core` traits.
* Outputs candidate `NoteSpan`s.

**`src/chord_detector/mod.rs`**
Orchestrates chord detection strategies.

**`src/chord_detector/rule_based.rs`**
Deterministic chord detection from `NoteSpan`s using `ChordKindId` tables.

**`src/chord_detector/template_matching.rs`**
Alternative detector using pitch-class templates; selected via config.

**`src/key_detector/mod.rs`**
Key detection façade.

**`src/key_detector/histogram.rs`**
Histogram-based key detection (K-S, etc.) emitting `KeyEvent`s.

**`src/tempo_meter_detector.rs`**
Estimates tempo and meter from onset/energy features, emits `TempoEvent`/`MeterEvent`.

**`src/swing_detector.rs`**
Estimates swing ratio from micro-timing.

**`src/segmenter/mod.rs`**
Segmenter façade.

**`src/segmenter/energy_segmenter.rs`**
Segments by energy / novelty curve.

**`src/segmenter/harmonic_segmenter.rs`**
Segments by chord / key changes.

**`src/postprocess.rs`**
Smoothing, hysteresis, pruning low-confidence events.

**`src/confidence.rs`**
Shared confidence scoring utilities.

**`tests/*`**
Golden cases:

* known MIDI snippets → expected chords/keys.
* synthetic audio → stable estimates.

---

## `mt-engine` (`std`)

Streaming engine and pipeline.

**`src/lib.rs`**
Public entry. Constructs `EngineRt` + `EngineSession`. Re-exports types.

**`src/types.rs`**
`EngineEvent`, `EngineSnapshot`, small engine-specific DTOs wrapping `mt-core` types.

**`src/config.rs`**
`EngineConfig`:

* connects analysis config with engine-level options (latency budget, history length, enabled analyzers).

**`src/engine_rt.rs`**
`EngineRt`:

* RT-safe API:

  * `push_audio_block(...)`
  * `push_midi(...)`
* Writes into `mt-alloc` SPSC queues / rings. No allocation, no logging, no locks.

**`src/engine_session.rs`**
`EngineSession`:

* Lives on non-RT thread.
* Pulls from queues.
* Invokes `mt-analysis` detectors via the static pipeline.
* Maintains state for snapshots and event history.

**`src/pipeline.rs`**
Static skeleton:

* Hard-wired order:

  * ingest → note normalization → chord/key → structure.
* Slots for pluggable strategy implementations.

**`src/event_bus.rs`**
Central place that collects `EngineEvent`s from pipeline stages and pushes into `event_ring` for consumers.

**`src/snapshot.rs`**
Builds consistent `EngineSnapshot` views:

* note grid, current chord/key, recent segments.

**`src/validate.rs`**
Runtime invariants:

* RT/non-RT use checks in debug builds.
* Config sanity checks.

**`src/logging.rs`**
Optional non-RT logging hooks. Never used in RT code.

**`tests/*`**
Integration tests:

* assert no allocations on RT hot path (custom harness).
* end-to-end: feed MIDI/audio, check produced events.

---

## `mt-ffi` (`std`)

Stable C ABI around `mt-engine`.

**`src/lib.rs`**
Exports FFI surface.

**`src/version.rs`**
ABI version constants and helpers.

**`src/error.rs`**
FFI error codes, no panics across boundary.

**`src/types.rs`**
C-compatible structs:

* `mt-chord_event`, `mt-key_event`, etc.
  Only POD and fixed-size types.

**`src/engine_handle.rs`**
Opaque pointer management:

* create/destroy `EngineRt` / `EngineSession` pairs.
* lifecycle and ownership.

**`src/api.rs`**
Extern "C" functions:

* init, configure
* RT push calls
* polling / snapshot retrieval
  Uses only `types.rs` and `engine_handle.rs`.

**`tests/*`**
FFI roundtrip tests via `libloading`:

* create engine, feed sample data, read outputs.

---

## `mt-cli` (`std`, bin)

Tooling and demos.

**`src/main.rs`**
Argument parsing and dispatch to subcommands.

**`src/commands/mod.rs`**
Command registration.

**`src/commands/analyze.rs`**
Runs engine on given file(s), prints chords/keys/segments.

**`src/commands/dump_timeline.rs`**
Exports events as JSON/CSV for inspection.

**`src/commands/benchmark.rs`**
Benchmarks pipeline throughput and latency.

**`src/commands/validate.rs`**
Runs invariants, schema checks, regression suites.

**`tests/*`**
CLI smoke tests and golden-output tests.

---

## `mt-semantic` (optional advanced layer)

On top of core events only.

**`src/lib.rs`**
Exports semantic helpers.

**`src/motif.rs`**
Motif extraction from `NoteSpan`s and `SegmentEvent`s.

**`src/voice_leading.rs`**
Voice-leading analysis between `ChordEvent`s.

**`src/functional_harmony.rs`**
Maps chords to functions (T/S/D) relative to keys.

**`src/graph.rs`**
Graph view over motifs, sections, functions.

---

If a specific file’s purpose still feels underspecified, point to it and I will refine until it is implementation-ready.
