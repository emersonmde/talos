/*
 * Raspberry Pi 5 UART proof linked with Talos' normal Pi 5 linker layout.
 *
 * The code path matches the successful assembly UART proof, but the section
 * layout mirrors the normal Rust image: .text.boot at 0x80000, a vector gap at
 * 0x80800, .text at 0x81000, and .rodata ending at the same image size as the
 * current Rust kernel. This isolates linker layout and file padding from Rust
 * startup code.
 */

.section .text.boot, "ax"
.global _start
.type _start, %function

_start:
    b image_entry
    .word 0
    .quad KERNEL_IMAGE_TEXT_OFFSET
    .quad __kernel_image_end - _start
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
    .asciz "TL\r\n"

.balign 8
    .ltorg

.section .vectors, "ax"
.balign 2048
    .space 1952, 0

.section .text, "ax"
.balign 4096
    .space 64048, 0

.section .rodata, "a"
.balign 4096
    .space 13672, 0
