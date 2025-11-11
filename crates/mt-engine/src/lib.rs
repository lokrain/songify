//! mt-engine
//!
//! Deterministic analysis pipeline engine built on:
//! - `mt-core`: semantic primitives
//! - `mt-alloc`: temporal and buffer structures
//! - `mt-analysis`: pluggable detectors
//!
//! Responsibilities:
//! - Interpret `PipelineConfig` (nodes + edges).
//! - Instantiate nodes from a `NodeRegistry`.
//! - Execute a DAG of `DynNode`s on `Value` data.
//! - Emit `EngineEvent`s and `EngineSnapshot`s.
//!
//! This crate does not implement detection logic itself; nodes are
//! thin adapters around `mt-analysis` and related crates.

#![forbid(unsafe_code)]
#![deny(clippy::all, clippy::pedantic)]
#![allow(
    clippy::module_name_repetitions,
    clippy::missing_errors_doc,
    clippy::must_use_candidate,
    clippy::too_many_arguments
)]

pub mod api;
pub mod config;
pub mod engine_session;
pub mod event_bus;
pub mod logging;
pub mod pipeline;
pub mod snapshot;
pub mod types;
pub mod validate;

pub use crate::{
    config::{EngineConfig, NodeConfig, PipelineConfig},
    engine_session::{Engine, EngineBuilder},
    event_bus::EventBus,
    pipeline::{DynNode, NodeRegistry},
    snapshot::EngineSnapshot,
    types::{
        AudioBlock, ENGINE_VERSION, EngineError, EngineEvent, EngineVersion, Value, ValueType,
    },
};
