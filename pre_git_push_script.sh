#!/usr/bin/env bash

set -e

echo "Formatting code..."
cargo fmt

echo "Running tests..."
cargo test

echo "Building project..."
cargo build

echo "Running Clippy (lint)..."
cargo clippy --all-targets --all-features -- -D warnings

echo "All checks passed. Safe to push!"
