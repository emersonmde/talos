# Phase 7 Close, Dup, And Read Syscall Source Inventory

Task: phase7-close-dup-read-syscall-source-inventory-20260529
Owner: worker
Status: accepted
Date: 2026-05-29
Milestone: Phase 7.4, File Descriptor Table

## Goal

Inventory the current source owners, accepted evidence, and missing contracts
for close, dup, and read syscalls after the accepted descriptor close core
closeout.

## Scope

- Added `docs/src/project/phase7-close-dup-read-syscall-source-inventory.md`.
- Mapped syscall dispatch, lower-EL routing, copy helpers,
  `ProcessDescriptorStore`, `DescriptorTable`, descriptor entry/object
  vocabulary, runtime-console0, TTY, and stdin/read ownership.
- Separated proven descriptor-write/process-descriptor capabilities from
  unproven close/dup/read syscall behavior and physical claims.
- Recommended exactly one next bounded task:
  `phase7-close-syscall-contract-20260529`.

## Changed Files

- `docs/src/SUMMARY.md`
- `docs/src/decisions/README.md`
- `docs/src/project/phase7-close-dup-read-syscall-source-inventory.md`
- `docs/src/roadmap.md`
- `tasks/2026-05-29-phase7-close-dup-read-syscall-source-inventory.md`

## Evidence

- Accepted dependency:
  `phase7-descriptor-close-core-closeout-checkpoint-20260529` at
  `c537670fa9879257db403f260b4a3797f9fd829a`.
- Inventory document:
  `docs/src/project/phase7-close-dup-read-syscall-source-inventory.md`.
- Source/evidence/gap matrix:
  the inventory lists close, dup, and read source owners, accepted evidence,
  and missing syscall-proof gates.
- Deferred-surface list:
  dup/read syscalls, QEMU/Pi 5 close/dup/read proof, process loading,
  VFS/filesystem, shell, networking, SSH, object finalization, and full POSIX
  descriptor readiness remain blocked.
- Static inspection:
  `git status --short` before edits was clean.
- Whitespace inspection:
  `git diff --check` passed.
- Documentation:
  `mdbook build` passed.

## Deferred Work

Close syscall contract/implementation, dup syscall contract/implementation,
read syscall contract/implementation, QEMU close/dup/read smoke, Pi 5 physical
close/dup/read proof, process loading, VFS/filesystem, stdin/read object model,
shell, networking, SSH, blocking/readiness, signals, restart semantics,
RP1/PCIe, UART interrupt ownership, DMA/cache-driver policy, and full POSIX
descriptor readiness remain blocked.

## Result

Accepted as the documentation-only close/dup/read syscall source inventory.
The next bounded task should be `phase7-close-syscall-contract-20260529`,
scoped to a close-only syscall contract before any implementation or runtime
proof work.
