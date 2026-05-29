# Phase 7 Descriptor Lifetime And Close Source Inventory

Task: phase7-descriptor-lifetime-close-source-inventory-20260529
Owner: worker
Status: accepted
Date: 2026-05-29
Milestone: Phase 7.4, File Descriptor Table

## Goal

Inventory descriptor lifetime and close-semantics source, documentation, and
test boundaries before any close/dup/read syscall contract.

## Scope

- Mapped src/posix.rs descriptor table close/dup primitives,
  DescriptorEntry/DescriptorObject/DescriptorAccess vocabulary,
  ProcessDescriptorOwner and ProcessDescriptorStore owner-table mutation
  surfaces, inherited stdio lifetime, and owner teardown gaps.
- Identified accepted unit-test evidence and missing focused tests for close,
  double close, descriptor reuse, dup interaction, EBADF behavior, and future
  open-file-description reference counting.
- Added the documentation inventory at
  docs/src/project/phase7-descriptor-lifetime-close-source-inventory.md.
- Updated roadmap, SUMMARY, and the decision log.
- Recommended exactly one next bounded task:
  phase7-descriptor-lifetime-close-contract-20260529.

## Non-Goals

- No Rust or assembly implementation changes.
- No QEMU run, Pi 5 hardware run, archive publication, or hardwareTestLock
  acquisition.
- No close/dup/read syscall contract or implementation, process loading,
  VFS/filesystem, shell, networking, SSH, RP1/PCIe, UART interrupt ownership,
  DMA/cache-driver policy, physical descriptor lifetime claim, or full POSIX
  descriptor readiness claim.

## Evidence

- static inspection: git status --short before documentation edits was clean.
- static source review: inspected src/posix.rs, src/syscall.rs,
  src/runtime_console.rs, src/scheduler.rs, src/target/qemu_virt.rs,
  src/target/rpi5.rs, accepted Phase 7 descriptor/process-descriptor docs,
  task records, and retained QEMU/Pi 5 descriptor evidence references.
- static documentation diff: added
  docs/src/project/phase7-descriptor-lifetime-close-source-inventory.md and
  this task record; updated docs/src/SUMMARY.md, docs/src/roadmap.md, and
  docs/src/decisions/README.md.
- whitespace inspection: git diff --check passed.
- documentation: mdbook build passed.
- Rust fmt/tests, QEMU runs, and Pi 5 hardware runs were not required because
  this task changes only Markdown documentation and durable worker state.

## Result

Accepted as the Milestone 7.4 descriptor lifetime and close-semantics source
inventory. The next bounded task should be
phase7-descriptor-lifetime-close-contract-20260529, a documentation-only
contract for table-local close behavior, process-owned mutable lookup,
required close/double-close/reuse/dup unit evidence, and deferred
open-file-description finalization.

Close/dup/read syscalls, process loading, VFS/filesystem, path copying, shell,
networking, SSH, RP1/PCIe, UART interrupt ownership, DMA/cache-driver policy,
Pi 5 physical close/dup/read claims, and full POSIX descriptor readiness remain
blocked.
