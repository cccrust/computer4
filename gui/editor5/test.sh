#!/bin/bash
set -e
cd /Users/Shared/ccc/project/computer4/gui/editor5
echo "Building..."
cargo build --release
echo "Running tests..."
cargo test --release
echo "All tests passed!"