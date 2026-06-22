#!/bin/bash
set -e
SIMDIR="/Users/Shared/ccc/project/computer4/riscv/verilog/single_cycle"
OSDIR="/Users/Shared/ccc/project/computer4/riscv/mini-riscv-os"
BIN2HEX="$SIMDIR/bin2hex.py"
CARGO_TARGET_DIR="${CARGO_TARGET_DIR:-/private/tmp/mini-riscv-os-target}"
export CARGO_TARGET_DIR

echo "========================================"
echo "  mini-riscv-os on RV64IM + Zicsr + C"
echo "========================================"

echo ""
echo "  Step 1: Build Rust OS lib ..."
(cd "$OSDIR" && cargo build --release --target riscv32imac-unknown-none-elf 2>&1 | grep -v "^$")

echo ""
echo "  Step 2: Compile asm + link os.elf ..."
riscv64-unknown-elf-gcc -nostdlib -fno-builtin -mcmodel=medany \
  -march=rv32ima_zicsr -mabi=ilp32 -fno-PIC \
  -c "$OSDIR/start.s" -o /tmp/start.o
riscv64-unknown-elf-gcc -nostdlib -fno-builtin -mcmodel=medany \
  -march=rv32ima_zicsr -mabi=ilp32 -fno-PIC \
  -c "$OSDIR/sys.s" -o /tmp/sys.o
riscv64-unknown-elf-gcc -nostdlib -fno-builtin -mcmodel=medany \
  -march=rv32ima_zicsr -mabi=ilp32 -fno-PIC \
  -T "$OSDIR/os.ld" -o /tmp/os.elf \
  /tmp/start.o /tmp/sys.o \
  "$OSDIR/target/riscv32imac-unknown-none-elf/release/libmini_riscv_os.a"

echo ""
echo "  Step 3: Convert to hex (pad to 16384 words) ..."
rm -f "$SIMDIR/program.hex" "$SIMDIR/single_cycle_tb"
riscv64-unknown-elf-objcopy -O binary /tmp/os.elf /tmp/os.bin
python3 "$BIN2HEX" /tmp/os.bin "$SIMDIR/program.hex"
python3 -c "
data = open('$SIMDIR/program.hex').read().strip().split()
target = 16384
while len(data) < target:
    data.append('00000000')
with open('$SIMDIR/program.hex', 'w') as f:
    f.write('\n'.join(data) + '\n')
"

echo ""
echo "  Step 4: Compile simulation (Verilator) ..."
cd "$SIMDIR"
verilator --binary --trace -o single_cycle_tb_vlt \
  --top-module tb_rv64i \
  -Wno-WIDTHEXPAND -Wno-WIDTHTRUNC -Wno-CASEINCOMPLETE -Wno-TIMESCALEMOD \
  rv64i_cpu.v tb_rv64i.v 2>&1

echo ""
echo "  Step 5: Run simulation ..."
cd "$SIMDIR/obj_dir"
ln -sf ../program.hex program.hex 2>/dev/null || true
./single_cycle_tb_vlt 2>&1 | grep -v '^\[.*\] PC=' | tee /tmp/vvp_out.txt

if grep -q "PASS" /tmp/vvp_out.txt; then
    echo ""
    echo "========================================"
    echo "  MINI-RISCV-OS: PASS"
    echo "========================================"
else
    echo ""
    echo "========================================"
    echo "  MINI-RISCV-OS: FAIL (no PASS)"
    echo "========================================"
    exit 1
fi
