# TODO — mt-analysis

## Algorithm depth

- [ ] Implement hybrid chord detector that fuses rule-based and template-matching scores instead of picking one or the other.
- [ ] Add adaptive key histogram weighting using tempo-aware window sizes.
- [ ] Extend `SimpleAudioNoteAnalyzer` with multi-channel support and energy-based downmix.
- [ ] Teach tempo/meter detector to emit confidence scores alongside events.
- [ ] Introduce optional ML-powered note detector slot guarded by feature flag.

## Config & UX

- [ ] Provide serde schemas/examples for all configs so CLI users can author pipeline settings safely.
- [ ] Add `AnalysisConfig::validate` to reject nonsensical combinations (e.g., hop > frame).
- [ ] Support runtime tuning presets (Safe/Balanced/Ultra) in the config layer.
- [ ] Document expected latency budgets per analyzer and expose them in metadata.

## Performance & determinism

- [ ] Benchmark per-analyzer latency with criterion and surface regressions via CI gates.
- [ ] Replace temporary `Vec` allocations in MIDI analyzer with smallvec to reduce churn.
- [ ] Verify audio analyzer determinism across architectures (f32 rounding).
- [ ] Add optional parallelisation hooks guarded by deterministic scheduling tests.

## Testing & evaluation

- [ ] Build golden MIDI suites for jazz, pop, and classical excerpts to evaluate chord/key accuracy.
- [ ] Add synthetic audio fixtures (sine sweeps, arpeggios) for regression coverage of audio note detection.
- [ ] Create property tests ensuring swing detector never reports ratios below 0.5 or above 0.85.
- [ ] Wire up snapshot tests that diff analyzer outputs against stored JSON expectations.

## Documentation & tooling

- [ ] Produce architecture doc describing the pipeline contract for new analyzers.
- [ ] Publish per-analyzer tuning guides (e.g., how window sizes affect chord latency).
- [ ] Provide developer script to regenerate golden fixtures from DAW exports.
- [ ] Add docstrings to trait methods clarifying ownership and expected ordering guarantees.
