# Phase 7 Syscall ABI Source Inventory

## Task

- Title: Phase 7 syscall ABI source inventory
- Owner: worker
- Date: 2026-05-29
- Milestone: Phase 7.3, Syscall ABI and Dispatch
- Scope: documentation-only source inventory before syscall ABI contract or
  implementation work

## Status

Accepted and committed as the Phase 7.3 syscall ABI source inventory.

## Work Performed

- Added docs/src/project/phase7-syscall-abi-source-inventory.md.
- Updated docs/src/roadmap.md to record the accepted Phase 7.3 source
  inventory frontier and keep the next task as a syscall ABI contract, not
  implementation.
- Updated docs/src/decisions/README.md with the inventory ADR.
- Updated docs/src/SUMMARY.md so the inventory is included in mdBook.

## Evidence

- static inspection: git status --short before edits was clean.
- static source review: inspected exception vector entry, Rust exception
  routing, QEMU and Pi 5 diagnostic EL0 trap proof surfaces, POSIX error and
  user-memory primitives, descriptor-table model, scheduler task/process-owner
  metadata, runtime-console, TTY, diagnostic command, roadmap, and decision
  log.
- static documentation diff summary: syscall ABI inventory doc added; roadmap
  current status and Phase 7.3 progress updated; decision log and mdBook
  summary updated; no Rust, assembly, boot-image, QEMU, hardware, syscall
  implementation, copy-in/copy-out helper, descriptor I/O, process loading,
  filesystem, shell, networking, SSH, RP1/PCIe, UART interrupt, or
  DMA/cache-driver behavior was changed.
- static inspection: git diff --check passed.
- documentation: mdbook build passed.

## Acceptance

Accepted; final commit hash is recorded in durable supervisor state for this
task after commit creation.
