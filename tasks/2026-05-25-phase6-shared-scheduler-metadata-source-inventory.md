# Phase 6 Shared Scheduler Metadata Source Inventory

Task ID: phase6-shared-scheduler-metadata-source-inventory-20260525
Status: accepted

## Goal

Inventory and define the first shared scheduler metadata ownership contract
before any shared run queue, task migration, load balancing, or multi-core
preemption implementation.

## Scope

- Inspected current scheduler task identity, per-core scheduler ownership,
  local runnable state, current-task reporting, remote wake request IDs,
  dispatch counters, context-switch boundaries, and failure diagnostics.
- Defined the minimal metadata model needed to name a task across cores while
  preserving target-owned local runnable queues and production secondary
  dispatch invariants.
- Named the smallest follow-on implementation slice and its QEMU/Pi 5
  validation surfaces.

## Non-Goals

No Rust implementation, boot archive, hardware publish, hardware run, shared
run queue, task migration, load balancing, work stealing, remote enqueue queue,
multi-core preemption, Phase 7, filesystem, networking, SSH, shell behavior,
RP1/PCIe, UART interrupt ownership, or DMA behavior.

## Source Inventory

- src/scheduler.rs: scheduler-local TaskId, Task, TaskState, ProcessOwnerId,
  KernelStack, ContextFrame, RunnableQueue, SchedulerCounters,
  SingleCoreScheduler, PerCoreScheduler, SchedulerCoreRole, RemoteWakeQueue,
  RemoteWakeRequest, and explicit wrong-owner/deferred/full/mismatch errors.
- src/smp.rs: logical CPU lifecycle, MPIDR/affinity identity, stack ownership,
  workload progress, and cache publication. This identifies cores but does not
  provide task migration or shared scheduler mutation.
- src/target/qemu_virt.rs and src/target/rpi5.rs: accepted diagnostic paths for
  per-core scheduler ownership, remote wake request/consumption, and production
  secondary dispatch. These paths already report owner, role, production flag,
  current task, queue length, dispatch counters, and cross-owner rejection.
- AArch64 exception/timer/GIC paths: own IRQ entry, local timer state, SGI
  acknowledge and EOI. They remain observation paths only for scheduler
  metadata and must not run shared scheduler mutation.
- Console, TTY, and diagnostic command modules: may report bounded metadata
  snapshots outside scheduler locks, but do not participate in scheduler
  mutation.
- Docs and accepted task records: current topology is CPU-local. Remote wake
  publication is a bounded signal path; target CPUs own request consumption,
  local runnable transitions, current-task updates, and dispatch.

## Contract

The first shared scheduler metadata model is a task identity and ownership
surface, not a shared dispatch topology. The minimum accepted record is TaskId,
owning LogicalCpuId, TaskState, optional ProcessOwnerId, kernel-stack bounds,
owner-local current/runnable membership, and enough generation or counter data
to reject stale snapshots.

Only the owning CPU may write its Task, local PerCoreScheduler, current_task,
local RunnableQueue, and target-owned remote wake consumption state. A remote
CPU may inspect or publish a bounded wake request, but must not mutate another
CPU's local scheduler state through shared metadata.

IPI and timer IRQ context remain bounded to acknowledge, classify, record
bounded state, EOI, and return. They must not allocate, format, print, poll
UART input, dispatch diagnostic commands, walk unbounded metadata, mutate
runnable queues, migrate tasks, or cross talos_aarch64_context_switch.

## Next Slice

The next bounded implementation task should be
phase6-shared-scheduler-metadata-core-20260525. It should add only metadata
types and local owner APIs for task identity/owner/state snapshots. It should
not add shared run queues, remote enqueue, task migration, load balancing, work
stealing, multi-core preemption, Phase 7 behavior, filesystem, networking, SSH,
shell behavior, RP1/PCIe, UART interrupt ownership, or DMA behavior.

## Evidence

- Static inspection: git status --short before edits showed only the worker
  durable-state promotion outside the Talos repo; the Talos repo itself was
  clean.
- Static review: inspected src/scheduler.rs, src/smp.rs, QEMU and Pi 5 target
  secondary paths, remote wake/IPI paths, production secondary dispatch task
  records, scheduler architecture docs, roadmap, decision log, QEMU transcript
  references, and Pi 5 evidence summary.
- Documentation: added
  docs/src/project/phase6-shared-scheduler-metadata-source-inventory.md;
  updated docs/src/architecture/scheduler.md, docs/src/roadmap.md,
  docs/src/decisions/README.md, and docs/src/SUMMARY.md.
- Validation: git diff --check passed.
- Documentation: mdbook build passed.

## Acceptance

Accepted as a documentation/source-inventory and contract task. Talos is ready
for phase6-shared-scheduler-metadata-core-20260525; it is not ready for shared
run queues, task migration, load balancing, multi-core preemption, Phase 7,
filesystem, networking, SSH, shell behavior, RP1/PCIe, UART interrupt
ownership, or DMA behavior.
