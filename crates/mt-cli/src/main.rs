//! Entry point for mt-cli.
//!
//! Thin, deterministic wrapper over mt-engine::api.

mod commands;

use std::process::ExitCode;

use anyhow::Result;
use clap::{Parser, Subcommand};

use crate::commands::{
    analyze::AnalyzeArgs,
    benchmark::BenchmarkArgs,
    dump_timeline::DumpTimelineArgs,
    validate::ValidateArgs,
};

#[derive(Debug, Parser)]
#[command(
    name = "mt-cli",
    version,
    about = "Music Theory & Analytics engine CLI",
    propagate_version = true
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Run offline analysis and emit JSON.
    Analyze(AnalyzeArgs),
    /// Run analysis and print a human-readable timeline.
    DumpTimeline(DumpTimelineArgs),
    /// Benchmark pipeline performance.
    Benchmark(BenchmarkArgs),
    /// Validate configuration and golden outputs.
    Validate(ValidateArgs),
}

fn main() -> ExitCode {
    if let Err(err) = run() {
        eprintln!("{err}");
        // Map anyhow roots to stable exit codes later if needed.
        return ExitCode::from(1);
    }
    ExitCode::SUCCESS
}

fn run() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Command::Analyze(args) => commands::analyze::run(args),
        Command::DumpTimeline(args) => commands::dump_timeline::run(args),
        Command::Benchmark(args) => commands::benchmark::run(args),
        Command::Validate(args) => commands::validate::run(args),
    }
}
