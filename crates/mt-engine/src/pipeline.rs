//! Graph runtime:
//! - Typed-node abstraction.
//! - DynNode wrapper using `Value`.
//! - NodeRegistry.
//! - PipelineGraph execution.

use std::collections::BTreeMap;

use crate::{
    config::{EdgeConfig, NodeConfig, PipelineConfig},
    types::{EngineError, EngineEvent, Value, ValueType},
};

/// Trait for strongly-typed node implementations.
///
/// I: input domain type
/// O: output domain type
pub trait TypedNode<I, O>: Send {
    fn id(&self) -> &'static str;
    fn process(&mut self, input: I) -> Result<O, EngineError>;
}

/// Convert from dynamic `Value` into a typed input.
pub trait FromValue: Sized {
    fn from_value(v: Value) -> Result<Self, EngineError>;
}

/// Convert typed output into dynamic `Value`.
pub trait IntoValue {
    fn into_value(self) -> Value;
}

// Implementations for concrete types used in v1.

impl FromValue for () {
    fn from_value(v: Value) -> Result<Self, EngineError> {
        match v {
            Value::Unit => Ok(()),
            other => Err(EngineError::TypeMismatch {
                node_id: "<unit>".to_string(),
                expected: ValueType::Unit,
                actual: value_type_of(&other),
            }),
        }
    }
}

impl IntoValue for () {
    fn into_value(self) -> Value {
        Value::Unit
    }
}

impl FromValue for Value {
    fn from_value(v: Value) -> Result<Self, EngineError> {
        Ok(v)
    }
}

impl IntoValue for Value {
    fn into_value(self) -> Value {
        self
    }
}

// Helper to compute ValueType at runtime.
fn value_type_of(v: &Value) -> ValueType {
    match v {
        Value::Unit => ValueType::Unit,
        Value::AudioBlock(_) => ValueType::AudioBlock,
        Value::MidiEvents(_) => ValueType::MidiEvents,
        Value::NoteEvents(_) => ValueType::NoteEvents,
        Value::ChordEvents(_) => ValueType::ChordEvents,
        Value::KeyEvents(_) => ValueType::KeyEvents,
        Value::SegmentEvents(_) => ValueType::SegmentEvents,
        Value::TempoEvents(_) => ValueType::TempoEvents,
        Value::MeterEvents(_) => ValueType::MeterEvents,
        Value::SwingEvents(_) => ValueType::SwingEvents,
    }
}

/// Type-erased node used inside the graph.
///
/// Implementations must be deterministic and side-effect free (modulo output).
pub trait DynNode: Send {
    /// Stable implementation ID (not instance ID).
    fn impl_id(&self) -> &'static str;

    /// Instance ID in this pipeline.
    fn instance_id(&self) -> &str;

    /// Expected input value type.
    fn input_type(&self) -> ValueType;

    /// Output value type.
    fn output_type(&self) -> ValueType;

    /// Process one input value into one output value.
    fn process_dyn(&mut self, input: Value) -> Result<Value, EngineError>;
}

/// Generic adapter from a strongly-typed node to DynNode.
pub struct NodeAdapter<I, O, N>
where
    I: FromValue,
    O: IntoValue,
    N: TypedNode<I, O>,
{
    instance_id: String,
    impl_id: &'static str,
    input_type: ValueType,
    output_type: ValueType,
    inner: N,
    _phantom_i: core::marker::PhantomData<I>,
    _phantom_o: core::marker::PhantomData<O>,
}

impl<I, O, N> NodeAdapter<I, O, N>
where
    I: FromValue,
    O: IntoValue,
    N: TypedNode<I, O>,
{
    pub fn new(instance_id: String, input_type: ValueType, output_type: ValueType, inner: N) -> Self {
        Self {
            impl_id: inner.id(),
            instance_id,
            input_type,
            output_type,
            inner,
            _phantom_i: core::marker::PhantomData,
            _phantom_o: core::marker::PhantomData,
        }
    }
}

impl<I, O, N> DynNode for NodeAdapter<I, O, N>
where
    I: FromValue,
    O: IntoValue,
    N: TypedNode<I, O>,
{
    fn impl_id(&self) -> &'static str {
        self.impl_id
    }

    fn instance_id(&self) -> &str {
        &self.instance_id
    }

    fn input_type(&self) -> ValueType {
        self.input_type
    }

    fn output_type(&self) -> ValueType {
        self.output_type
    }

    fn process_dyn(&mut self, input: Value) -> Result<Value, EngineError> {
        let typed_input = I::from_value(input)?;
        let typed_output = self.inner.process(typed_input)?;
        Ok(typed_output.into_value())
    }
}

/// Factory signature used by the node registry.
pub type DynNodeFactory = fn(&NodeConfig) -> Result<Box<dyn DynNode>, EngineError>;

/// Registry of available node implementations.
///
/// Keys are stable implementation IDs, e.g.:
/// - "identity"
/// - "mt.analysis.chord_detector.v1"
pub struct NodeRegistry {
    factories: BTreeMap<&'static str, DynNodeFactory>,
}

impl NodeRegistry {
    #[must_use]
    pub fn new() -> Self {
        Self {
            factories: BTreeMap::new(),
        }
    }

    /// Register a factory under a stable impl ID.
    pub fn register(&mut self, impl_id: &'static str, factory: DynNodeFactory) {
        self.factories.insert(impl_id, factory);
    }

    /// Instantiate a node from config.
    pub fn build(
        &self,
        cfg: &NodeConfig,
    ) -> Result<Box<dyn DynNode>, EngineError> {
        let factory = self
            .factories
            .get(cfg.impl_id.as_str())
            .ok_or_else(|| EngineError::NodeNotFound(cfg.impl_id.clone()))?;
        factory(cfg)
    }
}

/// One edge in the compiled graph: from node index to node index.
#[derive(Clone, Copy, Debug)]
struct Edge {
    from: usize,
    to: usize,
}

/// Compiled, validated pipeline graph.
pub struct PipelineGraph {
    /// Nodes stored in topological order.
    nodes: Vec<Box<dyn DynNode>>,
    /// Edges referencing indices into `nodes`.
    edges: Vec<Edge>,
}

impl PipelineGraph {
    /// Build graph from config + registry (no validation besides IDs).
    pub fn from_config(
        cfg: &PipelineConfig,
        registry: &NodeRegistry,
    ) -> Result<Self, EngineError> {
        // Map instance id -> index
        let mut index_of: BTreeMap<&str, usize> = BTreeMap::new();
        let mut nodes: Vec<Box<dyn DynNode>> = Vec::with_capacity(cfg.nodes.len());

        for node_cfg in &cfg.nodes {
            if index_of.contains_key(node_cfg.id.as_str()) {
                return Err(EngineError::DuplicateNodeId(node_cfg.id.clone()));
            }
            let node = registry.build(node_cfg)?;
            let idx = nodes.len();
            index_of.insert(node_cfg.id.as_str(), idx);
            nodes.push(node);
        }

        // Build edges
        let mut edges = Vec::with_capacity(cfg.edges.len());
        for e in &cfg.edges {
            let from_idx = *index_of
                .get(e.from.as_str())
                .ok_or_else(|| EngineError::EdgeReferencesUnknownNode {
                    from: e.from.clone(),
                    to: e.to.clone(),
                })?;
            let to_idx = *index_of
                .get(e.to.as_str())
                .ok_or_else(|| EngineError::EdgeReferencesUnknownNode {
                    from: e.from.clone(),
                    to: e.to.clone(),
                })?;
            edges.push(Edge { from: from_idx, to: to_idx });
        }

        // Simple cycle check using DFS.
        if has_cycle(nodes.len(), &edges) {
            return Err(EngineError::CycleInGraph);
        }

        Ok(Self { nodes, edges })
    }

    /// Execute the pipeline once starting from a given entry node input.
    ///
    /// v1 model:
    /// - Caller is responsible for choosing the entry node and wiring:
    ///   pass an initial `Value` to that node's `process_dyn`.
    /// - Edges propagate outputs to downstream nodes; last outputs
    ///   generate EngineEvents.
    pub fn execute(
        &mut self,
        entry_node_id: &str,
        entry_input: Value,
    ) -> Result<Vec<EngineEvent>, EngineError> {
        let mut events = Vec::new();

        // Map node index -> pending input value presence.
        let mut pending: Vec<Option<Value>> = vec![None; self.nodes.len()];

        // Find entry index.
        let entry_idx = self
            .nodes
            .iter()
            .enumerate()
            .find(|(_, n)| n.instance_id() == entry_node_id)
            .map(|(i, _)| i)
            .ok_or_else(|| EngineError::NodeNotFound(entry_node_id.to_string()))?;

        pending[entry_idx] = Some(entry_input);

        // Execute nodes in index order.
        // Assumes config was built in topological order or at least acyclic.
        for idx in 0..self.nodes.len() {
            if let Some(input) = pending[idx].take() {
                let node = &mut self.nodes[idx];
                let expected = node.input_type();
                let actual = value_type_of(&input);
                if expected != actual {
                    return Err(EngineError::TypeMismatch {
                        node_id: node.instance_id().to_string(),
                        expected,
                        actual,
                    });
                }
                let output = node.process_dyn(input)?;
                let out_ty = node.output_type();

                // Route to children.
                for edge in self.edges.iter().filter(|e| e.from == idx) {
                    // For v1 we route the same value clone to all downstream nodes (fan-out).
                    pending[edge.to] = Some(output.clone());
                }

                // Emit event for this node's output.
                events.push(EngineEvent::NodeOutput {
                    node_id: node.instance_id().to_string(),
                    value: match out_ty {
                        _ => output,
                    },
                });
            }
        }

        Ok(events)
    }
}

// Simple DFS-based cycle detection.
fn has_cycle(node_count: usize, edges: &[Edge]) -> bool {
    #[derive(Copy, Clone, PartialEq, Eq)]
    enum Mark {
        Temp,
        Perm,
    }

    fn visit(
        v: usize,
        edges: &[Edge],
        marks: &mut [Option<Mark>],
    ) -> bool {
        if matches!(marks[v], Some(Mark::Temp)) {
            return true;
        }
        if matches!(marks[v], Some(Mark::Perm)) {
            return false;
        }
        marks[v] = Some(Mark::Temp);
        for e in edges.iter().filter(|e| e.from == v) {
            if visit(e.to, edges, marks) {
                return true;
            }
        }
        marks[v] = Some(Mark::Perm);
        false
    }

    let mut marks: Vec<Option<Mark>> = vec![None; node_count];
    for v in 0..node_count {
        if marks[v].is_none() && visit(v, edges, &mut marks) {
            return true;
        }
    }
    false
}

/// Helper accessible inside crate.
pub(crate) fn value_type_of(v: &Value) -> ValueType {
    match v {
        Value::Unit => ValueType::Unit,
        Value::AudioBlock(_) => ValueType::AudioBlock,
        Value::MidiEvents(_) => ValueType::MidiEvents,
        Value::NoteEvents(_) => ValueType::NoteEvents,
        Value::ChordEvents(_) => ValueType::ChordEvents,
        Value::KeyEvents(_) => ValueType::KeyEvents,
        Value::SegmentEvents(_) => ValueType::SegmentEvents,
        Value::TempoEvents(_) => ValueType::TempoEvents,
        Value::MeterEvents(_) => ValueType::MeterEvents,
        Value::SwingEvents(_) => ValueType::SwingEvents,
    }
}
