# Phase 6 SMP-Safe Primitives Source Inventory

Task: `phase6-smp-safe-primitives-source-inventory-and-contract-20260524`

## Summary

This task accepts the Milestone 6.2 source inventory and synchronization
contract before Talos implements shared locks or scheduler data structures. It
is documentation-only and keeps scheduler migration, shared run queues,
cross-core wakeups, userspace, descriptors, filesystem, networking, SSH, shell
behavior, UART interrupts, RP1/PCIe, and DMA behavior deferred.

## Accepted Contract

- Local IRQ masking remains a boot-CPU interrupt-reentry primitive, not an SMP
  lock.
- SMP mutual exclusion must be a separate primitive backed by AArch64 atomics
  or exclusive operations.
- Lock acquisition/release must document memory ordering and keep blocking,
  printing, allocation, formatting, sleeps, and arbitrary callbacks out of
  held-lock regions.
- The accepted Pi 5 cache-maintenance lesson is carried forward: atomics and
  ordering do not replace explicit cache maintenance for early-boot sharing
  paths that are not yet covered by ordinary coherent-memory assumptions.
- Scheduler shared data structures, task migration, load balancing, IPIs, and
  cross-core wakeups remain outside this task.

## Evidence

- Static source inventory:
  `docs/src/project/phase6-smp-safe-primitives-source-inventory.md` names the
  inspected source files and accepted assumptions.
- Cache-maintenance lesson:
  the accepted Pi 5 PSCI alive proof required secondary `dc cvac`/`dsb sy`
  publication and primary `dc ivac`/`dsb sy` invalidation before snapshots.
- First implementation task:
  `phase6-spinlock-barrier-core-20260524`.

## Validation

- static inspection: `git status --short` was clean before documentation edits.
- fmt/lint/typecheck: `git diff --check` passed after documentation edits.
- static inspection: `mdbook` was unavailable in the container, so mdBook
  build was not run.
- Rust fmt/tests were not required because this task changed only Markdown
  documentation and durable task state.
