/*
 * Minimal Raspberry Pi 5 armstub diagnostic.
 *
 * This is not a boot path for Talos. It is a bounded lab diagnostic used to
 * prove whether Pi firmware reaches a configured custom armstub before the
 * kernel_2712.img handoff.
 */

.section .text, "ax"
.global _start

_start:
    ldr x9, =0x1c00030000

    mov w12, #0
    str w12, [x9, #0x30]
    mov w12, #0x7ff
    str w12, [x9, #0x44]
    mov w12, #(3 << 5)
    str w12, [x9, #0x2c]
    mov w12, #0x301
    str w12, [x9, #0x30]

    adr x10, armstub_marker

1:
    ldrb w11, [x10], #1
    cbz w11, 4f
    mov w13, #0x10000

2:
    ldr w12, [x9, #0x18]
    tbz w12, #5, 3f
    subs w13, w13, #1
    b.ne 2b

3:
    str w11, [x9]
    b 1b

4:
    wfe
    b 4b

armstub_marker:
    .asciz "S1\r\n"
