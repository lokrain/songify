//! Canonical chord kind catalog.
//!
//! Each kind is (id, name, intervals). Intervals are semitone offsets from root.

use crate::pitch::PitchClass;

/// Stable identifiers for supported chord kinds.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ChordKindId {
    Maj,
    Min,
    Dim,
    Aug,
    Sus2,
    Sus4,
    Power5,
    Maj7,
    Min7,
    Dom7,
    HalfDim7,
    Dim7,
    Maj6,
    Min6,
    SixNine,
}

/// Descriptor for a chord kind.
#[derive(Clone, Copy, Debug)]
pub struct ChordKind {
    pub id: ChordKindId,
    pub name: &'static str,
    /// Semitone offsets from root, must include 0.
    pub intervals: &'static [u8],
}

/// Static catalog of chord kinds.
///
/// This is intentionally compact; extended tensions / alterations can be modeled
/// as separate kinds or in higher layers.
pub const CHORD_KINDS: &[ChordKind] = &[
    ChordKind {
        id: ChordKindId::Maj,
        name: "maj",
        intervals: &[0, 4, 7],
    },
    ChordKind {
        id: ChordKindId::Min,
        name: "min",
        intervals: &[0, 3, 7],
    },
    ChordKind {
        id: ChordKindId::Dim,
        name: "dim",
        intervals: &[0, 3, 6],
    },
    ChordKind {
        id: ChordKindId::Aug,
        name: "aug",
        intervals: &[0, 4, 8],
    },
    ChordKind {
        id: ChordKindId::Sus2,
        name: "sus2",
        intervals: &[0, 2, 7],
    },
    ChordKind {
        id: ChordKindId::Sus4,
        name: "sus4",
        intervals: &[0, 5, 7],
    },
    ChordKind {
        id: ChordKindId::Power5,
        name: "5",
        intervals: &[0, 7],
    },
    ChordKind {
        id: ChordKindId::Maj7,
        name: "maj7",
        intervals: &[0, 4, 7, 11],
    },
    ChordKind {
        id: ChordKindId::Min7,
        name: "min7",
        intervals: &[0, 3, 7, 10],
    },
    ChordKind {
        id: ChordKindId::Dom7,
        name: "7",
        intervals: &[0, 4, 7, 10],
    },
    ChordKind {
        id: ChordKindId::HalfDim7,
        name: "m7b5",
        intervals: &[0, 3, 6, 10],
    },
    ChordKind {
        id: ChordKindId::Dim7,
        name: "dim7",
        intervals: &[0, 3, 6, 9],
    },
    ChordKind {
        id: ChordKindId::Maj6,
        name: "6",
        intervals: &[0, 4, 7, 9],
    },
    ChordKind {
        id: ChordKindId::Min6,
        name: "min6",
        intervals: &[0, 3, 7, 9],
    },
    ChordKind {
        id: ChordKindId::SixNine,
        name: "6/9",
        intervals: &[0, 4, 7, 9, 14],
    },
];

/// Returns the semitone intervals for a chord kind.
#[must_use]
pub fn chord_intervals(kind: ChordKindId) -> &'static [u8] {
    CHORD_KINDS
        .iter()
        .find(|k| k.id == kind)
        .map(|k| k.intervals)
        // Fallback is theoretically unreachable if ids and table are consistent.
        .unwrap_or(&[0])
}

/// Compute chord tones for a given root/kind into a fixed array.
///
/// Only the first `chord_intervals(kind).len()` entries are meaningful.
/// The rest are unspecified and should be ignored.
#[must_use]
pub fn chord_tones(root: PitchClass, kind: ChordKindId) -> [PitchClass; 8] {
    let intervals = chord_intervals(kind);
    let mut out = [root; 8];
    let mut i = 0;
    while i < intervals.len() && i < out.len() {
        out[i] = root.transpose(intervals[i] as i8);
        i += 1;
    }
    out
}
