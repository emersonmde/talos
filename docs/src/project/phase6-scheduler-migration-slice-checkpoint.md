# Phase 6 Scheduler Migration Slice Checkpoint

Status: accepted as the first Milestone 6.3 scheduler-migration slice
checkpoint.

This checkpoint reconciles the accepted scheduler migration readiness
inventory, CPU-local scheduler ownership implementation, QEMU substitute
evidence, cross-core wakeup/IPI inventory, retained gates, remaining risks, and
explicit deferrals before Talos starts broader scheduler migration, shared run
queues, Phase 7, filesystem, networking, SSH, or shell work.

## Accepted Work

- Scheduler migration readiness source inventory: commit `b75de5d`;
  checkpoint
  `docs/src/project/phase6-scheduler-migration-readiness-source-inventory.md`;
  task record
  `tasks/2026-05-25-phase6-scheduler-migration-readiness-source-inventory.md`.
- Per-core scheduler state core: commit `9decc46`; task record
  `tasks/2026-05-25-phase6-per-core-scheduler-state-core.md`.
- QEMU per-core scheduler ownership smoke: commit `33400ed`; task record
  `tasks/2026-05-25-phase6-qemu-per-core-scheduler-ownership-smoke.md`;
  transcript `target/qemu-per-core-scheduler-ownership-smoke.log`.
- Cross-core wakeup/IPI source inventory: commit `e92ff9d`; checkpoint
  `docs/src/project/phase6-cross-core-wakeup-ipi-source-inventory.md`; task
  record
  `tasks/2026-05-25-phase6-cross-core-wakeup-ipi-source-inventory.md`.

## Accepted Boundary

The first Milestone 6.3 slice accepts a CPU-local scheduler ownership model.
Each `PerCoreScheduler` is owned by one `LogicalCpuId`. CPU 0 remains the
only production scheduler owner, while secondary schedulers are
diagnostic/deferred owners. The underlying `SingleCoreScheduler` remains a
local FIFO runnable-queue model with no shared run queue, remote enqueue path,
task migration policy, sleep queue, wait queue, IPI state, or global task
lookup.

Local scheduler mutations that can race local timer/preemption handling keep
the existing rule: save and mask the local IRQ state around the short
CPU-local scheduler invariant, then restore the saved state before leaving the
critical section. The accepted `SpinLock<T>` and `lock_irqsave()` primitive
is available for future shared scheduler metadata, but this slice deliberately
does not put an SMP lock around a purely CPU-local runnable queue.

Raw IPI delivery is now the next implementation boundary, not scheduler
wakeup. The cross-core wakeup inventory accepts that a later scheduler-facing
remote wakeup must publish a bounded wake request and define wake-list or
remote-enqueue ownership before it mutates another CPU's scheduler state.

## Evidence Reconciliation

The readiness inventory selected CPU-local scheduler ownership as the first
slice after the accepted Milestone 6.2 lock/cache-coherence proof. It kept CPU
0 as the only production owner and deferred shared queues, migration, remote
wakeups, and secondary production scheduling.

The per-core scheduler state implementation added `LogicalCpuId`,
`SchedulerCoreRole`, `PerCoreSchedulerAccessError`, and
`PerCoreScheduler` while preserving existing `SingleCoreScheduler`
behavior. Focused no_std tests cover owner identity, boot-CPU current-task
ownership, rejected cross-owner queue mutation, deferred secondary production
dispatch, and retained local queue/counter behavior.

The QEMU per-core ownership smoke is accepted as substitute evidence. Under
QEMU virt with four CPUs, logical CPUs 0 through 3 each reported
`workload-complete`, owner identity matching the logical CPU, progress 4,
transitions 4, `errors=0`, and `ok=true`. The final classification was
`qemu-per-core-scheduler-ownership-complete` with `participants=4`,
`expected=4`, `lock-available=true`, `irq-ok=true`, and `PASS`.

The cross-core wakeup/IPI inventory identified the next raw interrupt-delivery
surface: QEMU and Pi 5 both currently use GICv2/GIC-400 facts, Talos' GICv2
wrapper does not yet expose SGI generation, SGIs use INTIDs 0 through 15, and
`GICD_SGIR` at offset `0xf00` must prove logical-CPU to target-list bit
mapping before any scheduler wakeup depends on it.

## Readiness Decision

Talos is ready for one bounded QEMU raw IPI implementation task:
`phase6-qemu-cross-core-ipi-delivery-smoke-20260525`.

That task should add the minimal GICv2 SGI surface, start QEMU virt with four
CPUs through the accepted PSCI path, send a diagnostic SGI from CPU 0 to each
secondary logical CPU, and report sender, receiver, SGI INTID, target-list bit,
acknowledgement/EOI, per-core counts, errors, and a PASS classification.

Talos is not ready for Pi 5 scheduler hardware proof or production remote
wakeup. A later serialized Pi 5 raw IPI proof is still required before physical
scheduler wakeups can depend on SGIs. Scheduler wakeup implementation must wait
until raw IPI delivery evidence exists and a separate task accepts wake-list or
remote-enqueue ownership and lock ordering.

No additional inventory task is needed before the QEMU raw IPI smoke unless
the implementation uncovers contradictory GICv2 source facts.

## Retained Gates

Retained as regression gates for this slice:

- `cargo fmt --all -- --check` and `cargo -Zjson-target-spec test` for
  scheduler data-structure invariants;
- `scripts/qemu-smoke.sh` for the broad QEMU boot smoke;
- `scripts/qemu-context-switch-smoke.sh` and
  `scripts/qemu-timer-preemption-smoke.sh` for retained single-core
  scheduler switch/preemption behavior when scheduler code changes;
- `scripts/qemu-per-core-scheduler-ownership-smoke.sh` for CPU-local
  scheduler ownership substitute evidence;
- `scripts/qemu-secondary-core-workload-smoke.sh` and
  `scripts/qemu-smp-lock-contention-smoke.sh` when the changed boundary
  touches secondary startup or SMP lock behavior;
- `scripts/rpi5-image.sh` and `scripts/rpi5-archive-review.sh` as image
  and archive inspection gates for Pi 5-targeted changes.

Retained hardware evidence for this slice is limited to previously accepted
Phase 6.2 lock/cache-coherence proof. No new hardware publish, hardware lock,
or hardware run belongs to this checkpoint.

## Deferred Work

The following remain explicitly deferred to later supervisor-planned tasks:

- shared run queues, global task lookup, remote enqueue queues, and wake lists;
- task migration, load balancing, work stealing, remote reschedule, and
  secondary-core production scheduling;
- raw Pi 5 SGI/IPI proof, production scheduler wakeups, and multi-core
  preemption;
- sleep queues, wait queues, blocking I/O readiness, runtime-console
  concurrency, UART interrupts, and descriptor-facing TTY behavior;
- userspace, EL0, syscalls, descriptor tables, file descriptors, user/kernel
  copy policy, process address spaces, and process lifetime rules;
- filesystem behavior, program loading, libc/Rust std support, portable
  userland, and local shell behavior;
- RP1/PCIe ownership, DMA, cache-coherent DMA driver policy, networking, SSH,
  Ethernet, and shell access.

## Remaining Risks

QEMU per-core scheduler ownership is substitute evidence. It proves the
CPU-local ownership shape and diagnostic secondary participation, but it does
not prove physical Pi 5 interrupt routing, secondary scheduler dispatch, or
production task execution across all cores.

The accepted lock/cache-coherence proof makes `SpinLock<T>` available for
future shared state, but it does not select a scheduler topology for shared
queues or remote wake lists. Any later shared scheduler structure still needs a
named owner, protecting lock, memory-ordering rule, and IRQ-context policy.

IPI context remains constrained: acknowledge/classify the SGI, record bounded
per-core evidence or wake-pending state, EOI, and return. It must not allocate,
format, print to serial, poll UART input, dispatch diagnostic commands, block,
sleep, take long locks, walk arbitrary scheduler queues, migrate tasks, or
cross the context-switch boundary.

## Next Recommendation

The next bounded task should be
`phase6-qemu-cross-core-ipi-delivery-smoke-20260525`.

The worker must not start Pi 5 hardware IPI proof, scheduler wakeup
implementation, shared run queues, task migration, Phase 7, filesystem,
networking, SSH, or shell work until the supervisor creates or promotes an
explicit durable task.

## Validation

- static inspection: `git status --short` was clean before checkpoint edits.
- static review: inspected first-slice task records, architecture docs,
  decision log, accepted QEMU transcript, roadmap, and evidence summaries.
- whitespace inspection: `git diff --check` passed after checkpoint edits.
- static inspection: `mdbook` was unavailable in the container, so mdBook
  build was not run.
- Rust fmt/tests were not required because this checkpoint changes only
  Markdown documentation and durable task state.
