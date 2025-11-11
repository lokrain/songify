//! `mt-cli dump-timeline`
//!
//! Runs analysis and prints a readable timeline.

use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::Args;

use mt_core::time::SampleTime;
use mt_engine::api::analyze_offline;
use mt_engine::types::{AnalyzeRequest, EngineEvent};

#[derive(Debug, Args)]
pub struct DumpTimelineArgs {
    #[arg(long = "audio")]
    pub audio: Vec<PathBuf>,

    #[arg(long = "midi")]
    pub midi: Vec<PathBuf>,

    #[arg(long = "pipeline-config")]
    pub pipeline_config: Option<PathBuf>,

    #[arg(long = "project-root")]
    pub project_root: Option<PathBuf>,
}

pub fn run(args: DumpTimelineArgs) -> Result<()> {
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

    for ev in resp.events {
        match ev {
            EngineEvent::Tempo(e) => {
                println!(
                    "[{}] tempo={} bpm",
                    fmt_smp(e.position),
                    e.bpm_x1000 as f64 / 1000.0
                );
            }
            EngineEvent::Meter(e) => {
                println!(
                    "[{}] meter={}/{}",
                    fmt_smp(e.position),
                    e.numerator,
                    e.denominator
                );
            }
            EngineEvent::Swing(e) => {
                println!(
                    "[{}] swing={:.3}",
                    fmt_smp(e.position),
                    e.ratio_x1000 as f64 / 1000.0
                );
            }
            EngineEvent::Note(n) => {
                println!(
                    "[{}] note id={} track={} midi={} vel={}",
                    fmt_smp(n.onset),
                    n.id.0,
                    n.track.0,
                    n.note.value(),
                    n.velocity
                );
            }
            EngineEvent::Chord(c) => {
                println!(
                    "[{}-{}] chord={} conf={:.3}",
                    fmt_smp(c.onset),
                    fmt_smp(c.offset),
                    c.chord,
                    c.confidence_x1000 as f64 / 1000.0
                );
            }
            EngineEvent::Key(k) => {
                println!(
                    "[{}] key={} conf={:.3}",
                    fmt_smp(k.position),
                    k.key,
                    k.confidence_x1000 as f64 / 1000.0
                );
            }
            EngineEvent::Segment(s) => {
                println!(
                    "[{}-{}] segment={:?} conf={:.3}",
                    fmt_smp(s.onset),
                    fmt_smp(s.offset),
                    s.kind,
                    s.confidence_x1000 as f64 / 1000.0
                );
            }
        }
    }

    Ok(())
}

fn fmt_smp(t: SampleTime) -> String {
    t.value().to_string()
}
