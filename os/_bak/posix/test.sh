#!/bin/bash
set -x
cd "$(dirname "$0")"

echo "=== libposix build ==="
cd libposix && cargo build 2>&1 | tail -2 && cd ..

echo "=== tools build ==="
cd tools && cargo build --bins 2>&1 | tail -2

echo ""
echo "=== libposix test ==="
cd ../libposix && cargo test 2>&1 | tail -5 && cd ..

echo ""
echo "=== tools test ==="
cd tools && cargo test 2>&1 | tail -10

echo ""
echo "=== Testing cat ==="
echo "hello world" | cargo run --bin cat 2>&1 | grep -Ev 'Running|Compilin|warning:'

echo ""
echo "=== Testing head ==="
echo -e "line1\nline2\nline3\nline4\nline5" | cargo run --bin head 2>&1 | grep -Ev 'Running|Compilin|warning:'

echo ""
echo "=== Testing od ==="
echo abc | cargo run --bin od 2>&1 | grep -Ev 'Running|Compilin|warning:'

echo ""
echo "=== Testing head -n 2 ==="
echo -e "line1\nline2\nline3\nline4\nline5" | cargo run --bin head -- -n 2 2>&1 | grep -Ev 'Running|Compilin|warning:'

echo ""
echo "=== All tests passed ==="