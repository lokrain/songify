//! C-compatible types exposed by the ABI.
//!
//! All types are POD and #[repr(C)] so they can be safely used from C.

use core::ffi::c_uchar;

use mt_core::{
    events::{ChordEvent, KeyEvent, NoteEvent, SegmentEvent, SegmentKind},
    midi::{MidiChannel, MidiEvent, MidiEventKind},
    pitch::MidiNote,
    time::SampleTime,
};

pub use crate::version::mt-ABI_VERSION;

/// Opaque handle to an engine instance (owned by Rust).
///
/// In C:
/// ```c
/// typedef struct mt-engine_handle mt-engine_handle;
/// ```
#[repr(C)]
pub struct MtEngineHandle {
    _private: [u8; 0],
}

/// C mirror of `MidiEvent`.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct MtMidiEvent {
    pub channel: c_uchar,
    pub kind: c_uchar,
    pub data1: c_uchar,
    pub data2: c_uchar,
}

impl From<MtMidiEvent> for MidiEvent {
    fn from(e: MtMidiEvent) -> Self {
        let channel = MidiChannel::new(e.channel).unwrap_or_else(|_| MidiChannel::new(0).unwrap());
        let kind = match e.kind {
            0 => MidiEventKind::NoteOn,
            1 => MidiEventKind::NoteOff,
            2 => MidiEventKind::ControlChange,
            3 => MidiEventKind::ProgramChange,
            4 => MidiEventKind::PitchBend,
            _ => MidiEventKind::Other,
        };
        MidiEvent {
            channel,
            kind,
            data1: e.data1,
            data2: e.data2,
        }
    }
}

/// C mirror of `NoteEvent` (flattened).
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct MtNoteEvent {
    pub id: u32,
    pub track: u16,
    pub onset_samples: i64,
    pub offset_samples: i64,
    pub midi_note: c_uchar,
    pub velocity: c_uchar,
}

impl From<&NoteEvent> for MtNoteEvent {
    fn from(n: &NoteEvent) -> Self {
        Self {
            id: n.id.0,
            track: n.track.0,
            onset_samples: n.onset.value(),
            offset_samples: n.offset.value(),
            midi_note: n.note.value(),
            velocity: n.velocity,
        }
    }
}

/// C mirror of `ChordEvent`.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct MtChordEvent {
    pub onset_samples: i64,
    pub offset_samples: i64,
    pub root_pc: c_uchar,
    pub kind_id: c_uchar,
    pub bass_pc: c_uchar, // 255 = none
    pub confidence_x1000: u16,
}

impl From<&ChordEvent> for MtChordEvent {
    fn from(e: &ChordEvent) -> Self {
        Self {
            onset_samples: e.onset.value(),
            offset_samples: e.offset.value(),
            root_pc: e.chord.root.as_u8(),
            kind_id: e.chord.kind as u8,
            bass_pc: e.chord.bass.map(|pc| pc.as_u8()).unwrap_or(u8::MAX),
            confidence_x1000: e.confidence_x1000,
        }
    }
}

/// C mirror of `KeyEvent`.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct MtKeyEvent {
    pub position_samples: i64,
    pub tonic_pc: c_uchar,
    pub is_minor: c_uchar,
    pub confidence_x1000: u16,
}

impl From<&KeyEvent> for MtKeyEvent {
    fn from(e: &KeyEvent) -> Self {
        Self {
            position_samples: e.position.value(),
            tonic_pc: e.key.tonic().as_u8(),
            is_minor: match e.key.mode() {
                mt_core::key::KeyMode::Major => 0,
                mt_core::key::KeyMode::Minor => 1,
            },
            confidence_x1000: e.confidence_x1000,
        }
    }
}

/// C mirror of `SegmentEvent`.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct MtSegmentEvent {
    pub onset_samples: i64,
    pub offset_samples: i64,
    pub kind: c_uchar,
    pub custom_kind_id: c_uchar,
    pub confidence_x1000: u16,
}

impl From<&SegmentEvent> for MtSegmentEvent {
    fn from(e: &SegmentEvent) -> Self {
        let (kind, custom) = match e.kind {
            SegmentKind::Intro => (0, 0),
            SegmentKind::Verse => (1, 0),
            SegmentKind::Chorus => (2, 0),
            SegmentKind::Bridge => (3, 0),
            SegmentKind::Solo => (4, 0),
            SegmentKind::Outro => (5, 0),
            SegmentKind::Other(id) => (100, id),
        };

        Self {
            onset_samples: e.onset.value(),
            offset_samples: e.offset.value(),
            kind,
            custom_kind_id: custom,
            confidence_x1000: e.confidence_x1000,
        }
    }
}
