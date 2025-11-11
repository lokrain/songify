//! Swing ratio estimation from note onsets.

use crate::config::SwingConfig;
use crate::traits::SwingAnalyzer;
use mt_core::events::{NoteEvent, TempoEvent};

pub struct SimpleSwingAnalyzer;

impl SwingAnalyzer for SimpleSwingAnalyzer {
    fn detect_swing_ratio(
        &self,
        notes: &[NoteEvent],
        tempo_events: &[TempoEvent],
        cfg: &SwingConfig,
    ) -> Option<f32> {
        if notes.len() < cfg.min_notes {
            return None;
        }
        let tempo = tempo_events.first()?;
        let bpm = tempo.bpm_x1000 as f32 / 1000.0;
        if bpm <= 0.0 {
            return None;
        }

        // Beat duration in samples (approx; real mapping is in engine).
        let beat_samples = (60.0 / bpm) * 44_100.0;

        let mut ratios = 0.0_f32;
        let mut count = 0_u32;

        // Look at pairs of 8th notes.
        for win in notes.windows(2) {
            let a = &win[0];
            let b = &win[1];
            let dt = (b.onset.value() - a.onset.value()) as f32;
            if dt <= 0.0 {
                continue;
            }
            let r = dt / (beat_samples / 2.0); // ratio vs straight 8th
            if r > 0.2 && r < 2.0 {
                ratios += r;
                count += 1;
            }
        }

        if count == 0 {
            None
        } else {
            Some(ratios / count as f32)
        }
    }
}
