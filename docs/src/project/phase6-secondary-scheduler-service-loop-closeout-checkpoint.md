# Phase 6 Secondary Scheduler Service Loop Closeout Checkpoint

Status: accepted as the Phase 6.3 secondary scheduler service-loop closeout
checkpoint.

This checkpoint reconciles the accepted secondary scheduler service-loop source
inventory, target-independent implementation, QEMU substitute proof, and
serialized Pi 5 hardware proof before Talos starts shared run queues, task
migration, load balancing, multi-core preemption, Phase 7, filesystem,
networking, SSH, or shell work.

## Accepted Work

- Secondary scheduler service-loop source inventory: commit a57ec9a; project
  page
  docs/src/project/phase6-secondary-scheduler-service-loop-source-inventory.md;
  task record
  tasks/2026-05-26-phase6-secondary-scheduler-service-loop-source-inventory.md.
- Secondary scheduler service-loop core: commit 5bbcbb9; task record
  tasks/2026-05-26-phase6-secondary-scheduler-service-loop-core.md;
  implementation in src/scheduler.rs.
- QEMU secondary scheduler service-loop smoke: commit f6eefd2; task record
  tasks/2026-05-26-phase6-qemu-secondary-scheduler-service-loop-smoke.md;
  transcript target/qemu-secondary-scheduler-service-loop-smoke.log.
- Pi 5 secondary scheduler service-loop proof: commit e0b290e; task record
  tasks/2026-05-26-phase6-pi5-secondary-scheduler-service-loop-proof.md;
  evidence summary
  tasks/evidence/2026-05-26-pi5-secondary-scheduler-service-loop-proof/summary.md.

## Evidence Reconciliation

The source inventory accepted a normal-control-flow secondary service-loop
boundary after secondary handoff has already established logical CPU identity,
exclusive stack ownership, and Rust control flow. It deliberately kept IPI and
timer IRQ paths as bounded recorders: interrupt context may acknowledge,
classify, record pending work, EOI, and return, but scheduler mutation,
remote-wake drains, dispatch, metadata refresh, allocation, formatting,
blocking, sleeping, UART polling, and arbitrary callbacks belong outside the
IRQ hot path.

The implementation adds SecondarySchedulerServiceLoop in src/scheduler.rs.
Its run_once entry point rejects boot-CPU use, cross-owner requests, and
deferred secondary roles before calling the accepted
CpuLocalSchedulerService::run_cycle for one owning secondary CPU. The cycle
can consume target-owned remote wake state, perform local runnable transition
and diagnostic dispatch, observe pending timer-preemption state, refresh
owner-published metadata, and return whether the cycle performed local work.

The focused QEMU proof starts logical CPUs 1, 2, and 3 through the accepted
PSCI/QEMU secondary path. Each secondary runs one owner-local service-loop
cycle, proves remote wake drain, local dispatch, no-work metadata refresh,
cross-owner rejection, deferred-role rejection, queue length zero, and reports
classification=qemu-secondary-scheduler-service-loop-complete. This is
QEMU/substitute evidence only.

Serialized Pi 5 evidence carries the same invariant to physical secondaries
after the accepted cacheable-MMU handoff. The accepted local1 run used archive
SHA256 56fb95ec7ff4092fa384a83f9af1705a0ec11a023a1e216f4563f9d18d6f24b3,
kernel SHA256 a9228747b7102024efa933e3d7acf6ed5ee800354fac5721a13115ab34c6184d,
and kernel size 102,824 bytes. TFTP evidence records
da591740/kernel_2712.img fetches from 10.42.1.4 at 102,824 bytes. Cursor-valid
serial showed Talos entry, the secondary scheduler service-loop proof start,
PSCI CPU_ON for logical CPUs 1, 2, and 3, per-core reports for
remote-wake drain, local dispatch, no-work metadata refresh, cross-owner
rejection, deferred-role rejection, local-queue preservation, errors=0,
ok=true, classification=pi5-secondary-scheduler-service-loop-complete, and
PASS. Restore evidence shows the pre-run snapshot was restored successfully.

The accepted proof surfaces remain diagnostic validation gates. They do not
create a general secondary runtime role, shared run queue, remote enqueue
authority, task migration path, load balancer, remote reschedule policy, or
multi-core preemption policy.

## Productized Versus Diagnostic

Productized by this slice:

- a target-independent normal-control-flow adapter for one secondary owner to
  run the CPU-local scheduler service;
- explicit rejection of boot-CPU use, cross-owner service, and deferred
  secondary roles;
- owner-local sequencing of remote wake drain, local runnable transition,
  diagnostic dispatch, timer-preemption observation, and metadata refresh;
- preservation of interrupt hot-path separation for IPI and timer events.

Still diagnostic-only or retained as validation surface:

- SchedulerCoreRole::SecondaryProductionDiagnostic remains the only accepted
  secondary production role;
- QEMU and Pi 5 service-loop proof flags/scripts are named regression gates,
  not supported runtime interfaces;
- secondary service-loop evidence covers one seeded owner-local cycle per
  secondary, not general task movement, fairness, load balancing, or
  multi-core preemption.

## Retained Gates

Retained regression gates for this slice:

- cargo fmt --all -- --check;
- cargo -Zjson-target-spec test with the documented QEMU 9.2.0 path;
- scripts/qemu-smoke.sh for broad QEMU boot coverage;
- scripts/qemu-secondary-scheduler-service-loop-smoke.sh for the focused
  QEMU substitute service-loop invariant;
- scripts/rpi5-secondary-scheduler-service-loop-image.sh,
  scripts/rpi5-secondary-scheduler-service-loop-boot-tree.sh, and
  scripts/rpi5-archive-review.sh before any Pi 5 service-loop proof rerun;
- hardwareTestLock, archive/kernel digests, TFTP fetch evidence,
  cursor-valid serial, classification, and restore proof for any physical
  Pi 5 scheduler service-loop claim.

## Deferred Work

The following remain explicitly deferred to later supervisor-planned tasks:

- shared run queues, remote enqueue queues, task migration, load balancing,
  work stealing, and remote reschedule;
- multi-core preemption, secondary timer-preemption policy, sleep queues, wait
  queues, blocking I/O readiness, and production task movement beyond
  explicitly seeded CPU-local diagnostics;
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

The accepted service loop proves that secondaries can run one owner-local
service cycle after accepted handoff state. It does not yet define where
runnable work lives across CPUs, when task ownership may move, how remote
enqueue differs from wake notification, how load balancing avoids stale
metadata decisions, or how multi-core timer preemption interacts with owner
queues.

The current secondary role remains diagnostic. A later task must either define
a non-diagnostic secondary scheduler role or keep proof entry points
quarantined as retained gates.

The QEMU gate currently uses an optimized build because the 4 KiB diagnostic
secondary stacks are too small for the debug proof workload. That is a proof
gate property; it is not evidence that production secondary stack sizing is
complete.

## Readiness Decision

Talos is ready for a bounded Phase 6.3 shared run-queue and migration source
inventory. That task should inventory current CPU-local queues, target-owned
remote wake drains, owner-published metadata, secondary service-loop entry,
IPI/timer recording, and diagnostic proof surfaces before proposing any shared
topology implementation.

Talos is not ready to implement shared run queues, task migration, load
balancing, multi-core preemption, Phase 7, filesystem, networking, SSH, or
shell work solely from this checkpoint.

## Validation

- static inspection: git status --short was clean before checkpoint edits.
- static review: inspected accepted service-loop source inventory, core, QEMU
  smoke, Pi 5 proof task records and evidence summaries, scheduler
  architecture docs, roadmap, and decision log.
- whitespace inspection: git diff --check passed after checkpoint edits.
- documentation: mdbook build passed.
- Rust fmt/tests, QEMU smoke reruns, and hardware runs were not required
  because this checkpoint changes only Markdown documentation and durable task
  state, and it references the accepted service-loop implementation/proof
  evidence.
