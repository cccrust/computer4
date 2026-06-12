#!/bin/bash

if [ "$1" = "-kernel" ]; then
    cargo run --release -q -- "$@" 2>/dev/null
elif [ -f "$1" ]; then
    cargo run --release -q -- "$1" 2>/dev/null
else
    KERNEL="${1:-/Users/Shared/ccc/project/computer0/os/xv6/kernel/kernel}"
    DISK="${2:-/Users/Shared/ccc/project/computer0/os/xv6/fs.img}"
    cargo run --release -q -- -kernel "$KERNEL" -drive "$DISK" -smp 1 "$@" 2>/dev/null
fi
