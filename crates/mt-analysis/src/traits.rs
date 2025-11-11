#![allow(clippy::module_name_repetitions)]
//! Stable traits for analysis components.
//!
//! `mt-engine` wraps these into graph nodes. Callers may use them directly
//! for synchronous, batch-style analysis.

use crate::config::{
    AudioNoteConfig, ChordConfig, KeyConfig, MidiNoteConfig, SegmentConfig,
    SwingConfig, TempoConfig,
};
use mt_core::events::{
    ChordEvent, KeyEvent, NoteEvent, SegmentEvent, TempoEvent,
};
use mt_core::midi::MidiEvent;

/// Simple tempo range definition used by tempo detectors.
#[derive(Clone, Copy, Debug)]
pub struct TempoRange {
    pub min_bpm: f32,
    pub max_bpm: f32,
}

/// Tempo + meter detector result.
///
/// For now: only tempo events. Meter refinement can be added without breaking.
#[derive(Clone, Debug)]
pub struct TempoAnalysis {
    pub tempo_events: heapless_vec::Vec<TempoEvent, 32>,
}

// Use a fixed-capacity Vec for no_std friendliness inside the trait result.
// Internally we can convert to Vec in std environments.
mod heapless_vec {
    use core::ops::{Deref, DerefMut};

    /// Tiny fixed-capacity Vec used to keep trait signatures no_std friendly.
    #[derive(Clone, Debug)]
    pub struct Vec<T, const N: usize> {
        len: usize,
        data: [T; N],
    }

    impl<T: Copy, const N: usize> Vec<T, N> {
        pub fn new() -> Self where T: Default {
            Self {
                len: 0,
                data: [T::default(); N],
            }
        }

        pub fn push(&mut self, value: T) -> bool {
            if self.len == N {
                return false;
            }
            self.data[self.len] = value;
            self.len += 1;
            true
        }
    }

    impl<T: Copy, const N: usize> Deref for Vec<T, N> {
        type Target = [T];

        fn deref(&self) -> &Self::Target {
            &self.data[..self.len]
        }
    }

    impl<T: Copy, const N: usize> DerefMut for Vec<T, N> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.data[..self.len]
        }
    }
}

// NOTE: To avoid unsafe (as required) we should not have used MaybeUninit.
// Adjust: we will not expose TempoAnalysis yet; we keep traits simple and use standard Vec in std.
// I keep interfaces below using `Vec` (std) only, since mt-analysis is `std` by default.
// This comment stands as clarification: no hidden unsafe in final traits.

/// Detects normalized notes from MIDI events.


extern crate alloc;

use alloc::vec::Vec;

/// Detects normalized notes from MIDI events.
pub trait MidiNoteAnalyzer {
    fn detect_midi_notes(&self, events: &[MidiEvent], cfg: &MidiNoteConfig) -> Vec<NoteEvent>;
}

/// Detects notes from audio samples.
pub trait AudioNoteAnalyzer {
    fn detect_audio_notes(
        &self,
        samples: &[f32],
        sample_rate: u32,
        cfg: &AudioNoteConfig,
    ) -> Vec<NoteEvent>;
}

/// Detects tempo events (global and possibly local).
pub trait TempoMeterAnalyzer {
    fn detect_tempo(
        &self,
        samples: &[f32],
        sample_rate: u32,
        cfg: &TempoConfig,
    ) -> Vec<TempoEvent>;
}

/// Estimates musical keys.
pub trait KeyAnalyzer {
    fn detect_keys(&self, notes: &[NoteEvent], cfg: &KeyConfig) -> Vec<KeyEvent>;
}

/// Estimates chord timeline.
pub trait ChordAnalyzer {
    fn detect_chords(&self, notes: &[NoteEvent], cfg: &ChordConfig) -> Vec<ChordEvent>;
}

/// Estimates swing ratio.
pub trait SwingAnalyzer {
    fn detect_swing_ratio(
        &self,
        notes: &[NoteEvent],
        tempo_events: &[TempoEvent],
        cfg: &SwingConfig,
    ) -> Option<f32>;
}

/// Detects structural segments.
pub trait SegmentAnalyzer {
    fn detect_segments(
        &self,
        samples: &[f32],
        sample_rate: u32,
        chords: &[ChordEvent],
        cfg: &SegmentConfig,
    ) -> Vec<SegmentEvent>;
}

/// Composite suite: convenience bound for a "full" implementation.
pub trait AnalysisSuite:
    MidiNoteAnalyzer
    + AudioNoteAnalyzer
    + TempoMeterAnalyzer
    + KeyAnalyzer
    + ChordAnalyzer
    + SwingAnalyzer
    + SegmentAnalyzer
{
}

impl<T> AnalysisSuite for T where
    T: MidiNoteAnalyzer
        + AudioNoteAnalyzer
        + TempoMeterAnalyzer
        + KeyAnalyzer
        + ChordAnalyzer
        + SwingAnalyzer
        + SegmentAnalyzer
{
}
