//! Simple in-memory event bus for engine outputs.
//!
//! For v1 this is just a buffer; can be replaced or extended later
//! without changing the graph core.

use crate::types::EngineEvent;

#[derive(Default)]
pub struct EventBus {
    events: Vec<EngineEvent>,
}

impl EventBus {
    #[must_use]
    pub fn new() -> Self {
        Self { events: Vec::new() }
    }

    pub fn append<I>(&mut self, it: I)
    where
        I: IntoIterator<Item = EngineEvent>,
    {
        self.events.extend(it);
    }

    #[must_use]
    pub fn drain(&mut self) -> Vec<EngineEvent> {
        let mut out = Vec::new();
        std::mem::swap(&mut out, &mut self.events);
        out
    }
}
