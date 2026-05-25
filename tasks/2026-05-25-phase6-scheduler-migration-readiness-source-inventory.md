# Phase 6 Scheduler Migration Readiness Source Inventory

Task: phase6-scheduler-migration-readiness-source-inventory-20260525

Status: accepted.

## Scope

Define the first Milestone 6.3 scheduler migration boundary after the accepted
Milestone 6.2 SMP-safe primitive closeout. This task changed documentation and
durable state only. It did not change Rust code, scripts, boot images, hardware
state, scheduler behavior, shared run queues, cross-core wakeups, IPIs,
userspace, filesystem, networking, SSH, shell behavior, RP1/PCIe, UART
interrupt ownership, or DMA/cache policy.

## Output

- Added docs/src/project/phase6-scheduler-migration-readiness-source-inventory.md.
- Added the checkpoint to docs/src/SUMMARY.md.
- Updated docs/src/architecture/scheduler.md with the accepted Phase 6.3
  readiness boundary.

## Evidence

- Before edits: git status --short showed a clean Talos worktree.
- Static review: inspected src/scheduler.rs, src/smp.rs, src/smp_sync.rs,
  src/arch/aarch64/mod.rs, src/target/qemu_virt.rs, src/target/rpi5.rs,
  docs/src/architecture/scheduler.md, Phase 4 and Phase 5 closeout docs,
  Phase 6.1 and 6.2 closeout docs, and the accepted Pi 5 SMP lock final
  evidence summary.
- Selected first slice: CPU-local scheduler ownership and local run queues
  first; CPU 0 remains the only production scheduler owner until secondary
  scheduler participation is explicitly planned.
- Explicit deferrals: shared run queues, global task lookup, task migration,
  load balancing, work stealing, cross-core wakeups, IPIs, remote reschedule,
  secondary-core production scheduling, per-core timer routing on secondary
  cores, cross-core preemption, sleep queues, wait queues, process address
  spaces, EL0, syscalls, descriptors, filesystem, networking, SSH, shell,
  runtime-console concurrency, UART interrupt ownership, RP1/PCIe, and
  DMA/cache-coherent driver policy.

## Validation

- whitespace inspection: git diff --check passed.
- static inspection: mdbook build was not run because mdbook is unavailable in
  the container.
- Rust fmt/tests were not required because this task changed only Markdown
  documentation and durable task state.

## Acceptance

Accepted as the Milestone 6.3 scheduler migration readiness checkpoint. The
next bounded implementation task remains
phase6-per-core-scheduler-state-core-20260525 and must not start shared queues,
task migration, IPIs, Phase 7, filesystem, networking, SSH, or shell work.
