# Minimal stubs for Rust panic functions
# All panic functions just jump to rust_begin_unwind (infinite loop)

.section .text
.globl _RNvNtCshql4lZlyZYo_4core9panicking18panic_bounds_check
_RNvNtCshql4lZlyZYo_4core9panicking18panic_bounds_check:
    j _RNvCseLSQwpavqd5_7___rustc17rust_begin_unwind
