//! Note events normalized from MIDI or audio.

use crate::{pitch::MidiNote, time::SampleTime, traits::HasPosition};

/// Logical track identifier (e.g., MIDI channel, stem index).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct TrackId(pub u16);

/// Stable note identifier within a session/project.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct NoteId(pub u32);

/// Normalized note event with onset/offset.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NoteEvent {
    pub id: NoteId,
    pub track: TrackId,
    pub onset: SampleTime,
    pub offset: SampleTime,
    pub note: MidiNote,
    pub velocity: u8,
}

impl HasPosition for NoteEvent {
    fn position(&self) -> SampleTime {
        self.onset
    }
}
