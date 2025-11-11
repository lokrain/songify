# mt-ffi

Stable C ABI layer for the Music Theory & Analytics engine.

Goals:

- Provide a compact, versioned interface for native hosts (C, C++, Rust FFI, Swift, etc.).
- Expose engine capabilities without leaking internal Rust types or layouts.
- Enforce:
  - no panics across FFI,
  - explicit status codes,
  - explicit ABI versioning,
  - deterministic behavior for a given engine build.

Key concepts:

- `mt-engine_handle` — opaque handle wrapping a single `Engine` instance.
- Functions to:
  - create / destroy engine,
  - push audio (f32 interleaved),
  - push MIDI events,
  - query analyzed events into caller-provided buffers.
- Data types:
  - `mt-chord_event`
  - `mt-key_event`
  - `mt-segment_event`
  - `mt-note_event`
  - `mt-midi_event`
  - `mt-ffi_status`

All types are `#[repr(C)]`, POD, and safe to use across language boundaries.

See `src/types.rs` and `src/api.rs` for the exact ABI.
