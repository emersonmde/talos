# Phase 6 Shared Run-Queue Migration Closeout Checkpoint

Status: accepted as the Phase 6.3 shared run-queue/migration closeout
checkpoint.

This checkpoint reconciles the accepted shared run-queue/migration source
inventory, contract, target-independent implementation, QEMU substitute proof,
and serialized Pi 5 hardware proof before Talos starts load balancing,
work stealing, multi-core preemption, Phase 7, filesystem, networking, SSH, or
shell work.

## Accepted Work

- Shared run-queue/migration source inventory: commit 6d6ccfd; project page
  docs/src/project/phase6-shared-runqueue-migration-source-inventory.md; task
  record tasks/2026-05-26-phase6-shared-runqueue-migration-source-inventory.md.
- Shared run-queue/migration contract: commit a2eb731; project page
  docs/src/project/phase6-shared-runqueue-migration-contract.md; task record
  tasks/2026-05-26-phase6-shared-runqueue-migration-contract.md.
- Boot-scenario cfg routing cleanup precursor: commit 489b557; task record
  tasks/2026-05-26-talos-boot-scenario-cfg-routing-cleanup.md.
- Shared run-queue core: commit 4e69f9d; task record
  tasks/2026-05-26-phase6-shared-runqueue-core.md; implementation in
  src/scheduler.rs.
- QEMU shared run-queue/migration smoke: commit fede0ec; task record
  tasks/2026-05-26-phase6-qemu-shared-runqueue-migration-smoke.md; transcript
  target/qemu-shared-runqueue-migration-smoke.log.
- Pi 5 shared run-queue/migration proof: commit cc9d984; task record
  tasks/2026-05-26-phase6-pi5-shared-runqueue-migration-proof.md; evidence
  summary
  tasks/evidence/2026-05-26-pi5-shared-runqueue-migration-proof/summary.md.

## Evidence Reconciliation

The source inventory showed that Talos had owner-local runnable queues,
target-owned remote wake mailboxes, owner-published scheduler metadata,
accepted SMP lock primitives, secondary service-loop entry, and diagnostic
proof routing, but no shared scheduler topology. It required a written
contract before implementation.

The accepted contract made task mutation single-owner at every instant. Remote
wake remains separate from remote enqueue or migration: RemoteWakeQueue carries
requests for already target-owned blocked tasks, while migration must publish a
complete owner-transfer entry through a shared structure and let the
destination owner consume it from normal scheduler control flow. The accepted
lock order is local IRQ save/mask, SMP scheduler lock acquisition, SMP lock
release, and local IRQ restore; no scheduler lock may be held across context
switching, printing, UART polling, allocation, blocking, sleeping, migration
callbacks, IPI send loops, timer reprogramming loops, or hardware lab waits.

The target-independent implementation in src/scheduler.rs adds
SharedRunQueue, SharedRunQueueEntry, MigrationState, SharedRunQueueError, and
SharedRunQueueLock. SharedRunQueue::publish_migration removes a runnable task
from the source-local queue after fresh metadata checks and records the
MigrationReserved -> SharedQueued transition. SharedRunQueue::consume_for_destination
lets an accepted destination owner enqueue the task into its local queue,
transfer owner-published metadata, remove the shared entry, and report
DestinationEnqueued. Deterministic errors cover stale metadata, wrong owners,
invalid CPUs, duplicate membership, full queues, unsupported task state,
deferred secondary role, and task mismatch.

The focused QEMU proof exercises the implemented core without bypassing it.
It publishes task 107 from owner 0 to owner 1, proves source-local queue
removal, shared-queue insertion and removal, destination-local enqueue,
metadata owner transfer, classification=qemu-shared-runqueue-migration-complete,
and PASS. This remains QEMU/substitute evidence.

The serialized Pi 5 proof carries the same named invariant to physical cores.
The accepted local1 run used archive SHA256
4d5c8e2666d64ddcc5df7b49c8d3a541b01634800917616cbdb88404a54630d5, kernel
SHA256 98a9cb87bcb89c38b19a097a05695a136aaf6b0eb911ec03c3b0c17eeab6a394, and
kernel size 102,952 bytes. TFTP evidence records da591740/kernel_2712.img
fetches from 10.42.1.4 at 102,952 bytes before restore. Cursor-valid serial
reported all four physical-core participants completing the implemented
shared run-queue/migration invariant with participants=4, expected=4,
errors=0, lock-available=true,
classification=pi5-shared-runqueue-migration-complete, and PASS. Restore
evidence shows the pre-run boot snapshot was restored successfully.

## Productized Versus Diagnostic

Productized by this slice:

- a target-independent shared owner-transfer queue for runnable task migration;
- explicit source-owner publish and destination-owner consume boundaries;
- deterministic migration states and failure reporting;
- unit-tested local-queue removal, shared-queue membership, destination
  enqueue, and owner-published metadata transfer;
- a retained lock wrapper aligned with the accepted SMP lock boundary.

Still diagnostic-only or retained as validation surface:

- qemu_shared_runqueue_migration and
  scripts/qemu-shared-runqueue-migration-smoke.sh;
- TALOS_RPI5_SHARED_RUNQUEUE_MIGRATION_PROOF and the focused Pi 5 image and
  boot-tree scripts;
- the diagnostic secondary production role used to prove destination-owner
  consumption;
- seeded task movement for proof purposes only, not target selection,
  fairness, affinity, load balancing, work stealing, running-task migration,
  remote reschedule, or a general multi-core runtime policy.

## Retained Gates

Retained regression gates for this slice:

- cargo fmt --all -- --check;
- cargo -Zjson-target-spec test;
- scripts/qemu-smoke.sh;
- scripts/qemu-secondary-scheduler-service-loop-smoke.sh for the prior
  owner-local service-loop invariant;
- scripts/qemu-shared-runqueue-migration-smoke.sh for the focused QEMU
  shared run-queue/migration invariant;
- scripts/rpi5-shared-runqueue-migration-image.sh,
  scripts/rpi5-shared-runqueue-migration-boot-tree.sh, and
  scripts/rpi5-archive-review.sh before any Pi 5 rerun of this proof;
- hardwareTestLock, archive/kernel digests, TFTP fetch evidence,
  cursor-valid serial, classification, and restore proof for any physical
  Pi 5 shared run-queue/migration claim.

## Deferred Work

The following remain explicitly deferred to later supervisor-planned tasks:

- load balancing, work stealing, target selection, fairness, affinity policy,
  remote reschedule, and running-task migration;
- multi-core timer preemption, asynchronous context capture on secondary
  cores, scheduler switching from exception frames, secondary
  timer-preemption policy, sleep queues, wait queues, and blocking I/O
  readiness;
- a non-diagnostic secondary scheduler role and durable secondary idle/wake
  policy;
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

The accepted proof establishes that a runnable task can move through the
implemented shared run-queue core and arrive in a destination owner's local
queue on QEMU and physical Pi 5 cores. It does not decide which CPU should
receive work, when balancing should run, how stale metadata should be handled
by a policy layer, or how fairness and affinity interact with migration.

Running-task migration remains intentionally rejected. Multi-core preemption
must separately define asynchronous context capture, exception-frame
ownership, timer/IPI ordering, and the handoff from interrupt recorders back
to normal scheduler control flow.

The Pi 5 proof is a diagnostic validation gate. It proves the named invariant
with seeded work across physical cores, not a general long-lived secondary
runtime, load balancer, or preemptive multi-core scheduler.

## Readiness Decision

Talos is ready for a bounded Phase 6.3 load-balancing source inventory. That
task should inventory scheduler metadata freshness, per-core runnable state,
shared run-queue capacity, candidate target selection inputs, fairness and
affinity constraints, remote reschedule needs, retained diagnostics, and the
boundary between policy and the accepted SharedRunQueue core before any
load-balancing implementation.

Talos is not ready to implement load balancing, work stealing,
multi-core preemption, Phase 7, filesystem, networking, SSH, or shell work
solely from this checkpoint.

## Validation

- static inspection: git status --short was clean before checkpoint edits.
- static review: inspected accepted source inventory, contract, cfg-routing
  cleanup task, shared run-queue core, QEMU smoke, Pi 5 proof task records and
  evidence summaries, scheduler architecture docs, roadmap, and decision log.
- whitespace inspection: git diff --check passed after checkpoint edits.
- documentation: mdbook build passed.
- Rust fmt/tests, QEMU smoke reruns, and hardware runs were not required
  because this checkpoint changes only Markdown documentation and durable task
  state, and it references already accepted implementation/proof evidence.
