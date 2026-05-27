# Phase 6 Load-Balancing Closeout Checkpoint

Status: accepted checkpoint for the Phase 6.3 load-balancing slice.

## Scope

This checkpoint reconciles the accepted load-balancing source inventory,
policy contract, target-independent core, QEMU substitute proof, serialized
Pi 5 hardware proof, retained diagnostics, and deferred work before Talos
starts multi-core preemption, Phase 7, filesystem, networking, SSH, or shell
work.

It does not add Rust implementation, boot scenarios, QEMU runs, hardware
runs, autonomous work stealing, running-task migration, remote reschedule,
multi-core timer preemption, userspace, descriptors, filesystem, networking,
SSH, shell behavior, RP1/PCIe, UART interrupt ownership, or DMA/cache-driver
policy.

## Accepted Work

The load-balancing source inventory is accepted in
`docs/src/project/phase6-load-balancing-source-inventory.md`. It identifies
the scheduler, metadata, run-queue, wake, timer, SMP, and diagnostic surfaces
that are available before policy design, and it separates target selection,
fairness/affinity, remote reschedule notification, and owner-transfer
mechanics.

The policy contract is accepted in
`docs/src/project/phase6-load-balancing-policy-contract.md`. It permits only a
conservative, deterministic, polling-compatible policy that selects one
source-local front runnable task and one eligible destination, then uses
`SharedRunQueue` as the sole owner-transfer mechanism.

The target-independent core is accepted in
`tasks/2026-05-27-phase6-load-balancing-core.md`. It adds
`LoadBalancingPolicy`, `LoadBalancingPlan`,
`LoadBalancingPublishReport`, and `LoadBalancingPolicyError` in
`src/scheduler.rs`, with unit-tested success and deterministic rejection
paths. It does not make a QEMU or Pi 5 proof claim by itself.

The QEMU substitute proof is accepted in
`tasks/2026-05-27-phase6-qemu-load-balancing-smoke.md`. The retained
`scripts/qemu-load-balancing-smoke.sh` gate builds the
`qemu_load_balancing_smoke` scenario and reports
`classification=qemu-load-balancing-smoke-complete` and PASS after proving
front-runnable selection, source-local removal, shared handoff,
destination-local enqueue, and metadata owner/generation refresh.

The serialized Pi 5 proof is accepted in
`tasks/2026-05-27-phase6-pi5-load-balancing-proof.md`, with compact evidence
in `tasks/evidence/2026-05-27-pi5-load-balancing-proof/summary.md`. The proof
uses hardware lock serialization, archive/kernel digest inspection, fresh
serial cursor evidence, TFTP fetch evidence, classification/PASS output, and
restore proof. It reports `classification=pi5-load-balancing-complete` for the
same named invariant as QEMU.

## Product Boundary

The accepted productized boundary is a deterministic policy primitive over
already accepted scheduler surfaces. A source owner can plan and publish one
front-runnable, non-current, source-owned task through `SharedRunQueue`; the
destination owner can consume that shared entry locally and refresh metadata.

This is not a general load balancer yet. It has no autonomous balancing loop,
no fairness or affinity model, no work stealing, no running-task migration, no
interrupt-driven remote reschedule, and no multi-core preemption.

## Retained Gates

The retained regression gates for this slice are:

- `cargo -Zjson-target-spec test` for the target-independent scheduler unit
  tests.
- `scripts/qemu-shared-runqueue-migration-smoke.sh` for the owner-transfer
  mechanism that load balancing uses.
- `scripts/qemu-load-balancing-smoke.sh` for QEMU substitute evidence of the
  policy path.
- `scripts/rpi5-load-balancing-image.sh` and
  `scripts/rpi5-load-balancing-boot-tree.sh` for reproducing the serialized
  Pi 5 proof when a future hardware task explicitly requires it.

The QEMU and Pi 5 load-balancing diagnostics remain proof surfaces, not
production scheduler loops.

## Deferred Work

Deferred work remains explicit:

- Production multi-core preemption, including cross-core timer/preemption
  ownership and context-switch boundaries.
- Autonomous balancing cadence and any remote reschedule notification path.
- Work stealing, fairness, priority, affinity, age, virtual runtime, and cache
  locality policy.
- Running-task migration and asynchronous context capture.
- Phase 7 userspace, syscalls, descriptors, filesystem, networking, SSH,
  shell behavior, RP1/PCIe, UART interrupt ownership, and DMA/cache-driver
  policy.
- Evidence archive cleanup for older large raw artifacts, which remains
  governed by the accepted evidence-retention policy and separate blocked or
  queued repository-health tasks.

## Next Recommendation

The next bounded Phase 6.3 task should be a documentation/source-inventory
task for multi-core preemption. It should reconcile the accepted timer IRQ
path, context-switch contract, CPU-local scheduler service, secondary service
loop, remote wake ownership, shared metadata, shared run-queue migration, and
load-balancing boundaries before any preemption implementation starts.

The recommended durable task id is
`phase6-multicore-preemption-source-inventory-20260527`. The task should not
implement code or run hardware unless the supervisor creates a separate proof
task.

## Validation

- static inspection: reviewed accepted load-balancing task records, compact
  Pi 5 evidence summary, scheduler architecture, roadmap, and decision log.
- whitespace inspection: `git diff --check` passed.
- documentation: `mdbook build` passed.
