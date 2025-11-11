//! Tiny shared traits used across event types.
//!
//! These are intentionally minimal to avoid over-abstracting mt-core.

use crate::time::SampleTime;

/// Types that have a primary timeline position.
pub trait HasPosition {
    fn position(&self) -> SampleTime;
}

/// Types that carry a confidence value in [0, 1000].
pub trait HasConfidence {
    fn confidence_x1000(&self) -> u16;
}
