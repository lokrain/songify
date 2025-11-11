//! Pitch representations.
//!
//! Goals:
//! - Distinguish *pitch-class* (mod 12) from *MIDI note* (0..127).
//! - Support spelled pitch-classes (letter + accidental) without heap.
//! - Provide safe constructors; use `from_unchecked` only when invariant is proven.

use core::fmt;

use crate::error::TheoryError;

/// Pitch classes are always mod 12.
pub const PITCH_CLASS_COUNT: u8 = 12;

/// Letter name without accidentals.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Letter {
    C,
    D,
    E,
    F,
    G,
    A,
    B,
}

/// Accidental relative to the natural letter.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Accidental {
    DoubleFlat,
    Flat,
    Natural,
    Sharp,
    DoubleSharp,
}

impl Accidental {
    /// Semitone offset applied to the natural letter.
    #[must_use]
    pub const fn semitone_offset(self) -> i8 {
        match self {
            Self::DoubleFlat => -2,
            Self::Flat => -1,
            Self::Natural => 0,
            Self::Sharp => 1,
            Self::DoubleSharp => 2,
        }
    }
}

/// Spelled pitch-class: letter + accidental, no octave.
///
/// Conversion uses simple tonal spelling; complex enharmonic policies belong elsewhere.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct SpelledPitchClass {
    pub letter: Letter,
    pub accidental: Accidental,
}

impl SpelledPitchClass {
    #[must_use]
    pub const fn new(letter: Letter, accidental: Accidental) -> Self {
        Self { letter, accidental }
    }

    /// Maps this spelling to a pitch-class using fixed simple rules.
    /// This is deterministic but not context-aware.
    #[must_use]
    pub fn to_pitch_class(self) -> PitchClass {
        let base = match self.letter {
            Letter::C => 0,
            Letter::D => 2,
            Letter::E => 4,
            Letter::F => 5,
            Letter::G => 7,
            Letter::A => 9,
            Letter::B => 11,
        } as i8;
        let v = base + self.accidental.semitone_offset();
        let wrapped = ((v % 12) + 12) % 12;
        // safe by modular arithmetic
        PitchClass::from_unchecked(wrapped as u8)
    }
}

impl fmt::Display for SpelledPitchClass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Accidental::*;
        use Letter::*;
        let acc = match self.accidental {
            DoubleFlat => "bb",
            Flat => "b",
            Natural => "",
            Sharp => "#",
            DoubleSharp => "x",
        };
        let l = match self.letter {
            C => "C",
            D => "D",
            E => "E",
            F => "F",
            G => "G",
            A => "A",
            B => "B",
        };
        write!(f, "{l}{acc}")
    }
}

/// Pitch-class as integer 0..11.
///
/// Invariant is enforced by constructors, except `from_unchecked` which is
/// for internal, proven-correct usages.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct PitchClass(u8);

impl PitchClass {
    /// Safe constructor; rejects values >= 12.
    pub const fn new(pc: u8) -> Result<Self, TheoryError> {
        if pc < PITCH_CLASS_COUNT {
            Ok(Self(pc))
        } else {
            Err(TheoryError::InvalidPitchClass(pc))
        }
    }

    /// Internal-only helper when the caller has already ensured 0..11.
    #[must_use]
    pub const fn from_unchecked(pc: u8) -> Self {
        Self(pc)
    }

    #[must_use]
    pub const fn as_u8(self) -> u8 {
        self.0
    }

    /// Transpose by a number of semitones modulo 12.
    #[must_use]
    pub const fn transpose(self, semitones: i8) -> Self {
        let v = self.0 as i16 + semitones as i16;
        let mut m = v % 12;
        if m < 0 {
            m += 12;
        }
        Self(m as u8)
    }
}

impl fmt::Display for PitchClass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Default sharp-oriented spelling for debug/UI; not theory-perfect.
        let s = match self.0 {
            0 => "C",
            1 => "C#",
            2 => "D",
            3 => "D#",
            4 => "E",
            5 => "F",
            6 => "F#",
            7 => "G",
            8 => "G#",
            9 => "A",
            10 => "A#",
            11 => "B",
            _ => "?", // invariant prevents this branch
        };
        write!(f, "{s}")
    }
}

/// MIDI note number 0..127.
///
/// Encodes absolute pitch (including octave) in equal temperament.
/// Higher layers interpret according to tuning if needed.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct MidiNote(u8);

impl MidiNote {
    pub const fn new(value: u8) -> Result<Self, TheoryError> {
        if value <= 127 {
            Ok(Self(value))
        } else {
            Err(TheoryError::InvalidMidiNote(value))
        }
    }

    #[must_use]
    pub const fn value(self) -> u8 {
        self.0
    }

    /// Pitch-class = value mod 12 (total semitones).
    #[must_use]
    pub const fn pitch_class(self) -> PitchClass {
        let pc = self.0 % PITCH_CLASS_COUNT;
        PitchClass::from_unchecked(pc)
    }

    /// MIDI octave number using the common -1 offset convention.
    #[must_use]
    pub const fn octave(self) -> i8 {
        (self.0 / 12) as i8 - 1
    }
}
