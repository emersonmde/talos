# Phase 6 Multi-Core Preemption Closeout Checkpoint

Status: accepted checkpoint for the Phase 6.3 multi-core preemption slice.

## Scope

This checkpoint reconciles the accepted multi-core preemption source
inventory, contract, target-independent core, QEMU substitute proof,
serialized Pi 5 hardware proof, retained diagnostics, risks, and deferred
work. It is the closeout boundary before any supervisor-planned later
scheduler or Phase 7 work starts.

It does not add Rust implementation, boot scenarios, QEMU runs, hardware
runs, direct IRQ/IPI-context scheduling, remote current-task switching,
running-task migration, autonomous work stealing, general remote reschedule,
userspace, descriptors, filesystem, networking, SSH, shell behavior,
RP1/PCIe, UART interrupt ownership, or DMA/cache-driver policy.

## Accepted Work

The multi-core preemption source inventory is accepted in
docs/src/project/phase6-multicore-preemption-source-inventory.md. It maps the
timer IRQ, scheduler, CPU-local scheduler service, secondary service loop,
IPI/wake, metadata, SharedRunQueue, and load-balancing boundaries that the
preemption contract needed to preserve.

The contract is accepted in
docs/src/project/phase6-multicore-preemption-contract.md. It permits timer and
IPI paths to record bounded local state only, and requires scheduler mutation
to run from normal owner-local control flow after interrupt return. It also
names deterministic defer or reject outcomes for wrong-owner access, stale
metadata, current-task mismatch, nested or preemption-disabled sections,
pending remote wake, and full queues.

The target-independent core is accepted in
tasks/2026-05-27-phase6-multicore-preemption-core.md. It adds
PerCorePreemptionState and CpuLocalSchedulerService::run_preemption_cycle in
src/scheduler.rs. The core coalesces duplicate local timer-preemption records,
keeps pending state owner-local, explicitly defers service while preemption is
disabled, and preflights owner/current-task authority before wake draining,
timer preemption, optional local dispatch, and metadata refresh.

The QEMU substitute proof is accepted in
tasks/2026-05-27-phase6-qemu-multicore-preemption-smoke.md. The retained
scripts/qemu-multicore-preemption-smoke.sh gate builds the
qemu_multicore_preemption_smoke scenario and reports
classification=qemu-multicore-preemption-smoke-complete after logical CPUs 1,
2, and 3 each record local pending timer-preemption state without scheduler
mutation and then service it through owner-local normal scheduler control
flow.

The serialized Pi 5 proof is accepted in
tasks/2026-05-27-phase6-pi5-multicore-preemption-proof.md, with compact
evidence in
tasks/evidence/2026-05-27-pi5-multicore-preemption-proof/summary.md. The
proof uses hardware lock serialization, archive/kernel digest inspection,
fresh serial cursor evidence, TFTP fetch evidence, classification/PASS output,
participant counts, and restore proof. It reports
classification=pi5-multicore-preemption-complete, participants=3, expected=3,
errors=0, and PASS for the same owner-local invariant as QEMU.

## Product Boundary

The accepted boundary is a diagnostic multi-core preemption primitive:
multiple scheduler owners can record local timer-preemption intent in bounded
state, prove that record-only paths do not mutate scheduler queues or current
tasks, and later service the pending request from the owning CPU's normal
scheduler control flow.

This is not yet a production timer integration. Talos still does not wire
every real timer IRQ into this state, does not perform scheduler dispatch from
IRQ or IPI context, does not switch another CPU's current task, and does not
run autonomous cross-core scheduling policy.

## Retained Gates

The retained regression gates for this slice are:

- cargo -Zjson-target-spec test for target-independent scheduler unit tests.
- scripts/qemu-timer-preemption-smoke.sh for the earlier single-core timer
  preemption boundary.
- scripts/qemu-secondary-scheduler-service-loop-smoke.sh for owner-local
  scheduler service ordering on secondary owners.
- scripts/qemu-shared-runqueue-migration-smoke.sh and
  scripts/qemu-load-balancing-smoke.sh for the accepted owner-transfer and
  load-balancing surfaces that preemption must not bypass.
- scripts/qemu-multicore-preemption-smoke.sh for QEMU substitute evidence of
  the multi-core preemption invariant.
- scripts/rpi5-multicore-preemption-image.sh and
  scripts/rpi5-multicore-preemption-boot-tree.sh for reproducing the
  serialized Pi 5 proof when a future hardware task explicitly requires it.

The QEMU and Pi 5 multi-core preemption diagnostics remain proof surfaces, not
supported scheduler interfaces.

## Risks And Deferrals

Deferred work remains explicit:

- Production timer integration that records preemption requests from the
  normal timer IRQ path on each scheduler owner.
- Any interrupt-driven remote reschedule notification path.
- Work stealing, autonomous balancing cadence, fairness, priority, affinity,
  age, virtual runtime, and cache locality policy.
- Running-task migration, remote current-task switching, and asynchronous
  context capture.
- Non-diagnostic secondary runtime roles and broader production scheduler
  policy.
- Phase 7 userspace, syscalls, descriptors, filesystem, networking, SSH,
  shell behavior, RP1/PCIe, UART interrupt ownership, and DMA/cache-driver
  policy.
- Evidence archive cleanup for older large raw artifacts, which remains
  governed by the accepted evidence-retention policy and separate repository
  health work.

## Next Planning Boundary

This checkpoint does not start a new phase and does not choose a broad next
direction. The supervisor should create the next explicit bounded task before
any additional scheduler productionization or Phase 7 work proceeds. That task
must carry its own scope, dependencies, acceptance criteria, validation gates,
documentation requirements, and evidence requirements.

## Validation

- static inspection: reviewed accepted multi-core preemption task records,
  compact Pi 5 evidence summary, scheduler architecture, roadmap, and decision
  log.
- whitespace inspection: git diff --check passed.
- documentation: mdbook build passed.
