//! `mt-cli validate`
//!
//! Runs engine-level validation and optional golden regression suite.

use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use clap::Args;
use serde::Deserialize;
use serde_json::Value;

use mt-engine::api::{analyze_offline, validate_offline};
use mt-engine::types::{AnalyzeRequest, EngineEvent};

#[derive(Debug, Args)]
pub struct ValidateArgs {
    /// Optional pipeline configuration.
    #[arg(long = "pipeline-config")]
    pub pipeline_config: Option<PathBuf>,

    /// Directory with golden cases: each case has input + expected JSON.
    #[arg(long = "golden-dir")]
    pub golden_dir: Option<PathBuf>,
}

#[derive(Debug, Deserialize)]
struct GoldenCase {
    /// Relative or absolute path to audio file.
    audio: PathBuf,
    /// Relative or absolute path to expected JSON (AnalyzeOutput format).
    expected: PathBuf,
}

pub fn run(args: ValidateArgs) -> Result<()> {
    // Basic config validation
    let dummy_req = AnalyzeRequest {
        audio_paths: Vec::new(),
        midi_paths: Vec::new(),
        pipeline_config: args.pipeline_config.clone(),
        project_root: None,
    };
    validate_offline(&dummy_req).context("engine configuration invalid")?;

    if let Some(dir) = args.golden_dir {
        run_golden_suite(&dir, args.pipeline_config.as_ref())?;
    }

    Ok(())
}

fn run_golden_suite(dir: &Path, pipeline_config: Option<&PathBuf>) -> Result<()> {
    let index_path = dir.join("cases.json");
    let data = fs::read_to_string(&index_path)
        .with_context(|| format!("failed to read golden index {}", index_path.display()))?;

    let cases: Vec<GoldenCase> = serde_json::from_str(&data)
        .with_context(|| format!("invalid JSON in {}", index_path.display()))?;

    let mut failures = 0usize;

    for case in cases {
        let audio_path = if case.audio.is_relative() {
            dir.join(&case.audio)
        } else {
            case.audio.clone()
        };
        let expected_path = if case.expected.is_relative() {
            dir.join(&case.expected)
        } else {
            case.expected.clone()
        };

        let req = AnalyzeRequest {
            audio_paths: vec![audio_path.clone()],
            midi_paths: Vec::new(),
            pipeline_config: pipeline_config.cloned(),
            project_root: None,
        };

        let resp = analyze_offline(req).with_context(|| {
            format!("engine analyze_offline failed for {}", audio_path.display())
        })?;

        let expected_str = fs::read_to_string(&expected_path)
            .with_context(|| format!("failed to read expected {}", expected_path.display()))?;
        let expected_json: Value = serde_json::from_str(&expected_str)
            .with_context(|| format!("invalid expected JSON {}", expected_path.display()))?;

        let got_json = encode_events_to_value(&resp)?;

        if !golden_equal(&got_json, &expected_json) {
            eprintln!(
                "golden mismatch: audio={} expected={}",
                audio_path.display(),
                expected_path.display()
            );
            failures += 1;
        }
    }

    if failures > 0 {
        anyhow::bail!("golden suite failed: {failures} mismatches");
    }

    Ok(())
}

fn encode_events_to_value(resp: &mt-engine::types::AnalyzeResponse) -> Result<Value> {
    use mt-engine::types::EngineEvent;

    let mut out = Vec::with_capacity(resp.events.len());

    for ev in &resp.events {
        let (kind, data) = match ev {
            EngineEvent::Tempo(e) => ("tempo", serde_json::to_value(e)?),
            EngineEvent::Meter(e) => ("meter", serde_json::to_value(e)?),
            EngineEvent::Swing(e) => ("swing", serde_json::to_value(e)?),
            EngineEvent::Note(e) => ("note", serde_json::to_value(e)?),
            EngineEvent::Chord(e) => ("chord", serde_json::to_value(e)?),
            EngineEvent::Key(e) => ("key", serde_json::to_value(e)?),
            EngineEvent::Segment(e) => ("segment", serde_json::to_value(e)?),
        };
        out.push(serde_json::json!({ "kind": kind, "data": data }));
    }

    Ok(serde_json::json!({ "events": out }))
}

fn golden_equal(a: &Value, b: &Value) -> bool {
    // Strict equality for now. Can be extended to tolerances later.
    a == b
}
