# mt-engine

Deterministic, config-driven analysis engine for the Music Theory & Analytics workspace.

Responsibilities:

- Load a pipeline description (nodes + edges).
- Instantiate nodes via a registry of implementations.
- Execute a directed acyclic graph (DAG) of analysis nodes.
- Emit structured `EngineEvent`s and `EngineSnapshot`s for consumers.
- Provide a stable surface for future plugins and external tools.

mt-engine is **orchestration only**. It does not implement music theory or DSP.
Those live in:

- `mt-core` — semantic kernel (pitches, chords, keys, events).
- `mt-signal-core` — low-level DSP primitives.
- `mt-alloc` — heap-based tempo maps, buffers, queues.
- `mt-analysis` — detectors and feature extractors.

This crate wires them into reusable pipelines.

## Design

- `std` crate (uses `Vec`, `String`, collections).
- Deterministic by construction:
  - No randomness.
  - No global mutable state.
  - Same inputs + config + versions → same outputs.
- Graph-based:
  - Nodes implement stable contracts (`DynNode`).
  - Pipelines defined declaratively via `PipelineConfig`.
  - NodeRegistry maps string IDs to implementations.
- Plugin-ready:
  - Dynamic plugins can later register additional node factories without changing the core.

## Core concepts

- **ValueType / Value**

  Stable type universe for data moving through the graph:
  audio blocks, MIDI events, note/chord/key/segment events, etc.

- **DynNode**

  Type-erased execution unit:
  - exposes `input_type`, `output_type`, `process_dyn(Value)`.

- **Typed nodes**

  Core nodes can be implemented with strong types and wrapped into `DynNode`
  using the provided adapter traits.

- **PipelineConfig**

  Serializable description of:
  - nodes (with `impl_id`),
  - edges (connections),
  - optional parameters.

- **Engine**

  High-level wrapper:
  - builds a validated pipeline from config + registry,
  - runs the DAG for given inputs,
  - yields `EngineSnapshot` with timeline events.

## Guarantees

- If `PipelineConfig`, node registry, engine version, and inputs are identical,
  outputs are identical.
- Any breaking change to:
  - `ValueType`,
  - node IDs,
  - pipeline validation rules,
  requires a semver major bump.

## Typical usage

- Library/SDK:
  - Construct `NodeRegistry` with built-in and custom nodes.
  - Load `PipelineConfig` (e.g. from TOML/JSON).
  - Build `Engine` via `EngineBuilder`.
  - Call `run_once` or feed slices incrementally and collect snapshots.

- CLI (`mt-cli`):
  - Wires files → `mt-engine` → JSON/MIDI exports.
