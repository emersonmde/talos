ENTRY(_start)

KERNEL_LOAD_ADDR = 0x40200000;
STACK_SIZE = 0x00040000; /* 256 KiB */
HEAP_SIZE = 0x00100000;  /* reserved for the first allocator milestone */

SECTIONS
{
    . = KERNEL_LOAD_ADDR;
    __kernel_start = .;

    .text.boot : ALIGN(4K) {
        KEEP(*(.text.boot))
    }

    .vectors : ALIGN(2K) {
        KEEP(*(.vectors))
    }

    .text : ALIGN(4K) {
        *(.text .text.*)
    }

    .rodata : ALIGN(4K) {
        *(.rodata .rodata.*)
    }

    .data : ALIGN(4K) {
        *(.data .data.*)
    }

    .bss (NOLOAD) : ALIGN(16) {
        __bss_start = .;
        *(.bss .bss.*)
        *(COMMON)
        __bss_end = .;
    }

    . = ALIGN(16);
    __heap_start = .;
    . += HEAP_SIZE;
    __heap_end = .;

    . = ALIGN(16);
    __stack_bottom = .;
    . += STACK_SIZE;
    __stack_top = .;

    . = ALIGN(4K);
    __kernel_end = .;

    /DISCARD/ : {
        *(.comment)
        *(.gnu*)
        *(.note*)
        *(.eh_frame*)
    }
}
