# Phase 6 Remote Wakeup Ownership Source Inventory

Task ID: phase6-remote-wakeup-ownership-source-inventory-20260525
Status: accepted

## Goal

Define remote wake-request ownership, lock ordering, and scheduler-facing IPI
rules before implementing any wakeup path.

## Scope

- Inventoried `PerCoreScheduler`, `TaskId`, local queue ownership,
  `SpinLock<T>`, IRQ masking, GICv2 SGI surfaces, and accepted QEMU/Pi 5 raw
  IPI evidence.
- Selected a bounded per-target remote wake-request list as the first model.
- Defined ownership, lock ordering, memory ordering, IPI hot-path constraints,
  duplicate wake behavior, and error handling.
- Named the next implementation proof without starting it.

## Non-Goals

No remote wakeup implementation, shared run queue, task migration, production
secondary scheduler dispatch, hardware run, boot archive, Phase 7/userspace,
descriptors, filesystem, networking, SSH, shell behavior, RP1/PCIe, UART
interrupt ownership, or DMA behavior.

## Evidence

- Static inspection: `git status --short` before edits showed a clean Talos
  worktree.
- Static source review: `src/scheduler.rs`, `src/smp_sync.rs`, `src/smp.rs`,
  and `src/arch/aarch64/gicv2.rs`.
- Static documentation review: `docs/src/architecture/scheduler.md`,
  `docs/src/architecture/interrupts-timers.md`,
  `docs/src/project/phase6-cross-core-wakeup-ipi-source-inventory.md`,
  `tasks/2026-05-25-phase6-qemu-cross-core-ipi-delivery-smoke.md`,
  `tasks/2026-05-25-phase6-pi5-cross-core-ipi-delivery-proof.md`, and
  `tasks/evidence/2026-05-25-pi5-cross-core-ipi-delivery-proof/summary.md`.
- Selected model: bounded per-target remote wake-request list, with target CPU
  ownership of consumption and no direct remote runnable-queue mutation.
- Next implementation proof:
  `phase6-qemu-remote-wakeup-request-smoke-20260525`.
- Validation: `git diff --check` passed.
- Static inspection: `mdbook` is unavailable in the container; mdBook build was
  not run.

## Acceptance

Accepted as the remote wake-request ownership inventory for the next QEMU
implementation proof. The accepted model preserves CPU-local scheduler
ownership: a remote sender may publish a bounded request and send an IPI, but
only the target CPU may consume that request and later decide local scheduler
effects. Production secondary dispatch, direct remote enqueue, shared run
queues, task migration, Pi 5 scheduler wakeups, and broader scheduler migration
remain deferred.
