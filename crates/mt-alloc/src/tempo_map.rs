//! TempoMap: deterministic mapping between sample positions and musical time.
//!
//! Responsibilities:
//! - Store ordered TempoEvent, MeterEvent, SwingEvent sequences.
//! - Provide monotonic conversions SampleTime <-> beats (in fixed-point).
//!
//! This is purely arithmetic; no wall-clock, no scheduling.

use alloc::vec::Vec;

use mt_core::events::{MeterEvent, SwingEvent, TempoEvent};
use mt_core::time::SampleTime;

/// Fixed-point beats type: beats * 1000.
pub type BeatsX1000 = i64;

#[derive(Debug, Clone)]
pub struct TempoMap {
    tempo_events: Vec<TempoEvent>,
    meter_events: Vec<MeterEvent>,
    swing_events: Vec<SwingEvent>,
    sample_rate: u32,
}

impl TempoMap {
    pub fn new(sample_rate: u32) -> Self {
        Self {
            tempo_events: Vec::new(),
            meter_events: Vec::new(),
            swing_events: Vec::new(),
            sample_rate,
        }
    }

    /// Insert a tempo event; must be added in non-decreasing time order.
    pub fn push_tempo(&mut self, ev: TempoEvent) {
        debug_assert!(
            self.tempo_events
                .last()
                .map(|p| p.position <= ev.position)
                .unwrap_or(true),
            "tempo events must be sorted by position"
        );
        self.tempo_events.push(ev);
    }

    pub fn push_meter(&mut self, ev: MeterEvent) {
        debug_assert!(
            self.meter_events
                .last()
                .map(|p| p.position <= ev.position)
                .unwrap_or(true),
            "meter events must be sorted by position"
        );
        self.meter_events.push(ev);
    }

    pub fn push_swing(&mut self, ev: SwingEvent) {
        debug_assert!(
            self.swing_events
                .last()
                .map(|p| p.position <= ev.position)
                .unwrap_or(true),
            "swing events must be sorted by position"
        );
        self.swing_events.push(ev);
    }

    /// Convert sample position to beats * 1000 using stepwise-constant tempo.
    ///
    /// For v1: ignores meter/swing; they are available for callers.
    pub fn sample_to_beats_x1000(&self, pos: SampleTime) -> BeatsX1000 {
        let mut last_pos = SampleTime::ZERO;
        let mut last_bpm_x1000 = 120_000; // default 120 BPM
        let mut acc_beats_x1000: i64 = 0;

        for ev in &self.tempo_events {
            if ev.position >= pos {
                break;
            }
            let dt = ev.position.value() - last_pos.value();
            let bpm = last_bpm_x1000 as i64;
            // beats = samples / sr * (bpm / 60)
            let beats_x1000 = dt as i128 * bpm as i128 / (self.sample_rate as i128 * 60) * 1000;
            acc_beats_x1000 = acc_beats_x1000.saturating_add(beats_x1000 as i64);
            last_pos = ev.position;
            last_bpm_x1000 = ev.bpm_x1000;
        }

        let dt = pos.value() - last_pos.value();
        let bpm = last_bpm_x1000 as i64;
        let beats_x1000 = dt as i128 * bpm as i128 / (self.sample_rate as i128 * 60) * 1000;
        acc_beats_x1000.saturating_add(beats_x1000 as i64)
    }
}
