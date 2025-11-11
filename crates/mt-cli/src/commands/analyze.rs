//! `mt-cli analyze`
//!
//! Runs full offline analysis and writes JSON to stdout or file.

use std::fs::File;
use std::io::{self, Write};
use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::Args;
use serde::Serialize;

use mt-engine::types::{AnalyzeRequest, AnalyzeResponse};
use mt-engine::api::analyze_offline;

#[derive(Debug, Args)]
pub struct AnalyzeArgs {
    /// Input audio file(s).
    #[arg(long = "audio")]
    pub audio: Vec<PathBuf>,

    /// Input MIDI file(s).
    #[arg(long = "midi")]
    pub midi: Vec<PathBuf>,

    /// Optional pipeline configuration (TOML/JSON).
    #[arg(long = "pipeline-config")]
    pub pipeline_config: Option<PathBuf>,

    /// Optional project root directory for caches/artifacts.
    #[arg(long = "project-root")]
    pub project_root: Option<PathBuf>,

    /// Output path for JSON result; if omitted, print to stdout.
    #[arg(long = "output")]
    pub output: Option<PathBuf>,
}

#[derive(Debug, Serialize)]
struct JsonEvent<'a> {
    kind: &'static str,
    #[serde(flatten)]
    data: serde_json::Value,
}

#[derive(Debug, Serialize)]
struct AnalyzeOutput {
    events: Vec<serde_json::Value>,
}

pub fn run(args: AnalyzeArgs) -> Result<()> {
    if args.audio.is_empty() && args.midi.is_empty() {
        anyhow::bail!("at least one --audio or --midi input is required");
    }

    let req = AnalyzeRequest {
        audio_paths: args.audio,
        midi_paths: args.midi,
        pipeline_config: args.pipeline_config,
        project_root: args.project_root,
    };

    let resp = analyze_offline(req).context("engine analyze_offline failed")?;

    let json = encode_response(&resp)?;
    write_output(&json, args.output.as_ref())
}

fn encode_response(resp: &AnalyzeResponse) -> Result<String> {
    use mt-engine::types::EngineEvent;

    let mut out = Vec::with_capacity(resp.events.len());

    for ev in &resp.events {
        let (kind, value) = match ev {
            EngineEvent::Tempo(e) => ("tempo", serde_json::to_value(e)?),
            EngineEvent::Meter(e) => ("meter", serde_json::to_value(e)?),
            EngineEvent::Swing(e) => ("swing", serde_json::to_value(e)?),
            EngineEvent::Note(e) => ("note", serde_json::to_value(e)?),
            EngineEvent::Chord(e) => ("chord", serde_json::to_value(e)?),
            EngineEvent::Key(e) => ("key", serde_json::to_value(e)?),
            EngineEvent::Segment(e) => ("segment", serde_json::to_value(e)?),
        };

        out.push(
            serde_json::json!({
                "kind": kind,
                "data": value
            })
        );
    }

    let root = AnalyzeOutput { events: out };
    let s = serde_json::to_string_pretty(&root)?;
    Ok(s)
}

fn write_output(json: &str, path: Option<&PathBuf>) -> Result<()> {
    match path {
        Some(p) => {
            let mut file = File::create(p)
                .with_context(|| format!("failed to create output file {}", p.display()))?;
            file.write_all(json.as_bytes())?;
        }
        None => {
            let mut stdout = io::stdout().lock();
            stdout.write_all(json.as_bytes())?;
            stdout.write_all(b"\n")?;
        }
    }
    Ok(())
}
