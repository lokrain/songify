//! mt-core
//!
//! This crate defines the **pure semantic kernel** for the system:
//! - Pitches, intervals, scales
//! - Chord kinds and chords
//! - Keys
//! - MIDI primitives
//! - Timeline events (tempo, meter, notes, chords, keys, segments)
//! - Tiny shared traits for position and confidence
//!
//! Design constraints:
//! - `no_std` capable (default if `std` feature is off).
//! - No heap allocations.
//! - No I/O, logging, or randomness.
//! - No panics for normal invalid input (use `TheoryError`).
//! - Stable enums/IDs for cross-crate references.
//! - Deterministic behavior only.

#![cfg_attr(not(feature = "std"), no_std)]
#![forbid(unsafe_code)]
#![deny(clippy::all, clippy::pedantic)]
#![allow(
    clippy::module_name_repetitions,
    clippy::missing_errors_doc,
    clippy::missing_const_for_fn,
    clippy::must_use_candidate,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::cast_possible_wrap
)]

#[cfg(feature = "std")]
extern crate std;

pub mod chord;
pub mod chord_kind;
pub mod error;
pub mod events;
pub mod interval;
pub mod key;
pub mod midi;
pub mod pitch;
pub mod scale;
pub mod time;
pub mod traits;

// Common re-exports for convenience in other crates.
pub use crate::{
    chord::Chord,
    chord_kind::{CHORD_KINDS, ChordKind, ChordKindId},
    error::TheoryError,
    interval::{Interval, IntervalClass, IntervalQuality},
    key::{Key, KeyMode},
    midi::{MidiChannel, MidiEvent, MidiEventKind},
    pitch::{Accidental, Letter, MidiNote, PITCH_CLASS_COUNT, PitchClass, SpelledPitchClass},
    time::{MusicalPosition, SampleTime},
};
