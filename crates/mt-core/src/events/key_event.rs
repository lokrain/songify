//! Local or global key estimation events.

use crate::{
    key::Key,
    time::SampleTime,
    traits::{HasConfidence, HasPosition},
};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KeyEvent {
    pub key: Key,
    pub position: SampleTime,
    pub confidence_x1000: u16,
}

impl HasPosition for KeyEvent {
    fn position(&self) -> SampleTime {
        self.position
    }
}

impl HasConfidence for KeyEvent {
    fn confidence_x1000(&self) -> u16 {
        self.confidence_x1000
    }
}
