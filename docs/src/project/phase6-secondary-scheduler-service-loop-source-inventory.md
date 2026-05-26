# Phase 6 Secondary Scheduler Service Loop Source Inventory

Status: accepted as a Phase 6.3 source inventory and contract for the
secondary scheduler service-loop boundary. No Rust implementation, boot image,
hardware run, shared run queue, remote enqueue queue, task migration, load
balancing, work stealing, multi-core preemption, Phase 7, filesystem,
networking, SSH, shell, RP1/PCIe, UART interrupt ownership, or DMA/cache-driver
policy is added by this inventory.

This inventory defines the next CPU-local productionization boundary after the
accepted `CpuLocalSchedulerService` core: secondary CPUs may enter a normal
service loop from accepted secondary handoff state, but local scheduler
mutation remains owner-only and the existing proof entry points remain
diagnostic gates.

## Source Inventory

- `src/smp.rs` owns secondary logical CPU identity, stack publication,
  `CoreLifecycle`, and `HandoffReady`. It proves that secondary cores can reach
  normal Rust control flow with a valid stack and identity, but it does not own
  scheduler queues, task dispatch, IPIs, migration, or load balancing.
- `src/scheduler.rs` owns `PerCoreScheduler`, `SchedulerCoreRole`,
  `RemoteWakeQueue`, `SharedSchedulerMetadata`, and
  `CpuLocalSchedulerService::run_cycle`. The service can sequence one owning
  CPU's remote wake drain, local runnable transition, pending timer-preemption
  request, CPU-local dispatch, and metadata refresh.
- `SchedulerCoreRole::SecondaryProductionDiagnostic` is still the only
  secondary role that passes production dispatch. `SecondaryDeferred` remains
  the default rejection boundary for secondary production dispatch.
- `src/smp_sync.rs` owns the accepted `SpinLock<T>`, IRQ-save composition, and
  `smp_full_barrier()` primitives. These primitives may protect named shared
  metadata or wake-request boundaries, but they do not make local runnable
  queues shared.
- Retained QEMU proof scripts remain the validation surface for the pieces the
  loop will order: `scripts/qemu-secondary-core-workload-smoke.sh`,
  `scripts/qemu-remote-wake-to-local-runnable-smoke.sh`,
  `scripts/qemu-production-secondary-dispatch-smoke.sh`,
  `scripts/qemu-shared-scheduler-metadata-smoke.sh`, and the broad
  `scripts/qemu-smoke.sh` boot gate.
- Retained Pi 5 proof scripts remain physical validation gates when a later
  task makes a hardware claim: secondary workload, remote wake to local
  runnable, production secondary dispatch, and shared scheduler metadata image
  and boot-tree scripts.

## Service-Loop Boundary

The secondary service loop should begin only after accepted secondary handoff
state is established: logical CPU identity is known, the secondary stack is
active, per-core state is published, cacheable-MMU handoff is already accepted
for Pi 5 proof paths, and the secondary is in normal kernel control flow rather
than asynchronous exception context.

Within one loop iteration, the secondary CPU should:

1. identify itself as the owning `LogicalCpuId` and operate only on its local
   scheduler, local diagnostic task table, target-owned remote wake queue, and
   owner-published metadata;
2. observe bounded pending work recorded by IPI or timer paths, without letting
   those interrupt paths run the scheduler;
3. call the CPU-local scheduler service from normal control flow to consume at
   most the accepted local work for that cycle;
4. dispatch only through the owner `PerCoreScheduler` and the accepted
   secondary diagnostic production role until a later task replaces the
   diagnostic role with a general runtime role;
5. publish refreshed owner metadata after local state changes;
6. return to the loop or an explicit idle/wait point without holding scheduler
   locks across context switch, printing, UART polling, diagnostic command
   dispatch, allocation, blocking, sleeping, migration, or arbitrary callbacks.

The loop is not a shared scheduler topology. It is a normal-control-flow owner
for already accepted CPU-local service behavior on secondary CPUs.

## Interrupt And Normal-Control Split

IPI context remains bounded to acknowledge, classify, record local pending
state, EOI, and return. It may cause the owning secondary's normal loop to run
the service later, but it must not drain unbounded queues, mutate a runnable
queue, dispatch a task, refresh global metadata, allocate, format output, poll
UART input, block, sleep, migrate work, or cross a context switch.

Timer IRQ context remains the local preemption request boundary. It may record
that the owner CPU should consider a timer-preemption handoff, but the actual
scheduler mutation and task selection stay in the normal service-loop cycle.

The normal service loop is therefore the only place this slice may combine
remote wake consumption, local blocked-to-runnable transitions, timer
preemption, dispatch, and metadata refresh for a secondary owner.

## Diagnostic Versus Runtime Boundary

Retained as diagnostic-only proof entry points:

- direct secondary workload loops used to prove bring-up and cache-visible
  state publication;
- focused QEMU and Pi 5 production secondary dispatch proofs that seed
  diagnostic tasks and run bounded dispatch counters;
- focused remote-wake and shared-metadata proof images and scripts;
- `SchedulerCoreRole::SecondaryProductionDiagnostic` as the explicit accepted
  secondary dispatch role.

Belongs behind the production service-loop boundary:

- a named secondary loop that starts from accepted handoff state;
- owner-only invocation of `CpuLocalSchedulerService::run_cycle`;
- local pending-work observation for IPI and timer requests;
- owner metadata refresh after local service mutations;
- explicit idle/no-work behavior for a secondary that has no accepted local
  task to dispatch.

The first implementation should preserve the diagnostic role name until a
later supervisor-planned task defines a non-diagnostic secondary production
role. Renaming the role now would broaden the contract beyond this inventory.

## Recommended Follow-Up

The next bounded task should be
`phase6-secondary-scheduler-service-loop-core-20260526`.

That task should add the smallest target-independent secondary service-loop
adapter around the accepted `CpuLocalSchedulerService`. It should keep write
scope to scheduler/SMP support code and focused tests, avoid Pi 5 hardware, and
prove at unit/QEMU substitute level that a secondary owner can enter the loop,
run one CPU-local service cycle, preserve interrupt hot-path boundaries, and
report explicit no-work/error outcomes.

Suggested validation gates for the follow-up:

- `cargo fmt --all -- --check`;
- `cargo -Zjson-target-spec test` with the documented QEMU 9.2.0 path;
- `scripts/qemu-smoke.sh`;
- a focused QEMU secondary service-loop smoke if the implementation adds a new
  QEMU-facing diagnostic entry point;
- `git diff --check`;
- `mdbook build` if docs are touched.

## Deferred Work

The following remain explicitly deferred:

- shared run queues, remote enqueue queues, task migration, load balancing,
  work stealing, remote reschedule, and general multi-core scheduler topology;
- secondary timer-preemption policy beyond the accepted pending-request handoff,
  sleep queues, wait queues, blocking I/O readiness, fairness policy, batching,
  and production task movement beyond explicitly seeded CPU-local diagnostics;
- replacing `SecondaryProductionDiagnostic` with a general non-diagnostic
  secondary runtime role;
- runtime-console concurrency, UART interrupts, descriptor-facing TTY behavior,
  userspace, EL0, syscalls, descriptor tables, file descriptors, user/kernel
  copy policy, process address spaces, and process lifetime rules;
- filesystem behavior, program loading, libc/Rust std support, portable
  userland, and local shell behavior;
- RP1/PCIe ownership, DMA, cache-coherent DMA driver policy, networking, SSH,
  Ethernet, and shell access.

## Validation

- static inspection: git status --short was clean before edits.
- static source/doc review: inspected `src/smp.rs`, `src/scheduler.rs`,
  `src/smp_sync.rs`, scheduler architecture docs, roadmap, decision log,
  accepted CPU-local service records, and retained QEMU/Pi 5 scheduler proof
  scripts.
- whitespace inspection: git diff --check passed.
- documentation: mdbook build passed.
- Rust fmt/tests and hardware runs were not required because this task changes
  only Markdown documentation and durable task state.
