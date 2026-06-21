# Minimal UART test for Phase 1 v0.4
# Writes "Hello\n" to UART THR (0x10000000), then loops forever

.section .text
.globl _start
_start:
    li t0, 0x10000000
    li t1, 0x48
    sb t1, 0(t0)
    li t1, 0x65
    sb t1, 0(t0)
    li t1, 0x6C
    sb t1, 0(t0)
    li t1, 0x6C
    sb t1, 0(t0)
    li t1, 0x6F
    sb t1, 0(t0)
    li t1, 0x0A
    sb t1, 0(t0)
1:  j 1b
