/*
 * Minimal Raspberry Pi 5 assembly-only UART proof.
 *
 * Keep this intentionally smaller than the raw loader diagnostic: firmware
 * entry, park secondary cores, touch one plausible PL011 UART path, and repeat
 * fixed bytes forever. No Rust, stack, BSS, exception vectors, watchdog, PSCI,
 * or multi-device fallback logic.
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

    mrs x0, mpidr_el1
    and x0, x0, #0xff
    cbz x0, primary_core

secondary_park:
    wfe
    b secondary_park

primary_core:
    ldr x13, =0x1f000f003c
    mov w12, #0x40
    str w12, [x13]
    ldr x13, =0x1f000f0040
    mov w12, #0x48
    str w12, [x13]
    ldr x13, =0x1f000d0074
    mov w12, #4
    str w12, [x13]
    ldr x13, =0x1f000d007c
    str w12, [x13]
    dsb sy

    ldr x9, =0x1f00030000

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

write_loop:
    adr x10, marker
write_byte:
    ldrb w11, [x10], #1
    cbz w11, delay
wait_tx:
    ldr w12, [x9, #0x18]
    tbnz w12, #5, wait_tx
    str w11, [x9]
    b write_byte

delay:
    ldr x0, =0x00200000
delay_loop:
    subs x0, x0, #1
    b.ne delay_loop
    b write_loop

marker:
    .asciz "TA\r\n"

.balign 8
    .ltorg

.balign 8
__image_end:
