# Phase 7 QEMU Read And Stdin Smoke Plan

Task: phase7-qemu-read-stdin-smoke-plan-20260529
Owner: worker
Status: accepted
Date: 2026-05-29
Milestone: Phase 7.4, File Descriptor Table

## Goal

Define the lower-AArch64 QEMU/substitute read/stdin smoke before runtime
evidence is claimed.

## Scope

- Added the documentation-only QEMU read/stdin smoke plan at
  docs/src/project/phase7-qemu-read-stdin-smoke-plan.md.
- Defined the qemu_read_stdin_smoke invariant for stable svc #0 talos_read
  x8 = 4 through the accepted ProcessOwnerId-backed descriptor path and
  FixedStdin proof buffer.
- Required QEMU/substitute observations for fd 0 duplication, copy-out
  -EFAULT, reserved-register -EINVAL, fd/error -EBADF, successful read of
  talos, duplicated-stdin short read of -stdin-qemu\n, bounded EOF, talos_nop,
  unknown-syscall, copy-probe quarantine, diagnostic-marker quarantine, final
  classification, and PASS.
- Named phase7-qemu-read-stdin-smoke-core-20260529 as the next mechanically
  derivable implementation/evidence task.

## Non-Goals

- No Rust or assembly behavior changes.
- No QEMU run, Pi 5 hardware run, boot archive publication, hardwareTestLock
  acquisition, hardware proof, process loading, VFS/filesystem, shell,
  networking, SSH, object finalization, runtime-console0/TTY/hardware stdin,
  RP1/PCIe, UART interrupt ownership, or DMA/cache-driver policy.
- No expansion from bounded proof stdin to real local TTY/filesystem/socket
  input.

## Changed Files

- docs/src/project/phase7-qemu-read-stdin-smoke-plan.md
- docs/src/SUMMARY.md
- docs/src/decisions/README.md
- docs/src/roadmap.md
- tasks/2026-05-29-phase7-qemu-read-stdin-smoke-plan.md

## Evidence

- Accepted core dependency:
  613c85a1423677a764f031328530e59b3f7998ea.
- Plan path:
  docs/src/project/phase7-qemu-read-stdin-smoke-plan.md.
- Expected retained log path for the later core:
  tasks/evidence/2026-05-29-qemu-read-stdin-smoke-core/qemu-read-stdin-smoke.log.
- Static documentation diff:
  qemu_read_stdin_smoke plan, SUMMARY link, roadmap status update, decision
  entry, and this task record.
- Whitespace inspection:
  git diff --check passed.
- Documentation:
  mdbook build passed.
- Staged whitespace inspection:
  git diff --cached --check passed before commit.

## Deferred Work

QEMU read/stdin smoke implementation and retained runtime evidence, Pi 5
physical read proof, runtime-console0/TTY/hardware stdin, process loading,
fork/spawn/exec, descriptor inheritance across exec, close-on-exec
application, process exit teardown, open-file-description reference counting,
object finalizers, file offsets, VFS/filesystem lookup, regular files,
directories, pipes, sockets, terminal sessions, foreground process groups,
blocking/readiness, nonblocking flags, poll/select, wait queues, signals,
restart semantics, path copying, argv/envp loading, per-thread errno storage,
libc/Rust std stdio, shell behavior, networking, SSH, RP1/PCIe, UART interrupt
ownership, DMA/cache-driver policy, and full POSIX descriptor readiness remain
blocked.

## Result

Accepted as the documentation-only QEMU/substitute read/stdin smoke plan. The
next bounded task is phase7-qemu-read-stdin-smoke-core-20260529, scoped to
implementing and retaining only the qemu_read_stdin_smoke evidence defined by
the accepted plan.
