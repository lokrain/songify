# Copilot Instructions for this Repository

Purpose: Help AI coding agents work productively in this Rust repo.

## Overview
- Project type: Rust Cargo project. `Cargo.lock` records a package `songify` v0.1.0. The root `Cargo.toml` appears to declare a workspace (`[workspace]`).
- Current tree includes `target/` build artifacts but no committed `src/` files. Expect code to live either at the root (`src/`) or as workspace members under subfolders when added.
- Treat `target/` as ephemeral; never edit or commit into it.

## Build and Run
- Default build: `cargo build` (workspace-level). If multiple members exist, build all.
- Package-specific build/run (recommended):
  - Build: `cargo build -p songify`
  - Run: `cargo run -p songify -- <args>`
- Clean: `cargo clean`
- Backtraces on errors: `RUST_BACKTRACE=1 cargo run -p songify`

## Tests and Linting
- Run tests: `cargo test -p songify`
- Format: `cargo fmt --all`
- Lints: `cargo clippy --all-targets --all-features -p songify -- -D warnings`

## Project Layout Conventions
- Workspace root: define members in root `Cargo.toml` under `[workspace]` → `members = ["<path>"]`.
- New binary crate: `cargo new songify --bin` (or create under `crates/songify`), then add the path to `members`.
- Library + binary pattern (preferred):
  - `src/lib.rs`: core logic (pure functions, types)
  - `src/main.rs`: thin CLI/service entrypoint using the library
  - Use `use songify::...` from `main.rs` to call library APIs

## Common Tasks for Agents
- When adding a crate:
  - Create the crate directory, then update root `[workspace]` members.
  - Set the crate `name = "songify"` in that crate’s `Cargo.toml` if it is the main binary/library.
- When adding modules:
  - Prefer small modules under `src/` and re-export via `pub mod` in `lib.rs`.
- Dependencies:
  - Add to the specific crate’s `Cargo.toml`. Keep root `Cargo.toml` focused on workspace settings only.

## Debugging Tips
- Use `println!`/`dbg!` for quick inspection. For richer logs, wire a logger (e.g., `env_logger`) and run with `RUST_LOG=debug`.
- For panics, enable backtraces: `RUST_BACKTRACE=1`.

## What’s Missing / To Confirm
- No committed source was found. If code exists elsewhere (e.g., `crates/songify` or `src/`), point agents to the canonical locations and any module conventions.
- If there are custom tasks, Makefiles, or CI rules, link them here so agents use the same flows.

## Example Commands
- Initialize main binary crate at root:
  - `cargo new . --bin`  (creates `src/main.rs` in-place)
- Or as workspace member:
  - `cargo new crates/songify --bin` and add `"crates/songify"` to `[workspace].members`
- Run with args:
  - `cargo run -p songify -- --input sample.txt`
