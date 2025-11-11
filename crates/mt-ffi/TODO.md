# TODO — mt-ffi

## ABI surface

- [ ] Finalise C header generation (cbindgen or hand-written) with correct typedefs and version guards.
- [ ] Ensure all exported functions use stable `mt-ffi_status` codes and document the mapping.
- [ ] Add functions for batch event draining (avoid per-event calls across FFI boundary).
- [ ] Provide `mt-engine_config_from_file` to let hosts load pipeline configs without reimplementing parsing.

## Engine handle & lifecycle

- [ ] Update `EngineHandle` to use the new mt-engine offline API (AnalyzeRequest/Response instead of Engine::new).
- [ ] Implement deterministic teardown that flushes outstanding events before drop.
- [ ] Add thread-safe reference counting (Arc) for scenarios where host shares handle across threads.
- [ ] Offer explicit `mt-engine_reset` to clear internal buffers without reallocating.

## Safety & validation

- [ ] Wrap all extern functions in `catch_unwind` to prevent Rust panics from crossing FFI.
- [ ] Add pointer validation utilities to detect null/invalid buffers early.
- [ ] Provide CI build that compiles a C smoke test exercising the API on Linux/macOS/Windows.
- [ ] Run `cargo miri` on FFI boundary tests to catch aliasing or lifetime issues.

## Documentation & examples

- [ ] Publish C and C++ usage examples (simple analyze call, event iteration).
- [ ] Document ABI versioning strategy and compatibility promises in README.
- [ ] Generate doxygen-style docs or Markdown tables describing each struct layout.
- [ ] Provide guidance for JUCE integration (threading expectations, real-time constraints).
