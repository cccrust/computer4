#!/bin/bash
# link_run.sh — Link .o → .elf → .bin → .hex for single-cycle CPU
# Usage: ./link_run.sh <example.o> [output.hex]

set -e

OBJ="$1"
OUT="${2:-program.hex}"
DIR="$(dirname "$0")"

if [ -z "$OBJ" ]; then
    echo "Usage: $0 <example.o> [output.hex]"
    echo "Example: $0 ../../rv4/examples/hello.o"
    exit 1
fi

# Auto-build stubs.o if missing
STUBS="$DIR/stubs.o"
if [ ! -f "$STUBS" ]; then
    riscv64-unknown-elf-as -mabi=lp64 -march=rv64ic -o "$STUBS" "$DIR/stubs.s" 2>&1
fi

LD_SCRIPT="$DIR/link.ld"
TMP_ELF=$(mktemp /tmp/link_elf_XXXXXX)
TMP_BIN=$(mktemp /tmp/link_bin_XXXXXX)

riscv64-unknown-elf-ld -T "$LD_SCRIPT" -o "$TMP_ELF" "$OBJ" "$STUBS" 2>&1
riscv64-unknown-elf-objcopy -O binary "$TMP_ELF" "$TMP_BIN" 2>&1
python3 "$DIR/bin2hex.py" "$TMP_BIN" "$OUT"

echo "Wrote $OUT ($(wc -c < "$TMP_BIN") bytes)"

rm -f "$TMP_ELF" "$TMP_BIN"
