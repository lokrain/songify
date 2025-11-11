//! Functional harmony classification.
//!
//! Maps a chord in a given key to T (tonic), S (subdominant), D (dominant), or Other.
//!
//! Rules (simplified, deterministic, well-documented):
//! - Compute scale-degree of chord root: (root_pc - key_tonic) mod 12.
//! - Tonic family: I, vi, iii
//! - Subdominant family: ii, IV
//! - Dominant family: V, vii°
//! - Qualities (maj/min/dim/dom7) refine mapping where ambiguous.
//!
//! This is intentionally conservative and explainable.

use mt_core::chord::Chord;
use mt_core::chord_kind::ChordKindId;
use mt_core::key::Key;
use mt_core::pitch::PitchClass;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Function {
    Tonic,
    Subdominant,
    Dominant,
    Other,
}

fn degree(tonic: PitchClass, root: PitchClass) -> u8 {
    let t = tonic.as_u8();
    let r = root.as_u8();
    (r + 12 - t) % 12
}

/// Classify chord function relative to key using deterministic rules.
#[must_use]
pub fn classify_function(key: Key, chord: Chord) -> Function {
    let deg = degree(key.tonic(), chord.root);

    match deg {
        // I, vi, iii area -> tonic functions
        0 | 9 | 4 => Function::Tonic,

        // V and vii° area -> dominant functions
        7 | 11 => Function::Dominant,

        // ii and IV area -> subdominant functions
        2 | 5 => Function::Subdominant,

        _ => {
            // Refine some common altered dominants/subV cases:
            match chord.kind {
                ChordKindId::Dom7 => {
                    // Treat any dom7 whose root is tritone from tonic as dominant.
                    let tritone = (key.tonic().as_u8() + 6) % 12;
                    if chord.root.as_u8() == tritone {
                        Function::Dominant
                    } else {
                        Function::Other
                    }
                }
                _ => Function::Other,
            }
        }
    }
}
