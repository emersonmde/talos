# Phase 6 Production Secondary Dispatch Closeout Checkpoint

Status: accepted as the Phase 6.3 production secondary dispatch closeout
checkpoint.

This checkpoint reconciles the accepted production secondary scheduler dispatch
source inventory, implementation, QEMU substitute proof, and serialized Pi 5
hardware proof before Talos starts shared scheduler metadata, shared run queues,
task migration, multi-core preemption, Phase 7, filesystem, networking, SSH, or
shell work.

## Accepted Work

- Production secondary scheduler dispatch source inventory and contract:
  commit 30bf2c7; task record
  tasks/2026-05-25-phase6-production-secondary-scheduler-dispatch-source-inventory.md.
- Production secondary dispatch core: commit b56b423; task record
  tasks/2026-05-25-phase6-production-secondary-dispatch-core.md.
- QEMU production secondary dispatch smoke: commit 3a94c00; task record
  tasks/2026-05-25-phase6-qemu-production-secondary-dispatch-smoke.md;
  transcript target/qemu-production-secondary-dispatch-smoke.log.
- Pi 5 production secondary dispatch proof: commit 7fc9d3d; task record
  tasks/2026-05-25-phase6-pi5-production-secondary-dispatch-proof.md;
  evidence summary
  tasks/evidence/2026-05-25-pi5-production-secondary-dispatch-proof/summary.md.

## Evidence Reconciliation

The accepted source inventory limits the first production secondary scheduler
slice to explicitly seeded CPU-local diagnostic kernel threads. Each
participating secondary owns its local PerCoreScheduler, local runnable queue,
current-task slot, diagnostic task state, and dispatch counters. Remote wake
requests remain bounded publications; only the target CPU may drain its own
request queue and mutate its own local scheduler state.

The implementation adds the explicit
SchedulerCoreRole::SecondaryProductionDiagnostic role and
PerCoreScheduler::dispatch_cpu_local_diagnostic_task(). The dispatch API
requires local ownership, the selected task to be the front local runnable
task, and the task to still be Runnable. Wrong-owner, deferred-role,
empty-queue, mismatched-task, and non-runnable-task cases are explicit errors
that leave local scheduler state intact.

The focused QEMU proof starts logical CPUs 1, 2, and 3 through PSCI and has
each secondary enter SecondaryProductionDiagnostic, dispatch three CPU-local
diagnostic tasks, publish stable current-task/local-queue/counter snapshots,
and reject cross-owner local queue and production-dispatch attempts. The
transcript classification is qemu-production-secondary-dispatch-complete.

Serialized Pi 5 evidence carries the same invariant to physical hardware. The
accepted local1 run used archive SHA256
70a601fcaf1580540a4055fef794ec1182327fac0e059b0b19075eae82476f50, kernel
SHA256 bf36772c529b16d1dbf81aa1575661942ab137a189fdedaae0e5394f4c8e924d, and
kernel size 98,664 bytes. Cursor-valid serial showed Talos entry, PSCI CPU_ON
for logical CPUs 1, 2, and 3, per-core reports with
role=secondary-production-diagnostic, production=true, progress 3, transition
count 6, production dispatch count 3, context switch count 3, empty local
queues, cross-owner queue rejection, cross-owner dispatch rejection, ok=true,
and classification=pi5-production-secondary-dispatch-complete with PASS.

The Pi 5 proof also records TFTP fetch evidence for the staged candidate,
post-publish boot-file evidence for the candidate kernel size, artifact
digests, hardwareTestLock ownership, and restore proof with restore-exit.txt
equal to 0.

## Retained Gates

Retained regression gates for this slice:

- cargo fmt --all -- --check and cargo -Zjson-target-spec test for scheduler,
  SMP, remote wake, and dispatch invariants;
- scripts/qemu-smoke.sh for broad QEMU boot coverage;
- scripts/qemu-per-core-scheduler-ownership-smoke.sh for CPU-local scheduler
  ownership;
- scripts/qemu-remote-wake-to-local-runnable-smoke.sh for target-owned
  remote request consumption into local runnable state;
- scripts/qemu-production-secondary-dispatch-smoke.sh for the production
  secondary diagnostic dispatch invariant;
- scripts/rpi5-image.sh, focused Pi 5 boot-tree scripts, and
  scripts/rpi5-archive-review.sh before any Pi 5-targeted proof;
- hardwareTestLock, TFTP/archive evidence, cursor-valid serial, artifact
  digests, classification, and restore proof for any physical Pi 5 claim.

The focused QEMU and Pi 5 production secondary dispatch scripts are retained
as named validation gates. No temporary diagnostic surface needs quarantine in
this checkpoint.

## Deferred Work

The following remain explicitly deferred to later supervisor-planned tasks:

- shared scheduler metadata, global task identity beyond scheduler-local
  diagnostic IDs, and cross-core task lookup;
- shared run queues, remote enqueue queues, task migration, load balancing,
  work stealing, remote reschedule, and multi-core preemption;
- secondary timer-preemption policy, sleep queues, wait queues, blocking I/O
  readiness, and production task selection beyond explicitly seeded CPU-local
  diagnostics;
- runtime-console concurrency, UART interrupts, descriptor-facing TTY
  behavior, userspace, EL0, syscalls, descriptor tables, file descriptors,
  user/kernel copy policy, process address spaces, and process lifetime rules;
- filesystem behavior, program loading, libc/Rust std support, portable
  userland, and local shell behavior;
- RP1/PCIe ownership, DMA, cache-coherent DMA driver policy, networking, SSH,
  Ethernet, and shell access.

## Remaining Risks

The accepted production secondary dispatch capability is still diagnostic and
CPU-local. It proves that physical secondary cores can enter a production-owned
dispatch path for explicitly seeded local diagnostic tasks, not that Talos has
a shared scheduler topology.

There is no global task registry, shared run queue, migration policy, load
balancer, work stealing, remote enqueue authority, or multi-core preemption
policy. The boot CPU production scheduler behavior remains separate from the
secondary diagnostic production role.

IPI context remains constrained to acknowledge/classify/record/EOI and return.
It must not allocate, format, print to serial, poll UART input, dispatch
diagnostic commands, block, sleep, take long locks, walk arbitrary scheduler
queues, migrate tasks, or cross the context-switch boundary.

## Readiness Decision

Talos is ready for one bounded Phase 6.3 shared scheduler metadata source
inventory and contract task. That task should inventory the current scheduler
task identity, runnable-state ownership, per-core current-task reporting,
remote wake request IDs, dispatch counters, context-switch boundaries, and
failure diagnostics needed before any later shared run queue or migration
implementation.

Talos is not ready for shared run queue implementation, task migration,
multi-core preemption, Phase 7, filesystem, networking, SSH, or shell work.

## Next Recommendation

The next bounded task should be
phase6-shared-scheduler-metadata-source-inventory-20260525.

That task should be documentation/source-inventory only. It should define the
minimal metadata ownership and validation boundary required before any future
shared run queue, task migration, load balancing, or multi-core preemption
implementation. It should not implement shared scheduler metadata or move tasks
between cores.

No broader scheduler migration or later roadmap work should start until the
supervisor creates or promotes an explicit durable task.

## Validation

- static inspection: git status --short was clean before checkpoint edits.
- static review: inspected accepted production secondary dispatch task
  records, scheduler architecture docs, roadmap, decision log, QEMU transcript,
  and Pi 5 evidence summary.
- whitespace inspection: git diff --check passed after checkpoint edits.
- documentation: mdbook build passed.
- Rust fmt/tests and hardware runs were not required because this checkpoint
  changes only Markdown documentation and durable task state.
