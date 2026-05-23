# Phase 3 Closeout Checkpoint

Status: accepted on 2026-05-23.

This checkpoint reconciles the accepted Phase 3 memory, MMU, and kernel-runtime
work before Talos starts Phase 4 interrupt, timer, and preemption tasks. It is a
planning and documentation boundary only; it introduces no new kernel code,
boot image, normal Pi 5 output, or hardware-facing behavior.

## Accepted Capabilities

| Capability | Evidence level | Commit |
| --- | --- | --- |
| Phase 3 runtime inventory names accepted Pi 5 memory/MMU/runtime reports and the remaining backlog. | static inspection, decision log | bf20f57 |
| Current page-frame ownership partitions are explicit: bootstrap-reserved, translation-table pages, reserved-unused, bootstrap-bump-owned, and outside-low-tail deferred. | no-std unit tests, QEMU smoke, Pi 5 image/archive inspection | 002dc25 |
| Entry, boot, retained diagnostics, memory-map, and FDT responsibilities are split out of the former catch-all files. | static inspection, no-std unit tests, QEMU smoke, Pi 5 image/archive inspection | b3ea5a6, 7b17870, aee4f93 |
| Stale Pi 5 bring-up diagnostics, cfg plumbing, and proof wrappers are deleted or documented, leaving current boot/format gates and retained allocator, panic, exception/fault, and translation-fault diagnostics. | static inspection, no-std unit tests, QEMU smoke, Pi 5 image/archive inspection, representative retained diagnostic image builds | 47b9e85 |
| A bounded page-frame free/reuse diagnostic exists over an explicit tracked window inside the accepted allocator-owned low-tail span, with caller-owned metadata outside managed frames. | no-std unit tests, QEMU smoke, Pi 5 image/archive inspection | d6c841d |
| Recoverable direct allocation failure and heap-expansion source policy are explicit while the global heap remains the accepted no-free bump allocator. | no-std unit tests, QEMU smoke, Pi 5 image/archive inspection | 4fa6ca9 |
| High-memory, DMA, and cache ownership boundaries are documented: current allocation is limited to 0x2f010000..0x3fc00000, and cache enablement is not a DMA coherency contract. | docs-only static inspection, diff check | ddabf65 |
| Lower-EL and userspace mapping readiness is documented: the current EL2 identity map is a kernel bring-up map, not a userspace isolation contract. | docs-only static inspection, diff check | 04c2d4d |

The latest accepted Pi 5 hardware boundary for the normal path remains the
post-allocator translation-table population println! evidence from
pi5-translation-population-post-println-20260523: TFTP served the 82,045-byte
candidate, and serial captured the post-allocator population line after the
accepted slot line and before the post-allocator memory/page-frame reports.

## Deferred Work

These items are intentionally not part of the accepted Phase 3 closeout:

- Dynamic page-frame-backed heap growth and general-purpose heap free/reuse:
  deferred until a concrete kernel or user-process pressure point needs it. The
  accepted artifacts are policy and diagnostic boundaries, not a new global heap
  implementation.
- High-memory allocation from firmware banks 0x40000000..0x100000000 and
  0x100000000..0x200000000: deferred until explicit mapping, reservation,
  ownership, and validation work exists.
- DMA-safe allocation, RP1/PCIe addressability, dma-ranges/IOMMU policy,
  driver buffer cache maintenance, and cacheable/non-cacheable DMA mapping:
  deferred to the RP1/PCIe/DMA driver era, currently Phase 11.
- Lower-EL entry, process address spaces, syscall ABI, file descriptors,
  copy-in/copy-out, and invalid-user-memory handling: deferred to the
  POSIX/EL0/syscall and later filesystem/userland phases, currently Phase 7 and
  beyond.
- SMP memory ownership, per-core scheduler state, and cross-core allocation
  pressure: deferred until Phase 6.
- Networking and SSH: deferred until local console, process, filesystem, and
  userland foundations exist; currently Phase 12.

## Recommendation

Phase 3 is closed enough to plan Phase 4. The next supervisor action should be
to create a bounded Phase 4 task queue for interrupt-controller discovery and
timer interrupt bring-up, using the accepted EL2 kernel map and low-tail
allocation boundary only.

Phase 4 tasks must not depend on high-memory allocation, DMA-safe buffers,
lower-EL isolation, process address spaces, or a free/reuse global heap unless a
new task explicitly designs and validates that dependency first.

## Validation

Checkpoint validation was documentation-only:

- git status --short was inspected before the checkpoint edits and showed no
  tracked Talos changes.
- git diff --check passed after the checkpoint edits.
- mdbook build was skipped because mdbook is not installed in the current
  container.
- No Rust fmt/tests, QEMU run, Pi 5 image build, or hardware run was required
  because this checkpoint changed only documentation and durable task state.
