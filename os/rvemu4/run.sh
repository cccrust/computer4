#!/bin/bash
set -x

KERNEL="${1:-/Users/Shared/ccc/project/computer0/os/xv6/kernel/kernel}"
DISK="${2:-/Users/Shared/ccc/project/computer0/os/xv6/fs.img}"

cargo run --release -- -kernel "$KERNEL" -drive "$DISK" -smp 1 "$@"
