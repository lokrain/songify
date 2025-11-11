# TODO — mt-core

## Theory primitives

- [ ] Extend `ChordKindId` catalogue with altered dominants, quartal voicings, and sus9 variants.
- [ ] Provide utilities to derive tensions/avoid notes for each chord kind.
- [ ] Add `ScaleId` entries for melodic minor modes, bebop scales, and pentatonics.
- [ ] Implement enharmonic resolution helpers that respect key context (e.g., spell Gb major chords correctly).

## Time & events

- [ ] Add arithmetic on `MusicalPosition` (add beats, compare against tempo map resolution).
- [ ] Provide constructors for `ChordEvent`, `KeyEvent`, etc. that enforce onset < offset at compile time.
- [ ] Introduce optional `serde` derives for all event structs under `serde` feature.
- [ ] Offer iterators for traversing event sequences sorted by onset/confidence.

## Validation & invariants

- [ ] Replace debug assertions with checked errors for out-of-order events where feasible.
- [ ] Add property tests ensuring transpose+invert round-trips for chords and intervals.
- [ ] Verify that `PitchClass::transpose` matches theoretical interval arithmetic across all semitone offsets.
- [ ] Provide fuzz tests for `MidiEvent` constructors to ensure invalid channels/notes are rejected.

## Documentation & examples

- [ ] Write module-level docs demonstrating how to compose key/chord helpers for educational tooling.
- [ ] Add doctests for pitch spelling and midi conversion functions.
- [ ] Publish example crate that uses mt-core only (no analysis) for algorithmic composition.
- [ ] Document feature flags (`std`, `serde`) and their impact on API surface.
