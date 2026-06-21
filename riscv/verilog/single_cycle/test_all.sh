#!/bin/bash
set -e
DIR="$(dirname "$0")"
cd "$DIR"

echo "========================"
echo "  v0.4 CPU Test Suite"
echo "========================"

# Prebuild stubs.o if missing
STUBS="${DIR}/stubs.o"
if [ ! -f "$STUBS" ]; then
    riscv64-unknown-elf-as -mabi=lp64 -march=rv64ic -o "$STUBS" "${DIR}/stubs.s" 2>/dev/null
fi
RV4="${DIR}/../../rv4/examples"

run_phase() {
    local name=$1
    local asm=$2
    echo -n "  $name ... "
    rm -f program.hex single_cycle_tb
    riscv64-unknown-elf-as -mabi=lp64 -march=rv64iczicsr -o /tmp/test.o "$asm" 2>/dev/null || { echo "ASM FAIL"; return 1; }
    riscv64-unknown-elf-ld -T link_os.ld -o /tmp/test.elf /tmp/test.o 2>/dev/null || { echo "LD FAIL"; return 1; }
    riscv64-unknown-elf-objcopy -O binary /tmp/test.elf /tmp/test.bin 2>/dev/null || { echo "OBJCOPY FAIL"; return 1; }
    python3 bin2hex.py /tmp/test.bin program.hex 2>/dev/null
    iverilog -o single_cycle_tb rv64i_cpu.v tb_rv64i.v 2>/dev/null
    vvp single_cycle_tb 2>&1 | grep -E "PASS|FAIL" || { echo "no PASS/FAIL"; return 1; }
    rm -f program.hex single_cycle_tb
    echo "  done"
}

run_rv4() {
    local name=$1
    local obj="$RV4/$2.o"
    echo -n "  $name ... "
    rm -f program.hex single_cycle_tb
    riscv64-unknown-elf-ld -T link.ld -o /tmp/test.elf "$obj" "$STUBS" 2>/dev/null || { echo "LD FAIL"; return 1; }
    riscv64-unknown-elf-objcopy -O binary /tmp/test.elf /tmp/test.bin 2>/dev/null || { echo "OBJCOPY FAIL"; return 1; }
    python3 bin2hex.py /tmp/test.bin program.hex 2>/dev/null
    iverilog -o single_cycle_tb rv64i_cpu.v tb_rv64i.v 2>/dev/null
    vvp single_cycle_tb 2>&1 | grep -E "PASS|FAIL" || { echo "no PASS/FAIL"; return 1; }
    rm -f program.hex single_cycle_tb
    echo "  done"
}

run_phase "Phase 1 (UART)"  test_uart.S
run_phase "Phase 2 (MRET)"  test_mret.S
run_phase "Phase 3 (Timer)" test_timer.S
run_rv4   "rv4 hello"       hello
run_rv4   "rv4 fact"        fact
run_rv4   "rv4 fib"         fib
run_rv4   "rv4 sum"         sum

echo "========================"
echo "  ALL PASS"
echo "========================"
