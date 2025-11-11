//! Energy-based segmentation.


extern crate alloc;
use alloc::vec::Vec;

use crate::config::SegmentConfig;
use crate::traits::SegmentAnalyzer;
use mt_core::events::{SegmentEvent, SegmentKind};
use mt_core::time::SampleTime;

pub struct EnergySegmenter;

impl SegmentAnalyzer for EnergySegmenter {
    fn detect_segments(
        &self,
        samples: &[f32],
        sample_rate: u32,
        _chords: &[mt_core::events::ChordEvent],
        cfg: &SegmentConfig,
    ) -> Vec<SegmentEvent> {
        let mut out = Vec::new();
        if samples.is_empty() {
            return out;
        }

        let win = (cfg.window_seconds * sample_rate as f32) as usize;
        if win == 0 || win >= samples.len() {
            // Single segment.
            out.push(SegmentEvent {
                kind: SegmentKind::Other(0),
                onset: SampleTime::new(0),
                offset: SampleTime::new(samples.len() as i64),
                confidence_x1000: 1000,
            });
            return out;
        }

        let min_len = (cfg.min_segment_seconds * sample_rate as f32) as i64;

        let mut energies = Vec::new();
        let mut i = 0usize;
        while i + win <= samples.len() {
            let mut sum = 0.0;
            for &s in &samples[i..i + win] {
                sum += s * s;
            }
            energies.push(sum / win as f32);
            i += win;
        }

        let mut segments = Vec::new();
        let mut seg_start = 0i64;
        let mut last_e = energies[0];

        for (idx, &e) in energies.iter().enumerate().skip(1) {
            let pos = (idx * win) as i64;
            let diff = (e - last_e).abs();
            if diff > last_e * 0.5 && pos - seg_start >= min_len {
                segments.push((seg_start, pos));
                seg_start = pos;
            }
            last_e = e.max(1e-9);
        }
        segments.push((seg_start, samples.len() as i64));

        for (s, e) in segments {
            out.push(SegmentEvent {
                kind: SegmentKind::Other(0),
                onset: SampleTime::new(s),
                offset: SampleTime::new(e),
                confidence_x1000: 800,
            });
        }

        out
    }
}
