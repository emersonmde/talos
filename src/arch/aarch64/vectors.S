.section .vectors, "ax"
.balign 2048
.global __exception_vectors
.type __exception_vectors, %function

.macro vector_entry kind
    .balign 128
    sub sp, sp, #32
    stp x0, x1, [sp]
    mov x0, #\kind
    str x0, [sp, #16]
    b __exception_entry_frame
.endm

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

.section .text.context_switch, "ax"
.balign 4
.global talos_aarch64_context_switch
.type talos_aarch64_context_switch, %function
talos_aarch64_context_switch:
    stp x19, x20, [x0, #0]
    stp x21, x22, [x0, #16]
    stp x23, x24, [x0, #32]
    stp x25, x26, [x0, #48]
    stp x27, x28, [x0, #64]
    stp x29, x30, [x0, #80]
    mov x9, sp
    str x9, [x0, #96]

    ldr x9, [x1, #96]
    mov sp, x9
    ldp x19, x20, [x1, #0]
    ldp x21, x22, [x1, #16]
    ldp x23, x24, [x1, #32]
    ldp x25, x26, [x1, #48]
    ldp x27, x28, [x1, #64]
    ldp x29, x30, [x1, #80]
    ret

.size talos_aarch64_context_switch, . - talos_aarch64_context_switch

.global talos_aarch64_kernel_thread_trampoline
.type talos_aarch64_kernel_thread_trampoline, %function
talos_aarch64_kernel_thread_trampoline:
    mov x0, x19
    blr x20
1:
    wfe
    b 1b

.size talos_aarch64_kernel_thread_trampoline, . - talos_aarch64_kernel_thread_trampoline

.global talos_aarch64_enter_el1_then_el0
.type talos_aarch64_enter_el1_then_el0, %function
talos_aarch64_enter_el1_then_el0:
    mov x9, sp
    msr SP_EL1, x9
    adr x10, 1f
    msr ELR_EL2, x10
    msr SPSR_EL2, x3
    isb
    eret
1:
    msr SP_EL0, x1
    msr ELR_EL1, x0
    msr SPSR_EL1, x2
    isb
    eret

.size talos_aarch64_enter_el1_then_el0, . - talos_aarch64_enter_el1_then_el0
