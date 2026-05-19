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

    adr x0, exception_vectors
    mrs x1, CurrentEL
    cmp x1, #(2 << 2)
    b.eq 0f
    cmp x1, #(3 << 2)
    b.eq 2f
    cmp x1, #(1 << 2)
    b.ne 1f
    msr vbar_el1, x0
    b 1f
0:
    msr vbar_el2, x0
    b 1f
2:
    msr vbar_el3, x0
1:
    isb

    ldr x9, =0x107d001000
    bl init_pl011_fixed
    adr x10, soc_initialized_marker
    bl write_marker

    ldr x9, =0x1f00030000
    adr x10, rp1_pcie2_preserved_marker
    bl write_marker

    ldr x9, =0x1f00030000
    bl init_pl011_fixed
    adr x10, rp1_pcie2_initialized_marker
    bl write_marker

    ldr x9, =0x1c00030000
    adr x10, rp1_preserved_marker
    bl write_marker

    ldr x9, =0x1c00030000
    bl init_pl011_fixed
    adr x10, rp1_initialized_marker
    bl write_marker

1:
    ldr x9, =0x1c00030000
    adr x10, heartbeat_marker
    bl write_marker
    ldr x0, =0x00400000
2:
    subs x0, x0, #1
    b.ne 2b
    ldr x9, =0x1c00030000
    adr x10, watchdog_marker
    bl write_marker
    bl trigger_psci_reset
    bl trigger_watchdog_reset
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

trigger_watchdog_reset:
    ldr x9, =0x107d200024
    ldr w12, =0x5a000001
    str w12, [x9]
    ldr x9, =0x107d20001c
    ldr w12, =0x5a000020
    str w12, [x9]
7:
    wfe
    b 7b

trigger_psci_reset:
    ldr x0, =0x84000009
    smc #0
    ret

rp1_preserved_marker:
    .asciz "L0\r\n"
rp1_initialized_marker:
    .asciz "L1\r\n"
rp1_pcie2_preserved_marker:
    .asciz "N0\r\n"
rp1_pcie2_initialized_marker:
    .asciz "N1\r\n"
soc_initialized_marker:
    .asciz "U1\r\n"
heartbeat_marker:
    .asciz "."
watchdog_marker:
    .asciz "W0\r\n"

.align 11
exception_vectors:
    b skip_exception
    .space 0x80 - (. - exception_vectors)
    b skip_exception
    .space 0x100 - (. - exception_vectors)
    b skip_exception
    .space 0x180 - (. - exception_vectors)
    b skip_exception
    .space 0x200 - (. - exception_vectors)
    b skip_exception
    .space 0x280 - (. - exception_vectors)
    b skip_exception
    .space 0x300 - (. - exception_vectors)
    b skip_exception
    .space 0x380 - (. - exception_vectors)
    b skip_exception
    .space 0x400 - (. - exception_vectors)
    b skip_exception
    .space 0x480 - (. - exception_vectors)
    b skip_exception
    .space 0x500 - (. - exception_vectors)
    b skip_exception
    .space 0x580 - (. - exception_vectors)
    b skip_exception
    .space 0x600 - (. - exception_vectors)
    b skip_exception
    .space 0x680 - (. - exception_vectors)
    b skip_exception
    .space 0x700 - (. - exception_vectors)
    b skip_exception
    .space 0x780 - (. - exception_vectors)
    b skip_exception

skip_exception:
    mrs x20, CurrentEL
    cmp x20, #(2 << 2)
    b.eq 7f
    cmp x20, #(3 << 2)
    b.eq 9f
    cmp x20, #(1 << 2)
    b.ne 8f
    mrs x21, elr_el1
    add x21, x21, #4
    msr elr_el1, x21
    eret
7:
    mrs x21, elr_el2
    add x21, x21, #4
    msr elr_el2, x21
    eret
9:
    mrs x21, elr_el3
    add x21, x21, #4
    msr elr_el3, x21
    eret
8:
    wfe
    b 8b
