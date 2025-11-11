//! Core error type for semantic validation.
//!
//! Used across mt-core for all "invalid value" scenarios.
//! No allocation, no std::error::Error in no_std mode.

use core::fmt;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TheoryError {
    /// Pitch-class outside 0..12.
    InvalidPitchClass(u8),
    /// MIDI note outside 0..127.
    InvalidMidiNote(u8),
    /// MIDI channel outside 0..15.
    InvalidMidiChannel(u8),
    /// Structurally invalid or unsupported interval.
    InvalidInterval,
    /// Scale pattern definition or lookup failed.
    InvalidScaleDefinition,
    /// Chord kind ID not found in catalog.
    InvalidChordKind,
    /// Chord inconsistent (e.g. slash bass not part of chord).
    InvalidChord,
    /// Invalid key representation.
    InvalidKey,
    /// Invalid or nonsensical time value.
    InvalidTime,
}

impl fmt::Display for TheoryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidPitchClass(v) => write!(f, "invalid pitch-class: {v}"),
            Self::InvalidMidiNote(v) => write!(f, "invalid MIDI note: {v}"),
            Self::InvalidMidiChannel(v) => write!(f, "invalid MIDI channel: {v}"),
            Self::InvalidInterval => write!(f, "invalid interval"),
            Self::InvalidScaleDefinition => write!(f, "invalid scale definition"),
            Self::InvalidChordKind => write!(f, "invalid chord kind"),
            Self::InvalidChord => write!(f, "invalid chord"),
            Self::InvalidKey => write!(f, "invalid key"),
            Self::InvalidTime => write!(f, "invalid time value"),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for TheoryError {}
