# Phase 6 Production Scheduler Runtime Closeout Checkpoint

Status: accepted checkpoint for the Phase 6.3 production scheduler runtime
timer/preemption slice.

## Scope

This checkpoint reconciles the accepted production scheduler runtime source
inventory, production timer/preemption contract, target-independent production
runtime core, focused QEMU substitute proof, serialized Pi 5 hardware proof,
retained diagnostics, risks, and deferred work. It is the closeout boundary
before any later scheduler productionization, Phase 7, filesystem, networking,
SSH, or shell work starts.

It does not add Rust implementation, boot scenarios, QEMU runs, hardware runs,
direct IRQ/IPI-context scheduling, remote current-task switching,
running-task migration, autonomous work stealing, general remote reschedule,
non-diagnostic secondary runtime roles, userspace, descriptors, filesystem,
networking, SSH, shell behavior, RP1/PCIe, UART interrupt ownership, or
DMA/cache-driver policy.

## Accepted Work

The production scheduler runtime source inventory is accepted in
`docs/src/project/phase6-production-scheduler-runtime-source-inventory.md`.
It maps the accepted diagnostic scheduler surfaces against the normal boot,
timer, IRQ, and owner-local runtime paths. It identifies the missing durable
per-CPU runtime boundary for scheduler state, preemption state, remote-wake
state, metadata access, current-task source, and role/capability.

The production timer/preemption contract is accepted in
`docs/src/project/phase6-production-timer-preemption-contract.md`. It permits
only bounded local timer-preemption recording in target IRQ handlers and
owner-local service through the accepted CPU-local scheduler service order.
IRQ/IPI context remains record-only and must not inspect queues, choose tasks,
consume remote wakes, refresh metadata, dispatch, print, allocate, block, or
take unbounded scheduler locks.

The first production timer/preemption core is accepted in
`tasks/2026-05-28-phase6-production-timer-preemption-core.md`. It adds
`ProductionSchedulerRuntime` in `src/scheduler.rs`, wires the normal QEMU and
Pi 5 timer IRQ handlers to record bounded local production preemption state
after the generic timer rearm helper and before EOI, and exposes
`ProductionSchedulerRuntime::service_pending_preemption` for owner-local
normal control flow.

The focused QEMU substitute proof is accepted in
`tasks/2026-05-28-phase6-qemu-production-timer-preemption-smoke.md`. The
retained `scripts/qemu-production-timer-preemption-smoke.sh` gate builds the
`qemu_production_timer_preemption_smoke` scenario and reports
`classification=qemu-production-timer-preemption-smoke-complete` after QEMU
logical CPUs 1, 2, and 3 each record through the target-owned production
timer IRQ adapter, prove record-only state does not mutate scheduler state,
and service pending preemption from owner-local normal control flow.

The serialized Pi 5 proof is accepted in
`tasks/2026-05-28-phase6-pi5-production-timer-preemption-proof.md`, with
compact evidence in
`tasks/evidence/2026-05-28-pi5-production-timer-preemption-proof/summary.md`.
The accepted local8 multi-observe run records fresh TFTP fetch evidence for
the 104,136-byte candidate kernel, archive and kernel SHA256 identity,
`participants=3 expected=3 errors=0`,
`classification=pi5-production-timer-preemption-complete`, PASS, and lab
restore proof.

## Product Boundary

The accepted boundary is the first production timer/preemption runtime
integration: normal target timer IRQ handlers may record local pending
preemption in durable per-CPU runtime state, and owner-local normal scheduler
control flow may service that pending state through
`ProductionSchedulerRuntime::service_pending_preemption`.

Scheduler mutation remains owner-local and outside IRQ/IPI context. The
service order remains target-owned remote wake consumption first, local timer
preemption second, optional local dispatch only when timer preemption did not
run, and metadata refresh last. Disabled preemption, stale metadata,
wrong-owner access, missing current task, current-task mismatch,
non-production-capable roles, and no-runnable-peer outcomes remain
deterministic defer/reject cases.

This is not a general multi-core scheduler loop. It does not accept remote
current-task switching, running-task migration, autonomous work stealing,
interrupt-driven remote reschedule, asynchronous exception-frame switching,
general non-diagnostic secondary runtime roles, userspace, descriptors,
filesystem, networking, SSH, or shell behavior.

## Retained Gates

The retained regression gates for this slice are:

- `cargo -Zjson-target-spec test` for target-independent scheduler unit tests.
- `scripts/qemu-smoke.sh` for the base QEMU boot path.
- `scripts/qemu-timer-preemption-smoke.sh` for the earlier single-core timer
  preemption boundary.
- `scripts/qemu-secondary-scheduler-service-loop-smoke.sh` for owner-local
  scheduler service ordering on secondary owners.
- `scripts/qemu-shared-runqueue-migration-smoke.sh` and
  `scripts/qemu-load-balancing-smoke.sh` for the accepted owner-transfer and
  load-balancing surfaces.
- `scripts/qemu-multicore-preemption-smoke.sh` for the diagnostic multi-core
  preemption invariant that production timer integration builds on.
- `scripts/qemu-production-timer-preemption-smoke.sh` for QEMU substitute
  evidence of the production timer IRQ adapter plus owner-local service path.
- `scripts/rpi5-production-timer-preemption-image.sh` and
  `scripts/rpi5-production-timer-preemption-boot-tree.sh` for reproducing the
  serialized Pi 5 proof when a future hardware task explicitly requires it.

The QEMU and Pi 5 production timer/preemption diagnostics remain proof
surfaces, not supported user interfaces or general-purpose scheduler entry
points.

## Risks And Deferrals

Deferred work remains explicit:

- Interrupt-driven remote reschedule notification and wakeup policy.
- Work stealing, autonomous balancing cadence, fairness, priority, affinity,
  age, virtual runtime, and cache locality policy.
- Running-task migration, remote current-task switching, and asynchronous
  context capture.
- General non-diagnostic secondary runtime roles and idle-loop policy.
- Phase 7 userspace, syscalls, descriptors, filesystem, networking, SSH,
  shell behavior, RP1/PCIe, UART interrupt ownership, and DMA/cache-driver
  policy.
- Evidence archive cleanup for older large raw artifacts, governed by the
  accepted evidence-retention policy and separate repository-health work.

## Next Planning Boundary

This checkpoint does not start a new phase and does not choose a broad next
direction. The supervisor should create the next explicit bounded task before
any additional scheduler productionization or Phase 7 work proceeds. That task
must carry its own scope, dependencies, acceptance criteria, validation gates,
documentation requirements, and evidence requirements.

## Validation

- static inspection: reviewed accepted production scheduler runtime inventory,
  contract, core, focused QEMU proof, serialized Pi 5 proof, compact Pi 5
  evidence summary, scheduler architecture, roadmap, and decision log.
- whitespace inspection: `git diff --check` passed.
- documentation: `mdbook build` passed.
