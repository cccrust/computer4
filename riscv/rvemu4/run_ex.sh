#!/bin/bash
BIN=./target/release/rvemu4
if [ ! -x "$BIN" ]; then
    cargo build --release -q
fi
for ex in hello fact fib sum; do
    echo "=== $ex ==="
    "$BIN" programs/$ex.o
    echo ""
done
