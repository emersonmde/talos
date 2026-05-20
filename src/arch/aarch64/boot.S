.section .text.boot, "ax"
.global _start
.type _start, %function

_start:
    b 3f
    .long 0
    .quad KERNEL_IMAGE_TEXT_OFFSET
    .quad __kernel_image_end - _start
#ifdef TALOS_TARGET_RPI5_BCM2712
    .quad 0xc
#else
    .quad 0
#endif
    .quad 0
    .quad 0
    .quad 0
    .long 0x644d5241
    .long 0

3:
    mov x19, x0
#ifdef TALOS_TARGET_RPI5_BCM2712
#ifdef TALOS_RPI5_CARGO_ASM_UART_PROOF
    ldr x9, =0x107d001000
    bl rpi5_write_marker

    ldr x13, =0x1f000f003c
    mov w12, #0x40
    str w12, [x13]
    ldr w12, [x13]
    ldr x13, =0x1f000f0040
    mov w12, #0x48
    str w12, [x13]
    ldr w12, [x13]
    ldr x13, =0x1f000d0074
    mov w12, #4
    str w12, [x13]
    ldr w12, [x13]
    ldr x13, =0x1f000d007c
    mov w12, #4
    str w12, [x13]
    ldr w12, [x13]
    dsb sy

    ldr x9, =0x1f00030000
    bl rpi5_write_marker

4:
    ldr x9, =0x107d001000
    bl rpi5_write_marker
    ldr x9, =0x1f00030000
    bl rpi5_write_marker
    b 4b
#else
    ldr x9, =0x107d001000
    bl rpi5_write_marker

    ldr x13, =0x1f000f003c
    mov w12, #0x40
    str w12, [x13]
    ldr w12, [x13]
    ldr x13, =0x1f000f0040
    mov w12, #0x48
    str w12, [x13]
    ldr w12, [x13]
    ldr x13, =0x1f000d0074
    mov w12, #4
    str w12, [x13]
    ldr w12, [x13]
    ldr x13, =0x1f000d007c
    mov w12, #4
    str w12, [x13]
    ldr w12, [x13]
    dsb sy

    ldr x9, =0x1f00030000
    bl rpi5_write_marker
#endif
#endif

    mrs x0, cpacr_el1
    orr x0, x0, #(0b11 << 20)
    msr cpacr_el1, x0
    isb

    adrp x0, __bss_start
    add x0, x0, :lo12:__bss_start
    adrp x1, __bss_end
    add x1, x1, :lo12:__bss_end
0:
    cmp x0, x1
    b.hs 1f
    str xzr, [x0], #8
    b 0b

1:
    adrp x0, __stack_top
    add x0, x0, :lo12:__stack_top
    mov sp, x0
    mov x0, x19
    bl rust_entry

2:
    wfe
    b 2b

.size _start, . - _start

#ifdef TALOS_TARGET_RPI5_BCM2712
rpi5_write_marker:
    adr x10, 9f
12:
    ldrb w11, [x10], #1
    cbz w11, 13f
    str w11, [x9]
    ldr w12, [x9, #0x18]
    b 12b
13:
    dsb sy
    ret

9:
#ifdef TALOS_RPI5_CARGO_ASM_UART_PROOF
    .asciz "CL\r\n"
#else
    .asciz "BI\r\n"
#endif

#endif
