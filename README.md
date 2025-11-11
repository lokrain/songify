# Music Theory & Analytics Engine

A real‑time harmony, key, chord, structure, and performance analysis engine designed for **professional DAW environments**, with **research‑grade correctness** and a **clear pedagogical representation layer** suitable for teaching, visual explanation, and guided composition.

This project provides:

* **A portable Rust core**, `no_std` at the theoretical layer.
* **Real‑time safe processing path** for VST3/AU/AAX plugin environments.
* **Dual‑domain time model**: sample‑accurate DSP + beat/bar grid with tempo, meter, and swing mapping.
* **Normalized musical event representation** suitable for both **analysis** and **instructional visualization**.
* **Replaceable analysis strategies** (rule‑based, template‑matching, ML‑assisted).
* **High‑level engine API** used by desktop, plugin, CLI, and eventually web.

---

## High‑Level Identity

| Aspect              | Value                                                                          |
| ------------------- | ------------------------------------------------------------------------------ |
| Primary Orientation | **Professional DAW Plugin Engine**                                             |
| Secondary Support   | **Research‑grade analysis reproducibility**                                    |
| Tertiary Support    | **Music‑education‑friendly symbolic output**                                   |
| Target Runs         | Plugin hosts (Ableton, FL, Logic, Reaper, Studio One), Desktop apps, CLI tools |
| Language            | Rust 2024, workspace multi‑crate architecture                                  |

---

## Core Principles

1. **Real‑Time Safety**

   * Audio thread API (`EngineRt`) performs **no allocations, no locks, no syscalls, no panics**.
   * Analysis state updates occur in **non‑RT** (`EngineSession`).

2. **Portable Theory First**

   * `mt-core` contains all theory primitives and musical event types.
   * Works in embedded, plugin, web, server environments.

3. **Dual Time Model (Reality‑Faithful)**

   * Every event may be referenced in:

     * **SampleTime (u64)** – exact DSP timeline.
     * **MusicalTime (bars / beats / ticks)** – human/music notation grid.
   * Conversions via pure tempo map.

4. **Normalized Notes Are Canonical**

   * Raw MIDI and audio detection both resolve to **NoteSpan**.
   * Harmony, key, structure operate on normalized representation.

5. **Analysis Pipeline is Fixed Structure + Pluggable Algorithms**

   * Skeleton pipeline is stable for clients.
   * Each stage is replaceable via traits.

6. **Pedagogical Transparency**

   * Harmony and structure events include **confidence** + **explanation hooks**.
   * Enables guided teaching and visual breakdown (e.g., Roman numeral view).

---

## Workspace Layout

```sh
mt-core/           # Theory & musical primitives (no_std)
mt-signal-core/    # Audio math & framing (no_std)
mt-alloc/          # Heap structures: tempo map, note normalization, queues
mt-analysis/       # Pluggable detectors (notes, harmony, key, segments)
mt-engine/         # Streaming engine (EngineRt + EngineSession)
mt-ffi/            # C ABI for plugin shells or host integration
mt-cli/            # Offline analysis tools for research/testing
mt-semantic/       # (Optional) Functional harmony, motif graphs, pedagogy tools
```

---

## Engine Model

```sh
  Ingest (RT)                Analysis (non‑RT)             Client API
┌───────────────┐       ┌─────────────────────┐       ┌──────────────────┐
│ EngineRt      │  -->  │ EngineSession       │  -->  │ snapshot(),       │
│ push_audio()  │       │ note→chord→key→seg  │       │ poll_events()     │
│ push_midi()   │       │ confidence scoring  │       │ timeline queries   │
└───────────────┘       └─────────────────────┘       └──────────────────┘
```

* **EngineRt** lives on the DAW real‑time thread.
* **EngineSession** performs analysis asynchronously.

---

## Example Public API Usage

```rust
let mut rt = EngineRt::new(config);
let mut session = EngineSession::new();

rt.push_audio_block(block_id, audio);
rt.push_midi(block_id, midi_events);

session.poll_events(&mut out);
let snapshot = session.snapshot_state();
```

---

## Supported Use Cases

| Use Case                            | Status            | Notes                     |
| ----------------------------------- | ----------------- | ------------------------- |
| Real‑time chord display in DAW      | ✅ Supported       | Low‑latency pipeline      |
| Automatic key + scale detection     | ✅ Supported       | Rule‑based & histogram    |
| Teaching visualizer: Roman numerals | ✅ Core compatible | Export via mt-semantic    |
| Motif + thematic analysis           | ⚙️ Optional       | In `mt-semantic`          |
| ML‑assisted harmony inference       | 🔜 Pluggable      | Strategy interface exists |

---

## Roadmap

### v0.1 (Foundation)

* Core theory correctness
* Real‑time safe engine
* Rule‑based chord + key detection

### v0.2 (Education & Visualization)

* Roman numeral layer
* Section/phrase segmentation tuning

### v0.3 (Advanced Analysis)

* ML/AI assisted chord probability maps
* Motif graph integration

---

## Development Standards

| Area       | Rule                                                        |
| ---------- | ----------------------------------------------------------- |
| Linting    | `#![deny(clippy::pedantic)]` everywhere                     |
| Code Style | `rustfmt.toml` enforced via CI                              |
| Tests      | Golden datasets + property tests for theory correctness     |
| RT Safety  | No alloc, no logs, no locks in `EngineRt` enforced by tests |
| Versioning | SemVer + ABI version field in mt-ffi                        |

---

## License & Governance

TBD based on distribution strategy: commercial plugin, dual-license core, or open-core.

A `LICENSE.md` placeholder should be added once business model is finalized.

---

This README will remain the *authoritative onboarding point*.

For full pipelines and examples see **ARCHITECTURE.md** (generated next).
