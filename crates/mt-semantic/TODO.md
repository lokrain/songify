# TODO — mt-semantic

## Motif analysis

- [ ] Add duration- and rhythm-aware motif matching (currently interval-only).
- [ ] Implement transposition-invariant motif clustering across keys.
- [ ] Provide scoring heuristics for motif significance (coverage, density) and expose thresholds in config.
- [ ] Build golden tests using annotated MIDI excerpts to validate motif discovery correctness.

## Voice leading & harmony

- [ ] Extend `compute_voice_leading` with voice assignment heuristics (avoid unrealistic jumps).
- [ ] Model contrary/oblique motion metrics and surface them to callers.
- [ ] Add support for poly-chord and slash-chord functional analysis in `functional_harmony`.
- [ ] Incorporate key-aware diatonic function fallback when chord kind is unknown.

## Semantic graph

- [ ] Optimise graph structure for large projects (arena allocator, adjacency compression).
- [ ] Introduce pathfinding utilities (e.g., find motif transitions) with deterministic ordering.
- [ ] Provide serialization/deserialization under feature flag for exporting to tooling.
- [ ] Add dot/mermaid exporters for visualisation in docs.

## Tooling & docs

- [ ] Document interplay between mt-core events and semantic layers (expected preconditions).
- [ ] Offer examples that build a semantic graph from a short chord progression.
- [ ] Create benchmarks to ensure motif discovery scales to multi-minute pieces.
- [ ] Add glossary of theoretical terms referenced by the crate.
