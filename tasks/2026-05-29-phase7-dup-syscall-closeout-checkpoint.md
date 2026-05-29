# Phase 7 Dup Syscall Closeout Checkpoint

Task: phase7-dup-syscall-closeout-checkpoint-20260529
Owner: worker
Status: accepted
Date: 2026-05-29
Milestone: Phase 7.4, File Descriptor Table

## Goal

Close out the accepted dup syscall contract, target-independent core, and
QEMU/substitute smoke evidence before any Pi 5 dup proof or read work.

## Scope

- Added docs/src/project/phase7-dup-syscall-closeout-checkpoint.md.
- Linked the closeout from docs/src/SUMMARY.md.
- Updated docs/src/roadmap.md and docs/src/decisions/README.md with the
  accepted QEMU/substitute dup syscall frontier and blocked surfaces.
- Preserved the retained QEMU evidence path:
  tasks/evidence/2026-05-29-qemu-dup-syscall-smoke-core/qemu-dup-syscall-smoke.log.

## Non-Goals

- No Rust or assembly behavior changes.
- No QEMU rerun, Pi 5 hardware run, boot archive publication, or
  hardwareTestLock acquisition.
- No read syscall behavior, process loading, VFS/filesystem, shell,
  networking, SSH, object finalization, RP1/PCIe, UART interrupt ownership,
  DMA/cache-driver policy, dup2/fcntl, or full POSIX descriptor readiness
  claim.

## Accepted Evidence Matrix

| Task | Commit | Evidence level |
| --- | --- | --- |
| phase7-dup-syscall-contract-20260529 | 041ca2f449afc9bd7889497720702b4f4f849bc3 | static documentation/source inspection |
| phase7-dup-syscall-core-20260529 | 2c30e4446f6611edb2bea1b75f226a6e919bf310 | fmt/unit tests/QEMU regression gates |
| phase7-qemu-dup-syscall-smoke-plan-20260529 | 37401fb7d9ff4924acd8a9ed072db1ec3441b261 | static documentation/source inspection |
| phase7-qemu-dup-syscall-smoke-core-20260529 | 5cce637bab95b227f5a98aba99b9104d2a017751 | QEMU/substitute serial evidence |

## Accepted Capability

The accepted dup syscall capability is limited to stable talos_dup x8 = 3
through the current ProcessOwnerId-backed ProcessDescriptorStore. The retained
QEMU/substitute smoke proves fd 1 duplicates to fd 3, full-table -EMFILE,
reserved-register -EINVAL without mutation, writes through both source and
duplicate stdout descriptors, close(fd 1) preserving fd 3, closed-descriptor
-EBADF cases, talos_nop and unknown-syscall regressions, copy-probe
quarantine, and diagnostic marker quarantine.

## Deferred Work

Pi 5 physical dup proof, read syscall behavior, stdin/read object model,
process loading, VFS/filesystem, shell, networking, SSH,
open-file-description reference counting, object finalization, blocking and
readiness, signals, restart semantics, RP1/PCIe, UART interrupt ownership,
DMA/cache-driver policy, dup2/fcntl, and full POSIX descriptor readiness
remain blocked.

## Recommended Next Task

The next bounded Milestone 7.4 task should be the already queued
documentation-only Pi 5 dup syscall proof plan,
phase7-pi5-dup-syscall-proof-plan-20260529. That task should not acquire
hardwareTestLock or run hardware; it should define the later serialized
physical proof before any Pi 5 dup action.

## Validation

- static inspection: git status --short before edits was clean.
- static evidence review: reviewed accepted dup contract, core task record,
  QEMU smoke plan, QEMU dup smoke task record, retained QEMU evidence path,
  validation gates, and deferred surfaces.
- whitespace inspection: git diff --check passed.
- documentation: mdbook build passed.
- staged whitespace inspection: git diff --cached --check passed before
  commit.
- Rust fmt/tests, QEMU runs, and Pi 5 hardware runs were not required because
  this checkpoint changes only Markdown documentation and durable worker
  state.

## Result

Accepted as the documentation-only dup syscall closeout checkpoint.
