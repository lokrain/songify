//! mt-analysis
//!
//! Pure, deterministic analysis algorithms built on top of `mt-core` types.
//! No I/O, no threading, no global state. All orchestration lives in `mt-engine`.
//!
//! This crate exposes:
//! - Configuration (`config`)
//! - Stable analysis traits (`traits`)
//! - Concrete analyzers for:
//!   - MIDI → normalized notes
//!   - Audio → notes (monophonic/simple polyphonic, deterministic)
//!   - Tempo + meter
//!   - Key (histogram-based)
//!   - Chords (template/rule-based over pitch classes)
//!   - Swing feel
//!   - Structural segmentation (energy + harmony)
//! - Confidence scoring and simple post-processing utilities.

#![cfg_attr(not(feature = "std"), no_std)]
#![forbid(unsafe_code)]
#![deny(clippy::all, clippy::pedantic)]
#![allow(
    clippy::module_name_repetitions,
    clippy::missing_errors_doc,
    clippy::missing_const_for_fn,
    clippy::must_use_candidate,
    clippy::cast_possible_truncation,
    clippy::cast_precision_loss,
    clippy::cast_sign_loss,
    clippy::cast_possible_wrap,
    clippy::too_many_arguments
)]

#[cfg(feature = "std")]
extern crate std;

pub mod config;
pub mod traits;
pub mod midi_note_detector;
pub mod audio_note_detector;
pub mod chord_detector;
pub mod key_detector;
pub mod tempo_meter_detector;
pub mod swing_detector;
pub mod segmenter;
pub mod postprocess;
pub mod confidence;

pub use crate::{
    config::AnalysisConfig,
    traits::*,
};
