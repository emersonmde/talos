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

#if defined(TALOS_RPI5_PSCI_SECONDARY_CORE_ALIVE_PROOF) || defined(TALOS_RPI5_SECONDARY_CORE_WORKLOAD_PROOF) || defined(TALOS_RPI5_SMP_LOCK_CACHE_COHERENCE_PROOF) || defined(TALOS_RPI5_CROSS_CORE_IPI_DELIVERY_PROOF)
    adrp x0, talos_rpi5_asm_start_line
    add x0, x0, :lo12:talos_rpi5_asm_start_line
    mov x1, #(talos_rpi5_asm_start_line_end - talos_rpi5_asm_start_line)
    bl talos_rpi5_early_uart_write
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

#if defined(TALOS_RPI5_PSCI_SECONDARY_CORE_ALIVE_PROOF) || defined(TALOS_RPI5_SECONDARY_CORE_WORKLOAD_PROOF) || defined(TALOS_RPI5_SMP_LOCK_CACHE_COHERENCE_PROOF) || defined(TALOS_RPI5_CROSS_CORE_IPI_DELIVERY_PROOF)
    adrp x0, talos_rpi5_asm_pre_rust_line
    add x0, x0, :lo12:talos_rpi5_asm_pre_rust_line
    mov x1, #(talos_rpi5_asm_pre_rust_line_end - talos_rpi5_asm_pre_rust_line)
    bl talos_rpi5_early_uart_write
#endif

    mov x0, x19
    bl rust_entry

2:
    wfe
    b 2b

.size _start, . - _start

#if defined(TALOS_RPI5_PSCI_SECONDARY_CORE_ALIVE_PROOF) || defined(TALOS_RPI5_SECONDARY_CORE_WORKLOAD_PROOF) || defined(TALOS_RPI5_SMP_LOCK_CACHE_COHERENCE_PROOF) || defined(TALOS_RPI5_CROSS_CORE_IPI_DELIVERY_PROOF)
talos_rpi5_early_uart_write:
    mov x22, x30
    cbz x1, 2f
1:
    ldrb w11, [x0], #1
    bl talos_rpi5_early_uart_putc
    subs x1, x1, #1
    b.ne 1b
    bl talos_rpi5_early_uart_wait_empty
2:
    br x22

talos_rpi5_early_uart_putc:
    mov x9, #0x1000
    movk x9, #0x7d00, lsl #16
    movk x9, #0x10, lsl #32
    str w11, [x9]
    ldr w10, [x9, #0x18]
    dsb sy
    ret

talos_rpi5_early_uart_wait_empty:
    mov x9, #0x1000
    movk x9, #0x7d00, lsl #16
    movk x9, #0x10, lsl #32
    mov x21, #0x200000
1:
    ldr w10, [x9, #0x18]
    tbnz w10, #7, 2f
    subs x21, x21, #1
    b.ne 1b
2:
    dsb sy
    ret

talos_rpi5_asm_start_line:
    .ascii "TALOS: asm_start\r\n"
talos_rpi5_asm_start_line_end:

talos_rpi5_asm_pre_rust_line:
    .ascii "TALOS: asm_pre_rust_entry\r\n"
talos_rpi5_asm_pre_rust_line_end:

.align 2
#endif

#if defined(TALOS_QEMU_SECONDARY_CORE_DISCRIMINATOR) || defined(TALOS_QEMU_SECONDARY_CORE_WORKLOAD_SMOKE) || defined(TALOS_QEMU_SMP_LOCK_CONTENTION_SMOKE) || defined(TALOS_QEMU_PER_CORE_SCHEDULER_OWNERSHIP_SMOKE) || defined(TALOS_QEMU_CROSS_CORE_IPI_DELIVERY_SMOKE) || defined(TALOS_RPI5_PSCI_SECONDARY_CORE_ALIVE_PROOF) || defined(TALOS_RPI5_SECONDARY_CORE_WORKLOAD_PROOF) || defined(TALOS_RPI5_SMP_LOCK_CACHE_COHERENCE_PROOF) || defined(TALOS_RPI5_CROSS_CORE_IPI_DELIVERY_PROOF)
#if defined(TALOS_QEMU_SECONDARY_CORE_DISCRIMINATOR) || defined(TALOS_QEMU_SECONDARY_CORE_WORKLOAD_SMOKE) || defined(TALOS_QEMU_SMP_LOCK_CONTENTION_SMOKE) || defined(TALOS_QEMU_PER_CORE_SCHEDULER_OWNERSHIP_SMOKE) || defined(TALOS_QEMU_CROSS_CORE_IPI_DELIVERY_SMOKE)
#define TALOS_SECONDARY_ENTRY_ASM talos_aarch64_qemu_secondary_entry
#define TALOS_SECONDARY_ENTRY_RUST talos_qemu_secondary_entry
#endif
#if defined(TALOS_RPI5_PSCI_SECONDARY_CORE_ALIVE_PROOF) || defined(TALOS_RPI5_SECONDARY_CORE_WORKLOAD_PROOF) || defined(TALOS_RPI5_SMP_LOCK_CACHE_COHERENCE_PROOF) || defined(TALOS_RPI5_CROSS_CORE_IPI_DELIVERY_PROOF)
#define TALOS_SECONDARY_ENTRY_ASM talos_aarch64_rpi5_secondary_entry
#define TALOS_SECONDARY_ENTRY_RUST talos_rpi5_secondary_entry
#endif

.global TALOS_SECONDARY_ENTRY_ASM
.type TALOS_SECONDARY_ENTRY_ASM, %function

TALOS_SECONDARY_ENTRY_ASM:
    mov x19, x0
#if defined(TALOS_RPI5_PSCI_SECONDARY_CORE_ALIVE_PROOF) || defined(TALOS_RPI5_SECONDARY_CORE_WORKLOAD_PROOF) || defined(TALOS_RPI5_SMP_LOCK_CACHE_COHERENCE_PROOF) || defined(TALOS_RPI5_CROSS_CORE_IPI_DELIVERY_PROOF)
    adrp x0, talos_rpi5_secondary_entry_line
    add x0, x0, :lo12:talos_rpi5_secondary_entry_line
    mov x1, #(talos_rpi5_secondary_entry_line_end - talos_rpi5_secondary_entry_line)
    bl talos_rpi5_early_uart_write
#endif
    cmp x19, #1
    b.lo 4f
    cmp x19, #3
    b.hi 4f

    adrp x1, talos_secondary_core_stacks
    add x1, x1, :lo12:talos_secondary_core_stacks
    add x1, x1, x19, lsl #12
    add x1, x1, #4096
    mov sp, x1

#if defined(TALOS_RPI5_PSCI_SECONDARY_CORE_ALIVE_PROOF) || defined(TALOS_RPI5_SECONDARY_CORE_WORKLOAD_PROOF) || defined(TALOS_RPI5_SMP_LOCK_CACHE_COHERENCE_PROOF) || defined(TALOS_RPI5_CROSS_CORE_IPI_DELIVERY_PROOF)
    adrp x0, talos_rpi5_secondary_pre_rust_line
    add x0, x0, :lo12:talos_rpi5_secondary_pre_rust_line
    mov x1, #(talos_rpi5_secondary_pre_rust_line_end - talos_rpi5_secondary_pre_rust_line)
    bl talos_rpi5_early_uart_write
#endif

    mov x0, x19
    bl TALOS_SECONDARY_ENTRY_RUST

4:
    wfe
    b 4b

.size TALOS_SECONDARY_ENTRY_ASM, . - TALOS_SECONDARY_ENTRY_ASM

.section .bss.secondary_core_stacks, "aw", %nobits
.align 12
.global talos_secondary_core_stacks
talos_secondary_core_stacks:
    .skip 4 * 4096
.global talos_secondary_core_stacks_end
talos_secondary_core_stacks_end:
.previous

#if defined(TALOS_RPI5_PSCI_SECONDARY_CORE_ALIVE_PROOF) || defined(TALOS_RPI5_SECONDARY_CORE_WORKLOAD_PROOF) || defined(TALOS_RPI5_SMP_LOCK_CACHE_COHERENCE_PROOF) || defined(TALOS_RPI5_CROSS_CORE_IPI_DELIVERY_PROOF)
talos_rpi5_secondary_entry_line:
    .ascii "TALOS: secondary_entry\r\n"
talos_rpi5_secondary_entry_line_end:

talos_rpi5_secondary_pre_rust_line:
    .ascii "TALOS: secondary_pre_rust_entry\r\n"
talos_rpi5_secondary_pre_rust_line_end:

.align 2
#endif
#endif
