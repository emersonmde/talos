# Phase 7 QEMU Close Syscall Smoke Plan

Task: phase7-qemu-close-syscall-smoke-plan-20260529
Status: accepted

## Scope

This documentation-only task planned the QEMU/substitute proof that
talos_close removes fd 1 and fd 2 from the current ProcessOwnerId-backed
process-owned inherited stdio table, that later talos_write on a closed
descriptor returns -EBADF before runtime-console0 side effects, and that an
unaffected descriptor remains usable until independently closed. It did not
add Rust behavior, assembly behavior, QEMU execution, Pi 5 hardware execution,
boot archive publication, hardwareTestLock acquisition, dup/read syscalls,
process loading, VFS/filesystem behavior, shell behavior, networking, SSH,
RP1/PCIe, UART interrupt ownership, DMA/cache-driver policy, or a physical
close syscall claim.

## Plan

- Added docs/src/project/phase7-qemu-close-syscall-smoke-plan.md.
- Required qemu_close_syscall_smoke to create a ProcessDescriptorStore,
  install one inherited stdio table for ProcessOwnerId 1, resolve the current
  owner through the accepted lookup API, and route talos_close through
  ProcessDescriptorStore::close_current_descriptor().
- Required lower-AArch64 evidence for close fd 1 success, fd 1 write
  -EBADF after close, reserved close fd 2 -EINVAL without mutation, fd 2 write
  success after fd 1 close, close fd 2 success, fd 2 write -EBADF after close,
  repeated close/bad fd -EBADF, talos_nop, unknown-syscall, proof-only
  talos_copy_probe quarantine, and diagnostic-marker quarantine observations.
- Named the retained QEMU/substitute evidence path and the next bounded
  implementation task: phase7-qemu-close-syscall-smoke-core-20260529.

## Evidence

- Accepted close syscall core commit:
  ab8915b9696a046b367830e9f5acfd632ee98788.
- Plan document:
  docs/src/project/phase7-qemu-close-syscall-smoke-plan.md.
- Static documentation diff:
  docs/src/SUMMARY.md, docs/src/roadmap.md, docs/src/decisions/README.md,
  and this task record were updated.
- Whitespace inspection:
  git diff --check passed.
- Documentation:
  mdbook build passed.

## Deferred Work

The implementation task remains blocked behind the accepted plan. Pi 5
physical close proof, dup/read behavior, process loading, VFS/filesystem,
stdin/read object model, shell, networking, SSH, RP1/PCIe, UART interrupt
ownership, DMA/cache-driver policy, object finalization, and full POSIX
descriptor claims remain blocked.

## Next Task

The next bounded task is phase7-qemu-close-syscall-smoke-core-20260529,
scoped to implementing and retaining the QEMU/substitute close syscall smoke
without acquiring hardwareTestLock or making a physical claim.
