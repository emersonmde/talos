.section .text.boot, "ax"
.global _start
.type _start, %function

_start:
    b 3f
    .long 0
    .quad KERNEL_LOAD_ADDR - 0x40000000
    .quad 0x200000
    .quad 0
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

    ldr x0, =__bss_start
    ldr x1, =__bss_end
0:
    cmp x0, x1
    b.hs 1f
    str xzr, [x0], #8
    b 0b

1:
    ldr x0, =__stack_top
    mov sp, x0
    mov x0, x19
    bl rust_entry

2:
    wfe
    b 2b

.size _start, . - _start
