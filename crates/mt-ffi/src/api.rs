//! Public FFI functions.
//!
//! All functions:
//! - are `extern "C"`,
//! - validate pointers and sizes,
//! - catch panics via `catch_unwind`,
//! - never unwind across FFI,
//! - return `mt-ffi_status`.

use core::ffi::c_void;
use core::slice;
use std::panic::catch_unwind;

use mt_core::midi::MidiEvent;
use mt_core::pitch::MidiNote;

use crate::engine_handle::{EngineHandle, from_raw_handle, into_box};
use crate::error::MtFfiStatus;
use crate::types::{
    MtChordEvent, MtEngineHandle, MtKeyEvent, MtMidiEvent, MtNoteEvent, MtSegmentEvent,
};

/// Helper: wrap a closure and map panics to mt-FFI_ERROR_PANIC.
fn guard<F>(f: F) -> MtFfiStatus
where
    F: FnOnce() -> MtFfiStatus + std::panic::UnwindSafe,
{
    match catch_unwind(f) {
        Ok(status) => status,
        Err(_) => MtFfiStatus::MtFfiErrorPanic,
    }
}

#[no_mangle]
pub extern "C" fn mt-engine_create_default(handle_out: *mut *mut MtEngineHandle) -> MtFfiStatus {
    guard(|| unsafe {
        if handle_out.is_null() {
            return MtFfiStatus::MtFfiErrorNull;
        }

        match EngineHandle::new_default() {
            Ok(inner) => {
                let boxed = Box::new(inner);
                let raw = Box::into_raw(boxed) as *mut MtEngineHandle;
                *handle_out = raw;
                MtFfiStatus::MtFfiOk
            }
            Err(_) => MtFfiStatus::MtFfiErrorEngine,
        }
    })
}

#[no_mangle]
pub extern "C" fn mt-engine_destroy(handle: *mut MtEngineHandle) -> MtFfiStatus {
    guard(|| unsafe {
        if handle.is_null() {
            return MtFfiStatus::MtFfiErrorNull;
        }
        // Drop the box; this drops the EngineHandle and Engine inside.
        let _ = into_box(handle);
        MtFfiStatus::MtFfiOk
    })
}

/// Push interleaved f32 audio into the engine.
///
/// - `sample_rate`: Hz
/// - `channels`: number of channels
/// - `frames`: number of frames; total samples = frames * channels
/// - `data`: pointer to interleaved samples
#[no_mangle]
pub extern "C" fn mt-engine_push_audio_f32_interleaved(
    handle: *mut MtEngineHandle,
    sample_rate: u32,
    channels: u16,
    frames: u32,
    data: *const f32,
) -> MtFfiStatus {
    guard(|| unsafe {
        if handle.is_null() || data.is_null() || channels == 0 || frames == 0 {
            return MtFfiStatus::MtFfiErrorInvalidArg;
        }

        let engine_handle = match from_raw_handle(handle) {
            Some(h) => h,
            None => return MtFfiStatus::MtFfiErrorNull,
        };

        // Safe because caller guarantees buffer of `frames * channels` floats.
        let sample_count = frames as usize * channels as usize;
        let slice = slice::from_raw_parts(data, sample_count);

        let mut guard = match engine_handle.engine.lock() {
            Ok(g) => g,
            Err(_) => return MtFfiStatus::MtFfiErrorEngine,
        };

        match guard.push_audio_f32_interleaved(sample_rate, channels, frames, slice) {
            Ok(()) => MtFfiStatus::MtFfiOk,
            Err(_) => MtFfiStatus::MtFfiErrorEngine,
        }
    })
}

/// Push MIDI events into the engine.
///
/// `events` points to `count` mt-midi_event structures.
#[no_mangle]
pub extern "C" fn mt-engine_push_midi(
    handle: *mut MtEngineHandle,
    events: *const MtMidiEvent,
    count: u32,
) -> MtFfiStatus {
    guard(|| unsafe {
        if handle.is_null() || events.is_null() {
            return MtFfiStatus::MtFfiErrorInvalidArg;
        }

        let engine_handle = match from_raw_handle(handle) {
            Some(h) => h,
            None => return MtFfiStatus::MtFfiErrorNull,
        };

        let slice = slice::from_raw_parts(events, count as usize);

        let mut midi = alloc::vec::Vec::with_capacity(slice.len());
        for e in slice {
            midi.push(MidiEvent::from(*e));
        }

        let mut guard = match engine_handle.engine.lock() {
            Ok(g) => g,
            Err(_) => return MtFfiStatus::MtFfiErrorEngine,
        };

        match guard.push_midi_events(&midi) {
            Ok(()) => MtFfiStatus::MtFfiOk,
            Err(_) => MtFfiStatus::MtFfiErrorEngine,
        }
    })
}

/// Copy analyzed note events into caller-provided buffer.
///
/// Writes at most `buffer_len` entries and sets `out_len` to the number written.
#[no_mangle]
pub extern "C" fn mt-engine_get_note_events(
    handle: *mut MtEngineHandle,
    buffer: *mut MtNoteEvent,
    buffer_len: u32,
    out_len: *mut u32,
) -> MtFfiStatus {
    guard(|| unsafe {
        if handle.is_null() || buffer.is_null() || out_len.is_null() {
            return MtFfiStatus::MtFfiErrorNull;
        }

        let engine_handle = match from_raw_handle(handle) {
            Some(h) => h,
            None => return MtFfiStatus::MtFfiErrorNull,
        };

        let guard = match engine_handle.engine.lock() {
            Ok(g) => g,
            Err(_) => return MtFfiStatus::MtFfiErrorEngine,
        };

        let events = guard.note_events();
        let max_copy = core::cmp::min(buffer_len as usize, events.len());
        let out_slice = slice::from_raw_parts_mut(buffer, max_copy);

        for (dst, src) in out_slice.iter_mut().zip(events.iter()) {
            *dst = MtNoteEvent::from(src);
        }

        *out_len = max_copy as u32;
        MtFfiStatus::MtFfiOk
    })
}

/// Chord events.
#[no_mangle]
pub extern "C" fn mt-engine_get_chord_events(
    handle: *mut MtEngineHandle,
    buffer: *mut MtChordEvent,
    buffer_len: u32,
    out_len: *mut u32,
) -> MtFfiStatus {
    guard(|| unsafe {
        if handle.is_null() || buffer.is_null() || out_len.is_null() {
            return MtFfiStatus::MtFfiErrorNull;
        }

        let engine_handle = match from_raw_handle(handle) {
            Some(h) => h,
            None => return MtFfiStatus::MtFfiErrorNull,
        };

        let guard = match engine_handle.engine.lock() {
            Ok(g) => g,
            Err(_) => return MtFfiStatus::MtFfiErrorEngine,
        };

        let events = guard.chord_events();
        let max_copy = core::cmp::min(buffer_len as usize, events.len());
        let out_slice = slice::from_raw_parts_mut(buffer, max_copy);

        for (dst, src) in out_slice.iter_mut().zip(events.iter()) {
            *dst = MtChordEvent::from(src);
        }

        *out_len = max_copy as u32;
        MtFfiStatus::MtFfiOk
    })
}

/// Key events.
#[no_mangle]
pub extern "C" fn mt-engine_get_key_events(
    handle: *mut MtEngineHandle,
    buffer: *mut MtKeyEvent,
    buffer_len: u32,
    out_len: *mut u32,
) -> MtFfiStatus {
    guard(|| unsafe {
        if handle.is_null() || buffer.is_null() || out_len.is_null() {
            return MtFfiStatus::MtFfiErrorNull;
        }

        let engine_handle = match from_raw_handle(handle) {
            Some(h) => h,
            None => return MtFfiStatus::MtFfiErrorNull,
        };

        let guard = match engine_handle.engine.lock() {
            Ok(g) => g,
            Err(_) => return MtFfiStatus::MtFfiErrorEngine,
        };

        let events = guard.key_events();
        let max_copy = core::cmp::min(buffer_len as usize, events.len());
        let out_slice = slice::from_raw_parts_mut(buffer, max_copy);

        for (dst, src) in out_slice.iter_mut().zip(events.iter()) {
            *dst = MtKeyEvent::from(src);
        }

        *out_len = max_copy as u32;
        MtFfiStatus::MtFfiOk
    })
}

/// Segment events.
#[no_mangle]
pub extern "C" fn mt-engine_get_segment_events(
    handle: *mut MtEngineHandle,
    buffer: *mut MtSegmentEvent,
    buffer_len: u32,
    out_len: *mut u32,
) -> MtFfiStatus {
    guard(|| unsafe {
        if handle.is_null() || buffer.is_null() || out_len.is_null() {
            return MtFfiStatus::MtFfiErrorNull;
        }

        let engine_handle = match from_raw_handle(handle) {
            Some(h) => h,
            None => return MtFfiStatus::MtFfiErrorNull,
        };

        let guard = match engine_handle.engine.lock() {
            Ok(g) => g,
            Err(_) => return MtFfiStatus::MtFfiErrorEngine,
        };

        let events = guard.segment_events();
        let max_copy = core::cmp::min(buffer_len as usize, events.len());
        let out_slice = slice::from_raw_parts_mut(buffer, max_copy);

        for (dst, src) in out_slice.iter_mut().zip(events.iter()) {
            *dst = MtSegmentEvent::from(src);
        }

        *out_len = max_copy as u32;
        MtFfiStatus::MtFfiOk
    })
}
