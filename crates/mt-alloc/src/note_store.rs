//! NoteStore:
//! - Stores NoteEvent in onset-sorted order.
//! - Provides simple time range queries.

use alloc::vec::Vec;

use mt_core::events::NoteEvent;
use mt_core::time::SampleTime;

#[derive(Debug, Default, Clone)]
pub struct NoteStore {
    notes: Vec<NoteEvent>,
}

impl NoteStore {
    pub fn new() -> Self {
        Self { notes: Vec::new() }
    }

    /// Insert a note keeping onset ordering.
    pub fn insert(&mut self, note: NoteEvent) {
        let idx = self.notes.binary_search_by_key(&note.onset, |n| n.onset).unwrap_or_else(|i| i);
        self.notes.insert(idx, note);
    }

    /// Bulk extend with pre-sorted notes (by onset).
    pub fn extend_sorted(&mut self, mut new_notes: Vec<NoteEvent>) {
        if new_notes.is_empty() {
            return;
        }
        // Simple merge; deterministic.
        let mut merged = Vec::with_capacity(self.notes.len() + new_notes.len());
        let mut i = 0;
        let mut j = 0;
        self.notes.sort_by_key(|n| n.onset);
        new_notes.sort_by_key(|n| n.onset);
        while i < self.notes.len() && j < new_notes.len() {
            if self.notes[i].onset <= new_notes[j].onset {
                merged.push(self.notes[i]);
                i += 1;
            } else {
                merged.push(new_notes[j]);
                j += 1;
            }
        }
        merged.extend_from_slice(&self.notes[i..]);
        merged.extend_from_slice(&new_notes[j..]);
        self.notes = merged;
    }

    /// Query notes whose onset is in [start, end).
    pub fn notes_in_range(&self, start: SampleTime, end: SampleTime) -> &[NoteEvent] {
        // Lower bound on start.
        let from = self.notes.partition_point(|n| n.onset < start);
        // First index where onset >= end.
        let to = self.notes.partition_point(|n| n.onset < end);
        &self.notes[from..to]
    }

    #[must_use]
    pub fn all(&self) -> &[NoteEvent] {
        &self.notes
    }
}
