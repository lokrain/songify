//! Histogram-based key detection (Krumhansl-like, simplified).

#[cfg(any(not(feature = "std"), feature = "alloc"))]
extern crate alloc;

use alloc::vec::Vec;

use crate::config::KeyConfig;
use crate::traits::KeyAnalyzer;
use mt_core::events::{KeyEvent, NoteEvent};
use mt_core::key::{Key, KeyMode};
use mt_core::pitch::PitchClass;
use mt_core::time::SampleTime;

pub struct HistogramKeyAnalyzer;

impl KeyAnalyzer for HistogramKeyAnalyzer {
    fn detect_keys(&self, notes: &[NoteEvent], cfg: &KeyConfig) -> Vec<KeyEvent> {
        if notes.is_empty() {
            return Vec::new();
        }

        // Simple global histogram; can be extended to sliding windows later.
        let mut hist = [0.0_f32; 12];
        for n in notes {
            let pc = n.note.pitch_class().as_u8() as usize;
            let dur = (n.offset.value() - n.onset.value()).max(1) as f32;
            hist[pc] += dur;
        }

        let (maj_tonic, maj_score) = best_key(&hist, KeyMode::Major);
        let (min_tonic, min_score) = best_key(&hist, KeyMode::Minor);

        let (mode, tonic, score) = if maj_score >= min_score {
            (KeyMode::Major, maj_tonic, maj_score)
        } else {
            (KeyMode::Minor, min_tonic, min_score)
        };

        let key = Key::new(PitchClass::new(tonic as u8).unwrap(), mode);
        let min_samples = (cfg.min_region_seconds * 44_100.0) as i64; // assume; engine can override later.

        let total_span = notes
            .last()
            .map(|n| n.offset.value() - notes[0].onset.value())
            .unwrap_or(0);

        if total_span < min_samples {
            return Vec::new();
        }

        let conf = if score <= 0.0 { 0 } else { 1000 };

        vec![KeyEvent {
            key,
            position: SampleTime::new(notes[0].onset.value()),
            confidence_x1000: conf,
        }]
    }
}

fn best_key(hist: &[f32; 12], mode: KeyMode) -> (u8, f32) {
    // Very simple major/minor profiles.
    let profile_maj: [f32; 12] = [6.35, 2.23, 3.48, 2.33, 4.38, 4.09, 2.52, 5.19, 2.39, 3.66, 2.29, 2.88];
    let profile_min: [f32; 12] = [6.33, 2.68, 3.52, 5.38, 2.60, 3.53, 2.54, 4.75, 3.98, 2.69, 3.34, 3.17];

    let base = match mode {
        KeyMode::Major => &profile_maj,
        KeyMode::Minor => &profile_min,
    };

    let mut best_tonic = 0u8;
    let mut best_score = f32::MIN;

    for tonic in 0..12 {
        let mut score = 0.0;
        for i in 0..12 {
            let idx = ((i + tonic) % 12) as usize;
            score += hist[idx] * base[i];
        }
        if score > best_score {
            best_score = score;
            best_tonic = tonic as u8;
        }
    }

    (best_tonic, best_score)
}
