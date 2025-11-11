//! EngineSnapshot: reduced view over a batch of EngineEvents.
//!
//! For v1 this is a thin wrapper. Higher-level crates can define
//! richer projections if needed.

use crate::types::{EngineEvent, Value};

#[derive(Clone, Debug)]
pub struct EngineSnapshot {
    /// All events emitted in this batch, in emission order.
    pub events: Vec<EngineEvent>,
}

impl EngineSnapshot {
    #[must_use]
    pub fn from_events(events: Vec<EngineEvent>) -> Self {
        Self { events }
    }

    /// Convenience: collect all values of a given node.
    #[must_use]
    pub fn values_for_node(&self, node_id: &str) -> Vec<&Value> {
        self.events
            .iter()
            .filter_map(|e| match e {
                EngineEvent::NodeOutput { node_id: id, value } if id == node_id => Some(value),
                _ => None,
            })
            .collect()
    }
}
