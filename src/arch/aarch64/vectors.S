.section .vectors, "ax"
.balign 2048
.global __exception_vectors
.type __exception_vectors, %function

.macro vector_entry kind
    .balign 128
    mov x3, #\kind
    b __exception_entry
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

__exception_entry:
    mrs x0, ESR_EL1
    mrs x1, ELR_EL1
    mrs x2, FAR_EL1
    bl rust_exception_handler

1:
    wfe
    b 1b

.size __exception_vectors, . - __exception_vectors
