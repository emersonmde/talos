/*
 * Minimal AArch64 UEFI application diagnostic.
 *
 * This is intentionally separate from the normal Talos kernel and the direct
 * Raspberry Pi firmware loader diagnostics. It proves that a UEFI-capable
 * intermediate loader can enter Talos-owned code before we return to direct
 * firmware handoff work.
 */

    .text
    .globl EfiMain
    .p2align 2
EfiMain:
    stp x29, x30, [sp, #-16]!
    mov x29, sp

    /*
     * EFI_SYSTEM_TABLE.ConOut is at byte offset 64 on AArch64. The
     * SIMPLE_TEXT_OUTPUT_PROTOCOL.OutputString function pointer is at offset 8.
     */
    ldr x2, [x1, #64]
    adr x1, efi_message
    mov x0, x2
    ldr x3, [x2, #8]
    blr x3

    mov x0, xzr
    ldp x29, x30, [sp], #16
    ret

    .section .rdata, "dr"
    .p2align 1
efi_message:
    .hword 'T', 'a', 'l', 'o', 's', ' ', 'E', 'F'
    .hword 'I', ' ', 'f', 'i', 'r', 's', 't', '-'
    .hword 'l', 'i', 'g', 'h', 't', ' ', 'P', 'A'
    .hword 'S', 'S', '\r', '\n', 0
