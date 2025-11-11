//! Public, stable API surface for `mt-engine`.
//!
//! This module is the only thing `mt-cli`, `mt-ffi`, and external SDK users
//! should depend on.
//!
//! Responsibilities:
//! - Provide deterministic, high-level entrypoints.
//! - Hide internal engine structure (`engine_rt`, `engine_session`, `pipeline`).
//! - Enforce validation and consistent error mapping.
//!
//! Non-responsibilities:
//! - No ad-hoc feature flags.
//! - No global mutable state.
//! - No random behavior.

use std::path::Path;

use crate::{
    config::EngineConfig,
    engine_session::OfflineSession,
    pipeline::Pipeline,
    types::{AnalyzeRequest, AnalyzeResponse, EngineError},
    validate,
};

/// Run a full offline analysis for the given request.
///
/// Deterministic contract:
/// - For the same `AnalyzeRequest` (paths, pipeline config, project root),
///   same engine version, same catalogs:
///   - the returned `AnalyzeResponse` is bit-for-bit identical.
/// - No hidden randomness.
/// - All non-path-related errors are reported as `EngineError` values.
///
/// Expected behavior:
/// - Loads or synthesizes an `EngineConfig`.
/// - Builds a `Pipeline` from that config.
/// - Creates an `OfflineSession`.
/// - Ingests all audio and MIDI inputs in deterministic order.
/// - Finalizes the session and returns collected events.
pub fn analyze_offline(req: AnalyzeRequest) -> Result<AnalyzeResponse, EngineError> {
    ensure_has_input(&req)?;

    let engine_cfg = load_engine_config(req.pipeline_config.as_deref())?;
    let pipeline = build_pipeline(&engine_cfg)?;
    let mut session = OfflineSession::new(engine_cfg, pipeline, req.project_root.clone())?;

    // Ingest audio files in the given order.
    for path in &req.audio_paths {
        session.ingest_audio_file(path)?;
    }

    // Ingest MIDI files in the given order.
    for path in &req.midi_paths {
        session.ingest_midi_file(path)?;
    }

    // Finalize and collect deterministic results.
    session.finalize()
}

/// Validate that the current engine configuration, catalogs, and pipeline
/// are structurally sound for the given request.
///
/// This does not run a full analysis and does not produce events.
///
/// Typical uses:
/// - CI sanity checks.
/// - `mt-cli validate` before running golden suites.
/// - API surface for embedding applications.
pub fn validate_offline(req: &AnalyzeRequest) -> Result<(), EngineError> {
    let engine_cfg = load_engine_config(req.pipeline_config.as_deref())?;
    let pipeline = build_pipeline(&engine_cfg)?;

    // Run internal validation hooks:
    // - catalogs are loaded and versioned,
    // - node registry is consistent,
    // - graph has no cycles and valid port wiring,
    // - RT/NR constraints as configured are coherent.
    validate::validate_engine(&engine_cfg, &pipeline)
}

/// Ensure at least one input is provided.
fn ensure_has_input(req: &AnalyzeRequest) -> Result<(), EngineError> {
    if req.audio_paths.is_empty() && req.midi_paths.is_empty() {
        return Err(EngineError::InvalidInput(
            "at least one audio or MIDI input is required".into(),
        ));
    }
    Ok(())
}

/// Load engine config from an optional path using `EngineConfig` defaults.
///
/// Contract for `EngineConfig::from_path_or_default`:
/// - If `Some(path)`: load and parse config deterministically.
/// - If `None`: return a deterministic default configuration.
/// - On error: return `EngineError::InvalidInput` or `EngineError::Internal`
///   with a stable message.
fn load_engine_config(path: Option<&Path>) -> Result<EngineConfig, EngineError> {
    EngineConfig::from_path_or_default(path)
}

/// Build a pipeline (node graph) for the given config.
///
/// Contract for `Pipeline::build`:
/// - Deterministic for a given `EngineConfig` and compiled-in node registry.
/// - Returns `EngineError::Pipeline` on any structural/semantic issue.
fn build_pipeline(cfg: &EngineConfig) -> Result<Pipeline, EngineError> {
    Pipeline::build(cfg)
}
