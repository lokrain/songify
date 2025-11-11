# TODO — mt-cli

## Command surface

- [ ] Implement `analyze` command wiring to the new `mt-engine::api::analyze_offline` contract (currently stubbed).
- [ ] Support reading multiple pipeline configs and merging overrides per invocation.
- [ ] Add `--watch` mode for iterative DAW export workflows.
- [ ] Provide `--output-format` flag to choose JSON, NDJSON, or pretty text for analyze/dump.

## UX & ergonomics

- [ ] Offer friendly error messages for common path mistakes (missing stems, unsupported sample rates).
- [ ] Add progress reporting / ETA for long-running analyses.
- [ ] Integrate optional colored output with `--no-colors` toggle for CI.
- [ ] Generate shell completions for bash/zsh/fish and publish them in build artifacts.

## Benchmarking & validation

- [ ] Flesh out `benchmark` command with percentile stats and CPU/alloc counters.
- [ ] Allow `validate` command to compare against golden JSON outputs and print diffs.
- [ ] Add smoke tests covering all subcommands via `assert_cmd`.
- [ ] Introduce integration tests that spin up a temporary project workspace and run CLI end-to-end.

## Docs & packaging

- [ ] Expand README with quickstart recipes and real examples.
- [ ] Document environment variables (e.g., telemetry opt-ins, cache directories).
- [ ] Prepare release checklist (version bump, changelog, binary upload).
- [ ] Provide troubleshooting section for common audio backend issues.
