# Phase 7 Descriptor-Write Core

Status: accepted as the target-independent Phase 7.3 descriptor-write core.

## Scope

- Implement only the accepted talos_write x8 = 1 core for fd 1 and fd 2
  runtime-console0 writes.
- Preserve copy_from_user validation before runtime-console side effects.
- Preserve talos_nop, unknown-syscall, and proof-only talos_copy_probe
  quarantine behavior.
- Keep QEMU smoke orchestration, Pi 5 hardware, stdin/read, close, dup,
  process loading, VFS/filesystem, shell, networking, SSH, RP1/PCIe, UART
  interrupt ownership, DMA/cache-driver policy, and phase transitions blocked.

## Implementation Notes

- src/syscall.rs adds the stable talos_write number, errno encoding for EIO,
  and a context-aware descriptor-write dispatch path.
- src/posix.rs adds the target-independent descriptor-table, user-copy, and
  runtime-console write helper for inherited stdout/stderr.
- src/runtime_console.rs adds a narrow byte-write facade used through
  runtime-console0 rather than target UART/MMIO backends.

## Evidence

- fmt/lint: cargo fmt --all -- --check passed.
- unit tests: cargo -Zjson-target-spec test passed, 218 tests.
- QEMU/substitute regression: scripts/qemu-syscall-smoke.sh passed with
  classification=qemu-syscall-smoke-complete and PASS.
- QEMU/substitute regression: scripts/qemu-pointer-copy-smoke.sh passed with
  classification=qemu-pointer-copy-smoke-complete and PASS.
- static build check: TALOS_BOOT_SCENARIO=rpi5_syscall_proof cargo
  -Zjson-target-spec build --target targets/aarch64-talos-rpi5-bcm2712.json
  passed after the existing Pi 5 syscall-proof exhaustiveness match was
  updated for the new talos_write syscall number.
- whitespace inspection: git diff --check passed.
- documentation: mdbook build was not run because docs/src was not touched by
  this task.
- commit: recorded in durable supervisor state after acceptance.
