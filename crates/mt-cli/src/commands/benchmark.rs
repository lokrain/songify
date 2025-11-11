//! `mt-cli benchmark`
//!
//! Measures offline pipeline performance for a single input.

use std::path::PathBuf;
use std::time::Instant;

use anyhow::{Context, Result};
use clap::Args;

use mt_engine::api::analyze_offline;
use mt_engine::types::AnalyzeRequest;

#[derive(Debug, Args)]
pub struct BenchmarkArgs {
    /// Audio file for benchmarking.
    #[arg(long = "audio")]
    pub audio: PathBuf,

    /// Optional pipeline configuration.
    #[arg(long = "pipeline-config")]
    pub pipeline_config: Option<PathBuf>,

    /// Number of runs.
    #[arg(long = "runs", default_value = "3")]
    pub runs: u32,
}

pub fn run(args: BenchmarkArgs) -> Result<()> {
    if args.runs == 0 {
        anyhow::bail!("--runs must be >= 1");
    }

    let req = AnalyzeRequest {
        audio_paths: vec![args.audio.clone()],
        midi_paths: Vec::new(),
        pipeline_config: args.pipeline_config,
        project_root: None,
    };

    let mut times_ms = Vec::with_capacity(args.runs as usize);

    for _ in 0..args.runs {
        let start = Instant::now();
        let _resp = analyze_offline(req.clone()).context("engine analyze_offline failed")?;
        let elapsed = start.elapsed();
        let ms = elapsed.as_secs_f64() * 1000.0;
        times_ms.push(ms);
    }

    times_ms.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let avg = times_ms.iter().sum::<f64>() / times_ms.len() as f64;
    let median = times_ms[times_ms.len() / 2];

    println!("runs: {}", args.runs);
    println!("avg_ms: {:.3}", avg);
    println!("median_ms: {:.3}", median);
    if let Some(min) = times_ms.first() {
        println!("min_ms: {:.3}", min);
    }
    if let Some(max) = times_ms.last() {
        println!("max_ms: {:.3}", max);
    }

    Ok(())
}
