# Phase 6 Secondary Scheduler Service Loop Source Inventory

Status: accepted.

Task id: phase6-secondary-scheduler-service-loop-source-inventory-20260526

## Goal

Define the CPU-local productionization boundary for running the accepted
scheduler service from a normal secondary-core control loop without adding
shared scheduler topology.

## Scope

- Inventoried secondary-core handoff, `SecondaryProductionDiagnostic`,
  `CpuLocalSchedulerService`, per-core scheduler state, remote wake queues,
  timer-preemption boundaries, and retained QEMU/Pi 5 scheduler proof entry
  points.
- Defined how a secondary CPU enters a normal owner-local service loop from
  accepted handoff state.
- Preserved the split between interrupt observation and normal-control-flow
  scheduler mutation.
- Named which existing proof entry points remain diagnostics and which behavior
  belongs behind the secondary service-loop boundary.
- Recommended the smallest implementation follow-up.

## Non-Goals

- No Rust implementation in this inventory task.
- No shared run queues, remote enqueue queues, task migration, load balancing,
  work stealing, remote reschedule, or multi-core preemption.
- No descriptor, syscall, userspace, filesystem, networking, SSH, or shell
  behavior.
- No Pi 5 hardware run.

## Evidence

- Service-loop inventory:
  docs/src/project/phase6-secondary-scheduler-service-loop-source-inventory.md.
- Architecture update: docs/src/architecture/scheduler.md.
- Roadmap update: docs/src/roadmap.md.
- Decision-log update: docs/src/decisions/README.md.

## Service-Loop Result

The accepted secondary service-loop boundary starts after secondary handoff has
published logical CPU identity, stack state, and normal kernel control flow.
Each loop iteration remains owner-local: it observes pending IPI/timer work,
calls the accepted `CpuLocalSchedulerService` for the owning logical CPU,
dispatches only through the owner `PerCoreScheduler`, refreshes owner metadata,
and returns to the loop or an explicit idle/no-work point.

IPI handlers remain acknowledge/classify/record/EOI paths. Timer IRQ handlers
remain pending-preemption recorders. Neither path may run scheduler dispatch,
walk unbounded queues, mutate runnable queues, refresh metadata, allocate,
format output, poll UART input, block, sleep, migrate work, or cross a context
switch.

The retained QEMU/Pi 5 secondary workload, remote wake, production secondary
dispatch, and shared metadata scripts remain diagnostic gates. The production
service-loop boundary is the normal-control-flow owner that should order those
accepted behaviors without creating a shared scheduler topology.

## Recommended Follow-Up

Queue `phase6-secondary-scheduler-service-loop-core-20260526` as the next
bounded implementation task. It should add a minimal target-independent
secondary service-loop adapter around `CpuLocalSchedulerService`, with
unit/QEMU substitute evidence for secondary owner entry, one service cycle,
interrupt hot-path separation, and explicit no-work/error outcomes.

It must not implement shared run queues, migration, load balancing,
multi-core preemption, Phase 7, filesystem, networking, SSH, shell, RP1/PCIe,
UART interrupt ownership, or DMA/cache policy.

## Validation

- static inspection: git status --short was clean before edits.
- static source/doc review: inspected `src/smp.rs`, `src/scheduler.rs`,
  `src/smp_sync.rs`, scheduler architecture docs, retained QEMU/Pi 5 proof
  scripts, roadmap, decision log, and accepted CPU-local service task records.
- whitespace inspection: git diff --check passed.
- documentation: mdbook build passed.
- Rust fmt/tests and hardware runs were not required because the task changed
  only Markdown documentation and durable task state.
