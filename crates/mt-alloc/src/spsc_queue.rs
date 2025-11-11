//! SpscQueue:
//! - Single-producer, single-consumer queue abstraction.
//! - Implementation is single-threaded and safe; can be used in RT-ish contexts.
//! - For true cross-thread lock-free behavior, implement in a separate crate
//!   or relax `unsafe` constraints in a small, audited module.


/// Bounded SPSC queue (logical contract).
///
/// Implementation:
/// - Internally same as EventRing.
/// - Caller must ensure only one producer and one consumer touch it at a time,
///   or wrap in appropriate synchronization in higher layers.
#[derive(Debug)]
pub struct SpscQueue<T> {
    ring: super::event_ring::EventRing<T>,
}

impl<T> SpscQueue<T> {
    pub fn with_capacity(cap: usize) -> Self {
        Self {
            ring: super::event_ring::EventRing::with_capacity(cap),
        }
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.ring.is_empty()
    }

    #[must_use]
    pub fn is_full(&self) -> bool {
        self.ring.is_full()
    }

    pub fn push(&mut self, value: T) -> Result<(), T> {
        self.ring.push(value)
    }

    pub fn pop(&mut self) -> Option<T> {
        self.ring.pop()
    }
}
