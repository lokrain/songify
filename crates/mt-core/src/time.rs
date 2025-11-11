//! Minimal time primitives.
//!
//! These types do not know about tempo maps or transport.
//! Those live in higher layers (`mt-alloc`, etc).

use core::cmp::Ordering;
use core::fmt;

use crate::error::TheoryError;

/// Sample-accurate position.
///
/// Invariant: opaque i64 is allowed to wrap logically, but API users treat it
/// as a timeline coordinate (e.g., 0 = start).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SampleTime(i64);

impl SampleTime {
    pub const ZERO: Self = Self(0);

    /// Constructs a new sample time. All i64 values are accepted.
    #[must_use]
    pub const fn new(value: i64) -> Self {
        Self(value)
    }

    /// Raw underlying value (samples).
    #[must_use]
    pub const fn value(self) -> i64 {
        self.0
    }

    #[must_use]
    pub const fn saturating_add(self, delta: i64) -> Self {
        Self(self.0.saturating_add(delta))
    }

    #[must_use]
    pub const fn saturating_sub(self, delta: i64) -> Self {
        Self(self.0.saturating_sub(delta))
    }
}

impl fmt::Display for SampleTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}smp", self.0)
    }
}

/// Musical grid position (bar/beat/sub-beat).
///
/// Completely independent from audio sample rate.
/// Conversion is handled by higher layers using tempo maps.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct MusicalPosition {
    pub bar: u32,
    pub beat: u16,
    pub unit: u16, // e.g. 1/960 of a quarter note.
}

impl MusicalPosition {
    /// Creates a new position. Bar must be >= 1 to avoid "bar 0" ambiguity.
    pub const fn new(bar: u32, beat: u16, unit: u16) -> Result<Self, TheoryError> {
        if bar == 0 {
            return Err(TheoryError::InvalidTime);
        }
        Ok(Self { bar, beat, unit })
    }

    /// Lexicographic comparison helper (bar → beat → unit).
    #[must_use]
    pub const fn cmp_lex(&self, other: &Self) -> Ordering {
        if self.bar != other.bar {
            return if self.bar < other.bar {
                Ordering::Less
            } else {
                Ordering::Greater
            };
        }

        if self.beat != other.beat {
            return if self.beat < other.beat {
                Ordering::Less
            } else {
                Ordering::Greater
            };
        }

        if self.unit != other.unit {
            return if self.unit < other.unit {
                Ordering::Less
            } else {
                Ordering::Greater
            };
        }
        Ordering::Equal
    }
}

impl core::cmp::PartialOrd for MusicalPosition {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp_lex(other))
    }
}

impl core::cmp::Ord for MusicalPosition {
    fn cmp(&self, other: &Self) -> Ordering {
        self.cmp_lex(other)
    }
}
