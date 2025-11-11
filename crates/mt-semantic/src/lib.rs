//! mt-semantic
//!
//! Higher-level semantics built strictly on `mt-core`:
//! - Motifs over melodic lines.
//! - Voice-leading cost between chords.
//! - Functional harmony classification (T/S/D/Other).
//! - Semantic graph to relate events and semantic entities.
//!
//! Constraints:
//! - Deterministic algorithms only.
//! - `no_std` + `alloc` compatible.
//! - No I/O, no logging, no randomness.
//! - All policies are explicit and testable.

#![cfg_attr(not(feature = "std"), no_std)]
#![forbid(unsafe_code)]
#![deny(clippy::all, clippy::pedantic)]
#![allow(
    clippy::module_name_repetitions,
    clippy::missing_errors_doc,
    clippy::missing_const_for_fn,
    clippy::must_use_candidate,
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_sign_loss
)]

extern crate alloc;

pub mod motif;
pub mod voice_leading;
pub mod functional_harmony;
pub mod graph;

pub use motif::{discover_motifs, Motif, MotifConfig, MotifInstance, MotifPattern};
pub use voice_leading::{compute_voice_leading, VoiceLeadingMove, VoiceLeadingSummary};
pub use functional_harmony::{classify_function, Function};
pub use graph::{
    SemanticEdge, SemanticEdgeKind, SemanticGraph, SemanticNode, SemanticNodeId, SemanticNodeKind,
};
