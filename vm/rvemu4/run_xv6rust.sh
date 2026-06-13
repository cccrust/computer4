#!/bin/bash
set -e
ROOT=/Users/Shared/ccc/project/computer4
XV6=$ROOT/os/xv6-rust-octopus
KERNEL=$XV6/target/riscv64gc-unknown-none-elf/release/octopos
FSIMG=$XV6/target/fs.img

if [ ! -f "$KERNEL" ]; then
    (cd "$XV6" && cargo build --release -p octopos)
fi
if [ ! -f "$FSIMG" ]; then
    (cd "$XV6" && cargo build --release --package user && ./mkfs.sh)
fi

exec cargo run --release -q -- -kernel "$KERNEL" -drive "$FSIMG" -smp 1
