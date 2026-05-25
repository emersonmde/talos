# Phase 6 Shared Scheduler Metadata Closeout Checkpoint

Status: accepted as the Phase 6.3 shared scheduler metadata closeout
checkpoint.

This checkpoint reconciles the accepted source inventory, implementation,
QEMU substitute proof, and serialized Pi 5 hardware proof for the first shared
scheduler metadata slice before Talos starts shared run queues, task
migration, load balancing, multi-core preemption, Phase 7, filesystem,
networking, SSH, or shell work.

## Accepted Work

- Shared scheduler metadata source inventory and contract: commit 09e6402;
  task record
  tasks/2026-05-25-phase6-shared-scheduler-metadata-source-inventory.md;
  project page docs/src/project/phase6-shared-scheduler-metadata-source-inventory.md.
- Shared scheduler metadata core: commit 77b326e; task record
  tasks/2026-05-25-phase6-shared-scheduler-metadata-core.md.
- QEMU shared scheduler metadata smoke: commit 4606abc; task record
  tasks/2026-05-25-phase6-qemu-shared-scheduler-metadata-smoke.md;
  transcript target/qemu-shared-scheduler-metadata-smoke.log.
- Pi 5 shared scheduler metadata proof: commit 87bc22c; task record
  tasks/2026-05-25-phase6-pi5-shared-scheduler-metadata-proof.md; evidence
  summary
  tasks/evidence/2026-05-25-pi5-shared-scheduler-metadata-proof/summary.md.

## Evidence Reconciliation

The source inventory accepted a metadata-only boundary for naming
scheduler-local tasks across cores. The record is intentionally read-oriented:
TaskId, owning LogicalCpuId, TaskState, optional ProcessOwnerId, kernel-stack
bounds, owner-local current/runnable membership, and a generation field for
stale snapshot rejection. The inventory explicitly preserved CPU-local
RunnableQueue ownership and target-owned remote wake consumption.

The implementation in src/scheduler.rs adds SchedulerTaskSnapshot,
SharedSchedulerMetadata, SharedSchedulerMetadataError, and
SharedSchedulerMetadataLock. The table is bounded, rejects duplicate/unknown
task IDs, rejects invalid owners, rejects stale generation-qualified lookups,
and rejects wrong-owner publication before mutating metadata. It does not
contain a shared runnable queue and does not grant mutation authority over
another CPU's PerCoreScheduler.

The focused QEMU proof starts logical CPUs 0 through 3 through the accepted
PSCI path. Each CPU publishes and queries the metadata table for task IDs 101,
201, 301, and 401, proves owner-task and boot-task lookup, rejects cross-owner
local scheduler mutation, rejects cross-owner metadata publication, preserves
local runnable queues, and reports
classification=qemu-shared-scheduler-metadata-complete.

Serialized Pi 5 evidence carries the same invariant to physical hardware. The
accepted local1 run used archive SHA256
7ec358f5809aee223364948fa20ba9b4e73f8fd76a1ac0238081926568f74bf0, kernel
SHA256 232cab18a49eb75ddc1969438d45ab1874359492028dfea81522f22507d24382, and
kernel size 99,136 bytes. TFTP evidence records da591740/kernel_2712.img
fetches at 99,136 bytes. Cursor-valid serial showed Talos entry, secondary
cacheable-MMU handoff, PSCI CPU_ON for logical CPUs 1, 2, and 3, per-core
reports for logical CPUs 0 through 3, task IDs 101/201/301/401, lookup
success, boot-task lookup success, cross-owner scheduler and metadata
rejection, preserved local runnable queues, final-metadata-len=4,
final-metadata-generation=4, errors=0, ok=true, and
classification=pi5-shared-scheduler-metadata-complete with PASS.

The Pi 5 proof records hardwareTestLock ownership, post-publish boot-file
evidence for the staged candidate, artifact digests, TFTP fetch proof,
cursor-valid serial, classification, and restore evidence with
restore-exit.txt equal to 0.

## Retained Gates

Retained regression gates for this slice:

- cargo fmt --all -- --check and cargo -Zjson-target-spec test for scheduler,
  SMP, metadata, remote wake, and dispatch invariants;
- scripts/qemu-smoke.sh for broad QEMU boot coverage;
- scripts/qemu-per-core-scheduler-ownership-smoke.sh for CPU-local scheduler
  ownership;
- scripts/qemu-remote-wake-to-local-runnable-smoke.sh for target-owned remote
  wake consumption into local runnable state;
- scripts/qemu-production-secondary-dispatch-smoke.sh for production
  secondary diagnostic dispatch;
- scripts/qemu-shared-scheduler-metadata-smoke.sh for the shared metadata
  invariant;
- scripts/rpi5-shared-scheduler-metadata-image.sh,
  scripts/rpi5-shared-scheduler-metadata-boot-tree.sh, and
  scripts/rpi5-archive-review.sh before any Pi 5 metadata proof rerun;
- hardwareTestLock, archive/kernel digests, TFTP fetch evidence,
  cursor-valid serial, classification, and restore proof for any physical
  Pi 5 scheduler metadata claim.

The focused QEMU and Pi 5 shared scheduler metadata scripts are retained as
named validation gates. No temporary shared-metadata diagnostic surface needs
quarantine in this checkpoint.

## Deferred Work

The following remain explicitly deferred to later supervisor-planned tasks:

- shared run queues, remote enqueue queues, task migration, load balancing,
  work stealing, and remote reschedule;
- multi-core preemption, secondary timer-preemption policy, sleep queues, wait
  queues, blocking I/O readiness, and production task movement beyond
  explicitly seeded CPU-local diagnostics;
- global mutable task registry authority beyond the bounded owner-published
  metadata table;
- runtime-console concurrency, UART interrupts, descriptor-facing TTY
  behavior, userspace, EL0, syscalls, descriptor tables, file descriptors,
  user/kernel copy policy, process address spaces, and process lifetime rules;
- filesystem behavior, program loading, libc/Rust std support, portable
  userland, and local shell behavior;
- RP1/PCIe ownership, DMA, cache-coherent DMA driver policy, networking, SSH,
  Ethernet, and shell access.

## Remaining Risks

The accepted metadata table names CPU-local scheduler tasks across cores, but
it is not a scheduler topology. It does not answer where runnable work should
live, when migration is legal, how load balancing works, how stale remote
decisions are invalidated, or how timer preemption should interact with
remote wakeups and local queues.

The current physical evidence is still diagnostic. It proves the metadata
invariant on Pi 5 cores in the accepted secondary cacheable-MMU regime, not
general production migration, remote enqueue, or cross-core dispatch.

The retained target logs and proof artifacts are useful but growing. Before
new feature work broadens Phase 6.3, the repository should audit which
evidence must remain checked in, which artifacts can stay generated, and which
diagnostic surfaces should be retired or kept as named validation gates.

## Readiness Decision

Talos is not ready to start shared run queue implementation, task migration,
load balancing, multi-core preemption, Phase 7, filesystem, networking, SSH,
or shell work solely from this checkpoint.

The next bounded task should be
talos-evidence-retention-policy-and-bloat-audit-20260525. That task should
produce an explicit evidence-retention policy and concrete cleanup follow-ups
for Phase 6.3 proof artifacts before broader scheduler productionization work.

## Validation

- static inspection: git status --short was clean before checkpoint edits.
- static review: inspected accepted shared scheduler metadata task records,
  scheduler architecture docs, roadmap, decision log, QEMU transcript
  references, and Pi 5 evidence summary.
- whitespace inspection: git diff --check passed after checkpoint edits.
- documentation: mdbook build passed.
- Rust fmt/tests and hardware runs were not required because this checkpoint
  changes only Markdown documentation and durable task state.
