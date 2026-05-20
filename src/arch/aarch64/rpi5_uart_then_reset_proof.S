/*
 * Minimal Raspberry Pi 5 UART fault-localization proof.
 *
 * Firmware-selected Image placement has proven BL33 entry with the no-MMIO
 * reset proof, while UART-only images still produce no marker. This proof
 * writes a short marker to the firmware-preserved RP1 UART0 data register and
 * then immediately invokes PSCI SYSTEM_RESET. A second TFTP boot sequence means
 * the UART stores did not stop execution; no reset side effect keeps suspicion
 * on the UART MMIO access itself.
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

    ldr x9, =0x1c00030000
    adr x10, marker
write_byte:
    ldrb w11, [x10], #1
    cbz w11, reset_loop
    str w11, [x9]
    b write_byte

reset_loop:
    ldr x0, =0x84000009
    smc #0
    wfe
    b reset_loop

marker:
    .asciz "TA\r\n"

.balign 8
    .ltorg

.balign 8
__image_end:
