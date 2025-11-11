//! Semantic graph model.
//!
//! Allows consumers to build graphs relating:
//! - concrete events (chords, keys, segments, motifs),
//! - abstract functions (T/S/D),
//! - other semantic nodes.
//!
//! This is a generic, deterministic, in-memory model.

use alloc::vec::Vec;

use mt_core::events::{ChordEvent, KeyEvent, SegmentEvent};
use mt_core::pitch::PitchClass;

use crate::functional_harmony::Function;
use crate::motif::{Motif, MotifInstance};

/// Opaque node identifier (index into `nodes`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct SemanticNodeId(pub u32);

/// Types of semantic nodes we represent.
#[derive(Clone, Debug, PartialEq)]
pub enum SemanticNodeKind {
    /// Concrete chord event at timeline.
    Chord(ChordEvent),
    /// Concrete key event.
    Key(KeyEvent),
    /// Concrete structural segment.
    Segment(SegmentEvent),
    /// A discovered motif with its instances.
    Motif(Motif),
    /// A link to a specific motif instance.
    MotifOccurrence {
        motif_id: SemanticNodeId,
        instance: MotifInstance,
    },
    /// Functional harmony annotation for a chord.
    HarmonyFunction {
        function: Function,
        chord_node: SemanticNodeId,
    },
    /// Generic pitch center or tonic.
    PitchCenter(PitchClass),
}

/// Directed edge kinds.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SemanticEdgeKind {
    /// Time adjacency or progression.
    Next,
    /// "Explains" or annotates another node.
    Annotates,
    /// Part-of relationship (motif occurrence in motif, chord in segment, etc.).
    PartOf,
    /// Generic association.
    Related,
}

/// Directed edge between two semantic nodes.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct SemanticEdge {
    pub from: SemanticNodeId,
    pub to: SemanticNodeId,
    pub kind: SemanticEdgeKind,
}

/// In-memory semantic graph.
#[derive(Debug, Default)]
pub struct SemanticGraph {
    pub nodes: Vec<SemanticNode>,
    pub edges: Vec<SemanticEdge>,
}

/// Node wrapper with id and kind.
#[derive(Clone, Debug, PartialEq)]
pub struct SemanticNode {
    pub id: SemanticNodeId,
    pub kind: SemanticNodeKind,
}

impl SemanticGraph {
    /// Create empty graph.
    #[must_use]
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }

    /// Add a node and return its id.
    pub fn add_node(&mut self, kind: SemanticNodeKind) -> SemanticNodeId {
        let id = SemanticNodeId(self.nodes.len() as u32);
        self.nodes.push(SemanticNode { id, kind });
        id
    }

    /// Add a directed edge.
    pub fn add_edge(&mut self, from: SemanticNodeId, to: SemanticNodeId, kind: SemanticEdgeKind) {
        self.edges.push(SemanticEdge { from, to, kind });
    }

    /// Convenience: link annotation node to its target.
    pub fn annotate(
        &mut self,
        annotator: SemanticNodeId,
        target: SemanticNodeId,
    ) {
        self.add_edge(annotator, target, SemanticEdgeKind::Annotates);
    }

    /// Convenience: mark part-of relation.
    pub fn part_of(
        &mut self,
        part: SemanticNodeId,
        whole: SemanticNodeId,
    ) {
        self.add_edge(part, whole, SemanticEdgeKind::PartOf);
    }

    /// Convenience: link sequential nodes.
    pub fn next(
        &mut self,
        from: SemanticNodeId,
        to: SemanticNodeId,
    ) {
        self.add_edge(from, to, SemanticEdgeKind::Next);
    }
}
