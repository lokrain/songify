//! Motif discovery over melodic note sequences.
//!
//! Strategy:
//! - Use pitch intervals between consecutive notes (in semitones).
//! - Build interval n-grams of length [min_len, max_len].
//! - A motif is any interval pattern with at least `min_occurrences` disjoint occurrences.
//! - All operations are deterministic, O(N^2) worst-case but fine for typical inputs.

use alloc::vec::Vec;

use mt_core::events::NoteEvent;

/// Canonical motif pattern: sequence of semitone intervals.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MotifPattern {
    pub intervals: Vec<i8>,
}

impl MotifPattern {
    #[must_use]
    pub fn len(&self) -> usize {
        self.intervals.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.intervals.is_empty()
    }
}

/// Single located occurrence of a motif in the note sequence.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MotifInstance {
    /// Index of the first note in the match.
    pub start_index: usize,
    /// Index of the last note in the match (inclusive).
    pub end_index: usize,
}

/// Motif definition plus all its instances.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Motif {
    pub pattern: MotifPattern,
    pub instances: Vec<MotifInstance>,
}

/// Configuration for deterministic motif discovery.
#[derive(Debug, Clone, Copy)]
pub struct MotifConfig {
    /// Minimal number of intervals (notes-1) in a motif.
    pub min_len: usize,
    /// Maximal number of intervals to consider.
    pub max_len: usize,
    /// Minimal number of non-overlapping occurrences.
    pub min_occurrences: usize,
}

impl MotifConfig {
    #[must_use]
    pub const fn default_strict() -> Self {
        Self { min_len: 3, max_len: 8, min_occurrences: 2 }
    }
}

/// Extract pitch intervals as semitones between consecutive notes.
///
/// Assumes `notes` is in ascending onset order. Caller is responsible for sorting.
fn intervals_from_notes(notes: &[NoteEvent]) -> Vec<i8> {
    let mut out = Vec::with_capacity(notes.len().saturating_sub(1));
    for w in notes.windows(2) {
        let a = w[0].note.value() as i16;
        let b = w[1].note.value() as i16;
        out.push((b - a) as i8);
    }
    out
}

/// Discover repeated interval patterns as motifs.
///
/// Deterministic:
/// - Always scans in the same order.
/// - Uses exact interval matches, no fuzziness.
#[must_use]
pub fn discover_motifs(notes: &[NoteEvent], cfg: MotifConfig) -> Vec<Motif> {
    if notes.len() < cfg.min_len + 1 || cfg.min_len == 0 || cfg.min_len > cfg.max_len {
        return Vec::new();
    }

    let intervals = intervals_from_notes(notes);
    let n = intervals.len();
    let mut motifs = Vec::new();

    // To avoid duplicates:
    // store seen patterns as Vec<i8> in motifs already.
    for len in cfg.min_len..=cfg.max_len {
        if len > n {
            break;
        }

        for start in 0..=n - len {
            let candidate = &intervals[start..start + len];

            // Skip if this pattern already recorded.
            if motifs.iter().any(|m: &Motif| m.pattern.intervals.as_slice() == candidate) {
                continue;
            }

            // Collect non-overlapping occurrences.
            let mut instances = Vec::new();
            let mut i = start;
            while i + len <= n {
                if &intervals[i..i + len] == candidate {
                    let inst = MotifInstance { start_index: i, end_index: i + len };
                    // Enforce non-overlap with last instance.
                    if instances
                        .last()
                        .map(|last: &MotifInstance| inst.start_index > last.end_index)
                        .unwrap_or(true)
                    {
                        instances.push(inst);
                        i += len;
                    } else {
                        i += 1;
                    }
                } else {
                    i += 1;
                }
            }

            if instances.len() >= cfg.min_occurrences {
                motifs.push(Motif {
                    pattern: MotifPattern { intervals: candidate.to_vec() },
                    instances,
                });
            }
        }
    }

    motifs
}
