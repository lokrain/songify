//! Simple rule-based chord detector over NoteEvents.
//!
//! Strategy:
//! - Partition timeline into fixed hops.
//! - For each slice, collect active pitch-classes (duration-weighted).
//! - Match against known chord templates (from mt_core::chord_kind).
//! - Emit ChordEvents with confidence based on template fit.

#[cfg(any(not(feature = "std"), feature = "alloc"))]
extern crate alloc;

use alloc::vec::Vec;

use crate::config::ChordConfig;
use crate::confidence::clamp01_to_confidence_x1000;
use crate::traits::ChordAnalyzer;
use mt_core::chord::Chord;
use mt_core::chord_kind::{chord_intervals, ChordKindId, CHORD_KINDS};
use mt_core::events::{ChordEvent, NoteEvent};
use mt_core::pitch::PitchClass;
use mt_core::time::SampleTime;

pub struct RuleBasedChordAnalyzer;

impl ChordAnalyzer for RuleBasedChordAnalyzer {
    fn detect_chords(&self, notes: &[NoteEvent], cfg: &ChordConfig) -> Vec<ChordEvent> {
        if notes.is_empty() {
            return Vec::new();
        }

        let start = notes[0].onset.value();
        let end = notes.iter().map(|n| n.offset.value()).max().unwrap_or(start);
        if end <= start {
            return Vec::new();
        }

        let hop = (cfg.hop_seconds * 44_100.0) as i64; // stable default; engine can adjust.
        let win = (cfg.window_seconds * 44_100.0) as i64;
        if hop <= 0 || win <= 0 {
            return Vec::new();
        }

        let mut out = Vec::new();
        let mut t = start;

        while t < end {
            let w_start = t;
            let w_end = (t + win).min(end);

            let mut pc_weights = [0.0_f32; 12];

            for n in notes {
                let on = n.onset.value();
                let off = n.offset.value();
                if off <= w_start || on >= w_end {
                    continue;
                }
                let is = on.max(w_start);
                let ie = off.min(w_end);
                if ie > is {
                    let pc = n.note.pitch_class().as_u8() as usize;
                    let dur = (ie - is) as f32;
                    pc_weights[pc] += dur;
                }
            }

            let (best_chord, score) = best_chord_match(&pc_weights);
            let conf = clamp01_to_confidence_x1000(score);
            if score >= cfg.min_confidence {
                let onset = SampleTime::new(w_start);
                let offset = SampleTime::new(w_end);
                out.push(ChordEvent {
                    chord: best_chord,
                    onset,
                    offset,
                    confidence_x1000: conf,
                });
            }

            t += hop;
        }

        merge_adjacent_same_chords(out)
    }
}

fn best_chord_match(pc_weights: &[f32; 12]) -> (Chord, f32) {
    let mut best_score = 0.0;
    let mut best = Chord::new(PitchClass::new(0).unwrap(), ChordKindId::Maj, None).unwrap();

    for root_pc in 0..12 {
        let root = PitchClass::new(root_pc as u8).unwrap();
        for kind in CHORD_KINDS {
            // template: 1.0 for chord tones, 0.0 for others.
            let mut chord_sum = 0.0;
            let mut total = 0.0;
            for pc in 0..12 {
                let weight = pc_weights[pc];
                if weight <= 0.0 {
                    continue;
                }
                total += weight;
                let pc_class = PitchClass::new(pc as u8).unwrap();
                let is_tone = chord_intervals(kind.id)
                    .iter()
                    .any(|iv| root.transpose(*iv as i8) == pc_class);
                if is_tone {
                    chord_sum += weight;
                }
            }
            if total > 0.0 {
                let score = chord_sum / total;
                if score > best_score {
                    best_score = score;
                    best = Chord::new(root, kind.id, None).unwrap();
                }
            }
        }
    }

    (best, best_score)
}

fn merge_adjacent_same_chords(mut chords: Vec<ChordEvent>) -> Vec<ChordEvent> {
    if chords.is_empty() {
        return chords;
    }
    chords.sort_by_key(|c| c.onset.value());
    let mut out = Vec::new();
    let mut cur = chords[0];

    for ev in chords.into_iter().skip(1) {
        if ev.chord == cur.chord && ev.onset.value() <= cur.offset.value() {
            // extend
            if ev.offset.value() > cur.offset.value() {
                cur.offset = ev.offset;
            }
            if ev.confidence_x1000 < cur.confidence_x1000 {
                cur.confidence_x1000 = ev.confidence_x1000;
            }
        } else {
            out.push(cur);
            cur = ev;
        }
    }

    out.push(cur);
    out
}
