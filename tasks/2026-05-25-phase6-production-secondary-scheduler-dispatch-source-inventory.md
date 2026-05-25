# Phase 6 Production Secondary Scheduler Dispatch Source Inventory

Task ID: phase6-production-secondary-scheduler-dispatch-source-inventory-20260525
Status: accepted

## Goal

Inventory the production secondary scheduler dispatch boundary and write the
contract before any secondary CPU enters the production scheduler loop.

## Scope

- Inspected the current scheduler, per-core state, secondary trampoline,
  timer/preemption, IPI/remote wake, console, and diagnostic-task paths that
  would be touched by production secondary dispatch.
- Defined the first acceptable production-secondary dispatch model: CPU-local
  runnable queues only, explicitly seeded diagnostic kernel threads,
  target-owned local mutations, and no shared run queue or migration.
- Named invariants for per-core current task, stack ownership,
  interrupt/preemption state, remote-wake drain ordering, and diagnostic output
  ownership.
- Identified the smallest implementation slice and QEMU/Pi 5 validation
  surfaces for that slice.

## Non-Goals

No Rust implementation, boot archive, hardware run, production secondary
dispatch enablement, shared run queue, global task lookup, task migration, load
balancing, multi-core preemption, userspace, descriptors, filesystem,
networking, SSH, shell behavior, RP1/PCIe, UART interrupt ownership, or DMA
behavior.

## Source Inventory

- src/scheduler.rs: owns Task, ContextFrame, KernelStack, RunnableQueue,
  SingleCoreScheduler, PerCoreScheduler, SchedulerCoreRole, and
  RemoteWakeQueue. The current role gate accepts BootCpuProduction and rejects
  SecondaryDeferred production dispatch. wake_blocked_local_task_from_remote_request()
  is already target-owned but does not run a dispatch loop.
- src/smp.rs: owns secondary lifecycle, logical-core state, secondary stack
  layout, cache-clean/invalidate publication, and the bounded
  run_controlled_secondary_workload() diagnostic. It does not own scheduler
  runnable queues or migration.
- src/target/qemu_virt.rs and src/target/rpi5.rs: start secondaries through
  PSCI/trampolines, publish per-core state, run diagnostic secondary
  workloads, run raw SGI/IPI and remote wake proofs, and currently demonstrate
  deferred secondary production dispatch.
- src/arch/aarch64/exceptions.rs, src/arch/aarch64/generic_timer.rs, and
  src/arch/aarch64/gicv2.rs: own exception dispatch, EL2 timer accounting,
  and GICv2 SGI/acknowledge/EOI surfaces. The first production-secondary slice
  may use existing raw SGI observation and local timer state as evidence, but
  must not switch from IPI or timer IRQ context.
- src/runtime_console.rs, src/tty.rs, and src/diagnostic_command.rs: own
  runtime output/input contracts and diagnostic command handling. The first
  secondary-dispatch diagnostic may report bounded state after dispatch, but
  secondary dispatch must not format, poll UART, or dispatch commands while
  holding scheduler or wake-request locks.
- src/main.rs and existing focused scripts: route QEMU diagnostics behind
  explicit cfg flags. A production-secondary dispatch proof should remain
  behind a named validation flag until QEMU and Pi 5 evidence accept it.

## Contract

The first production-secondary dispatch implementation may enable only
explicitly seeded CPU-local diagnostic kernel threads on logical secondary
CPUs. Each participating CPU owns its local PerCoreScheduler, local current
task slot, local runnable queue, task state for the diagnostic tasks seeded on
that CPU, and dispatch counters. A remote CPU may publish a bounded wake
request and signal with SGI INTID 1, but only the target CPU may drain its
request queue and mutate its own local scheduler state.

Secondary CPUs may enter production dispatch only after the accepted secondary
bring-up and cacheable-MMU handoff boundaries have established logical CPU
identity, exclusive stack ownership, and coherent per-core state publication.
The dispatch loop must run from normal secondary control flow, not from the
IPI or timer IRQ handler. The IPI handler remains bounded to
acknowledge/classify/record/EOI. Timer/preemption remains local to the CPU and
does not authorize multi-core preemption or shared scheduler state in this
slice.

The implementation must keep boot CPU scheduler behavior intact. CPU 0 remains
the existing production scheduler owner, and secondaries participate only
through the new explicit validation flag or diagnostic entry path until QEMU
and Pi 5 proof tasks accept the behavior.

The following remain deferred: shared run queues, global task lookup, remote
enqueue queues, task migration, load balancing, work stealing, multi-core
preemption, lower-EL/userspace, descriptors, filesystem, networking, SSH,
shell behavior, RP1/PCIe, UART interrupt ownership, and DMA/cache driver
policy.

## Smallest Next Slice

The next bounded task should be
phase6-production-secondary-dispatch-core-20260525. It should add the smallest
scheduler/secondary-core code needed for logical CPUs 1, 2, and 3 to enter a
production-owned dispatch loop for explicitly seeded CPU-local diagnostic
kernel threads, with per-core current-task reporting and bounded dispatch
counters. It must keep remote wake consumption target-owned and must not
introduce shared scheduler mutation.

## Validation Plan

- Static and unit gates: cargo fmt --all -- --check,
  cargo -Zjson-target-spec test, scripts/qemu-smoke.sh, scripts/rpi5-image.sh,
  and git diff --check.
- QEMU/substitute proof: a focused production-secondary dispatch smoke should
  show logical CPUs 1, 2, and 3 entering the production dispatch path for
  CPU-local diagnostic tasks, recording current-task ownership, local runnable
  transitions, dispatch counters, completion classification, and rejection of
  cross-owner local queue mutation.
- Pi 5 hardware proof: after QEMU acceptance, a serialized hardware run under
  hardwareTestLock must capture candidate archive digests, TFTP fetch proof,
  cursor-valid serial showing the same production dispatch invariant, final
  classification, and restore proof.
- Documentation gate: run mdbook build when available. If unavailable, record
  the availability inspection and keep git diff --check as the documentation
  whitespace gate.

## Evidence

- Static inspection: git status --short before edits showed a clean Talos
  worktree.
- Static review: inspected src/scheduler.rs, src/smp.rs, QEMU and Pi 5 target
  secondary paths, AArch64 exception/timer/GICv2 paths, console/output paths,
  scheduler architecture documentation, roadmap, decision log, and the
  accepted remote wake task/evidence records.
- Documentation: updated docs/src/architecture/scheduler.md,
  docs/src/roadmap.md, and docs/src/decisions/README.md.
- Validation: git diff --check passed.
- Documentation: mdbook build passed.

## Acceptance

Accepted as a documentation/source-inventory and contract task. Talos is ready
for the queued bounded implementation task
phase6-production-secondary-dispatch-core-20260525. It is not ready for shared
run queues, task migration, multi-core preemption, Phase 7, filesystem,
networking, SSH, shell behavior, RP1/PCIe, UART interrupt ownership, or DMA
behavior.
