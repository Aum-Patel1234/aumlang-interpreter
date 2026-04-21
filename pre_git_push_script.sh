#!/usr/bin/env bash

set -e

echo "Formatting code..."
cargo fmt

echo "Running tests..."
cargo test

echo "Building project..."
cargo build

echo "All checks passed. Safe to push!"
