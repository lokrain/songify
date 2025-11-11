//! Swing feel events.

use crate::{time::SampleTime, traits::HasPosition};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SwingEvent {
    pub position: SampleTime,
    /// Swing ratio * 1000. 500 = straight, ~666 = triplet swing.
    pub ratio_x1000: u16,
}

impl HasPosition for SwingEvent {
    fn position(&self) -> SampleTime {
        self.position
    }
}
