# TODO — mt-engine

## API contract

- [ ] Align `types.rs` with the public contract expected by mt-cli (EngineEvent enums, AnalyzeRequest/Response structs, rich EngineError hierarchy).
- [ ] Implement real `api::analyze_offline` flow that wires EngineConfig, Pipeline build, OfflineSession, and result collection.
- [ ] Provide `validate_offline` wrapper that exercises pipeline validation without running analysis.
- [ ] Expose semantic version constant and embed in responses for CLI to print.

## Pipeline runtime

- [ ] Finish `pipeline` module: enforce typed edges, add node registry population, and ensure deterministic execution order.
- [ ] Implement fan-out/fan-in semantics so nodes can receive multiple inputs (currently only basic pending map).
- [ ] Add streaming back-pressure handling for long pipelines (bounded queues, chunking strategy).
- [ ] Support value recycling or arenas to reduce allocation churn for high-throughput analysis.

## Sessions & I/O

- [ ] Implement `OfflineSession` (ingest audio/MIDI files, maintain deterministic ordering, produce EngineEvents).
- [ ] Add audio loader abstractions (WAV/AIFF/FLAC) with deterministic resampling to engine sample rate.
- [ ] Provide MIDI loader that leverages `mt-alloc::MidiNormalizer` and respects tempo maps.
- [ ] Allow project-root scoped caching (e.g., intermediate feature blobs) with opt-in eviction.

## Validation & diagnostics

- [ ] Flesh out `validate` module to check node graph, catalog versions, and configuration invariants.
- [ ] Surface structured diagnostics (file path, line, hint) instead of plain strings in EngineError.
- [ ] Add logging hooks (behind feature flag) for debugging pipeline execution traces.
- [ ] Integrate metrics counters for node timings, queued events, and memory usage.

## Testing & tooling

- [ ] Create golden end-to-end tests that run minimal pipeline on fixture audio/MIDI and diff JSON outputs.
- [ ] Add fuzz tests for pipeline builder (random DAGs respecting constraints) to ensure no panics.
- [ ] Provide criterion benchmarks for pipeline throughput and memory footprint.
- [ ] Document developer workflow (how to add a new node, register it, and expose via CLI/FFI).
