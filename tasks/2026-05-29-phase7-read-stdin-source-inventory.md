# Phase 7 Read And Stdin Source Inventory

Task: phase7-read-stdin-source-inventory-20260529
Owner: worker
Status: accepted
Date: 2026-05-29
Milestone: Phase 7.4, File Descriptor Table

## Goal

Inventory the concrete source owners, accepted evidence, and missing policy for
the first read/stdin descriptor syscall contract after the accepted Pi 5 dup
syscall proof closeout.

## Scope

- Added `docs/src/project/phase7-read-stdin-source-inventory.md`.
- Mapped syscall dispatch, copy-out/user-memory checks, ProcessDescriptorStore
  lookup, inherited fd 0, DescriptorEntry/Object vocabulary, runtime-console0,
  TTY/stdin surfaces, and retained write/close/dup evidence.
- Listed read/stdin gaps for byte source, EOF, blocking/readiness, partial
  reads, nonblocking mode, restart/signals, copy-out failures, object
  lifetime/finalization, and physical proof.
- Recommended exactly one next bounded task:
  `phase7-read-stdin-contract-20260529`.

## Changed Files

- `docs/src/SUMMARY.md`
- `docs/src/decisions/README.md`
- `docs/src/project/phase7-read-stdin-source-inventory.md`
- `docs/src/roadmap.md`
- `tasks/2026-05-29-phase7-read-stdin-source-inventory.md`

## Evidence

- Accepted dependency:
  `phase7-pi5-dup-syscall-proof-closeout-checkpoint-20260529` at
  `56eb38a89cfcd81a330242c69491020532ee7169`.
- Inventory document:
  `docs/src/project/phase7-read-stdin-source-inventory.md`.
- Source-owner summary:
  the inventory names `src/syscall.rs`, `src/posix.rs`,
  `src/runtime_console.rs`, `src/tty.rs`, the accepted TTY/stdin architecture
  note, and retained descriptor evidence as current owners.
- Deferred-surface list:
  read implementation, QEMU/Pi 5 read proof, process loading, VFS/filesystem,
  shell, networking, SSH, object finalization, RP1/PCIe, UART interrupt
  ownership, DMA/cache-driver policy, and full POSIX descriptor readiness
  remain blocked.
- Static inspection:
  `git status --short` before edits was clean.
- Whitespace inspection:
  `git diff --check` passed.
- Documentation:
  `mdbook build` passed.
- Staged whitespace inspection:
  `git diff --cached --check` passed before commit.

## Deferred Work

Read/stdin contract, read implementation, QEMU read/stdin smoke plan/core,
Pi 5 read/stdin proof, process loading, VFS/filesystem behavior, local shell,
networking, SSH, object finalization, descriptor lifetime beyond accepted
write/close/dup behavior, blocking/readiness, signals, restart semantics,
RP1/PCIe, UART interrupt ownership, DMA/cache-driver policy, and full POSIX
descriptor readiness remain blocked.

## Result

Accepted as the documentation-only read/stdin source inventory. The next
bounded task should be `phase7-read-stdin-contract-20260529`, scoped to a
read/stdin contract before any implementation or runtime proof work.
