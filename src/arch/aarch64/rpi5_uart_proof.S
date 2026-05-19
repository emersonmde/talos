/*
 * Minimal Raspberry Pi 5 assembly-only UART proof.
 *
 * Keep this intentionally smaller than the raw loader diagnostic: firmware
 * entry, park secondary cores, use the firmware-preserved RP1 UART0 path, and
 * repeat fixed bytes forever. No Rust, stack, BSS, exception vectors, GPIO mux,
 * PL011 reinitialization, watchdog, PSCI, or multi-device fallback logic.
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
    ldr x9, =0x1c00030000

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
