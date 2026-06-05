#!/bin/bash
set -e

echo "=== Shop4 Build & Test ==="

cd /Users/Shared/ccc/project/computer4/web/shop4

echo "Building backend..."
cargo build

echo "Running backend tests..."
cargo test

echo "Building test database with realistic data..."
cargo run --bin build_testdb

echo "Building frontend..."
cd frontend
npm install
npm run build

echo "=== All checks passed ==="
echo ""
echo "To start the server:"
echo "  cargo run"
echo ""
echo "To start the frontend (development):"
echo "  cd frontend && npm run dev"