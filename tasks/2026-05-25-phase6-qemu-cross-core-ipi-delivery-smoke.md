# Phase 6 QEMU Cross-Core IPI Delivery Smoke

Task ID: phase6-qemu-cross-core-ipi-delivery-smoke-20260525
Status: accepted

## Goal

Prove raw GICv2 SGI delivery under QEMU before any scheduler remote-wakeup
implementation depends on IPIs.

## Scope

- Added the smallest GICv2 SGI send surface needed for a diagnostic:
  distributor enablement, CPU-interface enablement, SGI priority setup, and
  GICD_SGIR target-list writes.
- Started QEMU virt with four CPUs through the accepted PSCI secondary-core
  path.
- Sent one diagnostic SGI from logical CPU 0 to each secondary logical CPU and
  recorded sender, receiver, SGI INTID, target-list bit, acknowledgement, EOI,
  per-core counts, participant count, and errors.
- Kept IPI handler work bounded to acknowledge, classify, count, EOI, and
  return.
- Added `scripts/qemu-cross-core-ipi-delivery-smoke.sh`.

## Non-Goals

No scheduler wakeup, remote enqueue, shared run queue, task migration,
production secondary scheduling, Pi 5 hardware proof, hardware publish,
hardware lock, userspace, descriptors, filesystem, networking, SSH, shell
behavior, RP1/PCIe, UART interrupt ownership, or DMA behavior.

## Evidence

- Static inspection: `git status --short` before implementation showed a clean
  Talos worktree.
- QEMU/substitute: `scripts/qemu-cross-core-ipi-delivery-smoke.sh` wrote
  `target/qemu-cross-core-ipi-delivery-smoke.log` and reported
  `classification=qemu-cross-core-ipi-delivery-complete` and `PASS`.
- SGI mapping: selected SGI INTID 1; logical CPU 1 uses target-list bit 0x02
  and SGIR 0x00020001; logical CPU 2 uses bit 0x04 and SGIR 0x00040001;
  logical CPU 3 uses bit 0x08 and SGIR 0x00080001.
- QEMU/substitute: each secondary reported `receive-count=1`, `eoi-count=1`,
  observed `intid=1`, and `ok=true`; final participants were 3 of 3 with
  errors 0.
- Unit tests: `cargo -Zjson-target-spec test` passed 109 no_std tests.
- QEMU/substitute retained gates: `scripts/qemu-smoke.sh`,
  `scripts/qemu-secondary-core-workload-smoke.sh`, and
  `scripts/qemu-per-core-scheduler-ownership-smoke.sh` passed.
- Formatting/static checks: `cargo fmt --all -- --check` and
  `git diff --check` passed.
- Static inspection: `mdbook` is unavailable in the container; mdBook build
  was not run.

## Acceptance

Accepted as a QEMU-only raw SGI/IPI delivery proof. Scheduler remote wakeups,
remote enqueue state, shared run queues, task migration, Pi 5 raw SGI hardware
proof, and production IPI use remain deferred to separately planned tasks.
