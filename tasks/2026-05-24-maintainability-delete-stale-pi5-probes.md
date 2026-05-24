# 2026-05-24 - Delete Stale Pi 5 Probe Surfaces

Status: accepted and committed as 964be83

Task: `talos-maintainability-delete-stale-pi5-probes-20260524`

## Scope

Remove historical Pi 5 probe code that was no longer advertised by `build.rs` or
supported by wrapper scripts, while preserving normal Pi 5 boot and retained
allocator, panic, exception/fault, translation-fault, page-frame, and
heap-policy diagnostics.

## Before Inventory

`src/arch/aarch64/boot.S` still contained dormant pre-Rust and exception
classifier blocks for these unadvertised flags:

```text
TALOS_RPI5_ASM_BTI_INDIRECT_TO_RUST_RESET_DIAGNOSTIC
TALOS_RPI5_ASM_DIRECT_RESET_DIAGNOSTIC
TALOS_RPI5_ASM_INDIRECT_RESET_DIAGNOSTIC
TALOS_RPI5_ASM_INDIRECT_TO_RUST_RESET_DIAGNOSTIC
TALOS_RPI5_ASM_TEXT_DIRECT_RESET_DIAGNOSTIC
TALOS_RPI5_ASM_TEXT_INDIRECT_RESET_DIAGNOSTIC
TALOS_RPI5_ASM_TEXT_JC_INDIRECT_RESET_DIAGNOSTIC
TALOS_RPI5_ASM_TO_RUST_RESET_DIAGNOSTIC
TALOS_RPI5_BRK_ELR_WRITE_RESET_DIAGNOSTIC
TALOS_RPI5_BRK_ERET_RESUME_RESET_DIAGNOSTIC
TALOS_RPI5_BRK_ERET_UART_MARKER_DIAGNOSTIC
TALOS_RPI5_BRK_SPSR_ERET_RESET_DIAGNOSTIC
TALOS_RPI5_BRK_SPSR_HANDLER_RESET_DIAGNOSTIC
TALOS_RPI5_BTI_CLASSIFIER_WITH_BRK_PRECHECK_DIAGNOSTIC
TALOS_RPI5_BTI_EXCEPTION_CLASSIFIER_DIAGNOSTIC
TALOS_RPI5_CARGO_ASM_UART_PROOF
TALOS_RPI5_DIRECT_EXCEPTION_CONTROL_DIAGNOSTIC
TALOS_RPI5_DIRECT_EXCEPTION_IMMEDIATE_RESET_DIAGNOSTIC
TALOS_RPI5_ENTRY_LOOP_DIAGNOSTIC
TALOS_RPI5_ENTRY_TALOS_LINE_DIAGNOSTIC
TALOS_RPI5_ENTRY_TALOS_LINE_RESET_DIAGNOSTIC
TALOS_RPI5_FALLTHROUGH_RUST_DIAGNOSTIC
TALOS_RPI5_FRESH_ENTRY_CONTINUE_DIAGNOSTIC
TALOS_RPI5_FRESH_ENTRY_LABEL
TALOS_RPI5_FRESH_ENTRY_RESET_DIAGNOSTIC
TALOS_RPI5_HANDOFF_UART_DIAGNOSTIC
TALOS_RPI5_PHASE_BSS_RESET_DIAGNOSTIC
TALOS_RPI5_PHASE_CONTINUE_DIAGNOSTIC
TALOS_RPI5_PHASE_CPACR_RESET_DIAGNOSTIC
TALOS_RPI5_PHASE_LADDER_DIAGNOSTIC
TALOS_RPI5_PHASE_P0_RESET_DIAGNOSTIC
TALOS_RPI5_PHASE_P1_RESET_DIAGNOSTIC
TALOS_RPI5_PHASE_P1_SHORT_RESET_DIAGNOSTIC
TALOS_RPI5_PHASE_P2_RESET_DIAGNOSTIC
TALOS_RPI5_PHASE_STACK_RESET_DIAGNOSTIC
TALOS_RPI5_PHASE_STACK_TO_RUST_RESET_DIAGNOSTIC
TALOS_RPI5_PHASE_STACK_TO_TEXT_RESET_DIAGNOSTIC
TALOS_RPI5_POST_STACK_NOP_DIAGNOSTIC
TALOS_RPI5_POST_STACK_RESET_DIAGNOSTIC
TALOS_RPI5_READABLE_BOOT_LOG_DIAGNOSTIC
TALOS_RPI5_TEXT_BOOT_FAR_DIAGNOSTIC
TALOS_RPI5_TEXT_BOOT_NEAR_BRANCH_DIAGNOSTIC
TALOS_RPI5_TEXT_BOOT_NEAR_DIAGNOSTIC
TALOS_RPI5_TEXT_BOOT_NEXT_BRANCH_DIAGNOSTIC
TALOS_RPI5_TEXT_SECTION_DIAGNOSTIC
TALOS_RPI5_TRANSITION_DIAGNOSTIC
TALOS_RPI5_UART_CANDIDATE_DIAGNOSTIC
TALOS_RPI5_VECTOR_SECTION_DIAGNOSTIC
```

`scripts/rpi5-archive-review.sh` still recognized stale loader modes:

```text
asm-uart-proof
asm-entry-reset-proof
asm-uart-proof-firmware-address
asm-uart-then-reset-firmware-address
asm-entry-reset-firmware-address
cargo-asm-uart-proof
transition-diagnostic
```

Historical standalone assembly/proof files existed under `src/arch/aarch64`:

```text
src/arch/aarch64/rpi5_armstub.S
src/arch/aarch64/rpi5_efi_diagnostic.S
src/arch/aarch64/rpi5_entry_reset_proof.S
src/arch/aarch64/rpi5_loader_diagnostic.S
src/arch/aarch64/rpi5_uart_linker_layout_proof.S
src/arch/aarch64/rpi5_uart_padded_proof.S
src/arch/aarch64/rpi5_uart_proof.S
src/arch/aarch64/rpi5_uart_then_reset_proof.S
```

## Changes

- Replaced `boot.S` with the supported normal boot path: arm64 Image
  header, `x0` preservation, CPACR enable, BSS clear, stack setup, and
  `rust_entry` handoff.
- Deleted the standalone Pi 5 loader/armstub/EFI/UART/reset proof assembly
  files.
- Tightened `scripts/rpi5-archive-review.sh` to recognize only the retained
  `raw-pi5` and `raw-pi5-circle-config` loader diagnostic modes.
- Updated `scripts/rpi5-boot-tree.sh` comments so historical proofs are not
  described as current runnable surfaces.

## After Inventory

The only remaining `TALOS_RPI5_*` flags in `src/arch/aarch64/*.S` are
the retained exception-report and exception-return diagnostics in
`vectors.S`, both advertised by `build.rs` and wrapper scripts.

```text
src/arch/aarch64/vectors.S:TALOS_RPI5_EXCEPTION_REPORT_DIAGNOSTIC
src/arch/aarch64/vectors.S:TALOS_RPI5_EXCEPTION_RETURN_DIAGNOSTIC
```

No `src/arch/aarch64/rpi5_*.S` proof files remain, and the archive reviewer
no longer accepts the stale loader/proof mode names listed above.

## Validation

- Static inspection: before inventory found 48 unadvertised `TALOS_RPI5_*`
  probe flags in `boot.S`; after cleanup, only
  `TALOS_RPI5_EXCEPTION_REPORT_DIAGNOSTIC` and
  `TALOS_RPI5_EXCEPTION_RETURN_DIAGNOSTIC` remain in
  `src/arch/aarch64/*.S`, both in the retained `vectors.S` path.
- Static inspection: `git ls-tree -r --name-only HEAD src/arch/aarch64`
  showed eight historical `rpi5_*.S` standalone files before cleanup; no
  `src/arch/aarch64/rpi5_*.S` files remain after cleanup.
- Shell syntax: `sh -n scripts/rpi5-archive-review.sh scripts/rpi5-boot-tree.sh`
  passed.
- Fmt/lint/typecheck: `cargo fmt --all -- --check` passed.
- Unit tests: `cargo -Zjson-target-spec test` passed 51 no_std tests.
- QEMU/substitute: `scripts/qemu-smoke.sh` passed with
  `talos: qemu smoke PASS`.
- Image/archive inspection: `scripts/rpi5-image.sh` produced
  `target/aarch64-talos-rpi5-bcm2712/debug/kernel_2712.img`.
- Fmt/lint/typecheck: `scripts/rpi5-format-guard-check.sh` passed with
  `Pi 5 formatted early-console build PASS`.
- Representative retained diagnostics: panic report, normal exception report,
  translation fault, alloc OOM, page-frame reuse, and heap expansion policy
  image scripts all built `kernel_2712.img` successfully.
- Archive inspection: a synthetic normal archive passed
  `scripts/rpi5-archive-review.sh`; a synthetic archive with
  `talos_loader_diagnostic=asm-uart-proof` was rejected with
  `unsupported talos_loader_diagnostic mode: asm-uart-proof`.
- Whitespace: `git diff --check` passed.
