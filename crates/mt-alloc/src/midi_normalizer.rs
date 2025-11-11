//! MIDI normalizer:
//! - Tracks note-on/off per (channel, note).
//! - Emits normalized NoteEvent with onset/offset SampleTime.
//!
//! Policy:
//! - If NoteOff missing, we can close on next NoteOn of same note/channel.
//! - All behavior is deterministic.

use alloc::vec::Vec;

use mt_core::events::{NoteEvent, NoteId, TrackId};
use mt_core::midi::{MidiEvent, MidiEventKind};
use mt_core::pitch::MidiNote;
use mt_core::time::SampleTime;

/// Internal state of an active note.
#[derive(Clone, Copy, Debug)]
struct ActiveNote {
    onset: SampleTime,
    velocity: u8,
}

#[derive(Debug)]
pub struct MidiNormalizer {
    /// [channel][note] -> ActiveNote
    active: [[Option<ActiveNote>; 128]; 16],
    next_id: u32,
    track_for_channel: [TrackId; 16],
}

impl MidiNormalizer {
    pub fn new() -> Self {
        // TrackId: simple mapping channel -> TrackId(channel).
        let mut tracks = [TrackId(0); 16];
        let mut i = 0u16;
        while i < 16 {
            tracks[i as usize] = TrackId(i);
            i += 1;
        }

        Self { active: [[None; 128]; 16], next_id: 1, track_for_channel: tracks }
    }

    /// Process a batch of MIDI events at a given time.
    ///
    /// Returns zero or more completed NoteEvents.
    pub fn process(&mut self, time: SampleTime, events: &[MidiEvent]) -> Vec<NoteEvent> {
        let mut out = Vec::new();

        for ev in events {
            let ch = ev.channel.value() as usize;
            match ev.kind {
                MidiEventKind::NoteOn if ev.data2 > 0 => {
                    let note = ev.data1 as usize;
                    // If a note was already active, close it deterministically.
                    if let Some(active) = self.active[ch][note] {
                        out.push(self.make_note_event(
                            ch,
                            note as u8,
                            active.onset,
                            time,
                            active.velocity,
                        ));
                    }
                    self.active[ch][note] = Some(ActiveNote { onset: time, velocity: ev.data2 });
                }
                MidiEventKind::NoteOn | MidiEventKind::NoteOff => {
                    let note = ev.data1 as usize;
                    if let Some(active) = self.active[ch][note] {
                        out.push(self.make_note_event(
                            ch,
                            note as u8,
                            active.onset,
                            time,
                            active.velocity,
                        ));
                        self.active[ch][note] = None;
                    }
                }
                _ => {}
            }
        }

        out
    }

    /// Flush any hanging notes at the given cutoff time.
    pub fn flush(&mut self, cutoff: SampleTime) -> Vec<NoteEvent> {
        let mut out = Vec::new();
        for ch in 0..16 {
            for note in 0..128 {
                if let Some(active) = self.active[ch][note] {
                    out.push(self.make_note_event(
                        ch,
                        note as u8,
                        active.onset,
                        cutoff,
                        active.velocity,
                    ));
                    self.active[ch][note] = None;
                }
            }
        }
        out
    }

    fn make_note_event(
        &mut self,
        ch: usize,
        note: u8,
        onset: SampleTime,
        offset: SampleTime,
        velocity: u8,
    ) -> NoteEvent {
        let id = NoteId(self.next_id);
        self.next_id = self.next_id.wrapping_add(1);

        NoteEvent {
            id,
            track: self.track_for_channel[ch],
            onset,
            offset,
            note: MidiNote::new(note).expect("0..127"),
            velocity,
        }
    }
}
