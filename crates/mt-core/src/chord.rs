//! Chord representation.
//!
//! This type is used as a stable, minimal chord descriptor for events
//! and APIs. Inversions and slash chords are supported via `bass`.

use core::fmt;

use crate::{
    chord_kind::{ChordKindId, chord_intervals},
    error::TheoryError,
    pitch::PitchClass,
};

/// Canonical chord: root + kind + optional bass (for inversions/slash).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Chord {
    pub root: PitchClass,
    pub kind: ChordKindId,
    pub bass: Option<PitchClass>,
}

impl Chord {
    /// Creates a chord; validates that bass (if present) is one of the chord tones.
    pub fn new(
        root: PitchClass,
        kind: ChordKindId,
        bass: Option<PitchClass>,
    ) -> Result<Self, TheoryError> {
        if let Some(bass_pc) = bass {
            let intervals = chord_intervals(kind);
            let mut is_member = false;
            for iv in intervals {
                if root.transpose(*iv as i8) == bass_pc {
                    is_member = true;
                    break;
                }
            }
            if !is_member {
                return Err(TheoryError::InvalidChord);
            }
        }

        Ok(Self { root, kind, bass })
    }
}

impl fmt::Display for Chord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use ChordKindId::*;
        let root = self.root;
        let suffix = match self.kind {
            Maj => "",
            Min => "m",
            Dim => "dim",
            Aug => "aug",
            Sus2 => "sus2",
            Sus4 => "sus4",
            Power5 => "5",
            Maj7 => "maj7",
            Min7 => "m7",
            Dom7 => "7",
            HalfDim7 => "m7b5",
            Dim7 => "dim7",
            Maj6 => "6",
            Min6 => "m6",
            SixNine => "6/9",
        };
        if let Some(bass) = self.bass {
            write!(f, "{root}{suffix}/{bass}")
        } else {
            write!(f, "{root}{suffix}")
        }
    }
}
