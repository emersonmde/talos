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
#elif defined(TALOS_TARGET_RPI5_BCM2712) && defined(TALOS_RPI5_ASM_TEXT_INDIRECT_RESET_DIAGNOSTIC)
    ldr x20, =rpi5_asm_text_reset_probe
    blr x20
4:
    wfe
    b 4b
#elif defined(TALOS_TARGET_RPI5_BCM2712) && defined(TALOS_RPI5_ASM_TEXT_DIRECT_RESET_DIAGNOSTIC)
    bl rpi5_asm_text_reset_probe
4:
    wfe
    b 4b
#elif defined(TALOS_TARGET_RPI5_BCM2712) && defined(TALOS_RPI5_ASM_TEXT_JC_INDIRECT_RESET_DIAGNOSTIC)
    ldr x20, =rpi5_asm_text_jc_reset_probe
    blr x20
4:
    wfe
    b 4b
#elif defined(TALOS_TARGET_RPI5_BCM2712) && defined(TALOS_RPI5_BTI_EXCEPTION_CLASSIFIER_DIAGNOSTIC)
    adr x0, rpi5_bti_exception_vectors
    mrs x1, CurrentEL
    cmp x1, #(1 << 2)
    b.eq 5f
    cmp x1, #(2 << 2)
    b.eq 6f
    cmp x1, #(3 << 2)
    b.eq 7f
    b rpi5_bti_classifier_park
5:
    msr vbar_el1, x0
    b 8f
6:
    msr vbar_el2, x0
    b 8f
7:
    msr vbar_el3, x0
8:
    isb
    ldr x20, =rpi5_bti_classifier_target
    blr x20
    b rpi5_bti_classifier_park
#elif defined(TALOS_TARGET_RPI5_BCM2712) && defined(TALOS_RPI5_BTI_CLASSIFIER_WITH_BRK_PRECHECK_DIAGNOSTIC)
    adr x0, rpi5_bti_brk_precheck_vectors
    mrs x1, CurrentEL
    cmp x1, #(1 << 2)
    b.eq 5f
    cmp x1, #(2 << 2)
    b.eq 6f
    cmp x1, #(3 << 2)
    b.eq 7f
    b rpi5_bti_brk_precheck_park
5:
    msr vbar_el1, x0
    b 8f
6:
    msr vbar_el2, x0
    b 8f
7:
    msr vbar_el3, x0
8:
    isb
    brk #0
rpi5_bti_brk_precheck_after_brk:
    ldr x20, =rpi5_bti_brk_precheck_target
    blr x20
    b rpi5_bti_brk_precheck_park
#elif defined(TALOS_TARGET_RPI5_BCM2712) && defined(TALOS_RPI5_DIRECT_EXCEPTION_CONTROL_DIAGNOSTIC)
    adr x0, rpi5_direct_exception_vectors
    mrs x1, CurrentEL
    cmp x1, #(1 << 2)
    b.eq 5f
    cmp x1, #(2 << 2)
    b.eq 6f
    cmp x1, #(3 << 2)
    b.eq 7f
    b rpi5_direct_exception_park
5:
    msr vbar_el1, x0
    b 8f
6:
    msr vbar_el2, x0
    b 8f
7:
    msr vbar_el3, x0
8:
    isb
    brk #0
    b rpi5_direct_exception_park
#elif defined(TALOS_TARGET_RPI5_BCM2712) && defined(TALOS_RPI5_DIRECT_EXCEPTION_IMMEDIATE_RESET_DIAGNOSTIC)
    adr x0, rpi5_direct_exception_immediate_vectors
    mrs x1, CurrentEL
    cmp x1, #(1 << 2)
    b.eq 5f
    cmp x1, #(2 << 2)
    b.eq 6f
    cmp x1, #(3 << 2)
    b.eq 7f
    b rpi5_direct_exception_immediate_park
5:
    msr vbar_el1, x0
    b 8f
6:
    msr vbar_el2, x0
    b 8f
7:
    msr vbar_el3, x0
8:
    isb
    brk #0
    b rpi5_direct_exception_immediate_park
#elif defined(TALOS_TARGET_RPI5_BCM2712) && defined(TALOS_RPI5_BRK_ERET_RESUME_RESET_DIAGNOSTIC)
    adr x0, rpi5_brk_eret_resume_reset_vectors
    mrs x1, CurrentEL
    cmp x1, #(1 << 2)
    b.eq 5f
    cmp x1, #(2 << 2)
    b.eq 6f
    cmp x1, #(3 << 2)
    b.eq 7f
    b rpi5_brk_eret_resume_reset_park
5:
    msr vbar_el1, x0
    b 8f
6:
    msr vbar_el2, x0
    b 8f
7:
    msr vbar_el3, x0
8:
    isb
    brk #0
    b rpi5_brk_eret_resume_reset_park
rpi5_brk_eret_resume_reset_continuation:
    ldr x0, =0x84000009
    smc #0
    b rpi5_brk_eret_resume_reset_park
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

#if defined(TALOS_TARGET_RPI5_BCM2712) && (defined(TALOS_RPI5_ASM_TEXT_INDIRECT_RESET_DIAGNOSTIC) || defined(TALOS_RPI5_ASM_TEXT_DIRECT_RESET_DIAGNOSTIC))
.section .text.rpi5_asm_text_indirect_reset_diagnostic, "ax"
.balign 16
.type rpi5_asm_text_reset_probe, %function
rpi5_asm_text_reset_probe:
    hint #34
    ldr x0, =0x84000009
    smc #0
4:
    wfe
    b 4b

.size rpi5_asm_text_reset_probe, . - rpi5_asm_text_reset_probe
.section .text.boot, "ax"
#endif

#if defined(TALOS_TARGET_RPI5_BCM2712) && defined(TALOS_RPI5_ASM_TEXT_JC_INDIRECT_RESET_DIAGNOSTIC)
.section .text.rpi5_asm_text_jc_indirect_reset_diagnostic, "ax"
.balign 16
.type rpi5_asm_text_jc_reset_probe, %function
rpi5_asm_text_jc_reset_probe:
    hint #38
    ldr x0, =0x84000009
    smc #0
4:
    wfe
    b 4b

.size rpi5_asm_text_jc_reset_probe, . - rpi5_asm_text_jc_reset_probe
.section .text.boot, "ax"
#endif

#if defined(TALOS_TARGET_RPI5_BCM2712) && defined(TALOS_RPI5_BTI_EXCEPTION_CLASSIFIER_DIAGNOSTIC)
.section .text.rpi5_bti_exception_classifier_diagnostic, "ax"
.balign 16
.type rpi5_bti_classifier_target, %function
rpi5_bti_classifier_target:
    hint #34
    ldr x0, =0x84000009
    smc #0
    b rpi5_bti_classifier_park

.size rpi5_bti_classifier_target, . - rpi5_bti_classifier_target

.balign 2048
rpi5_bti_exception_vectors:
    b rpi5_bti_exception_handler
    .space 0x80 - (. - rpi5_bti_exception_vectors)
    b rpi5_bti_exception_handler
    .space 0x100 - (. - rpi5_bti_exception_vectors)
    b rpi5_bti_exception_handler
    .space 0x180 - (. - rpi5_bti_exception_vectors)
    b rpi5_bti_exception_handler
    .space 0x200 - (. - rpi5_bti_exception_vectors)
    b rpi5_bti_exception_handler
    .space 0x280 - (. - rpi5_bti_exception_vectors)
    b rpi5_bti_exception_handler
    .space 0x300 - (. - rpi5_bti_exception_vectors)
    b rpi5_bti_exception_handler
    .space 0x380 - (. - rpi5_bti_exception_vectors)
    b rpi5_bti_exception_handler
    .space 0x400 - (. - rpi5_bti_exception_vectors)
    b rpi5_bti_exception_handler
    .space 0x480 - (. - rpi5_bti_exception_vectors)
    b rpi5_bti_exception_handler
    .space 0x500 - (. - rpi5_bti_exception_vectors)
    b rpi5_bti_exception_handler
    .space 0x580 - (. - rpi5_bti_exception_vectors)
    b rpi5_bti_exception_handler
    .space 0x600 - (. - rpi5_bti_exception_vectors)
    b rpi5_bti_exception_handler
    .space 0x680 - (. - rpi5_bti_exception_vectors)
    b rpi5_bti_exception_handler
    .space 0x700 - (. - rpi5_bti_exception_vectors)
    b rpi5_bti_exception_handler
    .space 0x780 - (. - rpi5_bti_exception_vectors)
    b rpi5_bti_exception_handler

rpi5_bti_exception_handler:
    mrs x20, CurrentEL
    cmp x20, #(1 << 2)
    b.eq 1f
    cmp x20, #(2 << 2)
    b.eq 2f
    cmp x20, #(3 << 2)
    b.eq 3f
    b rpi5_bti_classifier_park
1:
    mrs x21, esr_el1
    b 4f
2:
    mrs x21, esr_el2
    b 4f
3:
    mrs x21, esr_el3
4:
    lsr x21, x21, #26
    cmp x21, #0x0d
    b.ne rpi5_bti_classifier_park
    mov x23, #64
5:
    ldr x22, =0x10000000
6:
    subs x22, x22, #1
    b.ne 6b
    subs x23, x23, #1
    b.ne 5b
    ldr x0, =0x84000009
    smc #0

rpi5_bti_classifier_park:
    wfe
    b rpi5_bti_classifier_park

.section .text.boot, "ax"
#endif

#if defined(TALOS_TARGET_RPI5_BCM2712) && defined(TALOS_RPI5_BTI_CLASSIFIER_WITH_BRK_PRECHECK_DIAGNOSTIC)
.section .text.rpi5_bti_classifier_with_brk_precheck_diagnostic, "ax"
.balign 16
.type rpi5_bti_brk_precheck_target, %function
rpi5_bti_brk_precheck_target:
    hint #34
    ldr x0, =0x84000009
    smc #0
    b rpi5_bti_brk_precheck_park

.size rpi5_bti_brk_precheck_target, . - rpi5_bti_brk_precheck_target

.balign 2048
rpi5_bti_brk_precheck_vectors:
    b rpi5_bti_brk_precheck_handler
    .space 0x80 - (. - rpi5_bti_brk_precheck_vectors)
    b rpi5_bti_brk_precheck_handler
    .space 0x100 - (. - rpi5_bti_brk_precheck_vectors)
    b rpi5_bti_brk_precheck_handler
    .space 0x180 - (. - rpi5_bti_brk_precheck_vectors)
    b rpi5_bti_brk_precheck_handler
    .space 0x200 - (. - rpi5_bti_brk_precheck_vectors)
    b rpi5_bti_brk_precheck_handler
    .space 0x280 - (. - rpi5_bti_brk_precheck_vectors)
    b rpi5_bti_brk_precheck_handler
    .space 0x300 - (. - rpi5_bti_brk_precheck_vectors)
    b rpi5_bti_brk_precheck_handler
    .space 0x380 - (. - rpi5_bti_brk_precheck_vectors)
    b rpi5_bti_brk_precheck_handler
    .space 0x400 - (. - rpi5_bti_brk_precheck_vectors)
    b rpi5_bti_brk_precheck_handler
    .space 0x480 - (. - rpi5_bti_brk_precheck_vectors)
    b rpi5_bti_brk_precheck_handler
    .space 0x500 - (. - rpi5_bti_brk_precheck_vectors)
    b rpi5_bti_brk_precheck_handler
    .space 0x580 - (. - rpi5_bti_brk_precheck_vectors)
    b rpi5_bti_brk_precheck_handler
    .space 0x600 - (. - rpi5_bti_brk_precheck_vectors)
    b rpi5_bti_brk_precheck_handler
    .space 0x680 - (. - rpi5_bti_brk_precheck_vectors)
    b rpi5_bti_brk_precheck_handler
    .space 0x700 - (. - rpi5_bti_brk_precheck_vectors)
    b rpi5_bti_brk_precheck_handler
    .space 0x780 - (. - rpi5_bti_brk_precheck_vectors)
    b rpi5_bti_brk_precheck_handler

rpi5_bti_brk_precheck_handler:
    mrs x20, CurrentEL
    cmp x20, #(1 << 2)
    b.eq 1f
    cmp x20, #(2 << 2)
    b.eq 2f
    cmp x20, #(3 << 2)
    b.eq 3f
    b rpi5_bti_brk_precheck_park
1:
    mrs x21, esr_el1
    b 4f
2:
    mrs x21, esr_el2
    b 4f
3:
    mrs x21, esr_el3
4:
    lsr x21, x21, #26
    cmp x21, #0x3c
    b.eq rpi5_bti_brk_precheck_resume
    cmp x21, #0x0d
    b.eq rpi5_bti_brk_precheck_bte_reset
    b rpi5_bti_brk_precheck_park

rpi5_bti_brk_precheck_resume:
    adr x21, rpi5_bti_brk_precheck_after_brk
    cmp x20, #(1 << 2)
    b.eq 1f
    cmp x20, #(2 << 2)
    b.eq 2f
    cmp x20, #(3 << 2)
    b.eq 3f
    b rpi5_bti_brk_precheck_park
1:
    msr elr_el1, x21
    eret
2:
    msr elr_el2, x21
    eret
3:
    msr elr_el3, x21
    eret

rpi5_bti_brk_precheck_bte_reset:
    mov x23, #8
5:
    ldr x22, =0x10000000
6:
    subs x22, x22, #1
    b.ne 6b
    subs x23, x23, #1
    b.ne 5b
    ldr x0, =0x84000009
    smc #0

rpi5_bti_brk_precheck_park:
    wfe
    b rpi5_bti_brk_precheck_park

.section .text.boot, "ax"
#endif

#if defined(TALOS_TARGET_RPI5_BCM2712) && defined(TALOS_RPI5_BRK_ERET_RESUME_RESET_DIAGNOSTIC)
.section .text.rpi5_brk_eret_resume_reset_diagnostic, "ax"
.balign 2048
rpi5_brk_eret_resume_reset_vectors:
    b rpi5_brk_eret_resume_reset_handler
    .space 0x80 - (. - rpi5_brk_eret_resume_reset_vectors)
    b rpi5_brk_eret_resume_reset_handler
    .space 0x100 - (. - rpi5_brk_eret_resume_reset_vectors)
    b rpi5_brk_eret_resume_reset_handler
    .space 0x180 - (. - rpi5_brk_eret_resume_reset_vectors)
    b rpi5_brk_eret_resume_reset_handler
    .space 0x200 - (. - rpi5_brk_eret_resume_reset_vectors)
    b rpi5_brk_eret_resume_reset_handler
    .space 0x280 - (. - rpi5_brk_eret_resume_reset_vectors)
    b rpi5_brk_eret_resume_reset_handler
    .space 0x300 - (. - rpi5_brk_eret_resume_reset_vectors)
    b rpi5_brk_eret_resume_reset_handler
    .space 0x380 - (. - rpi5_brk_eret_resume_reset_vectors)
    b rpi5_brk_eret_resume_reset_handler
    .space 0x400 - (. - rpi5_brk_eret_resume_reset_vectors)
    b rpi5_brk_eret_resume_reset_handler
    .space 0x480 - (. - rpi5_brk_eret_resume_reset_vectors)
    b rpi5_brk_eret_resume_reset_handler
    .space 0x500 - (. - rpi5_brk_eret_resume_reset_vectors)
    b rpi5_brk_eret_resume_reset_handler
    .space 0x580 - (. - rpi5_brk_eret_resume_reset_vectors)
    b rpi5_brk_eret_resume_reset_handler
    .space 0x600 - (. - rpi5_brk_eret_resume_reset_vectors)
    b rpi5_brk_eret_resume_reset_handler
    .space 0x680 - (. - rpi5_brk_eret_resume_reset_vectors)
    b rpi5_brk_eret_resume_reset_handler
    .space 0x700 - (. - rpi5_brk_eret_resume_reset_vectors)
    b rpi5_brk_eret_resume_reset_handler
    .space 0x780 - (. - rpi5_brk_eret_resume_reset_vectors)
    b rpi5_brk_eret_resume_reset_handler

rpi5_brk_eret_resume_reset_handler:
    mrs x20, CurrentEL
    cmp x20, #(1 << 2)
    b.eq 1f
    cmp x20, #(2 << 2)
    b.eq 2f
    cmp x20, #(3 << 2)
    b.eq 3f
    b rpi5_brk_eret_resume_reset_park
1:
    mrs x21, esr_el1
    ldr x22, =rpi5_brk_eret_resume_reset_continuation
    lsr x21, x21, #26
    cmp x21, #0x3c
    b.ne rpi5_brk_eret_resume_reset_park
    msr elr_el1, x22
    eret
2:
    mrs x21, esr_el2
    ldr x22, =rpi5_brk_eret_resume_reset_continuation
    lsr x21, x21, #26
    cmp x21, #0x3c
    b.ne rpi5_brk_eret_resume_reset_park
    msr elr_el2, x22
    eret
3:
    mrs x21, esr_el3
    ldr x22, =rpi5_brk_eret_resume_reset_continuation
    lsr x21, x21, #26
    cmp x21, #0x3c
    b.ne rpi5_brk_eret_resume_reset_park
    msr elr_el3, x22
    eret

rpi5_brk_eret_resume_reset_park:
    wfe
    b rpi5_brk_eret_resume_reset_park

.section .text.boot, "ax"
#endif

#if defined(TALOS_TARGET_RPI5_BCM2712) && defined(TALOS_RPI5_DIRECT_EXCEPTION_CONTROL_DIAGNOSTIC)
.section .text.rpi5_direct_exception_control_diagnostic, "ax"
.balign 2048
rpi5_direct_exception_vectors:
    b rpi5_direct_exception_handler
    .space 0x80 - (. - rpi5_direct_exception_vectors)
    b rpi5_direct_exception_handler
    .space 0x100 - (. - rpi5_direct_exception_vectors)
    b rpi5_direct_exception_handler
    .space 0x180 - (. - rpi5_direct_exception_vectors)
    b rpi5_direct_exception_handler
    .space 0x200 - (. - rpi5_direct_exception_vectors)
    b rpi5_direct_exception_handler
    .space 0x280 - (. - rpi5_direct_exception_vectors)
    b rpi5_direct_exception_handler
    .space 0x300 - (. - rpi5_direct_exception_vectors)
    b rpi5_direct_exception_handler
    .space 0x380 - (. - rpi5_direct_exception_vectors)
    b rpi5_direct_exception_handler
    .space 0x400 - (. - rpi5_direct_exception_vectors)
    b rpi5_direct_exception_handler
    .space 0x480 - (. - rpi5_direct_exception_vectors)
    b rpi5_direct_exception_handler
    .space 0x500 - (. - rpi5_direct_exception_vectors)
    b rpi5_direct_exception_handler
    .space 0x580 - (. - rpi5_direct_exception_vectors)
    b rpi5_direct_exception_handler
    .space 0x600 - (. - rpi5_direct_exception_vectors)
    b rpi5_direct_exception_handler
    .space 0x680 - (. - rpi5_direct_exception_vectors)
    b rpi5_direct_exception_handler
    .space 0x700 - (. - rpi5_direct_exception_vectors)
    b rpi5_direct_exception_handler
    .space 0x780 - (. - rpi5_direct_exception_vectors)
    b rpi5_direct_exception_handler

rpi5_direct_exception_handler:
    mrs x20, CurrentEL
    cmp x20, #(1 << 2)
    b.eq 1f
    cmp x20, #(2 << 2)
    b.eq 2f
    cmp x20, #(3 << 2)
    b.eq 3f
    b rpi5_direct_exception_park
1:
    mrs x21, esr_el1
    b 4f
2:
    mrs x21, esr_el2
    b 4f
3:
    mrs x21, esr_el3
4:
    lsr x21, x21, #26
    cmp x21, #0x3c
    b.ne rpi5_direct_exception_park
    mov x23, #4
5:
    ldr x22, =0x10000000
6:
    subs x22, x22, #1
    b.ne 6b
    subs x23, x23, #1
    b.ne 5b
    ldr x0, =0x84000009
    smc #0

rpi5_direct_exception_park:
    wfe
    b rpi5_direct_exception_park

.section .text.boot, "ax"
#endif

#if defined(TALOS_TARGET_RPI5_BCM2712) && defined(TALOS_RPI5_DIRECT_EXCEPTION_IMMEDIATE_RESET_DIAGNOSTIC)
.section .text.rpi5_direct_exception_immediate_reset_diagnostic, "ax"
.balign 2048
rpi5_direct_exception_immediate_vectors:
    b rpi5_direct_exception_immediate_handler
    .space 0x80 - (. - rpi5_direct_exception_immediate_vectors)
    b rpi5_direct_exception_immediate_handler
    .space 0x100 - (. - rpi5_direct_exception_immediate_vectors)
    b rpi5_direct_exception_immediate_handler
    .space 0x180 - (. - rpi5_direct_exception_immediate_vectors)
    b rpi5_direct_exception_immediate_handler
    .space 0x200 - (. - rpi5_direct_exception_immediate_vectors)
    b rpi5_direct_exception_immediate_handler
    .space 0x280 - (. - rpi5_direct_exception_immediate_vectors)
    b rpi5_direct_exception_immediate_handler
    .space 0x300 - (. - rpi5_direct_exception_immediate_vectors)
    b rpi5_direct_exception_immediate_handler
    .space 0x380 - (. - rpi5_direct_exception_immediate_vectors)
    b rpi5_direct_exception_immediate_handler
    .space 0x400 - (. - rpi5_direct_exception_immediate_vectors)
    b rpi5_direct_exception_immediate_handler
    .space 0x480 - (. - rpi5_direct_exception_immediate_vectors)
    b rpi5_direct_exception_immediate_handler
    .space 0x500 - (. - rpi5_direct_exception_immediate_vectors)
    b rpi5_direct_exception_immediate_handler
    .space 0x580 - (. - rpi5_direct_exception_immediate_vectors)
    b rpi5_direct_exception_immediate_handler
    .space 0x600 - (. - rpi5_direct_exception_immediate_vectors)
    b rpi5_direct_exception_immediate_handler
    .space 0x680 - (. - rpi5_direct_exception_immediate_vectors)
    b rpi5_direct_exception_immediate_handler
    .space 0x700 - (. - rpi5_direct_exception_immediate_vectors)
    b rpi5_direct_exception_immediate_handler
    .space 0x780 - (. - rpi5_direct_exception_immediate_vectors)
    b rpi5_direct_exception_immediate_handler

rpi5_direct_exception_immediate_handler:
    mrs x20, CurrentEL
    cmp x20, #(1 << 2)
    b.eq 1f
    cmp x20, #(2 << 2)
    b.eq 2f
    cmp x20, #(3 << 2)
    b.eq 3f
    b rpi5_direct_exception_immediate_park
1:
    mrs x21, esr_el1
    b 4f
2:
    mrs x21, esr_el2
    b 4f
3:
    mrs x21, esr_el3
4:
    lsr x21, x21, #26
    cmp x21, #0x3c
    b.ne rpi5_direct_exception_immediate_park
    ldr x0, =0x84000009
    smc #0

rpi5_direct_exception_immediate_park:
    wfe
    b rpi5_direct_exception_immediate_park

.section .text.boot, "ax"
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
