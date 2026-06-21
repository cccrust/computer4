OUTPUT_ARCH(riscv)
ENTRY(_start)

SECTIONS {
    . = 0x00000;
    .text : {
        *(.text._start)
        *(.text)
        *(.text.*)
    }
    .rodata : {
        *(.rodata)
        *(.rodata.*)
        *(.srodata)
        *(.srodata.*)
    }
    .data : {
        *(.data)
        *(.data.*)
        *(.sdata)
        *(.sdata.*)
    }
    .bss : {
        *(.bss)
        *(.bss.*)
    }
    _end = .;
}
