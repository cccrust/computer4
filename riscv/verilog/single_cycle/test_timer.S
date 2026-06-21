# Phase 3: Timer Interrupt Test
# Sets up timer, enables interrupts, waits for IRQ
# UART output: 'A' before, 'T' after timer IRQ, 'B' after mret

.section .text
.globl _start
_start:
    li t0, 0x10000000
    li t1, 0x41          # 'A'
    sb t1, 0(t0)

    la t1, timer_handler
    csrw mtvec, t1

    li t1, 100           # mtimecmp = 100 (fires after 100 cycles)
    li t2, 0x02004000
    sd t1, 0(t2)

    li t1, 0x80          # mie.MTIE = bit 7
    csrw mie, t1

    li t1, 0x88          # mstatus.MIE=1, MPIE=1
    csrw mstatus, t1

loop:
    j loop

timer_handler:
    # Increase mtimecmp to avoid immediate re-fire
    li t0, 0x02004000
    ld t1, 0(t0)
    addi t1, t1, 100
    sd t1, 0(t0)

    li t0, 0x10000000
    li t1, 0x54          # 'T'
    sb t1, 0(t0)

    mret

    li t1, 0x46          # 'F' (shouldn't reach here)
    sb t1, 0(t0)

    j done

done:
    j done
