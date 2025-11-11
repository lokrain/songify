//! Voice-leading analysis between chords.
//!
//! Provides a deterministic, symmetric-friendly cost model:
//! - Build pitch-class sets for both chords (using root and kind).
//! - Greedy matching from source to nearest destination tones.
//! - Summarize total and maximal motion + whether crossings occur.
//!
//! This is intentionally simple and explainable.

use alloc::vec::Vec;

use mt_core::chord::Chord;
use mt_core::chord_kind::chord_tones;
use mt_core::pitch::PitchClass;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct VoiceLeadingMove {
    pub from: PitchClass,
    pub to: PitchClass,
    /// Signed semitone movement in [-6, +6] modulo 12, chosen minimal.
    pub semitones: i8,
}

#[derive(Debug, Clone, PartialEq)]
pub struct VoiceLeadingSummary {
    pub moves: Vec<VoiceLeadingMove>,
    /// Sum of absolute semitone motions.
    pub total_abs_motion: u32,
    /// Maximum absolute semitone motion among individual voices.
    pub max_abs_motion: u8,
    /// Whether any pair of voices crosses in relative order.
    pub has_crossing: bool,
}

fn pc_distance_minimal(from: PitchClass, to: PitchClass) -> i8 {
    let a = from.as_u8() as i16;
    let b = to.as_u8() as i16;
    let diff = b - a;
    let mut wrapped = diff % 12;
    if wrapped > 6 {
        wrapped -= 12;
    }
    if wrapped < -6 {
        wrapped += 12;
    }
    wrapped as i8
}

/// Compute a simple voice-leading summary between two chords.
///
/// Deterministic greedy matching:
/// - For each source pitch (sorted), choose the closest unused target pitch.
/// - If tie, choose the lower target.
#[must_use]
pub fn compute_voice_leading(from: Chord, to: Chord) -> VoiceLeadingSummary {
    // Collect unique pitch-classes for each chord into Vecs.
    fn chord_pcs(ch: Chord) -> Vec<PitchClass> {
        let tones = chord_tones(ch.root, ch.kind);
        let mut out = Vec::new();
        for pc in tones.iter().copied() {
            if !out.iter().any(|&x| x == pc) {
                out.push(pc);
            }
        }
        out.sort_by_key(|pc| pc.as_u8());
        out
    }

    let src = chord_pcs(from);
    let dst = chord_pcs(to);
    if src.is_empty() || dst.is_empty() {
        return VoiceLeadingSummary {
            moves: Vec::new(),
            total_abs_motion: 0,
            max_abs_motion: 0,
            has_crossing: false,
        };
    }

    let mut used = alloc::vec![false; dst.len()];
    let mut moves = Vec::with_capacity(src.len());

    for s in &src {
        let mut best_j = 0usize;
        let mut best_cost = i16::MAX;
        let mut best_pc = dst[0];

        for (j, dpc) in dst.iter().copied().enumerate() {
            if used[j] {
                continue;
            }
            let step = pc_distance_minimal(*s, dpc);
            let cost = step.unsigned_abs() as i16;
            if cost < best_cost || (cost == best_cost && dpc.as_u8() < best_pc.as_u8()) {
                best_cost = cost;
                best_pc = dpc;
                best_j = j;
            }
        }

        used[best_j] = true;
        moves.push(VoiceLeadingMove {
            from: *s,
            to: best_pc,
            semitones: pc_distance_minimal(*s, best_pc),
        });
    }

    // Compute metrics.
    let mut total_abs = 0u32;
    let mut max_abs = 0u8;

    for m in &moves {
        let abs = m.semitones.unsigned_abs() as u8;
        total_abs += u32::from(abs);
        if abs > max_abs {
            max_abs = abs;
        }
    }

    // Crossing: check if order of voices is inverted.
    let mut has_crossing = false;
    for i in 0..moves.len() {
        for j in i + 1..moves.len() {
            let a_from = moves[i].from.as_u8();
            let b_from = moves[j].from.as_u8();
            let a_to = moves[i].to.as_u8();
            let b_to = moves[j].to.as_u8();
            if (a_from < b_from && a_to > b_to) || (a_from > b_from && a_to < b_to) {
                has_crossing = true;
                break;
            }
        }
        if has_crossing {
            break;
        }
    }

    VoiceLeadingSummary {
        moves,
        total_abs_motion: total_abs,
        max_abs_motion: max_abs,
        has_crossing,
    }
}
