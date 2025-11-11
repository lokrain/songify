//! Key representation (tonic + mode).
//!
//! Detection logic lives in mt-analysis; this is purely structural.

use core::fmt;

use crate::{error::TheoryError, pitch::PitchClass};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum KeyMode {
    Major,
    Minor,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Key {
    tonic: PitchClass,
    mode: KeyMode,
}

impl Key {
    #[must_use]
    pub const fn new(tonic: PitchClass, mode: KeyMode) -> Self {
        Self { tonic, mode }
    }

    #[must_use]
    pub const fn tonic(self) -> PitchClass {
        self.tonic
    }

    #[must_use]
    pub const fn mode(self) -> KeyMode {
        self.mode
    }

    /// Helper from raw semitone index and minor flag. Used by detectors.
    pub fn from_semitone(pc: u8, minor: bool) -> Result<Self, TheoryError> {
        let tonic = PitchClass::new(pc)?;
        Ok(Self {
            tonic,
            mode: if minor {
                KeyMode::Minor
            } else {
                KeyMode::Major
            },
        })
    }
}

impl fmt::Display for KeyMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            KeyMode::Major => write!(f, "maj"),
            KeyMode::Minor => write!(f, "min"),
        }
    }
}

impl fmt::Display for Key {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.tonic, self.mode)
    }
}
