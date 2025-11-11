//! Interval representations.
//!
//! Two levels:
//! - IntervalClass: semitone distance mod 12.
//! - Interval: diatonic size + quality + semitone count.

use core::fmt;

use crate::error::TheoryError;

/// Interval quality for simple music-theory modeling.
///
/// This is enough for tonal/jazz plus diagnostics. Exotic cases can be
/// handled via tables in higher layers.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum IntervalQuality {
    Perfect,
    Major,
    Minor,
    Augmented,
    Diminished,
}

/// Class of interval: semitone distance modulo 12.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct IntervalClass {
    pub semitones: u8, // 0..=11
}

impl IntervalClass {
    pub const fn new(semitones: u8) -> Result<Self, TheoryError> {
        if semitones < 12 {
            Ok(Self { semitones })
        } else {
            Err(TheoryError::InvalidInterval)
        }
    }
}

/// Full interval descriptor.
///
/// `diatonic`: 1 = unison, 2 = second, etc.
/// `semitones`: signed total semitone distance.
/// No built-in validation against traditional rules; that belongs in analysis.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Interval {
    pub diatonic: u8,
    pub quality: IntervalQuality,
    pub semitones: i8,
}

impl Interval {
    pub const fn new(
        diatonic: u8,
        quality: IntervalQuality,
        semitones: i8,
    ) -> Result<Self, TheoryError> {
        if diatonic == 0 {
            return Err(TheoryError::InvalidInterval);
        }
        Ok(Self {
            diatonic,
            quality,
            semitones,
        })
    }
}

impl fmt::Display for Interval {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let q = match self.quality {
            IntervalQuality::Perfect => "P",
            IntervalQuality::Major => "M",
            IntervalQuality::Minor => "m",
            IntervalQuality::Augmented => "A",
            IntervalQuality::Diminished => "d",
        };
        write!(f, "{q}{}", self.diatonic)
    }
}
