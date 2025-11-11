//! Validation helpers for pipeline configs.
//!
//! These functions are used implicitly by PipelineGraph::from_config,
//! but are also available for tooling / CLI.

use std::collections::BTreeSet;

use crate::{
    config::PipelineConfig,
    types::EngineError,
};

/// Basic structural validation of a PipelineConfig.
pub fn validate_pipeline_config(cfg: &PipelineConfig) -> Result<(), EngineError> {
    let mut ids = BTreeSet::new();
    for n in &cfg.nodes {
        if !ids.insert(&n.id) {
            return Err(EngineError::DuplicateNodeId(n.id.clone()));
        }
    }

    for e in &cfg.edges {
        if !ids.contains(e.from.as_str()) || !ids.contains(e.to.as_str()) {
            return Err(EngineError::EdgeReferencesUnknownNode {
                from: e.from.clone(),
                to: e.to.clone(),
            });
        }
    }

    Ok(())
}
