//! Harmonic-change-based refinement for segments.

#[cfg(any(not(feature = "std"), feature = "alloc"))]
extern crate alloc;

use alloc::vec::Vec;

use crate::config::SegmentConfig;
use crate::traits::SegmentAnalyzer;
use mt_core::events::{ChordEvent, SegmentEvent, SegmentKind};
use mt_core::time::SampleTime;

pub struct HarmonicSegmenter;

impl SegmentAnalyzer for HarmonicSegmenter {
    fn detect_segments(
        &self,
        _samples: &[f32],
        _sample_rate: u32,
        chords: &[ChordEvent],
        cfg: &SegmentConfig,
    ) -> Vec<SegmentEvent> {
        let mut out = Vec::new();
        if chords.is_empty() {
            return out;
        }

        let min_len = (cfg.min_segment_seconds * 44_100.0) as i64; // default grid

        let mut seg_start = chords[0].onset.value();
        let mut last_chord = chords[0].chord;

        for ev in chords.iter().skip(1) {
            let pos = ev.onset.value();
            if ev.chord != last_chord && pos - seg_start >= min_len {
                out.push(SegmentEvent {
                    kind: SegmentKind::Other(1),
                    onset: SampleTime::new(seg_start),
                    offset: SampleTime::new(pos),
                    confidence_x1000: 700,
                });
                seg_start = pos;
                last_chord = ev.chord;
            }
        }

        let end = chords.last().map(|c| c.offset.value()).unwrap_or(seg_start);
        if end > seg_start {
            out.push(SegmentEvent {
                kind: SegmentKind::Other(1),
                onset: SampleTime::new(seg_start),
                offset: SampleTime::new(end),
                confidence_x1000: 700,
            });
        }

        out
    }
}
