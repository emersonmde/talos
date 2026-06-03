# Talos Scheduler, SMP, and Synchronization Review

Task: talos-review-scheduler-smp-sync-20260603
Status: accepted

## Scope

Reviewed scheduler data structures, per-core scheduler ownership, shared
scheduler metadata, shared run-queue migration, remote wake requests,
timer-preemption recording/service, secondary service loops, SMP per-core state,
spin locks, generic timer helpers, GICv2 helpers, and accepted Phase 6/QEMU
runtime evidence.

## Findings

- Fixed: SingleCoreScheduler::make_runnable changed the task state and
  transition counter before attempting to enqueue. A full runnable queue could
  leave a blocked task marked runnable without actually publishing it to the
  owner-local queue. The method now preflights capacity and preserves task state
  and counters on rejection.
- Fixed: voluntary_yield and timer_preempt rejected a full one-slot runnable
  queue even though the switch is valid: dequeue the selected next task, then
  requeue the current running task into the freed slot. Both paths now dequeue
  first, then requeue current, preserving FIFO order for larger queues and
  allowing capacity-1 schedulers to switch.
- Fixed: PerCoreState::snapshot read identity fields before acquiring the
  lifecycle publication value. Writers publish identity/stack/progress fields
  before the lifecycle release store, so snapshots now acquire lifecycle first
  before reading the associated fields.
- Not an issue: RemoteWakeQueue remains a bounded signal mailbox owned by the
  target CPU. It does not mutate another CPU's runnable queue and duplicate
  wake coalescing remains explicit.
- Not an issue: SharedRunQueue and SharedSchedulerMetadata remain separate
  locked boundaries because local runnable queues are still single-owner data.
  The review did not find a cross-owner queue mutation path.
- Not an issue: SecondaryProductionDiagnostic remains a deliberately named
  Phase 6 role for accepted secondary-core proof/service-loop evidence. It is
  not treated as general SMP process scheduling.
- Not an issue: GICv2 SGI helpers and generic timer helpers remain narrow
  low-level hardware adapters. No new interrupt routing, IPI policy, or timer
  feature was added in this review.
- Deferred: src/scheduler.rs is still large. A mechanical split during this
  task would risk creating arbitrary module boundaries before the Phase 8
  process/VFS/userspace integration establishes durable ownership cuts. The
  remaining risk is recorded for the full-system review cycles.

## Changes

- src/scheduler.rs now makes owner-local runnable publication all-or-nothing
  when the queue is full.
- src/scheduler.rs now allows voluntary yield and timer preemption to switch
  through a capacity-1 runnable queue by dequeuing the next task before
  requeuing the running task.
- src/scheduler.rs adds no_std regression coverage for full-queue
  make_runnable rejection and capacity-1 yield/preemption.
- src/smp.rs snapshots the lifecycle publication state before reading
  associated per-core identity, stack, and progress fields.

No new feature surface, hardware claim, userspace scheduling, process migration,
load balancing expansion, IPI policy, networking, RP1/PCIe, UART interrupt
ownership, or DMA/cache policy was added.

## Validation

- Static inspection: reviewed src/scheduler.rs, src/smp.rs, src/smp_sync.rs,
  src/arch/aarch64/generic_timer.rs, src/arch/aarch64/gicv2.rs,
  src/arch/aarch64/exceptions.rs IRQ dispatch surfaces, and relevant QEMU/RPi5
  scheduler/SMP smoke scripts with rg/sed.
- Dead-code/diagnostic inspection: rg reviewed scheduler/SMP/timer/GIC
  diagnostic, proof, cfg, unsafe, and dead-code surfaces. Retained diagnostic
  roles are explicitly Phase 6 regression/control surfaces.
- fmt: cargo fmt --all passed.
- unit tests: cargo -Zjson-target-spec test --quiet passed with the new
  scheduler regression coverage.
- QEMU/substitute smoke: ./scripts/qemu-scheduler-yield-smoke.sh passed with
  qemu-scheduler-yield-smoke: PASS.
- QEMU/substitute smoke: ./scripts/qemu-production-timer-preemption-smoke.sh
  passed with qemu-production-timer-preemption-smoke: PASS.
- QEMU/substitute smoke: ./scripts/qemu-secondary-scheduler-service-loop-smoke.sh
  passed with qemu-secondary-scheduler-service-loop: PASS.
- QEMU/substitute smoke: ./scripts/qemu-multicore-preemption-smoke.sh passed
  with qemu-multicore-preemption-smoke: PASS.
- fmt/lint/typecheck: cargo fmt --all -- --check passed; cargo
  -Zjson-target-spec check --quiet passed.
- unit tests rerun: cargo -Zjson-target-spec test --quiet passed after adding
  the task record.
- docs validation: /home/node/.cargo/bin/mdbook build passed after adding this
  task record.
- diff hygiene: git diff --check passed; git diff --cached --check pending
  before commit.
- hardwareTestLock remained unlocked/restored and unused; no hardware run was
  performed.

## Remaining Risks

- src/scheduler.rs should be split only when a real ownership boundary is clear,
  likely during or after the Phase 8 process/VFS/userspace integration path.
- RunnableQueue still does not reject duplicate TaskId values itself. Current
  call paths use higher-level duplicate checks where needed; a future process
  scheduler task should decide whether queue-local duplicate rejection becomes a
  base invariant.

Accepted commit: recorded in durable state for
talos-review-scheduler-smp-sync-20260603.
