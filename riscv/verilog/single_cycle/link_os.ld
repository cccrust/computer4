OUTPUT_ARCH(riscv)
ENTRY(_start)
BASE_ADDR = 0x80000000;

SECTIONS {
    . = BASE_ADDR;
    .text : {
        *(.text._start)
        *(.text)
        *(.text.*)
    }
    .rodata : {
        *(.rodata)
        *(.rodata.*)
    }
    .data : {
        *(.data)
        *(.data.*)
    }
    .bss : {
        *(.bss)
        *(.bss.*)
    }
    _end = .;
}
