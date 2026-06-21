#!/bin/bash
set -x

cargo test

# Build examples using RISC-V cross-compiler
rustc --target riscv64imac-unknown-none-elf --emit obj -C opt-level=z -C overflow-checks=off programs/hello.rs -o programs/hello.o
rustc --target riscv64imac-unknown-none-elf --emit obj -C opt-level=z -C overflow-checks=off programs/fact.rs -o programs/fact.o
rustc --target riscv64imac-unknown-none-elf --emit obj -C opt-level=z -C overflow-checks=off programs/fib.rs -o programs/fib.o
rustc --target riscv64imac-unknown-none-elf --emit obj -C opt-level=z -C overflow-checks=off programs/sum.rs -o programs/sum.o

# Run examples
for ex in hello fact fib sum; do
    echo "=== $ex ==="
    cargo run --release -q -- programs/$ex.o 2>/dev/null
    if [ $? -ne 0 ]; then
        echo "FAIL: $ex"
        exit 1
    fi
done

echo "All examples passed!"
