.section .vectors, "ax"
.balign 2048
.global __exception_vectors
.type __exception_vectors, %function

#if defined(TALOS_TARGET_RPI5_BCM2712) && defined(TALOS_RPI5_EXCEPTION_REPORT_DIAGNOSTIC)
.macro rpi5_uart10_wait_tx_ready
    mov x17, #0x10000
.Lwait_ready_loop\@:
    ldr w12, [x9, #0x18]
    tbz w12, #5, .Lwait_ready_done\@
    subs x17, x17, #1
    b.ne .Lwait_ready_loop\@
.Lwait_ready_done\@:
.endm

.macro rpi5_uart10_wait_tx_empty
    mov x17, #0x200000
.Lwait_empty_loop\@:
    ldr w12, [x9, #0x18]
    tbnz w12, #7, .Lwait_empty_done\@
    subs x17, x17, #1
    b.ne .Lwait_empty_loop\@
.Lwait_empty_done\@:
    dsb sy
.endm

.macro rpi5_uart10_write_payload payload
    adr x10, \payload
.Lpayload_loop\@:
    ldrb w11, [x10], #1
    cbz w11, .Lpayload_done\@
    rpi5_uart10_wait_tx_ready
    str w11, [x9]
    ldr w12, [x9, #0x18]
    b .Lpayload_loop\@
.Lpayload_done\@:
.endm

.macro rpi5_uart10_write_hex64 value
    mov w11, #'0'
    rpi5_uart10_wait_tx_ready
    str w11, [x9]
    ldr w12, [x9, #0x18]
    mov w11, #'x'
    rpi5_uart10_wait_tx_ready
    str w11, [x9]
    ldr w12, [x9, #0x18]

    mov x24, #60
.Lhex_loop\@:
    lsr x11, \value, x24
    and x11, x11, #0xf
    cmp x11, #10
    add x13, x11, #'0'
    add x14, x11, #('a' - 10)
    csel x11, x13, x14, lo
    rpi5_uart10_wait_tx_ready
    str w11, [x9]
    ldr w15, [x9, #0x18]
    subs x24, x24, #4
    b.pl .Lhex_loop\@
.endm

.macro rpi5_exception_asm_report
    ldr x9, =0x107d001000
    mov x16, x0
    mov x20, x1
    mov x21, x2
    mov x22, x3

    rpi5_uart10_write_payload 101f
    rpi5_uart10_write_hex64 x22
    rpi5_uart10_write_payload 102f
    rpi5_uart10_write_payload 103f
    rpi5_uart10_write_hex64 x16
    rpi5_uart10_write_payload 104f
    rpi5_uart10_write_hex64 x20
    rpi5_uart10_write_payload 105f
    rpi5_uart10_write_hex64 x21
    rpi5_uart10_write_payload 102f
    rpi5_uart10_wait_tx_empty
    b 106f
101:
    .asciz "TALOS: handler-entered vector="
102:
    .asciz "\r\n"
103:
    .asciz "TALOS: exception-info esr="
104:
    .asciz " elr="
105:
    .asciz " far="
    .balign 4
106:
.endm

.macro rpi5_exception_vector_marker
    ldr x9, =0x107d001000
    adr x10, 99f
97:
    ldrb w11, [x10], #1
    cbz w11, 98f
    str w11, [x9]
    ldr w12, [x9, #0x18]
    b 97b
98:
    dsb sy
    b 100f
99:
    .asciz "TALOS: vector-entry\r\n"
    .balign 4
100:
.endm
#endif

#if defined(TALOS_TARGET_RPI5_BCM2712) && defined(TALOS_RPI5_EXCEPTION_REPORT_DIAGNOSTIC)
.macro vector_entry kind
    .balign 128
    mov x3, #\kind
    b __exception_entry
.endm
#elif defined(TALOS_TARGET_RPI5_BCM2712) && defined(TALOS_RPI5_EXCEPTION_RETURN_DIAGNOSTIC)
.macro vector_entry kind
    .balign 128
    sub sp, sp, #32
    stp x0, x1, [sp]
    mov x0, #\kind
    str x0, [sp, #16]
    b __exception_entry_return
.endm
#else
.macro vector_entry kind
    .balign 128
    sub sp, sp, #32
    stp x0, x1, [sp]
    mov x0, #\kind
    str x0, [sp, #16]
    b __exception_entry_frame
.endm
#endif

__exception_vectors:
    vector_entry 0
    vector_entry 1
    vector_entry 2
    vector_entry 3
    vector_entry 4
    vector_entry 5
    vector_entry 6
    vector_entry 7
    vector_entry 8
    vector_entry 9
    vector_entry 10
    vector_entry 11
    vector_entry 12
    vector_entry 13
    vector_entry 14
    vector_entry 15

.size __exception_vectors, . - __exception_vectors

.section .text.exceptions, "ax"
.balign 4
#if defined(TALOS_TARGET_RPI5_BCM2712) && defined(TALOS_RPI5_EXCEPTION_RETURN_DIAGNOSTIC)
.global rpi5_brk_register_preserve_probe
.type rpi5_brk_register_preserve_probe, %function
rpi5_brk_register_preserve_probe:
    stp x19, x30, [sp, #-16]!
    movz x9, #0x7788
    movk x9, #0x5566, lsl #16
    movk x9, #0x3344, lsl #32
    movk x9, #0x1122, lsl #48
    movz x19, #0x2211
    movk x19, #0x4433, lsl #16
    movk x19, #0x6655, lsl #32
    movk x19, #0x8877, lsl #48
    brk #0
    str x9, [x0]
    str x19, [x1]

    movz x10, #0x7788
    movk x10, #0x5566, lsl #16
    movk x10, #0x3344, lsl #32
    movk x10, #0x1122, lsl #48
    cmp x9, x10
    cset x11, eq

    movz x10, #0x2211
    movk x10, #0x4433, lsl #16
    movk x10, #0x6655, lsl #32
    movk x10, #0x8877, lsl #48
    cmp x19, x10
    cset x12, eq
    and x0, x11, x12
    ldp x19, x30, [sp], #16
    ret

.size rpi5_brk_register_preserve_probe, . - rpi5_brk_register_preserve_probe

.type __exception_entry_return, %function
__exception_entry_return:
    sub sp, sp, #272
    stp x2, x3, [sp, #16]
    stp x4, x5, [sp, #32]
    stp x6, x7, [sp, #48]
    stp x8, x9, [sp, #64]
    stp x10, x11, [sp, #80]
    stp x12, x13, [sp, #96]
    stp x14, x15, [sp, #112]
    stp x16, x17, [sp, #128]
    stp x18, x19, [sp, #144]
    stp x20, x21, [sp, #160]
    stp x22, x23, [sp, #176]
    stp x24, x25, [sp, #192]
    stp x26, x27, [sp, #208]
    stp x28, x29, [sp, #224]
    str x30, [sp, #240]
    ldp x4, x5, [sp, #272]
    stp x4, x5, [sp]
    ldr x3, [sp, #288]

    mrs x4, CurrentEL
    cmp x4, #(2 << 2)
    b.eq 2f
    cmp x4, #(3 << 2)
    b.eq 3f

    mrs x0, ESR_EL1
    mrs x1, ELR_EL1
    mrs x2, FAR_EL1
    mrs x4, SPSR_EL1
    mov x5, sp
    bl rust_exception_handler
    msr ELR_EL1, x0
    b 4f

2:
    mrs x0, ESR_EL2
    mrs x1, ELR_EL2
    mrs x2, FAR_EL2
    mrs x4, SPSR_EL2
    mov x5, sp
    bl rust_exception_handler
    msr ELR_EL2, x0
    b 4f

3:
    mrs x0, ESR_EL3
    mrs x1, ELR_EL3
    mrs x2, FAR_EL3
    mrs x4, SPSR_EL3
    mov x5, sp
    bl rust_exception_handler
    msr ELR_EL3, x0

4:
    ldp x0, x1, [sp]
    ldp x2, x3, [sp, #16]
    ldp x4, x5, [sp, #32]
    ldp x6, x7, [sp, #48]
    ldp x8, x9, [sp, #64]
    ldp x10, x11, [sp, #80]
    ldp x12, x13, [sp, #96]
    ldp x14, x15, [sp, #112]
    ldp x16, x17, [sp, #128]
    ldp x18, x19, [sp, #144]
    ldp x20, x21, [sp, #160]
    ldp x22, x23, [sp, #176]
    ldp x24, x25, [sp, #192]
    ldp x26, x27, [sp, #208]
    ldp x28, x29, [sp, #224]
    ldr x30, [sp, #240]
    add sp, sp, #304
    eret

.size __exception_entry_return, . - __exception_entry_return
#endif

#if !defined(TALOS_RPI5_EXCEPTION_REPORT_DIAGNOSTIC) && !defined(TALOS_RPI5_EXCEPTION_RETURN_DIAGNOSTIC)
.type __exception_entry_frame, %function
__exception_entry_frame:
    sub sp, sp, #272
    stp x2, x3, [sp, #16]
    stp x4, x5, [sp, #32]
    stp x6, x7, [sp, #48]
    stp x8, x9, [sp, #64]
    stp x10, x11, [sp, #80]
    stp x12, x13, [sp, #96]
    stp x14, x15, [sp, #112]
    stp x16, x17, [sp, #128]
    stp x18, x19, [sp, #144]
    stp x20, x21, [sp, #160]
    stp x22, x23, [sp, #176]
    stp x24, x25, [sp, #192]
    stp x26, x27, [sp, #208]
    stp x28, x29, [sp, #224]
    str x30, [sp, #240]
    ldp x4, x5, [sp, #272]
    stp x4, x5, [sp]
    ldr x3, [sp, #288]

    cmp x3, #1
    b.eq 5f
    cmp x3, #5
    b.eq 5f
    cmp x3, #9
    b.eq 5f
    cmp x3, #13
    b.eq 5f

    mrs x4, CurrentEL
    cmp x4, #(2 << 2)
    b.eq 2f
    cmp x4, #(3 << 2)
    b.eq 3f

    mrs x0, ESR_EL1
    mrs x1, ELR_EL1
    mrs x2, FAR_EL1
    mrs x4, SPSR_EL1
    mov x5, sp
    bl rust_exception_handler
    b 4f

2:
    mrs x0, ESR_EL2
    mrs x1, ELR_EL2
    mrs x2, FAR_EL2
    mrs x4, SPSR_EL2
    mov x5, sp
    bl rust_exception_handler
    b 4f

3:
    mrs x0, ESR_EL3
    mrs x1, ELR_EL3
    mrs x2, FAR_EL3
    mrs x4, SPSR_EL3
    mov x5, sp
    bl rust_exception_handler

4:
    wfe
    b 4b

5:
    mrs x4, CurrentEL
    cmp x4, #(2 << 2)
    b.eq 6f
    cmp x4, #(3 << 2)
    b.eq 7f

    mov x0, x3
    mrs x1, ELR_EL1
    mrs x2, SPSR_EL1
    mov x3, sp
    bl rust_irq_handler
    b 8f

6:
    mov x0, x3
    mrs x1, ELR_EL2
    mrs x2, SPSR_EL2
    mov x3, sp
    bl rust_irq_handler
    b 8f

7:
    mov x0, x3
    mrs x1, ELR_EL3
    mrs x2, SPSR_EL3
    mov x3, sp
    bl rust_irq_handler

8:
    ldp x0, x1, [sp]
    ldp x2, x3, [sp, #16]
    ldp x4, x5, [sp, #32]
    ldp x6, x7, [sp, #48]
    ldp x8, x9, [sp, #64]
    ldp x10, x11, [sp, #80]
    ldp x12, x13, [sp, #96]
    ldp x14, x15, [sp, #112]
    ldp x16, x17, [sp, #128]
    ldp x18, x19, [sp, #144]
    ldp x20, x21, [sp, #160]
    ldp x22, x23, [sp, #176]
    ldp x24, x25, [sp, #192]
    ldp x26, x27, [sp, #208]
    ldp x28, x29, [sp, #224]
    ldr x30, [sp, #240]
    add sp, sp, #304
    eret

.size __exception_entry_frame, . - __exception_entry_frame
#endif

.type __exception_entry, %function
__exception_entry:
#if defined(TALOS_TARGET_RPI5_BCM2712) && defined(TALOS_RPI5_EXCEPTION_REPORT_DIAGNOSTIC)
    rpi5_exception_vector_marker
#endif
    mrs x4, CurrentEL
    cmp x4, #(2 << 2)
    b.eq 2f
    cmp x4, #(3 << 2)
    b.eq 3f

    mrs x0, ESR_EL1
    mrs x1, ELR_EL1
    mrs x2, FAR_EL1
#if defined(TALOS_TARGET_RPI5_BCM2712) && defined(TALOS_RPI5_EXCEPTION_REPORT_DIAGNOSTIC)
    rpi5_exception_asm_report
    bl rust_exception_handler
#elif defined(TALOS_TARGET_RPI5_BCM2712) && defined(TALOS_RPI5_EXCEPTION_RETURN_DIAGNOSTIC)
    mrs x4, SPSR_EL1
    mov x5, xzr
    mov x6, xzr
    bl rust_exception_handler
    msr ELR_EL1, x0
    eret
#else
    bl rust_exception_handler
#endif
    b 1f

2:
    mrs x0, ESR_EL2
    mrs x1, ELR_EL2
    mrs x2, FAR_EL2
#if defined(TALOS_TARGET_RPI5_BCM2712) && defined(TALOS_RPI5_EXCEPTION_REPORT_DIAGNOSTIC)
    rpi5_exception_asm_report
    bl rust_exception_handler
#elif defined(TALOS_TARGET_RPI5_BCM2712) && defined(TALOS_RPI5_EXCEPTION_RETURN_DIAGNOSTIC)
    mrs x4, SPSR_EL2
    mov x5, xzr
    mov x6, xzr
    bl rust_exception_handler
    msr ELR_EL2, x0
    eret
#else
    bl rust_exception_handler
#endif
    b 1f

3:
    mrs x0, ESR_EL3
    mrs x1, ELR_EL3
    mrs x2, FAR_EL3
#if defined(TALOS_TARGET_RPI5_BCM2712) && defined(TALOS_RPI5_EXCEPTION_REPORT_DIAGNOSTIC)
    rpi5_exception_asm_report
    bl rust_exception_handler
#elif defined(TALOS_TARGET_RPI5_BCM2712) && defined(TALOS_RPI5_EXCEPTION_RETURN_DIAGNOSTIC)
    mrs x4, SPSR_EL3
    mov x5, xzr
    mov x6, xzr
    bl rust_exception_handler
    msr ELR_EL3, x0
    eret
#else
    bl rust_exception_handler
#endif

1:
    wfe
    b 1b

.size __exception_entry, . - __exception_entry
