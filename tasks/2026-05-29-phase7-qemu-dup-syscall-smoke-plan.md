# Phase 7 QEMU Dup Syscall Smoke Plan

Task: phase7-qemu-dup-syscall-smoke-plan-20260529
Status: accepted

## Scope

This documentation-only task planned the QEMU/substitute proof that talos_dup
duplicates stdout through the current ProcessOwnerId-backed process-owned
inherited stdio table, that writes through both the source and duplicate reach
runtime-console0, and that closing one descriptor leaves the other usable
until independently closed. It did not add Rust behavior, assembly behavior,
QEMU execution, Pi 5 hardware execution, boot archive publication,
hardwareTestLock acquisition, read behavior, process loading, VFS/filesystem
behavior, shell behavior, networking, SSH, RP1/PCIe, UART interrupt ownership,
object finalization, DMA/cache-driver policy, or a physical dup syscall claim.

## Plan

- Added docs/src/project/phase7-qemu-dup-syscall-smoke-plan.md.
- Required qemu_dup_syscall_smoke to create a ProcessDescriptorStore, install
  one four-slot inherited stdio table for ProcessOwnerId 1, resolve the
  current owner through the accepted lookup API, and route talos_dup through
  ProcessDescriptorStore::dup_current_descriptor().
- Required lower-AArch64 evidence for dup fd 1 returning fd 3, table-full
  -EMFILE, reserved-register -EINVAL without mutation, writes through fd 1 and
  fd 3, close(fd 1) preserving fd 3, closed-source and closed-duplicate
  -EBADF, talos_nop, unknown-syscall, proof-only talos_copy_probe quarantine,
  and diagnostic-marker quarantine observations.
- Named the retained QEMU/substitute evidence path and the next bounded
  implementation task: phase7-qemu-dup-syscall-smoke-core-20260529.

## Evidence

- Accepted dup syscall core commit:
  2c30e4446f6611edb2bea1b75f226a6e919bf310.
- Plan document:
  docs/src/project/phase7-qemu-dup-syscall-smoke-plan.md.
- Expected PASS/classification names:
  classification=qemu-dup-syscall-smoke-complete and
  qemu-dup-syscall-smoke: PASS.
- Static documentation diff:
  docs/src/SUMMARY.md, docs/src/roadmap.md, docs/src/decisions/README.md,
  and this task record were updated.
- Whitespace inspection:
  git diff --check passed.
- Documentation:
  mdbook build passed.

## Deferred Work

The implementation task remains blocked behind the accepted plan. Pi 5
physical dup proof, read behavior, stdin/read object model, process loading,
VFS/filesystem, shell, networking, SSH, RP1/PCIe, UART interrupt ownership,
DMA/cache-driver policy, object finalization, and full POSIX descriptor claims
remain blocked.

## Next Task

The next bounded task is phase7-qemu-dup-syscall-smoke-core-20260529, scoped
to implementing and retaining the QEMU/substitute dup syscall smoke without
acquiring hardwareTestLock or making a physical claim.
