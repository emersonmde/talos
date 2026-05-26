# Talos Productionization Boundary Inventory

Status: accepted as a source-backed productionization boundary inventory.

This task reviewed the accepted scheduler/preemption, SMP dispatch,
wakeup/IPI, console/TTY, diagnostic command-channel, roadmap, diagnostic
surface, and evidence-retention boundaries before Talos starts shared run
queues, task migration, load balancing, multi-core preemption, Phase 7,
filesystem, networking, SSH, or shell work.

## Capability Inventory

### Scheduler And Preemption

- Current proof level: QEMU and Pi 5 Phase 4 timer-preemption evidence proves
  the boot CPU can record timer preemption requests in the IRQ hot path and
  switch EL2 kernel threads outside IRQ context.
- Productized boundary already present: `src/scheduler.rs` owns scheduler-local
  `TaskId`, `TaskState`, `Task`, `RunnableQueue`,
  `SingleCoreScheduler`, and short IRQ-masked mutation windows. The scheduler
  architecture doc preserves the split between cooperative switching, timer
  requests, and deferred POSIX process resources.
- Missing runtime semantics: no sleep queues, wait queues, blocking I/O,
  secondary timer-preemption policy, general scheduler service loop, global task
  registry, shared run queues, task migration, load balancing, or EL0/process
  scheduling exists.
- Smallest risk-reducing follow-up: a CPU-local scheduler service boundary
  source inventory and contract that defines normal-control-flow ordering for
  timer request handling, remote wake draining, local runnable transitions,
  secondary production dispatch, and metadata refresh without adding shared
  queues or migration.

### SMP Dispatch

- Current proof level: QEMU and serialized Pi 5 evidence proves logical CPUs 1,
  2, and 3 can enter an explicit `SecondaryProductionDiagnostic` role,
  dispatch seeded CPU-local diagnostic kernel threads, preserve local queue
  ownership, and reject wrong-owner or deferred-role dispatch.
- Productized boundary already present: `PerCoreScheduler` separates boot CPU,
  deferred secondary, and secondary production diagnostic roles. Local
  `RunnableQueue` ownership remains per logical CPU, and production secondary
  access requires an explicit local owner and enabled role.
- Missing runtime semantics: secondaries are not yet part of a normal scheduler
  runtime loop. There is no long-lived production task selection policy, no
  secondary timer ownership, no global task registry, and no migration or load
  balancing.
- Smallest risk-reducing follow-up: the same CPU-local scheduler service
  boundary inventory should name how secondary production dispatch is entered
  from ordinary secondary control flow and which retained diagnostics continue
  to prove the role.

### Wakeups And IPIs

- Current proof level: raw SGI/IPI delivery is accepted on QEMU and Pi 5.
  Remote wake-request publication, duplicate coalescing, target-owned drain,
  and target-owned local `Blocked` to `Runnable` transitions are accepted at
  QEMU and Pi 5 evidence levels.
- Productized boundary already present: `RemoteWakeQueue` is target-owned and
  bounded. Senders may publish or coalesce requests and signal SGI INTID 1.
  IPI context remains acknowledge/classify/record/EOI only; only the target CPU
  may drain and mutate its local scheduler state outside IPI context.
- Missing runtime semantics: the drain/wake service is still a diagnostic proof
  path, not a named production scheduler service. There is no remote enqueue
  queue, no shared run queue, no global task lookup with mutation authority, and
  no scheduler blocking/readiness integration.
- Smallest risk-reducing follow-up: define where a production CPU-local
  scheduler service drains wake requests relative to timer-preemption requests,
  local dispatch, and metadata refresh. Direct remote enqueue and shared queue
  work should remain deferred until that service ordering is accepted.

### Shared Scheduler Metadata

- Current proof level: QEMU and Pi 5 evidence proves logical CPUs 0 through 3
  can publish and query an owner-published shared scheduler metadata table,
  reject cross-owner publication, reject stale snapshots, and preserve local
  runnable queues.
- Productized boundary already present: `SharedSchedulerMetadata` and
  `SharedSchedulerMetadataLock` provide a bounded, SMP-protected,
  read-oriented table of owner-published task snapshots. The type deliberately
  does not contain a shared runnable queue or mutation authority over another
  CPU's `PerCoreScheduler`.
- Missing runtime semantics: no global task registry, lifecycle policy,
  removal path, generation reconciliation policy, migration hooks, load
  balancing, work stealing, or remote dispatch authority exists.
- Smallest risk-reducing follow-up: keep metadata as read-oriented evidence
  until the CPU-local service boundary defines when owners refresh it. Shared
  run-queue or migration inventory should wait until that refresh lifecycle is
  explicit.

### Console, TTY, And Diagnostic Command Channel

- Current proof level: runtime-console output, polling TTY RX, TTY
  canonical-lite line discipline, console input result contract, and the
  kernel-owned diagnostic command channel are accepted on QEMU and Pi 5.
- Productized boundary already present: `runtime-console0` is the default
  internal console identity; TTY line assembly sits above the console backend;
  `src/diagnostic_command.rs` consumes complete TTY lines and emits bounded
  diagnostic responses through a `DiagnosticResponseSink`.
- Missing runtime semantics: no descriptor tables, POSIX `read`/`write`,
  scheduler-blocking TTY, readiness polling, termios, userspace shell,
  filesystem-backed commands, networking, SSH, UART interrupts, or RP1 UART0
  ownership exists.
- Smallest risk-reducing follow-up: do not productize the command channel into
  a shell yet. Keep the retained command-channel smokes as validation gates
  until Phase 7 descriptor/syscall work or a separate console productization
  inventory names scheduler-blocking TTY semantics.

### Diagnostic Surface And Evidence Hygiene

- Current proof level: evidence-retention and diagnostic-surface audits are
  accepted. They separate retained validation gates from historical or
  retirement-candidate diagnostics.
- Productized boundary already present: accepted policies say raw evidence is
  retained in Git until external artifact storage or a bounded no-delete
  manifest plan exists; diagnostic surfaces may be retired only after accepted
  summaries and replacement gates are named.
- Missing runtime semantics: several older diagnostic scripts remain queued for
  cleanup, and large raw hardware captures remain in Git because external
  artifact storage is not available.
- Smallest risk-reducing follow-up: keep existing cleanup follow-ups queued and
  do not block CPU-local scheduler service planning on artifact movement. Any
  diagnostic deletion must remain a separate cleanup task with its own gates.

## Recommendation

The next Phase 6.3 task should be a documentation/source-inventory task:
`phase6-cpu-local-scheduler-service-boundary-source-inventory-20260526`.

That task should define the production CPU-local scheduler service boundary
that orders timer-preemption request handling, target-owned remote wake drains,
local runnable transitions, production secondary dispatch entry, and owner
metadata refresh. It should remain a contract and source inventory only.

Do not start shared run queues, remote enqueue queues, task migration, load
balancing, work stealing, multi-core preemption, Phase 7, filesystem,
networking, SSH, shell behavior, RP1/PCIe, UART interrupt ownership, or
DMA/cache-driver policy from this inventory.

## Queued Follow-Ups

- Queue `phase6-cpu-local-scheduler-service-boundary-source-inventory-20260526`
  as the recommended next scheduler productionization task.
- Keep `talos-evidence-archive-large-raw-lab-artifacts-20260525` blocked until
  external artifact storage or an explicit no-delete manifest-only plan exists.
- Keep `talos-evidence-summary-manifest-backfill-20260525`,
  `talos-retire-qemu-secondary-core-discriminator-20260525`, and
  `talos-retire-legacy-rpi5-runtime-exception-diagnostics-20260525` queued for
  supervisor ordering; they are cleanup work, not prerequisites for the next
  CPU-local scheduler service inventory.

## Validation

- static inspection: `git status --short` was clean before edits.
- static source/doc review: inspected `docs/src/architecture/scheduler.md`,
  `docs/src/architecture/diagnostic-command-channel.md`,
  `docs/src/architecture/tty-stdio.md`, `docs/src/architecture/console.md`,
  `docs/src/project/phase6-production-secondary-dispatch-closeout-checkpoint.md`,
  `docs/src/project/phase6-remote-wakeup-scheduler-integration-closeout.md`,
  `docs/src/project/phase6-shared-scheduler-metadata-closeout-checkpoint.md`,
  `docs/src/project/diagnostic-surface-policy.md`,
  `docs/src/project/evidence-retention-policy.md`, `docs/src/roadmap.md`,
  `docs/src/decisions/README.md`, `src/scheduler.rs`, `src/smp.rs`,
  `src/smp_sync.rs`, `src/runtime_console.rs`, `src/tty.rs`, and
  `src/diagnostic_command.rs`.
- whitespace inspection: `git diff --check` passed.
- documentation: `mdbook build` passed.
- Rust fmt/tests and hardware runs were not required because this task changed
  only Markdown documentation and durable task state.
