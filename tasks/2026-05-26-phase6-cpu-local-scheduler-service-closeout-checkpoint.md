# Phase 6 CPU-Local Scheduler Service Closeout Checkpoint

Task ID: phase6-cpu-local-scheduler-service-closeout-checkpoint-20260526
Status: accepted

## Goal

Checkpoint the accepted CPU-local scheduler service boundary and core before
secondary scheduler service-loop productionization or broader scheduler
topology work starts.

## Scope

- Reconciled the accepted CPU-local scheduler service boundary inventory and
  `CpuLocalSchedulerService` implementation.
- Recorded what is productized versus diagnostic-only around remote wake
  drains, timer-preemption handling, owner dispatch, secondary diagnostic
  roles, and metadata refresh.
- Preserved explicit deferrals for shared queues, migration, load balancing,
  multi-core preemption, Phase 7, filesystem, networking, SSH, shell,
  RP1/PCIe, UART interrupt ownership, and DMA/cache-driver policy.
- Named the next bounded Phase 6.3 task as secondary scheduler service-loop
  source inventory.

## Non-Goals

No Rust implementation, boot archive, QEMU proof, Pi 5 hardware run, shared
run queue, remote enqueue queue, task migration, load balancing, work stealing,
multi-core preemption, Phase 7, filesystem, networking, SSH, shell behavior,
RP1/PCIe, UART interrupt ownership, or DMA behavior was added.

## Evidence Reconciled

- Boundary inventory:
  docs/src/project/phase6-cpu-local-scheduler-service-boundary-source-inventory.md,
  accepted at commit 409884b.
- Core implementation:
  tasks/2026-05-26-phase6-cpu-local-scheduler-service-core.md, accepted at
  commit cb3bb40 with implementation in src/scheduler.rs.
- Architecture update: docs/src/architecture/scheduler.md records the accepted
  service order and `CpuLocalSchedulerService` result.
- Roadmap and decision log now record the closeout checkpoint and next bounded
  task recommendation.

The accepted invariant is a CPU-local normal-control-flow service for one
owning logical CPU: drain target-owned remote wakes, transition matching local
blocked tasks to runnable state, handle pending timer-preemption requests,
dispatch through the owner scheduler, and refresh owner-published metadata
after local mutations.

## Retained Gates

- cargo fmt --all -- --check.
- cargo -Zjson-target-spec test.
- scripts/qemu-smoke.sh.
- scripts/qemu-timer-preemption-smoke.sh.
- scripts/qemu-remote-wake-to-local-runnable-smoke.sh.
- scripts/qemu-production-secondary-dispatch-smoke.sh.
- scripts/qemu-shared-scheduler-metadata-smoke.sh.
- Physical Pi 5 scheduler proofs only under a later task with
  hardwareTestLock, artifact digests, TFTP evidence, cursor-valid serial,
  classification, and restore evidence.

## Deferred Work

Secondary scheduler service-loop productionization, shared run queues, remote
enqueue queues, task migration, load balancing, work stealing, remote
reschedule, multi-core preemption, Phase 7, filesystem, networking, SSH, shell
behavior, RP1/PCIe, UART interrupt ownership, and DMA/cache-coherent driver
policy remain deferred.

## Next Recommendation

The next bounded task should be
phase6-secondary-scheduler-service-loop-source-inventory-20260526. It should
inventory how secondaries leave the diagnostic production role and enter a
CPU-local scheduler service loop without creating shared run queues, migration,
load balancing, multi-core preemption, or later-roadmap behavior.

## Validation

- Static inspection: git status --short was clean before checkpoint edits.
- Static review: scheduler architecture docs, CPU-local service boundary
  inventory, accepted service-core task record, roadmap, decision log, and
  `CpuLocalSchedulerService` source/tests were reviewed.
- Whitespace inspection: git diff --check passed.
- Documentation: mdbook build passed.
- Rust fmt/tests, QEMU smoke reruns, and hardware runs were not required for
  this documentation checkpoint.

## Acceptance

Accepted as the Phase 6.3 CPU-local scheduler service closeout checkpoint.
Secondary service-loop work and broader scheduler topology require later
explicit tasks.
