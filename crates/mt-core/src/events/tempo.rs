//! Tempo change events.

use crate::{time::SampleTime, traits::HasPosition};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TempoEvent {
    /// Position in samples where this tempo becomes active.
    pub position: SampleTime,
    /// Tempo in BPM * 1000 (fixed point for determinism).
    pub bpm_x1000: u32,
}

impl HasPosition for TempoEvent {
    fn position(&self) -> SampleTime {
        self.position
    }
}
