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

    mov x0, x19
    bl rust_entry

2:
    wfe
    b 2b

.size _start, . - _start

#ifdef TALOS_QEMU_SECONDARY_CORE_DISCRIMINATOR
.global talos_aarch64_qemu_secondary_entry
.type talos_aarch64_qemu_secondary_entry, %function

talos_aarch64_qemu_secondary_entry:
    mov x19, x0
    cmp x19, #1
    b.lo 4f
    cmp x19, #3
    b.hi 4f

    adrp x1, talos_qemu_secondary_stacks
    add x1, x1, :lo12:talos_qemu_secondary_stacks
    add x1, x1, x19, lsl #12
    add x1, x1, #4096
    mov sp, x1

    mov x0, x19
    bl talos_qemu_secondary_entry

4:
    wfe
    b 4b

.size talos_aarch64_qemu_secondary_entry, . - talos_aarch64_qemu_secondary_entry

.section .bss.qemu_secondary_stacks, "aw", %nobits
.align 12
.global talos_qemu_secondary_stacks
talos_qemu_secondary_stacks:
    .skip 4 * 4096
.global talos_qemu_secondary_stacks_end
talos_qemu_secondary_stacks_end:
.previous
#endif
