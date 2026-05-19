/*
 * Minimal Raspberry Pi 5 assembly-only entry proof.
 *
 * This deliberately avoids UART and all MMIO. If BL31 enters this image as
 * BL33, the PSCI SYSTEM_RESET call should produce a second firmware boot. That
 * separates "kernel entry did not happen" from "entry happened but RP1 UART is
 * unavailable".
 */

.section .text, "ax"
.global _start

_start:
    b image_entry
    .word 0
    .quad 0
    .quad __image_end - _start
    .quad 0xc
    .quad 0
    .quad 0
    .quad 0
    .ascii "ARMd"
    .word 0

.balign 8
image_entry:
    mov x19, x0

reset_loop:
    ldr x0, =0x84000009
    smc #0
    wfe
    b reset_loop

.balign 8
    .ltorg

.balign 8
__image_end:
