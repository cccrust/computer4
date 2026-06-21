# Phase 2: mret test
# csrw mepc -> mret -> should jump to after_mret
# UART output: 'A' before mret, 'B' after

.section .text
.globl _start
_start:
    li t0, 0x10000000
    li t1, 0x41          # 'A'
    sb t1, 0(t0)

    la t1, after_mret
    csrw mepc, t1

    mret

    li t1, 0x46          # 'F' (fail: mret didn't jump)
    sb t1, 0(t0)
    j done

after_mret:
    li t1, 0x42          # 'B'
    sb t1, 0(t0)
    li t1, 0x0A          # newline
    sb t1, 0(t0)

done:
    j done
