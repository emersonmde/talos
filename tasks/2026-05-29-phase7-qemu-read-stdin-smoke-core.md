# Phase 7 QEMU Read And Stdin Smoke Core

Task: phase7-qemu-read-stdin-smoke-core-20260529
Status: accepted

## Scope

This task implemented and retained QEMU/substitute evidence for talos_read on
the accepted lower-AArch64 syscall path. It added qemu_read_stdin_smoke,
proved current-owner ProcessDescriptorStore lookup, fd 0 duplication, fixed
proof stdin copy-out, bounded short-read and EOF behavior, deterministic errno
cases, scalar regressions, and proof-only quarantine. It did not run Pi 5
hardware, publish a boot archive, acquire hardwareTestLock, add
runtime-console0/TTY/hardware stdin, process loading, VFS/filesystem behavior,
shell behavior, networking, SSH, RP1/PCIe, UART interrupt ownership, object
finalization, DMA/cache-driver policy, dup2/fcntl, or a full POSIX descriptor
claim.

## Changes

- Added qemu_read_stdin_smoke boot-scenario registration.
- Added a focused QEMU lower-EL payload, run path, exception handler, and PASS
  classification for read/stdin behavior in src/target/qemu_virt.rs.
- Added scripts/qemu-read-stdin-smoke.sh with retained evidence and grep
  gates.
- Updated docs/src/roadmap.md and docs/src/decisions/README.md.

## Evidence

- Retained QEMU/substitute log:
  tasks/evidence/2026-05-29-qemu-read-stdin-smoke-core/qemu-read-stdin-smoke.log.
- PASS/classification:
  qemu-read-stdin-smoke: final participants=11 expected=11 errors=0
  classification=qemu-read-stdin-smoke-complete; qemu-read-stdin-smoke: PASS.
- The log proves fd 0 duplicates to fd 3, guard copy-out returns -EFAULT
  without consuming stdin, reserved x3 returns -EINVAL without mutation, fd 1
  and fd 99 return -EBADF without mutation, fd 0 reads talos, fd 3 consumes
  the remaining -stdin-qemu\n as a short read, bounded EOF returns 0 without
  mutation, talos_nop returns zero, unknown syscall returns -ENOSYS,
  talos_copy_probe remains quarantined as -ENOSYS, and diagnostic marker
  0x7a10 remains proof-only.
- Regression gates:
  scripts/qemu-syscall-smoke.sh passed;
  scripts/qemu-descriptor-write-smoke.sh passed;
  scripts/qemu-close-syscall-smoke.sh passed;
  scripts/qemu-dup-syscall-smoke.sh passed.
- Static/unit gates:
  cargo fmt --all -- --check passed; cargo -Zjson-target-spec test passed with
  248 no_std tests.
- Documentation/static gates:
  git diff --check passed; mdbook build passed; git diff --cached --check
  passed before commit.
- Optional pointer-copy regression note:
  scripts/qemu-pointer-copy-smoke.sh was attempted but is not an acceptance
  gate for this task. It failed to compile in an unrelated pre-existing
  src/target/rpi5.rs pointer-copy finish path before running QEMU.

## Deferred Work

Pi 5 physical read proof, runtime-console0/TTY/hardware stdin, process
loading, VFS/filesystem, shell, networking, SSH, RP1/PCIe, UART interrupt
ownership, object finalization, DMA/cache-driver policy, dup2/fcntl, signals,
wait queues, nonblocking I/O, and full POSIX descriptor readiness remain
blocked.

## Next Task

The next bounded task is
phase7-read-stdin-closeout-checkpoint-20260529, scoped to reconciling the
accepted read/stdin inventory, contract, target-independent core, QEMU smoke
plan, retained QEMU evidence, regression gates, residual risks, and deferred
surfaces before any Pi 5 read/stdin proof plan.
