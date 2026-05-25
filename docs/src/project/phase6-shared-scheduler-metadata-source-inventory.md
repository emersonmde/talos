# Phase 6 Shared Scheduler Metadata Source Inventory

Task ID: phase6-shared-scheduler-metadata-source-inventory-20260525

## Goal

Inventory and define the first shared scheduler metadata ownership contract
before any shared run queue, task migration, load balancing, or multi-core
preemption implementation.

## Source Inventory

- src/scheduler.rs owns scheduler-local TaskId, Task, TaskState, KernelStack,
  ContextFrame, ProcessOwnerId, RunnableQueue, SchedulerCounters,
  SingleCoreScheduler, PerCoreScheduler, SchedulerCoreRole, RemoteWakeQueue,
  and the current target-owned wake consumption boundary. PerCoreScheduler
  stores the CPU owner, role, CPU-local current task, local runnable queue, and
  local counters. It rejects cross-owner local queue mutation and permits
  secondary production dispatch only through SecondaryProductionDiagnostic.
- src/smp.rs owns logical CPU lifecycle, MPIDR/affinity snapshots,
  per-secondary stack ownership, workload progress, cache publication, and
  secondary state reporting. It names CPU identity but does not own scheduler
  runnable queues, task lookup, migration, or shared dispatch policy.
- src/target/qemu_virt.rs and src/target/rpi5.rs contain the accepted
  diagnostic validation surfaces for per-core scheduler ownership, raw SGI/IPI
  delivery, remote wake-request publication and target-owned consumption, and
  production secondary dispatch. Those paths publish current-task IDs,
  queue lengths, dispatch counters, and cross-owner rejection evidence under
  focused cfg flags.
- src/arch/aarch64/exceptions.rs, src/arch/aarch64/gicv2.rs, and
  src/arch/aarch64/generic_timer.rs own exception entry, SGI acknowledge/EOI,
  and local timer accounting. They may observe bounded scheduler metadata in
  future diagnostics, but they must not perform shared lookup, run queues,
  migration, formatting, allocation, or context switches from IRQ context.
- src/runtime_console.rs, src/tty.rs, and src/diagnostic_command.rs own local
  output/input and diagnostic command handling. Scheduler metadata diagnostics
  may report bounded snapshots outside scheduler locks, but console and command
  dispatch remain outside scheduler mutation windows.
- docs/src/architecture/scheduler.md, docs/src/roadmap.md, and accepted Phase
  6.3 task records already define the current topology as CPU-local: remote
  CPUs may publish bounded wake requests and signal with SGI INTID 1, but only
  the target CPU drains requests, mutates local task state, updates current-task
  state, and dispatches local runnable work.

## Ownership Contract

The first shared scheduler metadata model may only introduce read-oriented
identity and ownership facts about scheduler tasks. The minimal record is:

- scheduler TaskId;
- owning LogicalCpuId;
- current TaskState;
- optional ProcessOwnerId;
- local kernel-stack bounds;
- current-task membership for the owner CPU;
- runnable membership for the owner CPU's local queue;
- diagnostic counters or generation fields needed to reject stale snapshots.

The owning CPU remains the only writer for its Task, PerCoreScheduler,
current_task, local runnable queue, and target-owned remote wake consumption.
A remote CPU must not mutate another CPU's local scheduler state through the
shared metadata view. The metadata may let future code name a task across CPUs
or detect that a task is owned elsewhere, but it must not authorize remote
enqueue, stealing, migration, load balancing, or preemption.

Writers must run from normal kernel control flow on the owner CPU with local
interrupt state handled by the existing scheduler critical-section rules. IPI
and timer IRQ context may record bounded observation state and return, but must
not allocate, format, print, poll UART input, dispatch diagnostic commands,
walk shared metadata with unbounded latency, mutate runnable queues, migrate
tasks, or cross talos_aarch64_context_switch.

The first implementation should keep the boot CPU's production behavior and the
accepted secondary diagnostic dispatch behavior intact. Any metadata snapshot
must be explicit about its evidence level: local static/unit tests and QEMU
substitute evidence are not Pi 5 hardware acceptance.

## Deferred Work

The following remain deferred and require separate supervisor-planned tasks:
shared run queues, global task lookup with mutation authority, remote enqueue
queues, task migration, load balancing, work stealing, multi-core preemption,
process scheduling, lower-EL/userspace, descriptors, filesystem, networking,
SSH, shell behavior, RP1/PCIe, UART interrupt ownership, and DMA/cache-coherent
driver policy.

## Smallest Next Slice

The next bounded task should be phase6-shared-scheduler-metadata-core-20260525.
It should add only the metadata types and local owner APIs needed to publish
and inspect CPU-local task identity/owner/state snapshots. It must not
implement shared run queues, remote enqueue, migration, load balancing, work
stealing, or multi-core preemption.

## Validation Plan

- Static and unit gates: cargo fmt --all -- --check, cargo -Zjson-target-spec
  test, retained QEMU scheduler smokes, scripts/qemu-smoke.sh,
  scripts/rpi5-image.sh, and git diff --check.
- QEMU/substitute proof: a focused metadata smoke should prove local owner
  publication for CPUs 0 through 3, stable task ID and owner snapshots,
  explicit stale/unknown/wrong-owner outcomes, and unchanged rejection of
  remote local-queue mutation.
- Pi 5 hardware proof: only after QEMU acceptance, a serialized run should
  capture archive/kernel digests, TFTP fetch evidence, cursor-valid serial
  showing the metadata invariant, classification, and restore proof.
- Documentation gate: run mdbook build when docs change.

## Acceptance

Accepted as a documentation/source-inventory and contract task. Talos is ready
for the queued bounded implementation task
phase6-shared-scheduler-metadata-core-20260525. It is not ready for shared run
queues, migration, load balancing, multi-core preemption, Phase 7, filesystem,
networking, SSH, shell behavior, RP1/PCIe, UART interrupt ownership, or DMA
behavior.
