//! Engine and pipeline configuration.
//!
//! Can be constructed programmatically or deserialized (with `serde` feature).

use crate::types::ValueType;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Configuration for a single node instance in the pipeline.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct NodeConfig {
    /// Unique identifier of this node instance within the pipeline.
    pub id: String,
    /// Implementation ID, resolved via `NodeRegistry`.
    pub impl_id: String,
    /// Declared input type for validation (optional hint).
    #[cfg_attr(feature = "serde", serde(default))]
    pub input_type: Option<ValueType>,
    /// Declared output type for validation (optional hint).
    #[cfg_attr(feature = "serde", serde(default))]
    pub output_type: Option<ValueType>,
}

/// Configuration for a directed edge between nodes.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct EdgeConfig {
    /// ID of source node.
    pub from: String,
    /// ID of target node.
    pub to: String,
}

/// Pipeline configuration: nodes + edges.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct PipelineConfig {
    /// Logical identifier for this pipeline (human + tooling).
    pub id: String,
    pub nodes: Vec<NodeConfig>,
    pub edges: Vec<EdgeConfig>,
}

/// Engine configuration wraps a pipeline; additional fields can be added later.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct EngineConfig {
    pub pipeline: PipelineConfig,
}

impl EngineConfig {
    #[must_use]
    pub fn new(pipeline: PipelineConfig) -> Self {
        Self { pipeline }
    }
}
