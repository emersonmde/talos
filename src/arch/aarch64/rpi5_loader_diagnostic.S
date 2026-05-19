/*
 * Minimal Raspberry Pi 5 raw-loader diagnostic.
 *
 * This binary is staged as kernel_2712.img without the arm64 Image header so
 * one hardware run can test the same raw-image contract used by Circle's Pi 5
 * bootloader before Talos proper is involved.
 */

.section .text, "ax"
.global _start

_start:
    ldr x0, =0x08000000
    mov sp, x0

    ldr x9, =0x1c00030000
    adr x10, rp1_preserved_marker
    bl write_marker

    ldr x9, =0x1c00030000
    bl init_pl011_fixed
    adr x10, rp1_initialized_marker
    bl write_marker

    ldr x9, =0x107d001000
    bl init_pl011_fixed
    adr x10, soc_initialized_marker
    bl write_marker

1:
    ldr x9, =0x1c00030000
    adr x10, heartbeat_marker
    bl write_marker
    ldr x0, =0x00400000
2:
    subs x0, x0, #1
    b.ne 2b
    b 1b

init_pl011_fixed:
    mov w12, #0
    str w12, [x9, #0x30]
    mov w12, #0x7ff
    str w12, [x9, #0x44]
    mov w12, #1
    str w12, [x9, #0x24]
    mov w12, #40
    str w12, [x9, #0x28]
    mov w12, #(3 << 5)
    str w12, [x9, #0x2c]
    mov w12, #0
    str w12, [x9, #0x38]
    mov w12, #0x301
    str w12, [x9, #0x30]
    ret

write_marker:
3:
    ldrb w11, [x10], #1
    cbz w11, 6f
    mov w13, #0x10000
4:
    ldr w12, [x9, #0x18]
    tbz w12, #5, 5f
    subs w13, w13, #1
    b.ne 4b
5:
    str w11, [x9]
    b 3b
6:
    ret

rp1_preserved_marker:
    .asciz "L0\r\n"
rp1_initialized_marker:
    .asciz "L1\r\n"
soc_initialized_marker:
    .asciz "U1\r\n"
heartbeat_marker:
    .asciz "."
