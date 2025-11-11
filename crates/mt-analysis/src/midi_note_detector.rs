//! MIDI → NoteEvent normalization.
//!
//! Deterministic, order-stable pairing of note-on/off into NoteEvent.

#[cfg(any(not(feature = "std"), feature = "alloc"))]
extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::vec::Vec;

use crate::config::MidiNoteConfig;
use crate::traits::MidiNoteAnalyzer;
use mt_core::events::{NoteEvent, NoteId, TrackId};
use mt_core::midi::{MidiEvent, MidiEventKind};
use mt_core::pitch::MidiNote;
use mt_core::time::SampleTime;

/// Simple stateful MIDI note normalizer.
///
/// Assumes events are sorted by time upstream (engine is responsible).
pub struct SimpleMidiNoteAnalyzer {
    pub track: TrackId,
}

impl SimpleMidiNoteAnalyzer {
    pub const fn new(track: TrackId) -> Self {
        Self { track }
    }
}

impl MidiNoteAnalyzer for SimpleMidiNoteAnalyzer {
    fn detect_midi_notes(&self, events: &[MidiEvent], cfg: &MidiNoteConfig) -> Vec<NoteEvent> {
        // active[(channel, note)] = (on_time, velocity)
        let mut active: BTreeMap<(u8, u8), (SampleTime, u8)> = BTreeMap::new();
        let mut out = Vec::new();
        let mut next_id = 1u32;

        for (i, ev) in events.iter().enumerate() {
            let t = SampleTime::new(i as i64); // engine should supply real times; here index-based.
            match ev.kind {
                MidiEventKind::NoteOn if ev.data2 > 0 => {
                    let key = (ev.channel.value(), ev.data1);
                    // If already active, either retrigger or close previous.
                    if let Some((on, vel)) = active.remove(&key) {
                        // Close previous if retrigger tolerance is set.
                        if cfg.retrigger_tolerance_samples == 0
                            || t.value() - on.value() > cfg.retrigger_tolerance_samples
                        {
                            out.push(NoteEvent {
                                id: NoteId(next_id),
                                track: self.track,
                                onset: on,
                                offset: t,
                                note: MidiNote::new(ev.data1).unwrap(),
                                velocity: vel,
                            });
                            next_id = next_id.wrapping_add(1);
                        }
                    }
                    active.insert(key, (t, ev.data2));
                }
                MidiEventKind::NoteOff | MidiEventKind::NoteOn => {
                    // NoteOn with velocity 0 is treated as NoteOff.
                    let key = (ev.channel.value(), ev.data1);
                    if let Some((on, vel)) = active.remove(&key) {
                        let offset = t;
                        if offset.value() > on.value() {
                            out.push(NoteEvent {
                                id: NoteId(next_id),
                                track: self.track,
                                onset: on,
                                offset,
                                note: MidiNote::new(ev.data1).unwrap(),
                                velocity: vel,
                            });
                            next_id = next_id.wrapping_add(1);
                        }
                    }
                }
                _ => {}
            }
        }

        // Drop unmatched actives (hanging notes). Deterministic; no guessing.
        out
    }
}
