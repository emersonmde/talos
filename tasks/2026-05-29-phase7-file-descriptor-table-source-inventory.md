# Phase 7 File Descriptor Table Source Inventory

Task: phase7-file-descriptor-table-source-inventory-20260529
Owner: worker
Status: accepted
Date: 2026-05-29
Milestone: Phase 7.4, File Descriptor Table

## Goal

Inventory the source owners, accepted contracts, retained evidence, and
missing contract boundaries for Milestone 7.4 file descriptor table work after
the accepted Milestone 7.3 syscall ABI/dispatch closeout.

## Scope

- Mapped accepted descriptor table, runtime-console/TTY stdio, talos_write,
  copy helper, scheduler process-owner, syscall routing, and deferred VFS,
  pipe, socket, device, filesystem, shell, networking, and hardware surfaces.
- Added the documentation inventory at
  docs/src/project/phase7-file-descriptor-table-source-inventory.md.
- Updated roadmap, SUMMARY, and the decision log to reflect a
  documentation-only Milestone 7.4 inventory.
- Recommended one next bounded contract task:
  phase7-process-descriptor-table-contract-20260529.

## Non-Goals

- No Rust or assembly implementation changes.
- No QEMU run, Pi 5 hardware run, archive publication, or hardwareTestLock
  acquisition.
- No process loading, VFS/filesystem behavior, shell behavior, networking,
  SSH, RP1/PCIe, UART interrupt ownership, DMA/cache-driver policy, or full
  POSIX descriptor claim.

## Evidence

- static inspection: git status --short before documentation edits was clean.
- static source review: inspected src/posix.rs, src/syscall.rs,
  src/runtime_console.rs, src/scheduler.rs, accepted descriptor table and
  descriptor syscall documents, accepted Milestone 7.3 closeout, and retained
  QEMU/Pi 5 descriptor-write evidence references.
- static documentation diff: added
  docs/src/project/phase7-file-descriptor-table-source-inventory.md and this
  task record; updated docs/src/SUMMARY.md, docs/src/roadmap.md, and
  docs/src/decisions/README.md.
- whitespace inspection: git diff --check passed.
- documentation: mdbook build passed.
- Rust fmt/tests, QEMU runs, and Pi 5 hardware runs were not required because
  this task changes only Markdown documentation and durable worker state.

## Result

Accepted as the Milestone 7.4 file descriptor table source inventory. The next
bounded task should be phase7-process-descriptor-table-contract-20260529, a
documentation-only contract for process-owned descriptor table ownership,
inherited stdio installation, current-process lookup, first table operations,
error cases, and future QEMU/substitute evidence boundaries. Stdin/read,
close/dup syscalls, VFS/filesystem, path copying, process loading, shell,
networking, SSH, RP1/PCIe, UART interrupt ownership, DMA/cache-driver policy,
and full POSIX descriptor claims remain blocked.
