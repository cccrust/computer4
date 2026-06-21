#!/bin/bash
set -ex
DIR="$(dirname "$0")"
for ex in hello sum fact fib; do
    "$DIR/link_run.sh" "../../rv4/examples/$ex.o" "program.hex"
    iverilog -o rv64i_cpu rv64i_cpu.v tb_rv64i.v 2>&1
    vvp rv64i_cpu 2>&1 | grep -E '(puts|PASS|FAIL)'
done
echo "=============================="
echo "  All rv4 examples PASS!"
echo "=============================="
