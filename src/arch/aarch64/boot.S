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
#ifdef TALOS_RPI5_TRANSITION_DIAGNOSTIC
    adr x10, marker_s0
    ldr x9, =0x107d001000
    bl rpi5_write_cstr
#else
    ldr x9, =0x107d001000
    bl rpi5_write_marker
#endif

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

#ifdef TALOS_RPI5_TRANSITION_DIAGNOSTIC
    adr x10, marker_s1
    bl rpi5_write_stage
    b 2f
#else
    ldr x9, =0x1f00030000
    bl rpi5_write_marker
#endif
#endif
#endif

    mrs x0, cpacr_el1
    orr x0, x0, #(0b11 << 20)
    msr cpacr_el1, x0
    isb
#if defined(TALOS_TARGET_RPI5_BCM2712) && defined(TALOS_RPI5_TRANSITION_DIAGNOSTIC)
    adr x10, marker_s2
    bl rpi5_write_stage
#endif

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
#if defined(TALOS_TARGET_RPI5_BCM2712) && defined(TALOS_RPI5_TRANSITION_DIAGNOSTIC)
    adr x10, marker_s3
    bl rpi5_write_stage
#endif
    adrp x0, __stack_top
    add x0, x0, :lo12:__stack_top
    mov sp, x0
#if defined(TALOS_TARGET_RPI5_BCM2712) && defined(TALOS_RPI5_TRANSITION_DIAGNOSTIC)
    adr x10, marker_s4
    bl rpi5_write_stage
#endif
    mov x0, x19
    bl rust_entry

2:
    wfe
    b 2b

.size _start, . - _start

#ifdef TALOS_TARGET_RPI5_BCM2712
#ifdef TALOS_RPI5_TRANSITION_DIAGNOSTIC
rpi5_write_stage:
    mov x17, x30
    mov x14, #16
14:
    ldr x9, =0x107d001000
    bl rpi5_write_cstr
    ldr x9, =0x1f00030000
    bl rpi5_write_cstr
    subs x14, x14, #1
    b.ne 14b
    mov x30, x17
    ret

rpi5_write_cstr:
    mov x15, x10
15:
    ldrb w11, [x15], #1
    cbz w11, 16f
    str w11, [x9]
    ldr w12, [x9, #0x18]
    b 15b
16:
    dsb sy
    ret
#endif

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

#ifdef TALOS_RPI5_TRANSITION_DIAGNOSTIC
marker_s0:
    .asciz "S0\r\n"
marker_s1:
    .asciz "S1\r\n"
marker_s2:
    .asciz "S2\r\n"
marker_s3:
    .asciz "S3\r\n"
marker_s4:
    .asciz "S4\r\n"
#endif

#endif
