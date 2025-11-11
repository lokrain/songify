//! Shared engine-level types:
//! - EngineError
//! - EngineVersion
//! - ValueType / Value (type universe for nodes)
//! - AudioBlock
//! - EngineEvent

use std::fmt;

use mt-core::events::{
    ChordEvent, KeyEvent, MeterEvent, NoteEvent, SegmentEvent, SwingEvent, TempoEvent,
};
use mt-core::midi::MidiEvent;

/// Version of the engine core.
///
/// Update when behavior or public contracts change.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EngineVersion {
    pub major: u16,
    pub minor: u16,
    pub patch: u16,
}

impl fmt::Display for EngineVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

/// Current engine version.
///
/// Bump on breaking or behavior-significant changes.
pub const ENGINE_VERSION: EngineVersion = EngineVersion {
    major: 0,
    minor: 1,
    patch: 0,
};

/// Engine-level error type.
///
/// Used for config, graph, and execution failures.
#[derive(Debug)]
pub enum EngineError {
    InvalidConfig(&'static str),
    NodeNotFound(String),
    DuplicateNodeId(String),
    EdgeReferencesUnknownNode {
        from: String,
        to: String,
    },
    CycleInGraph,
    TypeMismatch {
        node_id: String,
        expected: ValueType,
        actual: ValueType,
    },
    ExecutionFailed {
        node_id: String,
        message: &'static str,
    },
}

impl fmt::Display for EngineError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidConfig(msg) => write!(f, "invalid config: {msg}"),
            Self::NodeNotFound(id) => write!(f, "node not found: {id}"),
            Self::DuplicateNodeId(id) => write!(f, "duplicate node id: {id}"),
            Self::EdgeReferencesUnknownNode { from, to } => {
                write!(f, "edge references unknown node(s): {from} -> {to}")
            }
            Self::CycleInGraph => write!(f, "pipeline contains a cycle"),
            Self::TypeMismatch {
                node_id,
                expected,
                actual,
            } => write!(
                f,
                "type mismatch at node `{node_id}`: expected {expected:?}, got {actual:?}"
            ),
            Self::ExecutionFailed { node_id, message } => {
                write!(f, "execution failed at node `{node_id}`: {message}")
            }
        }
    }
}

impl std::error::Error for EngineError {}

/// Value "shape" for graph edges.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ValueType {
    Unit,
    AudioBlock,
    MidiEvents,
    NoteEvents,
    ChordEvents,
    KeyEvents,
    SegmentEvents,
    TempoEvents,
    MeterEvents,
    SwingEvents,
}

/// Audio block used at engine boundaries.
///
/// Samples are interleaved. Size is unconstrained; chunking happens above.
#[derive(Clone, Debug)]
pub struct AudioBlock {
    pub sample_rate: u32,
    pub channels: u16,
    pub frames: Vec<f32>,
}

/// Dynamic value passed along edges.
///
/// This is the only type-erased container inside the engine.
/// Actual node implementations should work on concrete types and use
/// the adapter traits in `pipeline` for conversion.
#[derive(Clone, Debug)]
pub enum Value {
    Unit,
    AudioBlock(AudioBlock),
    MidiEvents(Vec<MidiEvent>),
    NoteEvents(Vec<NoteEvent>),
    ChordEvents(Vec<ChordEvent>),
    KeyEvents(Vec<KeyEvent>),
    SegmentEvents(Vec<SegmentEvent>),
    TempoEvents(Vec<TempoEvent>),
    MeterEvents(Vec<MeterEvent>),
    SwingEvents(Vec<SwingEvent>),
}

/// High-level events emitted by the engine during or after execution.
///
/// For v1 this is minimal; can be extended in a backward-compatible way.
#[derive(Clone, Debug)]
pub enum EngineEvent {
    /// Node produced an output value.
    NodeOutput { node_id: String, value: Value },
}
