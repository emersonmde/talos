# Phase 6 Scheduler Migration Slice Checkpoint

Task: `phase6-scheduler-migration-slice-checkpoint-20260525`

Status: accepted.

## Scope

Close the first Milestone 6.3 scheduler-migration slice by reconciling
readiness, per-core scheduler state, QEMU ownership evidence, IPI/wakeup
inventory, retained gates, deferrals, and next risks.

This task changed documentation and durable state only. It did not implement
Rust code, scripts, boot images, hardware state, scheduler migration, shared
run queues, cross-core wakeups, IPIs, Phase 7 behavior, filesystem behavior,
networking, SSH, shell behavior, RP1/PCIe, UART interrupt ownership, or DMA
policy.

## Output

- Added
  `docs/src/project/phase6-scheduler-migration-slice-checkpoint.md`.
- Added the checkpoint to `docs/src/SUMMARY.md`.
- Updated `docs/src/roadmap.md` with the accepted first Milestone 6.3 slice
  status.
- Added a decision-log entry for the first Milestone 6.3 scheduler-migration
  slice closeout.

## Evidence

- Before edits: `git status --short` showed a clean Talos worktree.
- Static review: inspected first-slice task records, scheduler and
  interrupt/timer architecture docs, roadmap, decision log, the accepted QEMU
  per-core scheduler ownership transcript, and accepted hardware evidence
  summaries.
- Accepted commits reconciled: `b75de5d`, `9decc46`, `33400ed`, and
  `e92ff9d`.
- Accepted QEMU/substitute evidence:
  `target/qemu-per-core-scheduler-ownership-smoke.log` reported
  `participants=4 expected=4 errors=0 lock-available=true irq-ok=true`,
  `classification=qemu-per-core-scheduler-ownership-complete`, and `PASS`.
- Accepted hardware background:
  `tasks/evidence/2026-05-25-pi5-smp-lock-cache-coherence-final-proof/summary.md`
  remains the physical proof for `SpinLock<T>`, not scheduler wakeups or
  IPI routing.
- Next bounded recommendation:
  `phase6-qemu-cross-core-ipi-delivery-smoke-20260525`.

## Deferrals

The checkpoint explicitly defers shared run queues, global task lookup, remote
enqueue queues, wake lists, task migration, load balancing, work stealing,
remote reschedule, secondary-core production scheduling, raw Pi 5 SGI/IPI
proof, production scheduler wakeups, multi-core preemption, userspace, EL0,
syscalls, descriptor tables, filesystem behavior, program loading, libc/Rust
std support, portable userland, local shell behavior, runtime-console
concurrency, UART interrupts, blocking I/O, RP1/PCIe, DMA, cache-coherent DMA
driver policy, networking, SSH, and Ethernet.

## Validation

- whitespace inspection: `git diff --check` passed.
- static inspection: `mdbook` is unavailable in the container, so mdBook
  build was not run.
- Rust fmt/tests were not required because this task changed only Markdown
  documentation and durable task state.

## Acceptance

Accepted as the first Milestone 6.3 scheduler-migration slice checkpoint.
Talos is ready for the bounded QEMU raw IPI delivery smoke only; Pi 5 raw IPI
proof and scheduler wakeup implementation require later supervisor-planned
tasks.
