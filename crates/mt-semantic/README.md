# mt-semantic

Higher-level musical semantics for the Music Theory & Analytics workspace.

Built on `mt-core` primitives, this crate provides:

- Motif discovery over note sequences.
- Voice-leading evaluation between chords.
- Functional harmony classification (T/S/D/Other) per chord in key context.
- A compact semantic graph model tying motifs, harmony, and segments together.

Design:

- Deterministic and pure.
- `no_std` + `alloc` friendly.
- No audio I/O, no file I/O, no randomness.
- No engine/runtime concerns (that is `mt-engine`).
- Stable data structures suitable for tests, visualization, and downstream tools.

This crate does **not** guess; it encodes explicit, documented rules and algorithms that can be tested and versioned.
