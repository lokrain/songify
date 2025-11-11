# mt-cli

Command-line interface for the Music Theory & Analytics engine.

It provides deterministic, scriptable access to the `mt-engine` graph:

- Run offline analysis on audio/MIDI files.
- Dump timelines and events as JSON or text.
- Benchmark pipeline performance.
- Validate engine configuration and golden datasets.

This is the primary operational surface for:

- CI regression,
- batch analysis,
- power users who prefer the terminal.

## Design

- Thin wrapper over `mt-engine::api`.
- No music-theory logic implemented here.
- All behavior is deterministic and versioned through `mt-engine`.
- Output formats are stable and machine-friendly (JSON, line-based).

## Commands

### `analyze`

Run full offline analysis and emit results as JSON.

```bash
mt-cli analyze \
  --audio path/to/file.wav \
  --audio path/to/other.wav \
  --midi path/to/file.mid \
  --pipeline-config config/pipeline.toml \
  --project-root out/project-dir \
  --output out/result.json
