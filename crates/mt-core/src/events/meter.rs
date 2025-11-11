//! Time signature (meter) change events.

use crate::{time::SampleTime, traits::HasPosition};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MeterEvent {
    pub position: SampleTime,
    pub numerator: u8,
    pub denominator: u8,
}

impl HasPosition for MeterEvent {
    fn position(&self) -> SampleTime {
        self.position
    }
}
