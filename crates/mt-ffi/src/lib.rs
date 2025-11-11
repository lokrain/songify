//! mt-ffi
//!
//! Stable C ABI for the Music Theory & Analytics engine.
//!
//! Design:
//! - Opaque engine handle managed via create/destroy calls.
//! - Host pushes audio and MIDI into the engine.
//! - Host pulls analyzed events into caller-owned buffers.
//! - No panics cross FFI; all calls wrapped in `catch_unwind`.
//! - All functions return `mt_ffi_status` for predictable error handling.
//!
//! This crate defines the ABI contract for mt-engine.

#![deny(clippy::all, clippy::pedantic)]
#![allow(
    clippy::missing_errors_doc,
    clippy::missing_safety_doc,
    clippy::module_name_repetitions
)]

extern crate alloc;

mod version;
mod error;
mod types;
mod engine_handle;
mod api;

pub use crate::{
    error::MtFfiStatus,
    types::{
        MtChordEvent, MtKeyEvent, MtMidiEvent, MtNoteEvent, MtSegmentEvent,
        MT_ABI_VERSION,
    },
};
