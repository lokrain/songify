//! Shared configuration structures for analyzers.
//!
//! These are stable inputs. `mt-engine` and external callers use this
//! instead of ad-hoc knobs.

use crate::traits::TempoRange;

/// Global analysis config, grouping per-module configs.
///
/// This type is designed to be:
/// - serializable (with `serde`)
/// - stable across versions (breaking changes bump major).
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug)]
pub struct AnalysisConfig {
    pub tempo: TempoConfig,
    pub key: KeyConfig,
    pub chord: ChordConfig,
    pub swing: SwingConfig,
    pub segment: SegmentConfig,
    pub midi: MidiNoteConfig,
    pub audio_note: AudioNoteConfig,
}

impl Default for AnalysisConfig {
    fn default() -> Self {
        Self {
            tempo: TempoConfig::default(),
            key: KeyConfig::default(),
            chord: ChordConfig::default(),
            swing: SwingConfig::default(),
            segment: SegmentConfig::default(),
            midi: MidiNoteConfig::default(),
            audio_note: AudioNoteConfig::default(),
        }
    }
}

/// Tempo + meter detection configuration.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug)]
pub struct TempoConfig {
    pub tempo_range: TempoRange,
    /// Frame size in samples for onset/energy envelope.
    pub frame_size: usize,
    /// Hop size in samples.
    pub hop_size: usize,
}

impl Default for TempoConfig {
    fn default() -> Self {
        Self {
            tempo_range: TempoRange {
                min_bpm: 60.0,
                max_bpm: 200.0,
            },
            frame_size: 2048,
            hop_size: 512,
        }
    }
}

/// Key detection configuration.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug)]
pub struct KeyConfig {
    /// Minimum duration (in seconds) per key region for it to be emitted.
    pub min_region_seconds: f32,
}

impl Default for KeyConfig {
    fn default() -> Self {
        Self {
            min_region_seconds: 4.0,
        }
    }
}

/// Chord detection configuration.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug)]
pub struct ChordConfig {
    /// Window length in seconds for chord estimation.
    pub window_seconds: f32,
    /// Hop length in seconds.
    pub hop_seconds: f32,
    /// Minimum confidence to emit chord events (0..1).
    pub min_confidence: f32,
}

impl Default for ChordConfig {
    fn default() -> Self {
        Self {
            window_seconds: 1.0,
            hop_seconds: 0.5,
            min_confidence: 0.2,
        }
    }
}

/// Swing detection configuration.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug)]
pub struct SwingConfig {
    /// Minimum note density to attempt swing estimation.
    pub min_notes: usize,
}

impl Default for SwingConfig {
    fn default() -> Self {
        Self { min_notes: 16 }
    }
}

/// Segmentation configuration.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug)]
pub struct SegmentConfig {
    /// Window seconds for energy-based novelty.
    pub window_seconds: f32,
    /// Minimum segment length in seconds.
    pub min_segment_seconds: f32,
}

impl Default for SegmentConfig {
    fn default() -> Self {
        Self {
            window_seconds: 3.0,
            min_segment_seconds: 8.0,
        }
    }
}

/// MIDI -> NoteEvent normalization.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug)]
pub struct MidiNoteConfig {
    /// Maximum gap in samples to treat overlapping note-ons as retriggers.
    pub retrigger_tolerance_samples: i64,
}

impl Default for MidiNoteConfig {
    fn default() -> Self {
        Self {
            retrigger_tolerance_samples: 0,
        }
    }
}

/// Audio -> note detection.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug)]
pub struct AudioNoteConfig {
    /// Frame size for pitch estimation.
    pub frame_size: usize,
    /// Hop size for scanning.
    pub hop_size: usize,
    /// Minimum RMS threshold for considering a frame pitched.
    pub rms_threshold: f32,
    /// Minimum note length in seconds.
    pub min_note_seconds: f32,
}

impl Default for AudioNoteConfig {
    fn default() -> Self {
        Self {
            frame_size: 2048,
            hop_size: 512,
            rms_threshold: 0.01,
            min_note_seconds: 0.08,
        }
    }
}
