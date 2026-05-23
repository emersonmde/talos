.section .text.boot, "ax"
.global _start
.type _start, %function

#if defined(TALOS_TARGET_RPI5_BCM2712) && defined(TALOS_RPI5_READABLE_BOOT_LOG_DIAGNOSTIC)
.macro rpi5_write_firmware_uart_marker_once uart_base, bytes_label
    ldr x9, =\uart_base
    adr x10, \bytes_label
.Lfirmware_uart_marker_once_loop_\@:
    ldrb w11, [x10], #1
    cbz w11, .Lfirmware_uart_marker_once_done_\@
    strb w11, [x9]
    b .Lfirmware_uart_marker_once_loop_\@
.Lfirmware_uart_marker_once_done_\@:
    dsb sy
.endm

.macro rpi5_write_repeated_firmware_uart_marker uart_base, bytes_label
    mov x20, #64
.Lfirmware_uart_marker_repeat_loop_\@:
    rpi5_write_firmware_uart_marker_once \uart_base, \bytes_label
    mov x14, #0x8000
.Lfirmware_uart_marker_delay_loop_\@:
    subs x14, x14, #1
    b.ne .Lfirmware_uart_marker_delay_loop_\@
    subs x20, x20, #1
    b.ne .Lfirmware_uart_marker_repeat_loop_\@
    dsb sy
.endm

.macro rpi5_write_primitive_uart_bytes uart_base, bytes_label
    ldr x9, =\uart_base
    adr x10, \bytes_label
1:
    ldrb w11, [x10], #1
    cbz w11, 2f
    strb w11, [x9]
    ldr w12, [x9, #0x18]
    b 1b
2:
    dsb sy
.endm

.macro rpi5_prepare_readable_uart_path
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
.endm

.macro rpi5_write_readable_bytes bytes_label
    ldr x9, =0x1f00030000
    adr x10, \bytes_label
1:
    ldrb w11, [x10], #1
    cbz w11, 2f
    str w11, [x9]
    ldr w12, [x9, #0x18]
    b 1b
2:
    dsb sy
.endm
#endif

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
#ifdef TALOS_RPI5_PHASE_CONTINUE_DIAGNOSTIC
    ldr x9, =0x107d001000
    adr x10, rpi5_phase_ladder_p0_marker
    bl rpi5_write_marker_payload_paced
    ldr x9, =0x1f00030000
    adr x10, rpi5_phase_ladder_p0_marker
    bl rpi5_write_marker_payload_paced
#endif
#if defined(TALOS_RPI5_UART_CANDIDATE_DIAGNOSTIC)
4:
    ldr x9, =0x107d001000
    adr x10, rpi5_uart10_candidate_marker
    bl rpi5_write_marker_payload
    ldr x9, =0x1f00030000
    adr x10, rpi5_rp1_uart0_candidate_marker
    bl rpi5_write_marker_payload
    mov x20, #0x8000
5:
    subs x20, x20, #1
    b.ne 5b
    b 4b
#elif defined(TALOS_RPI5_PHASE_P0_RESET_DIAGNOSTIC)
    ldr x9, =0x107d001000
    mov x20, #64
4:
    adr x10, rpi5_phase_ladder_p0_marker
    bl rpi5_write_marker_payload
    subs x20, x20, #1
    b.ne 4b
    ldr x0, =0x84000009
    smc #0
5:
    wfe
    b 5b
#elif defined(TALOS_RPI5_PHASE_LADDER_DIAGNOSTIC)
    ldr x9, =0x107d001000
    adr x10, rpi5_phase_ladder_p0_marker
    bl rpi5_write_marker_payload
#elif defined(TALOS_RPI5_PHASE_CPACR_RESET_DIAGNOSTIC) || defined(TALOS_RPI5_PHASE_BSS_RESET_DIAGNOSTIC) || defined(TALOS_RPI5_PHASE_STACK_RESET_DIAGNOSTIC) || defined(TALOS_RPI5_PHASE_STACK_TO_TEXT_RESET_DIAGNOSTIC) || defined(TALOS_RPI5_PHASE_STACK_TO_RUST_RESET_DIAGNOSTIC)
    mov x20, #16
4:
    ldr x9, =0x107d001000
    adr x10, rpi5_boundary_before_cpacr_marker
    bl rpi5_write_marker_payload
    ldr x9, =0x1f00030000
    adr x10, rpi5_boundary_before_cpacr_marker
    bl rpi5_write_marker_payload
    mov x21, #0x8000
5:
    subs x21, x21, #1
    b.ne 5b
    subs x20, x20, #1
    b.ne 4b
#elif defined(TALOS_RPI5_ENTRY_LOOP_DIAGNOSTIC) || defined(TALOS_RPI5_ENTRY_TALOS_LINE_DIAGNOSTIC)
4:
    ldr x9, =0x107d001000
    bl rpi5_write_marker
    ldr x9, =0x1f00030000
    bl rpi5_write_marker
    mov x20, #0x8000
5:
    subs x20, x20, #1
    b.ne 5b
    b 4b
#elif defined(TALOS_RPI5_ENTRY_TALOS_LINE_RESET_DIAGNOSTIC)
    mov x22, #16
4:
    ldr x9, =0x107d001000
    bl rpi5_write_marker
    ldr x9, =0x1f00030000
    bl rpi5_write_marker
    mov x20, #0x8000
5:
    subs x20, x20, #1
    b.ne 5b
    subs x22, x22, #1
    b.ne 4b
    ldr x0, =0x84000009
    smc #0
6:
    wfe
    b 6b
#elif defined(TALOS_RPI5_FRESH_ENTRY_RESET_DIAGNOSTIC) || defined(TALOS_RPI5_FRESH_ENTRY_CONTINUE_DIAGNOSTIC)
    mov x22, #16
4:
    ldr x9, =0x107d001000
    adr x10, rpi5_fresh_entry_marker
    bl rpi5_write_marker_payload
    ldr x9, =0x1f00030000
    adr x10, rpi5_fresh_entry_marker
    bl rpi5_write_marker_payload
    mov x20, #0x8000
5:
    subs x20, x20, #1
    b.ne 5b
    subs x22, x22, #1
    b.ne 4b
#if defined(TALOS_RPI5_FRESH_ENTRY_RESET_DIAGNOSTIC)
    ldr x0, =0x84000009
    smc #0
6:
    wfe
    b 6b
#endif
#else
#ifdef TALOS_RPI5_READABLE_BOOT_LOG_DIAGNOSTIC
    rpi5_write_repeated_firmware_uart_marker 0x1c00030000, rpi5_readable_boot_log_firmware_marker
    b rpi5_readable_boot_log_park
#endif
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

    ldr x9, =0x1c00030000
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
#endif

#ifdef TALOS_RPI5_TRANSITION_DIAGNOSTIC
    ldr x9, =0x1f00030000
    bl rpi5_write_marker
#endif
#endif
#endif
#endif

    mrs x0, cpacr_el1
    orr x0, x0, #(0b11 << 20)
    msr cpacr_el1, x0
    isb
#if defined(TALOS_TARGET_RPI5_BCM2712) && (defined(TALOS_RPI5_PHASE_CPACR_RESET_DIAGNOSTIC) || defined(TALOS_RPI5_PHASE_BSS_RESET_DIAGNOSTIC) || defined(TALOS_RPI5_PHASE_STACK_RESET_DIAGNOSTIC) || defined(TALOS_RPI5_PHASE_STACK_TO_TEXT_RESET_DIAGNOSTIC) || defined(TALOS_RPI5_PHASE_STACK_TO_RUST_RESET_DIAGNOSTIC))
#ifdef TALOS_RPI5_PHASE_CPACR_RESET_DIAGNOSTIC
    mov x20, #64
#else
    mov x20, #16
#endif
4:
    ldr x9, =0x107d001000
    adr x10, rpi5_boundary_after_cpacr_marker
    bl rpi5_write_marker_payload_paced
    ldr x9, =0x1f00030000
    adr x10, rpi5_boundary_after_cpacr_marker
    bl rpi5_write_marker_payload_paced
    subs x20, x20, #1
    b.ne 4b
#ifdef TALOS_RPI5_PHASE_CPACR_RESET_DIAGNOSTIC
    bl rpi5_wait_uart_tx_empty_bounded
    ldr x0, =0x84000009
    smc #0
5:
    wfe
    b 5b
#endif
#endif
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
#if defined(TALOS_TARGET_RPI5_BCM2712) && (defined(TALOS_RPI5_PHASE_BSS_RESET_DIAGNOSTIC) || defined(TALOS_RPI5_PHASE_STACK_RESET_DIAGNOSTIC) || defined(TALOS_RPI5_PHASE_STACK_TO_TEXT_RESET_DIAGNOSTIC) || defined(TALOS_RPI5_PHASE_STACK_TO_RUST_RESET_DIAGNOSTIC))
#ifdef TALOS_RPI5_PHASE_BSS_RESET_DIAGNOSTIC
    mov x20, #64
#else
    mov x20, #16
#endif
4:
    ldr x9, =0x107d001000
    adr x10, rpi5_boundary_after_bss_marker
    bl rpi5_write_marker_payload_paced
    ldr x9, =0x1f00030000
    adr x10, rpi5_boundary_after_bss_marker
    bl rpi5_write_marker_payload_paced
    subs x20, x20, #1
    b.ne 4b
#ifdef TALOS_RPI5_PHASE_BSS_RESET_DIAGNOSTIC
#if defined(TALOS_RPI5_PHASE_STACK_RESET_DIAGNOSTIC)
    bl rpi5_wait_uart_tx_empty_bounded
    ldr x0, =0x84000009
    smc #0
5:
    wfe
    b 5b
#elif defined(TALOS_RPI5_PHASE_STACK_TO_RUST_RESET_DIAGNOSTIC)
    bl rpi5_wait_uart_tx_empty_bounded
    bl rpi5_stack_to_rust_reset_probe
5:
    wfe
    b 5b
#endif
#endif
#endif
    adrp x0, __stack_top
    add x0, x0, :lo12:__stack_top
    mov sp, x0
#if defined(TALOS_TARGET_RPI5_BCM2712) && (defined(TALOS_RPI5_PHASE_STACK_RESET_DIAGNOSTIC) || defined(TALOS_RPI5_PHASE_STACK_TO_TEXT_RESET_DIAGNOSTIC) || defined(TALOS_RPI5_PHASE_STACK_TO_RUST_RESET_DIAGNOSTIC))
#ifdef TALOS_RPI5_PHASE_STACK_TO_TEXT_RESET_DIAGNOSTIC
    mov x20, #8
#else
    mov x20, #64
#endif
4:
    ldr x9, =0x107d001000
    adr x10, rpi5_boundary_after_stack_marker
    bl rpi5_write_marker_payload_paced
    ldr x9, =0x1f00030000
    adr x10, rpi5_boundary_after_stack_marker
    bl rpi5_write_marker_payload_paced
    subs x20, x20, #1
    b.ne 4b
#if defined(TALOS_RPI5_PHASE_STACK_RESET_DIAGNOSTIC)
    bl rpi5_wait_uart_tx_empty_bounded
    ldr x0, =0x84000009
    smc #0
5:
    wfe
    b 5b
#elif defined(TALOS_RPI5_PHASE_STACK_TO_TEXT_RESET_DIAGNOSTIC)
    bl rpi5_wait_uart_tx_empty_bounded
    mov x20, #8
6:
    ldr x9, =0x107d001000
    adr x10, rpi5_boundary_before_text_branch_marker
    bl rpi5_write_marker_payload_paced
    ldr x9, =0x1f00030000
    adr x10, rpi5_boundary_before_text_branch_marker
    bl rpi5_write_marker_payload_paced
    subs x20, x20, #1
    b.ne 6b
    bl rpi5_wait_uart_tx_empty_bounded
    bl rpi5_stack_to_text_reset_probe
5:
    wfe
    b 5b
#elif defined(TALOS_RPI5_PHASE_STACK_TO_RUST_RESET_DIAGNOSTIC)
    bl rpi5_wait_uart_tx_empty_bounded
    bl rpi5_stack_to_rust_reset_probe
5:
    wfe
    b 5b
#endif
#endif
#if defined(TALOS_TARGET_RPI5_BCM2712) && defined(TALOS_RPI5_PHASE_LADDER_DIAGNOSTIC)
    ldr x9, =0x107d001000
#if defined(TALOS_RPI5_PHASE_P1_RESET_DIAGNOSTIC) || defined(TALOS_RPI5_PHASE_P1_SHORT_RESET_DIAGNOSTIC)
    mov x20, #64
2:
#if defined(TALOS_RPI5_PHASE_P1_SHORT_RESET_DIAGNOSTIC)
    adr x10, rpi5_phase_ladder_p1_short_marker
#else
    adr x10, rpi5_phase_ladder_p1_marker
#endif
    bl rpi5_write_marker_payload
    subs x20, x20, #1
    b.ne 2b
    ldr x0, =0x84000009
    smc #0
3:
    wfe
    b 3b
#elif defined(TALOS_RPI5_PHASE_P2_RESET_DIAGNOSTIC)
    adr x10, rpi5_phase_ladder_p1_short_marker
    bl rpi5_write_marker_payload_paced
    mov x20, #64
2:
    adr x10, rpi5_phase_ladder_p2_short_marker
    bl rpi5_write_marker_payload_paced
    subs x20, x20, #1
    b.ne 2b
    bl rpi5_wait_uart_tx_empty_bounded
    ldr x0, =0x84000009
    smc #0
3:
    wfe
    b 3b
#else
    adr x10, rpi5_phase_ladder_p1_marker
    bl rpi5_write_marker_payload
    ldr x9, =0x107d001000
    adr x10, rpi5_phase_ladder_p2_marker
    bl rpi5_write_marker_payload
#endif
#endif
#if defined(TALOS_TARGET_RPI5_BCM2712) && defined(TALOS_RPI5_PHASE_CONTINUE_DIAGNOSTIC)
    ldr x9, =0x107d001000
    adr x10, rpi5_phase_ladder_p1_short_marker
    bl rpi5_write_marker_payload_paced
    ldr x9, =0x1f00030000
    adr x10, rpi5_phase_ladder_p1_short_marker
    bl rpi5_write_marker_payload_paced
    ldr x9, =0x107d001000
    adr x10, rpi5_phase_ladder_p2_short_marker
    bl rpi5_write_marker_payload_paced
    ldr x9, =0x1f00030000
    adr x10, rpi5_phase_ladder_p2_short_marker
    bl rpi5_write_marker_payload_paced
    bl rpi5_wait_uart_tx_empty_bounded
#endif
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
#elif defined(TALOS_TARGET_RPI5_BCM2712) && defined(TALOS_RPI5_BRK_ELR_WRITE_RESET_DIAGNOSTIC)
    adr x0, rpi5_brk_elr_write_reset_vectors
    mrs x1, CurrentEL
    cmp x1, #(1 << 2)
    b.eq 5f
    cmp x1, #(2 << 2)
    b.eq 6f
    cmp x1, #(3 << 2)
    b.eq 7f
    b rpi5_brk_elr_write_reset_park
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
    b rpi5_brk_elr_write_reset_park
rpi5_brk_elr_write_reset_continuation:
    b rpi5_brk_elr_write_reset_park
#elif defined(TALOS_TARGET_RPI5_BCM2712) && defined(TALOS_RPI5_BRK_SPSR_ERET_RESET_DIAGNOSTIC)
    adr x0, rpi5_brk_spsr_eret_reset_vectors
    mrs x1, CurrentEL
    cmp x1, #(1 << 2)
    b.eq 5f
    cmp x1, #(2 << 2)
    b.eq 6f
    cmp x1, #(3 << 2)
    b.eq 7f
    b rpi5_brk_spsr_eret_reset_park
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
    b rpi5_brk_spsr_eret_reset_park
rpi5_brk_spsr_eret_reset_continuation:
    ldr x0, =0x84000009
    smc #0
    b rpi5_brk_spsr_eret_reset_park
#elif defined(TALOS_TARGET_RPI5_BCM2712) && defined(TALOS_RPI5_BRK_SPSR_HANDLER_RESET_DIAGNOSTIC)
    adr x0, rpi5_brk_spsr_handler_reset_vectors
    mrs x1, CurrentEL
    cmp x1, #(1 << 2)
    b.eq 5f
    cmp x1, #(2 << 2)
    b.eq 6f
    cmp x1, #(3 << 2)
    b.eq 7f
    b rpi5_brk_spsr_handler_reset_park
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
    b rpi5_brk_spsr_handler_reset_park
#elif defined(TALOS_TARGET_RPI5_BCM2712) && defined(TALOS_RPI5_BRK_ERET_UART_MARKER_DIAGNOSTIC)
    adr x0, rpi5_brk_eret_uart_marker_vectors
    mrs x1, CurrentEL
    cmp x1, #(1 << 2)
    b.eq 5f
    cmp x1, #(2 << 2)
    b.eq 6f
    cmp x1, #(3 << 2)
    b.eq 7f
    b rpi5_brk_eret_uart_marker_park
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
    b rpi5_brk_eret_uart_marker_park
rpi5_brk_eret_uart_marker_continuation:
    mov x20, #8
4:
    ldr x9, =0x107d001000
    bl rpi5_write_eret_marker
    ldr x9, =0x1f00030000
    bl rpi5_write_eret_marker
    subs x20, x20, #1
    b.ne 4b
    ldr x0, =0x84000009
    smc #0
    b rpi5_brk_eret_uart_marker_park
#elif defined(TALOS_TARGET_RPI5_BCM2712) && defined(TALOS_RPI5_READABLE_BOOT_LOG_DIAGNOSTIC)
    rpi5_write_readable_bytes rpi5_readable_boot_log_start
    adr x0, rpi5_readable_boot_log_vectors
    mrs x1, CurrentEL
    cmp x1, #(1 << 2)
    b.eq 5f
    cmp x1, #(2 << 2)
    b.eq 6f
    cmp x1, #(3 << 2)
    b.eq 7f
    b rpi5_readable_boot_log_park
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
    b rpi5_readable_boot_log_park
#elif defined(TALOS_TARGET_RPI5_BCM2712) && defined(TALOS_RPI5_VECTOR_SECTION_DIAGNOSTIC)
    bl rpi5_vector_section_marker_loop
#elif defined(TALOS_TARGET_RPI5_BCM2712) && defined(TALOS_RPI5_TEXT_SECTION_DIAGNOSTIC)
    bl rpi5_text_section_marker_loop
#endif

#if defined(TALOS_TARGET_RPI5_BCM2712) && defined(TALOS_RPI5_HANDOFF_UART_DIAGNOSTIC)
    mov x20, #8
4:
    ldr x9, =0x107d001000
    bl rpi5_write_marker
    ldr x9, =0x1f00030000
    bl rpi5_write_marker
    subs x20, x20, #1
    b.ne 4b
#endif
    mov x0, x19
    bl rust_entry

2:
    wfe
    b 2b

.size _start, . - _start

#if defined(TALOS_TARGET_RPI5_BCM2712) && defined(TALOS_RPI5_HANDOFF_UART_DIAGNOSTIC)
.balign 4
.global rpi5_rust_entry_re_marker_park
.type rpi5_rust_entry_re_marker_park, %function
rpi5_rust_entry_re_marker_park:
    mov x20, #16
4:
    ldr x9, =0x107d001000
    bl rpi5_write_rust_entry_re_marker
    ldr x9, =0x1f00030000
    bl rpi5_write_rust_entry_re_marker
    ldr x21, =0x200000
8:
    subs x21, x21, #1
    b.ne 8b
    subs x20, x20, #1
    b.ne 4b

5:
    wfe
    b 5b

.size rpi5_rust_entry_re_marker_park, . - rpi5_rust_entry_re_marker_park

.balign 4
.type rpi5_write_rust_entry_re_marker, %function
rpi5_write_rust_entry_re_marker:
    adr x10, 9f
6:
    ldrb w11, [x10], #1
    cbz w11, 7f
    str w11, [x9]
    ldr w12, [x9, #0x18]
    b 6b
7:
    dsb sy
    ret

9:
    .asciz "RE\r\n"

.size rpi5_write_rust_entry_re_marker, . - rpi5_write_rust_entry_re_marker
.balign 4
#endif

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

#if defined(TALOS_TARGET_RPI5_BCM2712) && defined(TALOS_RPI5_PHASE_STACK_TO_TEXT_RESET_DIAGNOSTIC)
.section .text.rpi5_phase_stack_to_text_reset_diagnostic, "ax"
.balign 16
.type rpi5_stack_to_text_reset_probe, %function
rpi5_stack_to_text_reset_probe:
    hint #34
    mov x22, #16
1:
    ldr x9, =0x107d001000
    adr x10, rpi5_stack_to_text_marker
2:
    ldrb w11, [x10], #1
    cbz w11, 3f
    mov x21, #0x10000
4:
    ldr w12, [x9, #0x18]
    tbz w12, #5, 5f
    subs x21, x21, #1
    b.ne 4b
5:
    str w11, [x9]
    b 2b
3:
    ldr x9, =0x1f00030000
    adr x10, rpi5_stack_to_text_marker
6:
    ldrb w11, [x10], #1
    cbz w11, 7f
    mov x21, #0x10000
8:
    ldr w12, [x9, #0x18]
    tbz w12, #5, 9f
    subs x21, x21, #1
    b.ne 8b
9:
    str w11, [x9]
    b 6b
7:
    subs x22, x22, #1
    b.ne 1b
    mov x21, #0x200000
10:
    ldr w12, [x9, #0x18]
    tbnz w12, #7, 11f
    subs x21, x21, #1
    b.ne 10b
11:
    dsb sy
    ldr x0, =0x84000009
    smc #0
12:
    wfe
    b 12b

rpi5_stack_to_text_marker:
    .asciz "TALOS: stack to text\r\n"

.size rpi5_stack_to_text_reset_probe, . - rpi5_stack_to_text_reset_probe
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

#if defined(TALOS_TARGET_RPI5_BCM2712) && defined(TALOS_RPI5_BRK_ERET_UART_MARKER_DIAGNOSTIC)
.section .text.rpi5_brk_eret_uart_marker_diagnostic, "ax"
.balign 2048
rpi5_brk_eret_uart_marker_vectors:
    b rpi5_brk_eret_uart_marker_handler
    .space 0x80 - (. - rpi5_brk_eret_uart_marker_vectors)
    b rpi5_brk_eret_uart_marker_handler
    .space 0x100 - (. - rpi5_brk_eret_uart_marker_vectors)
    b rpi5_brk_eret_uart_marker_handler
    .space 0x180 - (. - rpi5_brk_eret_uart_marker_vectors)
    b rpi5_brk_eret_uart_marker_handler
    .space 0x200 - (. - rpi5_brk_eret_uart_marker_vectors)
    b rpi5_brk_eret_uart_marker_handler
    .space 0x280 - (. - rpi5_brk_eret_uart_marker_vectors)
    b rpi5_brk_eret_uart_marker_handler
    .space 0x300 - (. - rpi5_brk_eret_uart_marker_vectors)
    b rpi5_brk_eret_uart_marker_handler
    .space 0x380 - (. - rpi5_brk_eret_uart_marker_vectors)
    b rpi5_brk_eret_uart_marker_handler
    .space 0x400 - (. - rpi5_brk_eret_uart_marker_vectors)
    b rpi5_brk_eret_uart_marker_handler
    .space 0x480 - (. - rpi5_brk_eret_uart_marker_vectors)
    b rpi5_brk_eret_uart_marker_handler
    .space 0x500 - (. - rpi5_brk_eret_uart_marker_vectors)
    b rpi5_brk_eret_uart_marker_handler
    .space 0x580 - (. - rpi5_brk_eret_uart_marker_vectors)
    b rpi5_brk_eret_uart_marker_handler
    .space 0x600 - (. - rpi5_brk_eret_uart_marker_vectors)
    b rpi5_brk_eret_uart_marker_handler
    .space 0x680 - (. - rpi5_brk_eret_uart_marker_vectors)
    b rpi5_brk_eret_uart_marker_handler
    .space 0x700 - (. - rpi5_brk_eret_uart_marker_vectors)
    b rpi5_brk_eret_uart_marker_handler
    .space 0x780 - (. - rpi5_brk_eret_uart_marker_vectors)
    b rpi5_brk_eret_uart_marker_handler

rpi5_brk_eret_uart_marker_handler:
    mrs x20, CurrentEL
    cmp x20, #(1 << 2)
    b.eq 1f
    cmp x20, #(2 << 2)
    b.eq 2f
    cmp x20, #(3 << 2)
    b.eq 3f
    b rpi5_brk_eret_uart_marker_park
1:
    mrs x21, esr_el1
    ldr x22, =rpi5_brk_eret_uart_marker_continuation
    lsr x21, x21, #26
    cmp x21, #0x3c
    b.ne rpi5_brk_eret_uart_marker_park
    mov x23, #0x3c5
    msr spsr_el1, x23
    msr elr_el1, x22
    eret
2:
    mrs x21, esr_el2
    ldr x22, =rpi5_brk_eret_uart_marker_continuation
    lsr x21, x21, #26
    cmp x21, #0x3c
    b.ne rpi5_brk_eret_uart_marker_park
    mov x23, #0x3c9
    msr spsr_el2, x23
    msr elr_el2, x22
    eret
3:
    mrs x21, esr_el3
    ldr x22, =rpi5_brk_eret_uart_marker_continuation
    lsr x21, x21, #26
    cmp x21, #0x3c
    b.ne rpi5_brk_eret_uart_marker_park
    mov x23, #0x3cd
    msr spsr_el3, x23
    msr elr_el3, x22
    eret

rpi5_brk_eret_uart_marker_park:
    wfe
    b rpi5_brk_eret_uart_marker_park

.section .text.boot, "ax"
#endif

#if defined(TALOS_TARGET_RPI5_BCM2712) && defined(TALOS_RPI5_READABLE_BOOT_LOG_DIAGNOSTIC)
.section .text.rpi5_readable_boot_log_diagnostic, "ax"
.balign 2048
rpi5_readable_boot_log_vectors:
    b rpi5_readable_boot_log_handler
    .space 0x80 - (. - rpi5_readable_boot_log_vectors)
    b rpi5_readable_boot_log_handler
    .space 0x100 - (. - rpi5_readable_boot_log_vectors)
    b rpi5_readable_boot_log_handler
    .space 0x180 - (. - rpi5_readable_boot_log_vectors)
    b rpi5_readable_boot_log_handler
    .space 0x200 - (. - rpi5_readable_boot_log_vectors)
    b rpi5_readable_boot_log_handler
    .space 0x280 - (. - rpi5_readable_boot_log_vectors)
    b rpi5_readable_boot_log_handler
    .space 0x300 - (. - rpi5_readable_boot_log_vectors)
    b rpi5_readable_boot_log_handler
    .space 0x380 - (. - rpi5_readable_boot_log_vectors)
    b rpi5_readable_boot_log_handler
    .space 0x400 - (. - rpi5_readable_boot_log_vectors)
    b rpi5_readable_boot_log_handler
    .space 0x480 - (. - rpi5_readable_boot_log_vectors)
    b rpi5_readable_boot_log_handler
    .space 0x500 - (. - rpi5_readable_boot_log_vectors)
    b rpi5_readable_boot_log_handler
    .space 0x580 - (. - rpi5_readable_boot_log_vectors)
    b rpi5_readable_boot_log_handler
    .space 0x600 - (. - rpi5_readable_boot_log_vectors)
    b rpi5_readable_boot_log_handler
    .space 0x680 - (. - rpi5_readable_boot_log_vectors)
    b rpi5_readable_boot_log_handler
    .space 0x700 - (. - rpi5_readable_boot_log_vectors)
    b rpi5_readable_boot_log_handler
    .space 0x780 - (. - rpi5_readable_boot_log_vectors)
    b rpi5_readable_boot_log_handler

rpi5_readable_boot_log_handler:
    mrs x20, CurrentEL
    cmp x20, #(1 << 2)
    b.eq 1f
    cmp x20, #(2 << 2)
    b.eq 2f
    cmp x20, #(3 << 2)
    b.eq 3f
    b rpi5_readable_boot_log_park
1:
    mrs x21, esr_el1
    lsr x21, x21, #26
    cmp x21, #0x3c
    b.ne rpi5_readable_boot_log_park
    b rpi5_readable_boot_log_reset
2:
    mrs x21, esr_el2
    lsr x21, x21, #26
    cmp x21, #0x3c
    b.ne rpi5_readable_boot_log_park
    b rpi5_readable_boot_log_reset
3:
    mrs x21, esr_el3
    lsr x21, x21, #26
    cmp x21, #0x3c
    b.ne rpi5_readable_boot_log_park

rpi5_readable_boot_log_reset:
    rpi5_write_readable_bytes rpi5_readable_boot_log_brk_handler
    rpi5_write_readable_bytes rpi5_readable_boot_log_reset_requested
    ldr x0, =0x84000009
    smc #0

rpi5_readable_boot_log_park:
    wfe
    b rpi5_readable_boot_log_park

rpi5_readable_boot_log_firmware_marker:
    .asciz "TALOS-FW\r\n"
rpi5_readable_boot_log_primitive_bcm2712_uart10:
    .asciz "B0\r\n"
rpi5_readable_boot_log_primitive_rp1_uart0:
    .asciz "R0\r\n"
rpi5_readable_boot_log_early:
    .asciz "EP\r\nTALOS: EP\r\n"
rpi5_readable_boot_log_start:
    .asciz "CL\r\nTALOS: CL\r\n"
rpi5_readable_boot_log_brk_handler:
    .asciz "BH\r\nTALOS: BH\r\n"
rpi5_readable_boot_log_reset_requested:
    .asciz "RR\r\nTALOS: RR\r\n"

.section .text.boot, "ax"
#endif

#if defined(TALOS_TARGET_RPI5_BCM2712) && defined(TALOS_RPI5_BRK_SPSR_ERET_RESET_DIAGNOSTIC)
.section .text.rpi5_brk_spsr_eret_reset_diagnostic, "ax"
.balign 2048
rpi5_brk_spsr_eret_reset_vectors:
    b rpi5_brk_spsr_eret_reset_handler
    .space 0x80 - (. - rpi5_brk_spsr_eret_reset_vectors)
    b rpi5_brk_spsr_eret_reset_handler
    .space 0x100 - (. - rpi5_brk_spsr_eret_reset_vectors)
    b rpi5_brk_spsr_eret_reset_handler
    .space 0x180 - (. - rpi5_brk_spsr_eret_reset_vectors)
    b rpi5_brk_spsr_eret_reset_handler
    .space 0x200 - (. - rpi5_brk_spsr_eret_reset_vectors)
    b rpi5_brk_spsr_eret_reset_handler
    .space 0x280 - (. - rpi5_brk_spsr_eret_reset_vectors)
    b rpi5_brk_spsr_eret_reset_handler
    .space 0x300 - (. - rpi5_brk_spsr_eret_reset_vectors)
    b rpi5_brk_spsr_eret_reset_handler
    .space 0x380 - (. - rpi5_brk_spsr_eret_reset_vectors)
    b rpi5_brk_spsr_eret_reset_handler
    .space 0x400 - (. - rpi5_brk_spsr_eret_reset_vectors)
    b rpi5_brk_spsr_eret_reset_handler
    .space 0x480 - (. - rpi5_brk_spsr_eret_reset_vectors)
    b rpi5_brk_spsr_eret_reset_handler
    .space 0x500 - (. - rpi5_brk_spsr_eret_reset_vectors)
    b rpi5_brk_spsr_eret_reset_handler
    .space 0x580 - (. - rpi5_brk_spsr_eret_reset_vectors)
    b rpi5_brk_spsr_eret_reset_handler
    .space 0x600 - (. - rpi5_brk_spsr_eret_reset_vectors)
    b rpi5_brk_spsr_eret_reset_handler
    .space 0x680 - (. - rpi5_brk_spsr_eret_reset_vectors)
    b rpi5_brk_spsr_eret_reset_handler
    .space 0x700 - (. - rpi5_brk_spsr_eret_reset_vectors)
    b rpi5_brk_spsr_eret_reset_handler
    .space 0x780 - (. - rpi5_brk_spsr_eret_reset_vectors)
    b rpi5_brk_spsr_eret_reset_handler

rpi5_brk_spsr_eret_reset_handler:
    mrs x20, CurrentEL
    cmp x20, #(1 << 2)
    b.eq 1f
    cmp x20, #(2 << 2)
    b.eq 2f
    cmp x20, #(3 << 2)
    b.eq 3f
    b rpi5_brk_spsr_eret_reset_park
1:
    mrs x21, esr_el1
    ldr x22, =rpi5_brk_spsr_eret_reset_continuation
    lsr x21, x21, #26
    cmp x21, #0x3c
    b.ne rpi5_brk_spsr_eret_reset_park
    mov x23, #0x3c5
    msr spsr_el1, x23
    msr elr_el1, x22
    eret
2:
    mrs x21, esr_el2
    ldr x22, =rpi5_brk_spsr_eret_reset_continuation
    lsr x21, x21, #26
    cmp x21, #0x3c
    b.ne rpi5_brk_spsr_eret_reset_park
    mov x23, #0x3c9
    msr spsr_el2, x23
    msr elr_el2, x22
    eret
3:
    mrs x21, esr_el3
    ldr x22, =rpi5_brk_spsr_eret_reset_continuation
    lsr x21, x21, #26
    cmp x21, #0x3c
    b.ne rpi5_brk_spsr_eret_reset_park
    mov x23, #0x3cd
    msr spsr_el3, x23
    msr elr_el3, x22
    eret

rpi5_brk_spsr_eret_reset_park:
    wfe
    b rpi5_brk_spsr_eret_reset_park

.section .text.boot, "ax"
#endif

#if defined(TALOS_TARGET_RPI5_BCM2712) && defined(TALOS_RPI5_BRK_SPSR_HANDLER_RESET_DIAGNOSTIC)
.section .text.rpi5_brk_spsr_handler_reset_diagnostic, "ax"
.balign 2048
rpi5_brk_spsr_handler_reset_vectors:
    b rpi5_brk_spsr_handler_reset_handler
    .space 0x80 - (. - rpi5_brk_spsr_handler_reset_vectors)
    b rpi5_brk_spsr_handler_reset_handler
    .space 0x100 - (. - rpi5_brk_spsr_handler_reset_vectors)
    b rpi5_brk_spsr_handler_reset_handler
    .space 0x180 - (. - rpi5_brk_spsr_handler_reset_vectors)
    b rpi5_brk_spsr_handler_reset_handler
    .space 0x200 - (. - rpi5_brk_spsr_handler_reset_vectors)
    b rpi5_brk_spsr_handler_reset_handler
    .space 0x280 - (. - rpi5_brk_spsr_handler_reset_vectors)
    b rpi5_brk_spsr_handler_reset_handler
    .space 0x300 - (. - rpi5_brk_spsr_handler_reset_vectors)
    b rpi5_brk_spsr_handler_reset_handler
    .space 0x380 - (. - rpi5_brk_spsr_handler_reset_vectors)
    b rpi5_brk_spsr_handler_reset_handler
    .space 0x400 - (. - rpi5_brk_spsr_handler_reset_vectors)
    b rpi5_brk_spsr_handler_reset_handler
    .space 0x480 - (. - rpi5_brk_spsr_handler_reset_vectors)
    b rpi5_brk_spsr_handler_reset_handler
    .space 0x500 - (. - rpi5_brk_spsr_handler_reset_vectors)
    b rpi5_brk_spsr_handler_reset_handler
    .space 0x580 - (. - rpi5_brk_spsr_handler_reset_vectors)
    b rpi5_brk_spsr_handler_reset_handler
    .space 0x600 - (. - rpi5_brk_spsr_handler_reset_vectors)
    b rpi5_brk_spsr_handler_reset_handler
    .space 0x680 - (. - rpi5_brk_spsr_handler_reset_vectors)
    b rpi5_brk_spsr_handler_reset_handler
    .space 0x700 - (. - rpi5_brk_spsr_handler_reset_vectors)
    b rpi5_brk_spsr_handler_reset_handler
    .space 0x780 - (. - rpi5_brk_spsr_handler_reset_vectors)
    b rpi5_brk_spsr_handler_reset_handler

rpi5_brk_spsr_handler_reset_handler:
    mrs x20, CurrentEL
    cmp x20, #(1 << 2)
    b.eq 1f
    cmp x20, #(2 << 2)
    b.eq 2f
    cmp x20, #(3 << 2)
    b.eq 3f
    b rpi5_brk_spsr_handler_reset_park
1:
    mrs x21, esr_el1
    lsr x21, x21, #26
    cmp x21, #0x3c
    b.ne rpi5_brk_spsr_handler_reset_park
    mov x23, #0x3c5
    msr spsr_el1, x23
    b rpi5_brk_spsr_handler_reset_system_reset
2:
    mrs x21, esr_el2
    lsr x21, x21, #26
    cmp x21, #0x3c
    b.ne rpi5_brk_spsr_handler_reset_park
    mov x23, #0x3c9
    msr spsr_el2, x23
    b rpi5_brk_spsr_handler_reset_system_reset
3:
    mrs x21, esr_el3
    lsr x21, x21, #26
    cmp x21, #0x3c
    b.ne rpi5_brk_spsr_handler_reset_park
    mov x23, #0x3cd
    msr spsr_el3, x23

rpi5_brk_spsr_handler_reset_system_reset:
    ldr x0, =0x84000009
    smc #0

rpi5_brk_spsr_handler_reset_park:
    wfe
    b rpi5_brk_spsr_handler_reset_park

.section .text.boot, "ax"
#endif

#if defined(TALOS_TARGET_RPI5_BCM2712) && defined(TALOS_RPI5_BRK_ELR_WRITE_RESET_DIAGNOSTIC)
.section .text.rpi5_brk_elr_write_reset_diagnostic, "ax"
.balign 2048
rpi5_brk_elr_write_reset_vectors:
    b rpi5_brk_elr_write_reset_handler
    .space 0x80 - (. - rpi5_brk_elr_write_reset_vectors)
    b rpi5_brk_elr_write_reset_handler
    .space 0x100 - (. - rpi5_brk_elr_write_reset_vectors)
    b rpi5_brk_elr_write_reset_handler
    .space 0x180 - (. - rpi5_brk_elr_write_reset_vectors)
    b rpi5_brk_elr_write_reset_handler
    .space 0x200 - (. - rpi5_brk_elr_write_reset_vectors)
    b rpi5_brk_elr_write_reset_handler
    .space 0x280 - (. - rpi5_brk_elr_write_reset_vectors)
    b rpi5_brk_elr_write_reset_handler
    .space 0x300 - (. - rpi5_brk_elr_write_reset_vectors)
    b rpi5_brk_elr_write_reset_handler
    .space 0x380 - (. - rpi5_brk_elr_write_reset_vectors)
    b rpi5_brk_elr_write_reset_handler
    .space 0x400 - (. - rpi5_brk_elr_write_reset_vectors)
    b rpi5_brk_elr_write_reset_handler
    .space 0x480 - (. - rpi5_brk_elr_write_reset_vectors)
    b rpi5_brk_elr_write_reset_handler
    .space 0x500 - (. - rpi5_brk_elr_write_reset_vectors)
    b rpi5_brk_elr_write_reset_handler
    .space 0x580 - (. - rpi5_brk_elr_write_reset_vectors)
    b rpi5_brk_elr_write_reset_handler
    .space 0x600 - (. - rpi5_brk_elr_write_reset_vectors)
    b rpi5_brk_elr_write_reset_handler
    .space 0x680 - (. - rpi5_brk_elr_write_reset_vectors)
    b rpi5_brk_elr_write_reset_handler
    .space 0x700 - (. - rpi5_brk_elr_write_reset_vectors)
    b rpi5_brk_elr_write_reset_handler
    .space 0x780 - (. - rpi5_brk_elr_write_reset_vectors)
    b rpi5_brk_elr_write_reset_handler

rpi5_brk_elr_write_reset_handler:
    mrs x20, CurrentEL
    cmp x20, #(1 << 2)
    b.eq 1f
    cmp x20, #(2 << 2)
    b.eq 2f
    cmp x20, #(3 << 2)
    b.eq 3f
    b rpi5_brk_elr_write_reset_park
1:
    mrs x21, esr_el1
    ldr x22, =rpi5_brk_elr_write_reset_continuation
    lsr x21, x21, #26
    cmp x21, #0x3c
    b.ne rpi5_brk_elr_write_reset_park
    msr elr_el1, x22
    b rpi5_brk_elr_write_reset_system_reset
2:
    mrs x21, esr_el2
    ldr x22, =rpi5_brk_elr_write_reset_continuation
    lsr x21, x21, #26
    cmp x21, #0x3c
    b.ne rpi5_brk_elr_write_reset_park
    msr elr_el2, x22
    b rpi5_brk_elr_write_reset_system_reset
3:
    mrs x21, esr_el3
    ldr x22, =rpi5_brk_elr_write_reset_continuation
    lsr x21, x21, #26
    cmp x21, #0x3c
    b.ne rpi5_brk_elr_write_reset_park
    msr elr_el3, x22

rpi5_brk_elr_write_reset_system_reset:
    ldr x0, =0x84000009
    smc #0

rpi5_brk_elr_write_reset_park:
    wfe
    b rpi5_brk_elr_write_reset_park

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
#ifndef TALOS_RPI5_READABLE_BOOT_LOG_DIAGNOSTIC
#if defined(TALOS_RPI5_UART_CANDIDATE_DIAGNOSTIC) || defined(TALOS_RPI5_PHASE_LADDER_DIAGNOSTIC) || defined(TALOS_RPI5_PHASE_P0_RESET_DIAGNOSTIC) || defined(TALOS_RPI5_PHASE_P1_RESET_DIAGNOSTIC) || defined(TALOS_RPI5_PHASE_P1_SHORT_RESET_DIAGNOSTIC) || defined(TALOS_RPI5_PHASE_P2_RESET_DIAGNOSTIC) || defined(TALOS_RPI5_PHASE_CPACR_RESET_DIAGNOSTIC) || defined(TALOS_RPI5_PHASE_BSS_RESET_DIAGNOSTIC) || defined(TALOS_RPI5_PHASE_STACK_RESET_DIAGNOSTIC) || defined(TALOS_RPI5_PHASE_STACK_TO_TEXT_RESET_DIAGNOSTIC) || defined(TALOS_RPI5_PHASE_STACK_TO_RUST_RESET_DIAGNOSTIC) || defined(TALOS_RPI5_PHASE_CONTINUE_DIAGNOSTIC) || defined(TALOS_RPI5_ENTRY_TALOS_LINE_RESET_DIAGNOSTIC) || defined(TALOS_RPI5_FRESH_ENTRY_RESET_DIAGNOSTIC) || defined(TALOS_RPI5_FRESH_ENTRY_CONTINUE_DIAGNOSTIC)
.balign 4
rpi5_write_marker_payload:
12:
    ldrb w11, [x10], #1
    cbz w11, 13f
    str w11, [x9]
    ldr w12, [x9, #0x18]
    b 12b
13:
    dsb sy
    ret

#if defined(TALOS_RPI5_PHASE_P2_RESET_DIAGNOSTIC) || defined(TALOS_RPI5_PHASE_CPACR_RESET_DIAGNOSTIC) || defined(TALOS_RPI5_PHASE_BSS_RESET_DIAGNOSTIC) || defined(TALOS_RPI5_PHASE_STACK_RESET_DIAGNOSTIC) || defined(TALOS_RPI5_PHASE_STACK_TO_TEXT_RESET_DIAGNOSTIC) || defined(TALOS_RPI5_PHASE_STACK_TO_RUST_RESET_DIAGNOSTIC) || defined(TALOS_RPI5_PHASE_CONTINUE_DIAGNOSTIC)
.balign 4
rpi5_write_marker_payload_paced:
12:
    ldrb w11, [x10], #1
    cbz w11, 13f
    mov x21, #0x10000
14:
    ldr w12, [x9, #0x18]
    tbz w12, #5, 15f
    subs x21, x21, #1
    b.ne 14b
15:
    str w11, [x9]
    b 12b
13:
    dsb sy
    ret

.balign 4
rpi5_wait_uart_tx_empty_bounded:
    mov x21, #0x200000
12:
    ldr w12, [x9, #0x18]
    tbnz w12, #7, 13f
    subs x21, x21, #1
    b.ne 12b
13:
    dsb sy
    ret
#endif

#ifdef TALOS_RPI5_UART_CANDIDATE_DIAGNOSTIC
rpi5_uart10_candidate_marker:
    .asciz "TALOS: uart10\r\n"

rpi5_rp1_uart0_candidate_marker:
    .asciz "TALOS: rp1-uart0\r\n"
#endif

#if defined(TALOS_RPI5_PHASE_LADDER_DIAGNOSTIC) || defined(TALOS_RPI5_PHASE_CONTINUE_DIAGNOSTIC)
rpi5_phase_ladder_p0_marker:
    .asciz "TALOS: P0 asm-entry\r\n"

rpi5_phase_ladder_p1_marker:
    .asciz "TALOS: P1 asm-stack\r\n"

rpi5_phase_ladder_p1_short_marker:
    .asciz "TALOS: P1\r\n"

rpi5_phase_ladder_p2_marker:
    .asciz "TALOS: P2 asm-before-rust\r\n"

rpi5_phase_ladder_p2_short_marker:
    .asciz "TALOS: P2\r\n"
#endif
#if defined(TALOS_RPI5_PHASE_CPACR_RESET_DIAGNOSTIC) || defined(TALOS_RPI5_PHASE_BSS_RESET_DIAGNOSTIC) || defined(TALOS_RPI5_PHASE_STACK_RESET_DIAGNOSTIC) || defined(TALOS_RPI5_PHASE_STACK_TO_TEXT_RESET_DIAGNOSTIC) || defined(TALOS_RPI5_PHASE_STACK_TO_RUST_RESET_DIAGNOSTIC)
rpi5_boundary_before_cpacr_marker:
    .asciz "TALOS: before CPACR\r\n"

rpi5_boundary_after_cpacr_marker:
    .asciz "TALOS: after CPACR\r\n"
#endif
#if defined(TALOS_RPI5_PHASE_BSS_RESET_DIAGNOSTIC) || defined(TALOS_RPI5_PHASE_STACK_RESET_DIAGNOSTIC) || defined(TALOS_RPI5_PHASE_STACK_TO_TEXT_RESET_DIAGNOSTIC) || defined(TALOS_RPI5_PHASE_STACK_TO_RUST_RESET_DIAGNOSTIC)
rpi5_boundary_after_bss_marker:
    .asciz "TALOS: after BSS\r\n"
#endif
#if defined(TALOS_RPI5_PHASE_STACK_RESET_DIAGNOSTIC) || defined(TALOS_RPI5_PHASE_STACK_TO_TEXT_RESET_DIAGNOSTIC) || defined(TALOS_RPI5_PHASE_STACK_TO_RUST_RESET_DIAGNOSTIC)
rpi5_boundary_after_stack_marker:
    .asciz "TALOS: after stack\r\n"
#endif
#if defined(TALOS_RPI5_PHASE_STACK_TO_TEXT_RESET_DIAGNOSTIC)
rpi5_boundary_before_text_branch_marker:
    .asciz "TALOS: before text branch\r\n"
#endif
#if defined(TALOS_RPI5_FRESH_ENTRY_RESET_DIAGNOSTIC) || defined(TALOS_RPI5_FRESH_ENTRY_CONTINUE_DIAGNOSTIC)
#ifndef TALOS_RPI5_FRESH_ENTRY_LABEL
#define TALOS_RPI5_FRESH_ENTRY_LABEL "TALOS: fresh entry reset\r\n"
#endif
rpi5_fresh_entry_marker:
    .asciz TALOS_RPI5_FRESH_ENTRY_LABEL
#endif
.balign 4
#endif

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
#elif defined(TALOS_RPI5_HANDOFF_UART_DIAGNOSTIC)
    .asciz "AH\r\n"
#elif defined(TALOS_RPI5_ENTRY_LOOP_DIAGNOSTIC)
    .asciz "EL\r\n"
#elif defined(TALOS_RPI5_ENTRY_TALOS_LINE_DIAGNOSTIC) || defined(TALOS_RPI5_ENTRY_TALOS_LINE_RESET_DIAGNOSTIC)
    .asciz "TALOS: entry\r\n"
#else
    .asciz "BI\r\n"
#endif
#endif

#if defined(TALOS_RPI5_BRK_ERET_UART_MARKER_DIAGNOSTIC)
.balign 4
.type rpi5_write_eret_marker, %function
rpi5_write_eret_marker:
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
    .asciz "EM\r\n"
.size rpi5_write_eret_marker, . - rpi5_write_eret_marker
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
