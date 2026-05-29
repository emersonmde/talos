# Phase 7 Close Syscall Contract

Task: phase7-close-syscall-contract-20260529
Owner: worker
Status: accepted
Date: 2026-05-29
Milestone: Phase 7.4, File Descriptor Table

## Goal

Define the first close syscall contract through the current process-owned
descriptor table without expanding dup or read behavior.

## Scope

- Added the documentation contract at
  docs/src/project/phase7-close-syscall-contract.md.
- Fixed the talos_close syscall boundary: stable svc #0, x8 = 2, descriptor
  argument in x0, reserved-zero x1 through x5, x0 = 0 on success, and
  negative errno on failure.
- Defined ProcessDescriptorStore::close_current_descriptor() as the
  process-owned ownership rule for the later implementation task.
- Preserved accepted talos_nop, talos_write, unknown-syscall, descriptor-write,
  and proof-only copy-probe quarantine invariants.
- Recommended exactly one next bounded task:
  phase7-close-syscall-core-20260529.

## Non-Goals

- No Rust or assembly implementation changes.
- No QEMU run, Pi 5 hardware run, archive publication, or hardwareTestLock
  acquisition.
- No dup/read syscall contract, stdin/read object model, process loading,
  VFS/filesystem, shell, networking, SSH, RP1/PCIe, UART interrupt ownership,
  DMA/cache-driver policy, physical close/dup/read claim, object finalization,
  or full POSIX descriptor readiness claim.

## Changed Files

- docs/src/SUMMARY.md
- docs/src/decisions/README.md
- docs/src/project/phase7-close-syscall-contract.md
- docs/src/roadmap.md
- tasks/2026-05-29-phase7-close-syscall-contract.md

## Evidence

- Accepted dependency:
  phase7-close-dup-read-syscall-source-inventory-20260529 at
  8e17c1d0be80f860ef83bc02a01035dacd78d439.
- Contract document:
  docs/src/project/phase7-close-syscall-contract.md.
- Deferred-surface list:
  dup/read syscalls, QEMU/Pi 5 close/dup/read proof, process loading,
  VFS/filesystem, stdin/read object model, shell, networking, SSH,
  blocking/readiness, signals, restart semantics, object finalization,
  RP1/PCIe, UART interrupt ownership, DMA/cache-driver policy, and full POSIX
  descriptor readiness remain blocked.
- Static inspection:
  git status --short before documentation edits was clean.
- Whitespace inspection:
  git diff --check passed.
- Documentation:
  mdbook build passed.

## Result

Accepted as the documentation-only close syscall contract. The next bounded
task should be phase7-close-syscall-core-20260529, scoped to target-independent
close syscall dispatch and focused tests before any QEMU or Pi 5 proof work.
