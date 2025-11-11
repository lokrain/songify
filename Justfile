set shell := ["bash", "-cu"]

# Hygiene
fmt:
	cargo fmt --all

clippy:
	cargo clippy --all-targets --all-features -- -D warnings -D clippy::pedantic

lint: fmt clippy

# Build & test
build:
	cargo build --workspace

test:
	cargo test --workspace --all-features -- --nocapture

bench:
	cargo bench --workspace --features bench || true

# Checks
check-msrv:
	rustc --version

check-no-std:
	cargo check -p mt-core --no-default-features
	cargo check -p mt-signal-core --no-default-features

# Release
release:
	cargo build --workspace --release

# Dev flows
smoke:
	just build && just test

ci:
	just lint && just smoke
