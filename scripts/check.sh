#!/usr/bin/env sh
# Run from any working directory with Rust available on PATH.
set -eu
cd "$(dirname "$0")/.."
cargo fmt --check
cargo check --locked
cargo test --locked
cargo clippy --all-targets --locked -- -D warnings
cargo build --release --locked
