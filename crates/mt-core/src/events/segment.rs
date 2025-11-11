//! Structural segment events (intro, verse, etc.).

use crate::{
    time::SampleTime,
    traits::{HasConfidence, HasPosition},
};

/// High-level section kind.
/// `Other(id)` allows callers to define their own labels without heap.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SegmentKind {
    Intro,
    Verse,
    Chorus,
    Bridge,
    Solo,
    Outro,
    Other(u8),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SegmentEvent {
    pub kind: SegmentKind,
    pub onset: SampleTime,
    pub offset: SampleTime,
    pub confidence_x1000: u16,
}

impl HasPosition for SegmentEvent {
    fn position(&self) -> SampleTime {
        self.onset
    }
}

impl HasConfidence for SegmentEvent {
    fn confidence_x1000(&self) -> u16 {
        self.confidence_x1000
    }
}
