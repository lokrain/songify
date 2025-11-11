//! Scale patterns and helpers.
//!
//! This is a static catalog of common patterns.
//! No heap; patterns are references into &'static [u8].

use crate::{error::TheoryError, pitch::PitchClass};

/// Identifiers for built-in scale patterns.
/// This enum is stable and can be persisted.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ScaleId {
    Major,
    NaturalMinor,
    HarmonicMinor,
    MelodicMinorAsc,
    Dorian,
    Phrygian,
    Lydian,
    Mixolydian,
    Locrian,
    WholeTone,
    Chromatic,
}

/// Description of a scale pattern (degree offsets in semitones).
#[derive(Clone, Copy, Debug)]
pub struct ScalePattern {
    pub id: ScaleId,
    pub name: &'static str,
    /// Ascending semitone offsets from tonic, must be sorted, start at 0.
    pub degrees: &'static [u8],
}

/// Static catalog of built-in scale patterns.
pub const SCALE_PATTERNS: &[ScalePattern] = &[
    ScalePattern {
        id: ScaleId::Major,
        name: "Ionian / Major",
        degrees: &[0, 2, 4, 5, 7, 9, 11],
    },
    ScalePattern {
        id: ScaleId::NaturalMinor,
        name: "Natural Minor",
        degrees: &[0, 2, 3, 5, 7, 8, 10],
    },
    ScalePattern {
        id: ScaleId::HarmonicMinor,
        name: "Harmonic Minor",
        degrees: &[0, 2, 3, 5, 7, 8, 11],
    },
    ScalePattern {
        id: ScaleId::MelodicMinorAsc,
        name: "Melodic Minor (asc.)",
        degrees: &[0, 2, 3, 5, 7, 9, 11],
    },
    ScalePattern {
        id: ScaleId::Dorian,
        name: "Dorian",
        degrees: &[0, 2, 3, 5, 7, 9, 10],
    },
    ScalePattern {
        id: ScaleId::Phrygian,
        name: "Phrygian",
        degrees: &[0, 1, 3, 5, 7, 8, 10],
    },
    ScalePattern {
        id: ScaleId::Lydian,
        name: "Lydian",
        degrees: &[0, 2, 4, 6, 7, 9, 11],
    },
    ScalePattern {
        id: ScaleId::Mixolydian,
        name: "Mixolydian",
        degrees: &[0, 2, 4, 5, 7, 9, 10],
    },
    ScalePattern {
        id: ScaleId::Locrian,
        name: "Locrian",
        degrees: &[0, 1, 3, 5, 6, 8, 10],
    },
    ScalePattern {
        id: ScaleId::WholeTone,
        name: "Whole Tone",
        degrees: &[0, 2, 4, 6, 8, 10],
    },
    ScalePattern {
        id: ScaleId::Chromatic,
        name: "Chromatic",
        degrees: &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11],
    },
];

/// Lookup a pattern by id.
#[must_use]
pub fn scale_pattern(id: ScaleId) -> Option<&'static ScalePattern> {
    SCALE_PATTERNS.iter().find(|p| p.id == id)
}

/// Build pitch-classes for a tonic and pattern.
/// Returns `(len, pcs)` so consumers know how many entries are valid.
pub fn build_scale(
    tonic: PitchClass,
    pattern: ScaleId,
) -> Result<(usize, [PitchClass; 12]), TheoryError> {
    let def = scale_pattern(pattern).ok_or(TheoryError::InvalidScaleDefinition)?;

    let mut out = [tonic; 12];
    let mut i = 0usize;
    while i < def.degrees.len() {
        let semis = def.degrees[i] as i8;
        out[i] = tonic.transpose(semis);
        i += 1;
    }
    Ok((def.degrees.len(), out))
}
