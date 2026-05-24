# Phase 6 SMP-Safe Primitives Source Inventory

Status: accepted as the Milestone 6.2 source inventory and synchronization
contract before implementing shared locks or scheduler data structures.

This checkpoint defines the first SMP-safe primitive boundary for Talos. It
does not add Rust implementation changes, publish a boot archive, power-cycle
hardware, acquire the hardware lock, migrate the scheduler to SMP, add shared
run queues, add IPIs or cross-core wakeups, enter EL0, add descriptors,
filesystem behavior, networking, SSH, shell behavior, RP1/PCIe, DMA behavior,
or UART interrupts.

## Source Inventory

Repository synchronization surfaces:

- `src/smp.rs` is the only current multi-core state boundary. It uses
  `AtomicU64` fields for per-core lifecycle, context, MPIDR, affinity, stack
  pointer, and controlled workload progress. Secondary writers use
  `Ordering::Release`; primary snapshots use `Ordering::Acquire`.
- `src/smp.rs` also owns explicit cache-maintenance helpers for the accepted
  Pi 5 proof: `dc cvac` plus `dsb sy` to clean state updates to the point of
  coherency, and focused diagnostic invalidation with `dc ivac` plus `dsb sy`
  before primary-side snapshots.
- `src/target/rpi5.rs` calls PSCI `CPU_ON` through SMC, starts cores 1-3 with
  Pi 5 MPIDR affinities `0x100`, `0x200`, and `0x300`, invalidates per-core
  state before observing secondary reports, and keeps the accepted alive and
  workload proofs behind focused diagnostic gates.
- `src/target/qemu_virt.rs` retains QEMU substitute gates for secondary-core
  bring-up and controlled workload. QEMU reports affinities `0x1`, `0x2`, and
  `0x3` and remains useful for target-independent regressions, not Pi 5
  hardware claims.
- `src/arch/aarch64/mod.rs` owns the existing boot-CPU
  `single_core_irq_mask_save()` / `single_core_irq_restore()` primitive. It
  snapshots `DAIF`, masks `PSTATE.I` with `DAIFSet`, restores with `DAIFClr`
  when appropriate, and uses `isb`. This is an interrupt-local critical
  section primitive, not an SMP lock.
- `src/scheduler.rs` remains a pure single-core scheduler data model. Its
  runnable queue and counters use ordinary mutable state through `&mut self`.
  Call sites in `src/target/qemu_virt.rs` and `src/target/rpi5.rs` protect
  short scheduler mutation windows with the single-core IRQ mask primitive,
  but no scheduler data structure is safe for concurrent secondary access.
- `docs/src/architecture/interrupts-timers.md` and
  `docs/src/architecture/scheduler.md` explicitly state that IRQ masking is
  not SMP mutual exclusion and that later multi-core work needs real locking.

Accepted evidence surfaces:

- `scripts/qemu-secondary-core-discriminator.sh` proves the QEMU PSCI
  secondary-core handoff shape.
- `scripts/qemu-secondary-core-workload-smoke.sh` proves the controlled
  workload path under QEMU.
- `scripts/rpi5-psci-secondary-core-alive-image.sh` and
  `scripts/rpi5-secondary-core-workload-image.sh` remain serialized hardware
  regression entry points when a future task explicitly needs Pi 5 proof.
- Focused no_std tests in `src/smp.rs` cover lifecycle naming, per-core state,
  stack-slot ownership, MPIDR mapping, and controlled workload progress.

## Accepted Cache-Maintenance Lesson

The Pi 5 PSCI alive proof did not become acceptable until secondary cores made
their per-core state visible outside their private cache view and the primary
invalidated before reading. Plain atomics were necessary for compiler-visible
ordering, but they were not sufficient for the diagnostic memory visibility
problem observed during boot-time secondary startup.

Milestone 6.2 therefore separates four responsibilities:

- local IRQ masking: prevents boot-CPU interrupt reentry around short local
  invariants, but does not exclude other cores;
- SMP mutual exclusion: serializes shared kernel data across cores and must be
  implemented with AArch64 exclusive or atomic read-modify-write operations;
- memory ordering: defines acquire/release or stronger ordering around lock
  acquire/release and published shared state;
- cache maintenance: remains explicit when the code shares state across
  early-boot cache or MMU boundaries not covered by ordinary coherent memory
  assumptions.

The first lock implementation must not hide cache maintenance inside a generic
spinlock API. Cache maintenance is a separate hardware contract that should be
named at call sites or behind a purpose-built early-boot sharing helper until
Talos has a broader coherent-memory and DMA policy.

## Primitive Contract

The next implementation slice should add only the minimum primitives needed to
protect future shared scheduler state:

- `SpinLock<T>` or an equivalent narrow mutual-exclusion primitive backed by
  AArch64 atomics or exclusive load/store operations;
- a lock guard that releases on drop and documents that it must not allocate,
  format, print, block, sleep, or call arbitrary callbacks while held;
- an IRQ-save wrapper or composition rule for data touched by both normal code
  and local interrupt context: mask local IRQs first, then acquire the SMP
  lock, release the lock, then restore the previous IRQ mask state;
- acquire semantics on lock acquisition and release semantics on unlock, with
  stronger barriers only where a specific hardware proof requires them;
- a non-recursive policy: attempting to take the same lock twice on one core is
  misuse, not a supported nesting model;
- an explicit statement that lock ownership is CPU-local, not task-local, until
  the scheduler gains per-core current-task state.

Per-core state remains owned by `src/smp.rs`. The initial lock should not
change secondary-core bring-up lifecycles, boot-time workload diagnostics, or
production scheduler participation.

## Validation Gate Policy

Retained as Milestone 6.2 gates:

- static inspection of `src/smp.rs`, `src/arch/aarch64/mod.rs`,
  `src/scheduler.rs`, `src/target/qemu_virt.rs`, and `src/target/rpi5.rs`
  before implementing a lock;
- focused unit tests for lock state, guard release, no-copy/no-reentrant
  policy, and IRQ-mask composition helpers;
- `scripts/qemu-secondary-core-workload-smoke.sh` as the first QEMU
  multi-core contention harness once the primitive exists;
- serialized Pi 5 proof only after QEMU contention passes and the durable task
  acquires `hardwareTestLock`.

Milestone 6.1-only evidence:

- the accepted PSCI alive proof and controlled workload proof demonstrate
  secondary-core startup and cache-visible state publication, but they do not
  prove a shared scheduler, lock correctness under contention, IPI delivery,
  cross-core preemption, concurrent console writes, or long-running SMP
  behavior.

Deferred by name:

- shared run queues, per-core run queues, task migration, load balancing,
  cross-core wakeups, IPIs, cross-core preemption, concurrent runtime-console
  ownership, UART interrupts, EL0, syscalls, descriptors, filesystem behavior,
  networking, SSH, shell behavior, RP1/PCIe, and DMA/cache-coherent driver
  policy.

## Next Worker Task

The next bounded implementation task is
`phase6-spinlock-barrier-core-20260524`. It should implement the narrow
SMP mutual-exclusion and barrier core described here, without changing
scheduler data structures or starting Milestone 6.3.

## Validation

- static inspection: `git status --short` was clean before documentation
  edits.
- static inspection: inspected `src/smp.rs`, `src/arch/aarch64/mod.rs`,
  `src/scheduler.rs`, `src/target/rpi5.rs`, `src/target/qemu_virt.rs`,
  `docs/src/architecture/interrupts-timers.md`,
  `docs/src/architecture/scheduler.md`, accepted Phase 6.1 task records, and
  accepted Pi 5 evidence summaries.
- fmt/lint/typecheck: `git diff --check` passed after documentation edits.
- static inspection: `mdbook` was unavailable in the container, so mdBook
  build was not run.
- Rust fmt/tests were not required because this task changed only Markdown
  documentation and durable task state.
