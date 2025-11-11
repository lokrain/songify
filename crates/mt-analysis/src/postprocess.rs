//! Generic post-processing helpers (smoothing, pruning).

#[cfg(any(not(feature = "std"), feature = "alloc"))]
extern crate alloc;

use alloc::vec::Vec;

use mt_core::events::{ChordEvent, KeyEvent, SegmentEvent};

/// Remove segments shorter than `min_len` samples.
pub fn prune_short_segments(segments: Vec<SegmentEvent>, min_len: i64) -> Vec<SegmentEvent> {
    segments
        .into_iter()
        .filter(|s| (s.offset.value() - s.onset.value()) >= min_len)
        .collect()
}

/// Merge consecutive identical keys.
pub fn merge_keys(mut keys: Vec<KeyEvent>) -> Vec<KeyEvent> {
    if keys.is_empty() {
        return keys;
    }
    keys.sort_by_key(|k| k.position.value());
    let mut out = Vec::new();
    let mut cur = keys[0];

    for ev in keys.into_iter().skip(1) {
        if ev.key == cur.key && ev.position.value() <= cur.position.value() {
            // keep earliest; ignore overlaps
            continue;
        } else {
            out.push(cur);
            cur = ev;
        }
    }

    out.push(cur);
    out
}

/// Merge consecutive identical chords.
pub fn merge_chords(mut chords: Vec<ChordEvent>) -> Vec<ChordEvent> {
    if chords.is_empty() {
        return chords;
    }
    chords.sort_by_key(|c| c.onset.value());
    let mut out = Vec::new();
    let mut cur = chords[0];

    for ev in chords.into_iter().skip(1) {
        if ev.chord == cur.chord && ev.onset.value() <= cur.offset.value() {
            if ev.offset.value() > cur.offset.value() {
                cur.offset = ev.offset;
            }
        } else {
            out.push(cur);
            cur = ev;
        }
    }

    out.push(cur);
    out
}
