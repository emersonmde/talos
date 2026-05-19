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
#ifdef TALOS_TARGET_RPI5_BCM2712
    bl rpi5_uart0_marker
#endif

    mov x19, x0

    mrs x0, cpacr_el1
    orr x0, x0, #(0b11 << 20)
    msr cpacr_el1, x0
    isb

    ldr x0, =__bss_start
    ldr x1, =__bss_end
0:
    cmp x0, x1
    b.hs 1f
    str xzr, [x0], #8
    b 0b

1:
    ldr x0, =__stack_top
    mov sp, x0
    mov x0, x19
    bl rust_entry

2:
    wfe
    b 2b

.size _start, . - _start

#ifdef TALOS_TARGET_RPI5_BCM2712
.type rpi5_uart0_marker, %function
rpi5_uart0_marker:
    ldr x9, =0x1c00030000
    mov w12, #0
    str w12, [x9, #0x30]
    mov w12, #0x7ff
    str w12, [x9, #0x44]
    mov w12, #(3 << 5)
    str w12, [x9, #0x2c]
    mov w12, #0x301
    str w12, [x9, #0x30]
    adr x10, 4f
5:
    ldrb w11, [x10], #1
    cbz w11, 7f
    mov w13, #0x10000
6:
    ldr w12, [x9, #0x18]
    tbz w12, #5, 8f
    subs w13, w13, #1
    b.ne 6b
8:
    str w11, [x9]
    b 5b
7:
    ret

4:
    .asciz "T1\r\n"

.size rpi5_uart0_marker, . - rpi5_uart0_marker
#endif
