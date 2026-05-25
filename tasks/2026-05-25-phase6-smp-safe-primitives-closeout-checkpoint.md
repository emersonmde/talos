# Phase 6 SMP-Safe Primitives Closeout Checkpoint

Task: `phase6-smp-safe-primitives-closeout-checkpoint-20260524`

Status: accepted.

## Scope

Close Milestone 6.2 by reconciling the accepted SMP-safe primitive contract,
`SpinLock<T>` implementation, QEMU contention smoke, Pi 5 physical
cache/coherence proof, proof-scaffolding cleanup, retained validation surfaces,
remaining risks, and explicit deferrals.

This task changed documentation and durable state only. It did not change Rust
code, scripts, boot images, hardware state, scheduler behavior, shared run
queues, IPIs, userspace, filesystem, networking, SSH, shell behavior, RP1/PCIe,
or DMA/cache policy.

## Output

- Added
  `docs/src/project/phase6-smp-safe-primitives-closeout-checkpoint.md`.
- Added the checkpoint to `docs/src/SUMMARY.md`.
- Updated `docs/src/roadmap.md` to mark Milestone 6.2 closeout accepted and
  to name scheduler migration readiness as the next supervisor-planned source
  inventory, not an implementation shortcut.
- Updated `docs/src/architecture/scheduler.md` to state that the accepted
  lock proof does not make scheduler data structures SMP-safe.
- Added the final Pi 5 SMP lock cache/coherence proof decision to
  `docs/src/decisions/README.md`.

## Evidence

- Before edits: `git status --short` showed a clean Talos worktree.
- Static review: Phase 6.2 task records, accepted hardware evidence summaries,
  `docs/src/roadmap.md`, `docs/src/decisions/README.md`,
  `docs/src/architecture/scheduler.md`,
  `docs/src/project/phase6-smp-safe-primitives-source-inventory.md`, and
  `docs/src/project/phase6-secondary-cacheable-mmu-handoff-source-inventory.md`
  were inspected.
- Accepted commits reconciled: `6067f64`, `4290c36`, `895448b`,
  `dddb27e`, `a45cf92`, `79937bc`, `d8a2087`, `85f53c8`,
  `0a3b50f`, and `9a80fa8`.
- Hardware evidence reconciled:
  `tasks/evidence/2026-05-25-pi5-secondary-cacheable-mmu-handoff-proof/summary.md`
  and
  `tasks/evidence/2026-05-25-pi5-smp-lock-cache-coherence-final-proof/summary.md`.
- Historical/rejected evidence retained:
  `tasks/evidence/2026-05-24-pi5-smp-lock-cache-coherence-proof/summary.md`.

## Deferrals

The closeout explicitly defers scheduler migration, shared run queues,
per-core run queues, task migration, load balancing, cross-core wakeups, IPIs,
cross-core preemption, userspace, EL0, syscalls, descriptor tables, filesystem
behavior, program loading, libc/Rust std support, portable userland, local
shell behavior, runtime-console concurrency, UART interrupts, blocking I/O,
RP1/PCIe ownership, DMA, cache-coherent DMA driver policy, networking, SSH, and
Ethernet until later supervisor-planned tasks.

## Validation

- whitespace inspection: `git diff --check` passed.
- static inspection: `mdbook build` was not run because `mdbook` is
  unavailable in the container.
- Rust fmt/tests were not required because this task changed only Markdown
  documentation and durable task state.

## Acceptance

Accepted as the Milestone 6.2 SMP-safe primitives closeout. The generic
`SpinLock<T>` has QEMU substitute and serialized Pi 5 physical proof for the
bounded lock/cache-coherence diagnostic, and Milestone 6.3 or later scheduler
work must wait for a new supervisor-planned task.
