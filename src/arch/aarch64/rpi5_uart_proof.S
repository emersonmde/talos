/*
 * Minimal Raspberry Pi 5 assembly-only UART proof.
 *
 * Keep this intentionally smaller than the raw loader diagnostic: firmware
 * entry, write fixed bytes to the Pi 5 debug UART and the 40-pin RP1 UART0,
 * then repeat forever. No Rust, stack, BSS, exception vectors, watchdog,
 * PSCI, MPIDR filtering, or broad fallback logic.
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

    ldr x9, =0x107d001000
    bl write_marker

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
    bl write_marker

write_loop:
    ldr x9, =0x107d001000
    bl write_marker
    ldr x9, =0x1f00030000
    bl write_marker

delay:
    ldr x0, =0x00200000
delay_loop:
    subs x0, x0, #1
    b.ne delay_loop
    b write_loop

write_marker:
    adr x10, marker
1:
    ldrb w11, [x10], #1
    cbz w11, 2f
    str w11, [x9]
    ldr w12, [x9, #0x18]
    b 1b
2:
    dsb sy
    ret

marker:
    .asciz "TA\r\n"

.balign 8
    .ltorg

.balign 8
__image_end:
