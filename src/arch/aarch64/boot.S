.section .text.boot, "ax"
.global _start
.type _start, %function

_start:
    b 3f
    .long 0
    .quad KERNEL_IMAGE_TEXT_OFFSET
    .quad __kernel_image_end - _start
#ifdef TALOS_TARGET_RPI5_BCM2712
    .quad 0xc
#else
    .quad 0
#endif
    .quad 0
    .quad 0
    .quad 0
    .long 0x644d5241
    .long 0

3:
    mov x19, x0
#ifdef TALOS_TARGET_RPI5_BCM2712
#ifdef TALOS_RPI5_CARGO_ASM_UART_PROOF
    ldr x9, =0x107d001000
    bl rpi5_write_marker

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
    bl rpi5_write_marker

4:
    ldr x9, =0x107d001000
    bl rpi5_write_marker
    ldr x9, =0x1f00030000
    bl rpi5_write_marker
    b 4b
#else
#ifdef TALOS_RPI5_TRANSITION_DIAGNOSTIC
    ldr x9, =0x107d001000
    bl rpi5_write_marker
#else
    ldr x9, =0x107d001000
    bl rpi5_write_marker
#endif

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

#ifdef TALOS_RPI5_TRANSITION_DIAGNOSTIC
    ldr x9, =0x1f00030000
    bl rpi5_write_marker
#else
    ldr x9, =0x1f00030000
    bl rpi5_write_marker
#endif
#endif
#endif

    mrs x0, cpacr_el1
    orr x0, x0, #(0b11 << 20)
    msr cpacr_el1, x0
    isb
    adrp x0, __bss_start
    add x0, x0, :lo12:__bss_start
    adrp x1, __bss_end
    add x1, x1, :lo12:__bss_end
0:
    cmp x0, x1
    b.hs 1f
    str xzr, [x0], #8
    b 0b

1:
    adrp x0, __stack_top
    add x0, x0, :lo12:__stack_top
    mov sp, x0
#if defined(TALOS_TARGET_RPI5_BCM2712) && defined(TALOS_RPI5_TRANSITION_DIAGNOSTIC)
4:
    ldr x9, =0x107d001000
    bl rpi5_write_marker
    ldr x9, =0x1f00030000
    bl rpi5_write_marker
    b 4b
#elif defined(TALOS_TARGET_RPI5_BCM2712) && defined(TALOS_RPI5_TEXT_BOOT_FAR_DIAGNOSTIC)
    bl rpi5_text_boot_far_marker_loop
#elif defined(TALOS_TARGET_RPI5_BCM2712) && defined(TALOS_RPI5_TEXT_BOOT_NEAR_DIAGNOSTIC)
    bl rpi5_text_boot_near_marker_loop
#elif defined(TALOS_TARGET_RPI5_BCM2712) && defined(TALOS_RPI5_TEXT_BOOT_NEAR_BRANCH_DIAGNOSTIC)
    b rpi5_text_boot_near_branch_marker_loop
#elif defined(TALOS_TARGET_RPI5_BCM2712) && defined(TALOS_RPI5_TEXT_BOOT_NEXT_BRANCH_DIAGNOSTIC)
    b rpi5_text_boot_next_branch_marker_loop
rpi5_text_boot_next_branch_marker_loop:
4:
    ldr x9, =0x107d001000
    bl rpi5_write_marker
    ldr x9, =0x1f00030000
    bl rpi5_write_marker
    b 4b
#elif defined(TALOS_TARGET_RPI5_BCM2712) && defined(TALOS_RPI5_FALLTHROUGH_RUST_DIAGNOSTIC)
    mov x20, #0x4000
4:
    ldr x9, =0x107d001000
    bl rpi5_write_marker
    ldr x9, =0x1f00030000
    bl rpi5_write_marker
    subs x20, x20, #1
    b.ne 4b
#elif defined(TALOS_TARGET_RPI5_BCM2712) && defined(TALOS_RPI5_POST_STACK_NOP_DIAGNOSTIC)
    nop
4:
    ldr x9, =0x107d001000
    bl rpi5_write_marker
    ldr x9, =0x1f00030000
    bl rpi5_write_marker
    b 4b
#elif defined(TALOS_TARGET_RPI5_BCM2712) && defined(TALOS_RPI5_POST_STACK_RESET_DIAGNOSTIC)
    mov x20, #8
4:
    ldr x9, =0x107d001000
    bl rpi5_write_marker
    ldr x9, =0x1f00030000
    bl rpi5_write_marker
    subs x20, x20, #1
    b.ne 4b
5:
    ldr x0, =0x84000009
    smc #0
    wfe
    b 5b
#elif defined(TALOS_TARGET_RPI5_BCM2712) && defined(TALOS_RPI5_ASM_INDIRECT_RESET_DIAGNOSTIC)
    ldr x20, =rpi5_asm_indirect_reset_probe
    blr x20
4:
    wfe
    b 4b
#elif defined(TALOS_TARGET_RPI5_BCM2712) && defined(TALOS_RPI5_ASM_DIRECT_RESET_DIAGNOSTIC)
    bl rpi5_asm_direct_reset_probe
4:
    wfe
    b 4b
#elif defined(TALOS_TARGET_RPI5_BCM2712) && defined(TALOS_RPI5_ASM_TO_RUST_RESET_DIAGNOSTIC)
    bl rpi5_asm_to_rust_reset_probe
4:
    wfe
    b 4b
#elif defined(TALOS_TARGET_RPI5_BCM2712) && defined(TALOS_RPI5_ASM_INDIRECT_TO_RUST_RESET_DIAGNOSTIC)
    ldr x20, =rpi5_asm_to_rust_reset_probe
    blr x20
4:
    wfe
    b 4b
#elif defined(TALOS_TARGET_RPI5_BCM2712) && defined(TALOS_RPI5_ASM_BTI_INDIRECT_TO_RUST_RESET_DIAGNOSTIC)
    ldr x20, =rpi5_asm_bti_to_rust_reset_veneer
    blr x20
4:
    wfe
    b 4b
#elif defined(TALOS_TARGET_RPI5_BCM2712) && defined(TALOS_RPI5_VECTOR_SECTION_DIAGNOSTIC)
    bl rpi5_vector_section_marker_loop
#elif defined(TALOS_TARGET_RPI5_BCM2712) && defined(TALOS_RPI5_TEXT_SECTION_DIAGNOSTIC)
    bl rpi5_text_section_marker_loop
#endif
    mov x0, x19
    bl rust_entry

2:
    wfe
    b 2b

.size _start, . - _start

#if defined(TALOS_TARGET_RPI5_BCM2712) && defined(TALOS_RPI5_ASM_INDIRECT_RESET_DIAGNOSTIC)
.balign 4
.type rpi5_asm_indirect_reset_probe, %function
rpi5_asm_indirect_reset_probe:
    ldr x0, =0x84000009
    smc #0
4:
    wfe
    b 4b

.size rpi5_asm_indirect_reset_probe, . - rpi5_asm_indirect_reset_probe
#endif

#if defined(TALOS_TARGET_RPI5_BCM2712) && defined(TALOS_RPI5_ASM_BTI_INDIRECT_TO_RUST_RESET_DIAGNOSTIC)
.balign 4
.type rpi5_asm_bti_to_rust_reset_veneer, %function
rpi5_asm_bti_to_rust_reset_veneer:
    hint #34
    b rpi5_asm_to_rust_reset_probe

.size rpi5_asm_bti_to_rust_reset_veneer, . - rpi5_asm_bti_to_rust_reset_veneer
#endif

#if defined(TALOS_TARGET_RPI5_BCM2712) && defined(TALOS_RPI5_ASM_DIRECT_RESET_DIAGNOSTIC)
.balign 4
.type rpi5_asm_direct_reset_probe, %function
rpi5_asm_direct_reset_probe:
    ldr x0, =0x84000009
    smc #0
4:
    wfe
    b 4b

.size rpi5_asm_direct_reset_probe, . - rpi5_asm_direct_reset_probe
#endif

#ifdef TALOS_TARGET_RPI5_BCM2712
rpi5_write_marker:
    adr x10, 9f
12:
    ldrb w11, [x10], #1
    cbz w11, 13f
    str w11, [x9]
    ldr w12, [x9, #0x18]
    b 12b
13:
    dsb sy
    ret

9:
#ifdef TALOS_RPI5_CARGO_ASM_UART_PROOF
    .asciz "CL\r\n"
#elif defined(TALOS_RPI5_TRANSITION_DIAGNOSTIC)
    .asciz "SP\r\n"
#elif defined(TALOS_RPI5_TEXT_BOOT_FAR_DIAGNOSTIC)
    .asciz "BF\r\n"
#elif defined(TALOS_RPI5_TEXT_BOOT_NEAR_DIAGNOSTIC)
    .asciz "BN\r\n"
#elif defined(TALOS_RPI5_TEXT_BOOT_NEAR_BRANCH_DIAGNOSTIC)
    .asciz "BB\r\n"
#elif defined(TALOS_RPI5_TEXT_BOOT_NEXT_BRANCH_DIAGNOSTIC)
    .asciz "NB\r\n"
#elif defined(TALOS_RPI5_FALLTHROUGH_RUST_DIAGNOSTIC)
    .asciz "FR\r\n"
#elif defined(TALOS_RPI5_POST_STACK_NOP_DIAGNOSTIC)
    .asciz "PN\r\n"
#elif defined(TALOS_RPI5_POST_STACK_RESET_DIAGNOSTIC)
    .asciz "RS\r\n"
#elif defined(TALOS_RPI5_VECTOR_SECTION_DIAGNOSTIC)
    .asciz "VX\r\n"
#elif defined(TALOS_RPI5_TEXT_SECTION_DIAGNOSTIC)
    .asciz "TX\r\n"
#else
    .asciz "BI\r\n"
#endif

#if defined(TALOS_TARGET_RPI5_BCM2712) && defined(TALOS_RPI5_TEXT_SECTION_DIAGNOSTIC)
.section .text.rpi5_text_section_diagnostic, "ax"
.type rpi5_text_section_marker_loop, %function
rpi5_text_section_marker_loop:
4:
    ldr x9, =0x107d001000
    bl rpi5_write_marker
    ldr x9, =0x1f00030000
    bl rpi5_write_marker
    b 4b

.size rpi5_text_section_marker_loop, . - rpi5_text_section_marker_loop
#endif

#if defined(TALOS_TARGET_RPI5_BCM2712) && defined(TALOS_RPI5_TEXT_BOOT_FAR_DIAGNOSTIC)
.balign 512
.type rpi5_text_boot_far_marker_loop, %function
rpi5_text_boot_far_marker_loop:
4:
    ldr x9, =0x107d001000
    bl rpi5_write_marker
    ldr x9, =0x1f00030000
    bl rpi5_write_marker
    b 4b

.size rpi5_text_boot_far_marker_loop, . - rpi5_text_boot_far_marker_loop
#endif

#if defined(TALOS_TARGET_RPI5_BCM2712) && defined(TALOS_RPI5_TEXT_BOOT_NEAR_DIAGNOSTIC)
.balign 4
.type rpi5_text_boot_near_marker_loop, %function
rpi5_text_boot_near_marker_loop:
4:
    ldr x9, =0x107d001000
    bl rpi5_write_marker
    ldr x9, =0x1f00030000
    bl rpi5_write_marker
    b 4b

.size rpi5_text_boot_near_marker_loop, . - rpi5_text_boot_near_marker_loop
#endif

#if defined(TALOS_TARGET_RPI5_BCM2712) && defined(TALOS_RPI5_TEXT_BOOT_NEAR_BRANCH_DIAGNOSTIC)
.balign 4
.type rpi5_text_boot_near_branch_marker_loop, %function
rpi5_text_boot_near_branch_marker_loop:
4:
    ldr x9, =0x107d001000
    bl rpi5_write_marker
    ldr x9, =0x1f00030000
    bl rpi5_write_marker
    b 4b

.size rpi5_text_boot_near_branch_marker_loop, . - rpi5_text_boot_near_branch_marker_loop
#endif

#if defined(TALOS_TARGET_RPI5_BCM2712) && defined(TALOS_RPI5_VECTOR_SECTION_DIAGNOSTIC)
.section .vectors.rpi5_vector_section_diagnostic, "ax"
.type rpi5_vector_section_marker_loop, %function
rpi5_vector_section_marker_loop:
4:
    ldr x9, =0x107d001000
    bl rpi5_write_marker
    ldr x9, =0x1f00030000
    bl rpi5_write_marker
    b 4b

.size rpi5_vector_section_marker_loop, . - rpi5_vector_section_marker_loop
#endif

#endif
