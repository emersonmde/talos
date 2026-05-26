# Phase 6 CPU-Local Scheduler Service Closeout Checkpoint

Status: accepted as the Phase 6.3 CPU-local scheduler service closeout
checkpoint.

This checkpoint reconciles the accepted CPU-local scheduler service boundary
inventory and implementation before Talos starts secondary scheduler
service-loop productionization, shared scheduler topology, shared run queues,
task migration, load balancing, multi-core preemption, Phase 7, filesystem,
networking, SSH, or shell work.

## Accepted Work

- CPU-local scheduler service boundary source inventory: commit 409884b;
  project page
  docs/src/project/phase6-cpu-local-scheduler-service-boundary-source-inventory.md.
- CPU-local scheduler service core: commit cb3bb40; task record
  tasks/2026-05-26-phase6-cpu-local-scheduler-service-core.md; implementation
  in src/scheduler.rs.

## Evidence Reconciliation

The accepted boundary inventory ordered existing Phase 6.3 diagnostic slices
into one normal-control-flow service: establish the owning logical CPU, drain
target-owned remote wakes outside IPI context, convert matching local blocked
tasks to runnable state, handle pending timer-preemption requests, dispatch
only through the owner scheduler, refresh owner-published metadata, and return
without holding scheduler locks across context switching, printing, UART
polling, diagnostic command dispatch, allocation, blocking, sleeping,
migration, or arbitrary callbacks.

The implementation adds `CpuLocalSchedulerService::run_cycle` in
src/scheduler.rs. The service consumes one target-owned remote wake request,
applies the matching local wake transition, handles an optional pending
timer-preemption request, dispatches through the owner `PerCoreScheduler` when
timer preemption did not already select the next task, and refreshes
owner-published metadata after local mutations.

The accepted unit tests cover the service order and explicit failure
boundaries:

- `cpu_local_scheduler_service_drains_wakes_dispatches_and_refreshes_metadata`
  proves remote-wake drain, local runnable transition, owner dispatch, and
  metadata refresh.
- `cpu_local_scheduler_service_handles_timer_preemption_before_metadata_refresh`
  proves a just-woken local task participates in pending timer preemption
  before metadata refresh.
- `cpu_local_scheduler_service_preserves_explicit_error_boundaries` preserves
  deferred secondary role, unknown metadata, duplicate local runnable, and
  no-runnable dispatch outcomes.

The task record also preserves the existing wrong-owner, wrong-target,
non-blocked task, stale metadata, and remote-wake ownership boundaries. The
core is target-independent and does not create a shared scheduler topology,
remote enqueue authority, task migration path, load balancer, or multi-core
preemption policy.

## Productized Versus Diagnostic

Productized by this slice:

- a target-independent CPU-local service order for one owning logical CPU;
- target-owned remote wake consumption into local scheduler rules;
- normal-control-flow handling of pending local timer-preemption requests;
- owner-only CPU-local dispatch through `PerCoreScheduler`;
- owner-published scheduler metadata refresh after local state mutations;
- explicit error reporting for wake, dispatch, timer, and metadata failures.

Still diagnostic-only or retained as validation surface:

- `SchedulerCoreRole::SecondaryProductionDiagnostic` remains the only
  accepted secondary production role;
- QEMU and Pi 5 remote-wake, production secondary dispatch, and shared
  metadata proofs remain retained gates for their bounded invariants;
- the existing local diagnostic tasks and smoke scripts are validation
  surfaces, not a general multi-core runtime or user-facing command model.

## Retained Gates

Retained regression gates for this slice:

- cargo fmt --all -- --check;
- cargo -Zjson-target-spec test with the documented QEMU 9.2.0 path;
- scripts/qemu-smoke.sh for broad QEMU boot coverage;
- scripts/qemu-timer-preemption-smoke.sh for local timer-preemption request
  and dispatch evidence;
- scripts/qemu-remote-wake-to-local-runnable-smoke.sh for target-owned remote
  wake drains and local blocked-to-runnable transitions;
- scripts/qemu-production-secondary-dispatch-smoke.sh for CPU-local secondary
  diagnostic dispatch;
- scripts/qemu-shared-scheduler-metadata-smoke.sh for owner-published metadata
  refresh and lookup invariants;
- physical Pi 5 remote wake, production secondary dispatch, and shared
  metadata proof scripts only when a later task makes a physical scheduler
  claim under `hardwareTestLock`.

## Deferred Work

The following remain explicitly deferred to later supervisor-planned tasks:

- secondary scheduler service-loop productionization beyond the accepted
  diagnostic secondary role;
- shared run queues, remote enqueue queues, task migration, load balancing,
  work stealing, remote reschedule, and multi-core preemption;
- secondary timer-preemption policy, sleep queues, wait queues, blocking I/O
  readiness, and production task movement beyond explicitly seeded CPU-local
  diagnostics;
- global mutable task registry authority beyond the owner-published metadata
  table;
- runtime-console concurrency, UART interrupts, descriptor-facing TTY
  behavior, userspace, EL0, syscalls, descriptor tables, file descriptors,
  user/kernel copy policy, process address spaces, and process lifetime rules;
- filesystem behavior, program loading, libc/Rust std support, portable
  userland, and local shell behavior;
- RP1/PCIe ownership, DMA, cache-coherent DMA driver policy, networking, SSH,
  Ethernet, and shell access.

## Remaining Risks

The accepted CPU-local service orders existing local pieces, but it still does
not answer how secondary cores should run the service as a durable loop, how
service wakeups are scheduled outside diagnostics, or how owner-local service
cycles should be triggered in a broader runtime.

The service handles one local task and one consumed remote wake request per
cycle. Broader batching, starvation policy, fairness, sleeping/wait queues,
and task movement remain future scheduler design work.

The current evidence is static/unit/QEMU substitute evidence. No new Pi 5
hardware claim was made by the service core or this checkpoint.

## Readiness Decision

Talos is ready for one bounded Phase 6.3 secondary scheduler service-loop
source inventory. That task should inventory how secondaries leave the
diagnostic production role and enter a CPU-local scheduler service loop without
creating shared run queues, migration, load balancing, multi-core preemption,
Phase 7, filesystem, networking, SSH, or shell behavior.

Talos is not ready for shared scheduler topology, shared run queues, task
migration, load balancing, multi-core preemption, Phase 7, filesystem,
networking, SSH, or shell work solely from this checkpoint.

## Validation

- static inspection: git status --short was clean before checkpoint edits.
- static review: inspected docs/src/architecture/scheduler.md,
  docs/src/project/phase6-cpu-local-scheduler-service-boundary-source-inventory.md,
  tasks/2026-05-26-phase6-cpu-local-scheduler-service-core.md,
  docs/src/roadmap.md, docs/src/decisions/README.md, and the
  `CpuLocalSchedulerService` implementation and tests in src/scheduler.rs.
- whitespace inspection: git diff --check passed after checkpoint edits.
- documentation: mdbook build passed.
- Rust fmt/tests, QEMU smoke reruns, and hardware runs were not required
  because this checkpoint changes only Markdown documentation and durable task
  state, and it references the accepted service-core evidence.
