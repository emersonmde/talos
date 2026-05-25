# Phase 6 Cross-Core Wakeup and IPI Source Inventory

Task: phase6-cross-core-wakeup-ipi-source-inventory-20260525

Status: accepted.

## Scope

Define the source-backed cross-core wakeup and IPI contract before any
implementation uses shared scheduler state across cores. This task changed
documentation and durable state only. It did not implement SGI delivery,
scheduler migration, shared run queues, remote wakeups, task migration,
hardware publish/run behavior, userspace, descriptors, filesystem, networking,
SSH, shell behavior, RP1/PCIe, UART interrupt ownership, or DMA policy.

## Output

- Added docs/src/project/phase6-cross-core-wakeup-ipi-source-inventory.md.
- Linked the checkpoint from docs/src/SUMMARY.md.
- Updated docs/src/architecture/interrupts-timers.md with the GICv2 SGI/IPI
  source inventory and handler constraints.
- Updated docs/src/architecture/scheduler.md with the accepted remote wakeup
  ownership boundary.
- Updated docs/src/roadmap.md and docs/src/decisions/README.md with the
  accepted source-inventory result.

## Evidence

- Before edits: git status --short showed a clean Talos worktree.
- Static review: inspected src/arch/aarch64/gicv2.rs,
  src/target/qemu_virt.rs, src/target/rpi5.rs, src/scheduler.rs,
  src/smp.rs, src/smp_sync.rs, docs/src/architecture/interrupts-timers.md,
  docs/src/architecture/scheduler.md,
  docs/src/project/phase6-scheduler-migration-readiness-source-inventory.md,
  tasks/2026-05-25-phase6-per-core-scheduler-state-core.md, and
  tasks/2026-05-25-phase6-qemu-per-core-scheduler-ownership-smoke.md.
- Source inventory: QEMU virt uses GICv2 at 0x0800_0000/0x0801_0000; Pi 5
  uses GIC-400/GICv2 at 0x10_7fff_9000/0x10_7fff_a000; the current GICv2
  wrapper lacks SGI generation; SGI delivery needs a minimal GICD_SGIR offset
  0xf00 surface and a proof of logical CPU to target-list bit mapping.
- Selected first proof strategy: split raw IPI delivery from scheduler wakeup.
  Run a QEMU SGI/IPI delivery smoke first, require a later serialized Pi 5
  proof before hardware scheduler wakeups, and defer remote enqueue/wake-list
  implementation until a separate task accepts ownership and lock ordering.
- Explicit deferrals: shared run queues, global task lookup, task migration,
  load balancing, work stealing, remote scheduler dispatch, sleep queues, wait
  queues, multi-core preemption, concurrent console output, userspace,
  descriptors, filesystem, networking, SSH, shell behavior, UART interrupts,
  RP1/PCIe, and DMA/cache-coherent driver policy.

## Validation

- whitespace inspection: git diff --check passed.
- static inspection: mdbook is unavailable in the container.
- Rust fmt/tests were not required because this task changed only Markdown
  documentation and durable task state.

## Acceptance

Accepted as the Milestone 6.3 cross-core wakeup/IPI source inventory. The next
bounded implementation task should be
phase6-qemu-cross-core-ipi-delivery-smoke-20260525, limited to QEMU SGI
delivery evidence without scheduler migration, remote wake queues, task
migration, Pi 5 hardware claims, userspace, descriptors, filesystem,
networking, SSH, or shell behavior.
