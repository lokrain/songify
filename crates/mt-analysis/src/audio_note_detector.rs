//! Audio → NoteEvent detection (simple baseline).
//!
//! Assumptions:
//! - Mono or downmixed buffer.
//! - Predominant-pitch, not full polyphony.
//! - Good enough as a starting node; replaceable under same trait.

use std::vec::Vec;

use crate::config::AudioNoteConfig;
use crate::traits::AudioNoteAnalyzer;
use mt_core::events::{NoteEvent, NoteId, TrackId};
use mt_core::pitch::MidiNote;
use mt_core::time::SampleTime;

pub struct SimpleAudioNoteAnalyzer {
    pub track: TrackId,
}

impl SimpleAudioNoteAnalyzer {
    pub const fn new(track: TrackId) -> Self {
        Self { track }
    }
}

impl AudioNoteAnalyzer for SimpleAudioNoteAnalyzer {
    fn detect_audio_notes(
        &self,
        samples: &[f32],
        sample_rate: u32,
        cfg: &AudioNoteConfig,
    ) -> Vec<NoteEvent> {
        let frame = cfg.frame_size.max(1);
        let hop = cfg.hop_size.max(1);
        if samples.len() < frame {
            return Vec::new();
        }

        // For each frame:
        // - compute RMS; skip if below threshold
        // - estimate frequency via zero-crossing rate
        // - map to nearest MIDI note
        // - cluster consecutive frames with same MIDI note into notes
        let mut midi_by_frame: Vec<Option<u8>> = Vec::new();

        let mut i = 0usize;
        while i + frame <= samples.len() {
            let frame_slice = &samples[i..i + frame];
            let rms = rms(frame_slice);
            if rms < cfg.rms_threshold {
                midi_by_frame.push(None);
            } else if let Some(freq) = estimate_freq_zc(frame_slice, sample_rate) {
                if let Some(m) = freq_to_midi(freq) {
                    midi_by_frame.push(Some(m));
                } else {
                    midi_by_frame.push(None);
                }
            } else {
                midi_by_frame.push(None);
            }
            i += hop;
        }

        // Group consecutive frames.
        let min_note_samples =
            (cfg.min_note_seconds * sample_rate as f32) as i64;

        let mut out = Vec::new();
        let mut next_id = 1u32;

        let mut cur_midi: Option<u8> = None;
        let mut cur_start_frame: usize = 0;

        for (idx, m) in midi_by_frame.iter().copied().enumerate() {
            match (cur_midi, m) {
                (None, Some(n)) => {
                    cur_midi = Some(n);
                    cur_start_frame = idx;
                }
                (Some(n0), Some(n1)) if n0 == n1 => {
                    // continue
                }
                (Some(n0), x) if x != Some(n0) => {
                    // close
                    let start_sample = (cur_start_frame * hop) as i64;
                    let end_sample = (idx * hop + frame) as i64;
                    let dur = end_sample - start_sample;
                    if dur >= min_note_samples {
                        let note = MidiNote::new(n0).unwrap();
                        out.push(NoteEvent {
                            id: NoteId(next_id),
                            track: self.track,
                            onset: SampleTime::new(start_sample),
                            offset: SampleTime::new(end_sample),
                            note,
                            velocity: 100,
                        });
                        next_id = next_id.wrapping_add(1);
                    }
                    cur_midi = None;
                }
                _ => {}
            }
        }

        // Tail
        if let Some(n0) = cur_midi {
            let start_sample = (cur_start_frame * hop) as i64;
            let end_sample = (midi_by_frame.len() * hop + frame) as i64;
            let dur = end_sample - start_sample;
            if dur >= min_note_samples {
                let note = MidiNote::new(n0).unwrap();
                out.push(NoteEvent {
                    id: NoteId(next_id),
                    track: self.track,
                    onset: SampleTime::new(start_sample),
                    offset: SampleTime::new(end_sample),
                    note,
                    velocity: 100,
                });
            }
        }

        out
    }
}

fn rms(frame: &[f32]) -> f32 {
    if frame.is_empty() {
        return 0.0;
    }
    let mut sum = 0.0;
    for &s in frame {
        sum += s * s;
    }
    (sum / frame.len() as f32).sqrt()
}

fn estimate_freq_zc(frame: &[f32], sample_rate: u32) -> Option<f32> {
    if frame.len() < 2 {
        return None;
    }
    let mut crossings = 0u32;
    let mut prev = frame[0];

    for &x in &frame[1..] {
        if (prev <= 0.0 && x > 0.0) || (prev >= 0.0 && x < 0.0) {
            crossings += 1;
        }
        prev = x;
    }

    if crossings < 2 {
        return None;
    }

    // Approx: two zero-crossings per period.
    let period_samples = (2.0 * frame.len() as f32) / crossings as f32;
    if period_samples <= 0.0 {
        return None;
    }

    let freq = sample_rate as f32 / period_samples;
    if freq.is_finite() && freq > 20.0 && freq < 5000.0 {
        Some(freq)
    } else {
        None
    }
}

fn freq_to_midi(freq: f32) -> Option<u8> {
    if !(freq > 0.0) {
        return None;
    }
    let m = 69.0 + 12.0 * (freq / 440.0).log2();
    if m < 0.0 || m > 127.0 || !m.is_finite() {
        None
    } else {
        Some(m.round() as u8)
    }
}
