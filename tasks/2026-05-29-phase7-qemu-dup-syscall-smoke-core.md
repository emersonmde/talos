# Phase 7 QEMU Dup Syscall Smoke Core

Task: phase7-qemu-dup-syscall-smoke-core-20260529
Status: accepted

## Scope

This task implemented and retained QEMU/substitute evidence for talos_dup on
the accepted lower-AArch64 syscall path. It added qemu_dup_syscall_smoke,
proved current-owner ProcessDescriptorStore lookup and descriptor lifetime
behavior for duplicated stdout, and did not run Pi 5 hardware, publish a boot
archive, acquire hardwareTestLock, add read behavior, process loading,
VFS/filesystem behavior, shell behavior, networking, SSH, RP1/PCIe, UART
interrupt ownership, object finalization, DMA/cache-driver policy, dup2/fcntl,
or a full POSIX descriptor claim.

## Changes

- Added qemu_dup_syscall_smoke boot-scenario registration.
- Added a focused QEMU lower-EL payload, run path, exception handler, and PASS
  classification for dup behavior in src/target/qemu_virt.rs.
- Added scripts/qemu-dup-syscall-smoke.sh with retained evidence and grep
  gates.
- Updated docs/src/roadmap.md and docs/src/decisions/README.md.

## Evidence

- Retained QEMU/substitute log:
  tasks/evidence/2026-05-29-qemu-dup-syscall-smoke-core/qemu-dup-syscall-smoke.log.
- PASS/classification:
  qemu-dup-syscall-smoke: final participants=14 expected=14 errors=0
  classification=qemu-dup-syscall-smoke-complete; qemu-dup-syscall-smoke:
  PASS.
- The log proves fd 1 duplicates to fd 3, fd 2 duplication returns -EMFILE in
  the full four-slot table, reserved x1 returns -EINVAL without mutation,
  writes through fd 1 and fd 3 reach runtime-console0, close(fd 1) preserves fd
  3, closed fd 1/fd 3 and dup(closed fd 1) return -EBADF, talos_nop returns
  zero, unknown syscall returns -ENOSYS, talos_copy_probe remains quarantined
  as -ENOSYS, and diagnostic marker 0x7a10 remains proof-only.
- Regression gates:
  scripts/qemu-descriptor-write-smoke.sh passed;
  scripts/qemu-close-syscall-smoke.sh passed.
- Static/unit gates:
  cargo fmt --all -- --check passed; cargo -Zjson-target-spec test passed with
  239 no_std tests.
- Documentation/static gates:
  git diff --check passed; mdbook build passed.

## Deferred Work

Pi 5 physical dup proof, read syscall behavior, stdin/read object model,
process loading, VFS/filesystem, shell, networking, SSH, RP1/PCIe, UART
interrupt ownership, object finalization, DMA/cache-driver policy, dup2/fcntl,
and full POSIX descriptor readiness remain blocked.

## Next Task

The next bounded task is
phase7-dup-syscall-closeout-checkpoint-20260529, scoped to reconciling the
accepted dup contract/core, QEMU dup plan, retained QEMU evidence, regression
gates, residual risks, and deferred surfaces before any Pi 5 dup proof plan.
