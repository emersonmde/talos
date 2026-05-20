ENTRY(_start)

KERNEL_DRAM_BASE = 0x00080000;
KERNEL_IMAGE_TEXT_OFFSET = 0x00000000;
KERNEL_LOAD_ADDR = KERNEL_DRAM_BASE + KERNEL_IMAGE_TEXT_OFFSET;
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
        KEEP(*(.text.rpi5_text_section_diagnostic))
        *(.text .text.*)
    }

    .rodata : ALIGN(4K) {
        *(.rodata .rodata.*)
    }

    .data : ALIGN(4K) {
        *(.data .data.*)
    }

    __kernel_image_end = .;

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
