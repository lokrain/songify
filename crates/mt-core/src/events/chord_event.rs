//! Chord events over time.

use crate::{
    chord::Chord,
    time::SampleTime,
    traits::{HasConfidence, HasPosition},
};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ChordEvent {
    pub chord: Chord,
    pub onset: SampleTime,
    pub offset: SampleTime,
    /// Confidence * 1000 in [0, 1000].
    pub confidence_x1000: u16,
}

impl HasPosition for ChordEvent {
    fn position(&self) -> SampleTime {
        self.onset
    }
}

impl HasConfidence for ChordEvent {
    fn confidence_x1000(&self) -> u16 {
        self.confidence_x1000
    }
}
