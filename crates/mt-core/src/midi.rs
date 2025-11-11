//! Minimal MIDI primitives for semantic use.
//!
//! Parsing and file I/O live in higher layers. Here we only define
//! stable, POD-like representations.

use core::fmt;

use crate::{error::TheoryError, pitch::MidiNote};

/// MIDI channel 0..15.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct MidiChannel(u8);

impl MidiChannel {
    pub const fn new(ch: u8) -> Result<Self, TheoryError> {
        if ch < 16 {
            Ok(Self(ch))
        } else {
            Err(TheoryError::InvalidMidiChannel(ch))
        }
    }

    #[must_use]
    pub const fn value(self) -> u8 {
        self.0
    }
}

/// Coarse classification of MIDI event kind.
///
/// Detailed decoding is performed upstream; this is enough for note tracking
/// and basic control routing in analysis.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum MidiEventKind {
    NoteOn,
    NoteOff,
    ControlChange,
    ProgramChange,
    PitchBend,
    Other,
}

/// Compact MIDI event view.
///
/// `data1` / `data2` interpretation depends on `kind`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct MidiEvent {
    pub channel: MidiChannel,
    pub kind: MidiEventKind,
    pub data1: u8,
    pub data2: u8,
}

impl MidiEvent {
    #[must_use]
    pub const fn note_on(channel: MidiChannel, note: MidiNote, velocity: u8) -> Self {
        Self {
            channel,
            kind: MidiEventKind::NoteOn,
            data1: note.value(),
            data2: velocity,
        }
    }

    #[must_use]
    pub const fn note_off(channel: MidiChannel, note: MidiNote, velocity: u8) -> Self {
        Self {
            channel,
            kind: MidiEventKind::NoteOff,
            data1: note.value(),
            data2: velocity,
        }
    }
}

impl fmt::Display for MidiEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "ch{} {:?} {} {}",
            self.channel.value(),
            self.kind,
            self.data1,
            self.data2
        )
    }
}
