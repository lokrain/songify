//! High-level Engine wrapper around PipelineGraph.
//!
//! This keeps engine usage simple for SDK/CLI callers.

use crate::{
    config::EngineConfig,
    event_bus::EventBus,
    pipeline::{NodeRegistry, PipelineGraph},
    snapshot::EngineSnapshot,
    types::{EngineError, Value},
};

/// Builder that wires config + registry into a validated Engine.
pub struct EngineBuilder<'a> {
    cfg: &'a EngineConfig,
    registry: &'a NodeRegistry,
}

impl<'a> EngineBuilder<'a> {
    #[must_use]
    pub fn new(cfg: &'a EngineConfig, registry: &'a NodeRegistry) -> Self {
        Self { cfg, registry }
    }

    /// Build an `Engine` with a compiled, validated graph.
    pub fn build(self) -> Result<Engine, EngineError> {
        let graph = PipelineGraph::from_config(&self.cfg.pipeline, self.registry)?;
        Ok(Engine {
            graph,
            bus: EventBus::new(),
        })
    }
}

/// Engine instance.
///
/// Stateless w.r.t. configuration; state is inside nodes that live
/// in the `PipelineGraph`.
pub struct Engine {
    graph: PipelineGraph,
    bus: EventBus,
}

impl Engine {
    /// Execute the pipeline once, starting from the given entry node.
    ///
    /// Returns an `EngineSnapshot` containing all emitted events.
    pub fn run_once(
        &mut self,
        entry_node_id: &str,
        input: Value,
    ) -> Result<EngineSnapshot, EngineError> {
        let events = self.graph.execute(entry_node_id, input)?;
        self.bus.append(events);
        Ok(EngineSnapshot::from_events(self.bus.drain()))
    }
}
