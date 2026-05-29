# Phase 7 QEMU Process Descriptor Stdio Smoke Plan

Task: phase7-qemu-process-descriptor-stdio-smoke-plan-20260529
Status: accepted

## Scope

This documentation-only task planned the QEMU/substitute proof that
talos_write fd 1 and fd 2 route through a ProcessOwnerId-backed
process-owned inherited stdio table. It did not add Rust behavior, assembly
behavior, QEMU execution, Pi 5 hardware execution, boot archive publication,
hardwareTestLock acquisition, close, dup, read syscalls, process loading,
VFS/filesystem behavior, shell behavior, networking, SSH, RP1/PCIe, UART
interrupt ownership, DMA/cache-driver policy, or a physical descriptor-table
claim.

## Plan

- Added
  docs/src/project/phase7-qemu-process-descriptor-stdio-smoke-plan.md.
- Required qemu_process_descriptor_stdio_smoke to create a
  ProcessDescriptorStore, install one inherited stdio table for
  ProcessOwnerId 1, resolve the current owner through the accepted lookup API,
  and route talos_write through that process-owned table.
- Carried forward fd 1/fd 2 success, fd 0/fd 99 -EBADF, guard-range -EFAULT,
  reserved-register -EINVAL, talos_nop, unknown-syscall, proof-only
  talos_copy_probe quarantine, and diagnostic-marker quarantine observations.
- Named the retained QEMU/substitute evidence path and the next bounded
  implementation task:
  phase7-qemu-process-descriptor-stdio-smoke-core-20260529.

## Evidence

- Accepted process descriptor table core commit:
  a30944d53aefd58ca89a7d197d12bae0790beb73.
- Plan document:
  docs/src/project/phase7-qemu-process-descriptor-stdio-smoke-plan.md.
- Static documentation diff:
  docs/src/SUMMARY.md, docs/src/roadmap.md, docs/src/decisions/README.md,
  and this task record were updated.
- Whitespace inspection:
  git diff --check passed.
- Documentation:
  mdbook build passed.

## Deferred Work

The implementation task remains blocked behind the accepted plan. Pi 5
physical proof, stdin/read behavior, close/dup/read syscalls, PID allocation,
fork/spawn/exec, process loading, VFS/filesystem, shell, networking, SSH,
RP1/PCIe, UART interrupt ownership, DMA/cache-driver policy, and full POSIX
descriptor claims remain blocked.

## Next Task

The next bounded task is
phase7-qemu-process-descriptor-stdio-smoke-core-20260529, scoped to
implementing and retaining the QEMU/substitute process-owned stdio smoke
without acquiring hardwareTestLock or making a physical claim.
