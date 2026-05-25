# Phase 6 Scheduler Migration Readiness Source Inventory

Status: accepted as the Milestone 6.3 source inventory and first-slice
scheduler migration contract.

This checkpoint defines the first scheduler migration boundary after the
accepted Phase 6.2 SMP-safe primitive closeout. It is documentation-only. It
does not implement Rust scheduler changes, publish a boot archive, acquire the
hardware lock, power-cycle hardware, add shared run queues, migrate tasks, add
cross-core wakeups, add IPIs, enter EL0, add syscalls, descriptors, filesystem
behavior, networking, SSH, shell behavior, RP1/PCIe behavior, UART interrupt
ownership, or DMA/cache-coherent driver policy.

## Source Inventory

Scheduler data model:

- src/scheduler.rs remains a pure single-core scheduler model. TaskId is
  scheduler-local, Task owns kernel-thread state and saved ContextFrame,
  RunnableQueue is a fixed FIFO over task IDs, and SingleCoreScheduler owns
  the runnable queue plus transition, yield, preemption, and dispatch counters
  through ordinary mutable access.
- The scheduler structs do not contain atomics, locks, CPU ids, migration
  state, load-balancing state, IPI state, sleep queues, wait queues, process
  address spaces, descriptor tables, EL0 state, filesystem state, networking
  state, or console ownership policy.
- Current QEMU and Pi 5 timer-preemption diagnostics instantiate a local
  `SingleCoreScheduler<2>` inside diagnostic-only smoke state. Those smokes
  prove the Phase 4 single-core dispatch contract; they are not production
  multi-core scheduler participation.

Per-core and SMP state:

- src/smp.rs owns MAX_CORES, secondary stack-slot layout, per-core lifecycle,
  MPIDR/logical identity, stack pointer, and diagnostic workload progress.
  This is the accepted secondary-core bring-up boundary, not a scheduler
  ownership boundary.
- src/smp.rs uses acquire/release atomics and explicit cache maintenance for
  boot-time diagnostic publication where Pi 5 evidence required it. Those
  helpers remain separate from the generic scheduler and from `SpinLock<T>`.
- src/smp_sync.rs owns the accepted `SpinLock<T>`, SpinLockGuard, AArch64
  lock_irqsave() composition, and smp_full_barrier(). The primitive has QEMU
  contention evidence and Pi 5 physical cache/coherence proof, but no
  scheduler data structure is wired to it yet.

Architecture and target call sites:

- src/arch/aarch64/mod.rs owns talos_aarch64_context_switch,
  talos_aarch64_kernel_thread_trampoline, IRQ enable/disable helpers,
  single_core_irq_mask_save(), single_core_irq_restore(), and timer IRQ
  routing helpers. The existing IRQ mask primitive protects only local
  boot-CPU invariants.
- src/target/qemu_virt.rs contains QEMU scheduler-yield and timer-preemption
  smokes that mutate the diagnostic scheduler after masking local IRQs, then
  cross the cooperative context-switch boundary outside the IRQ handler.
- src/target/qemu_virt.rs also contains secondary-core bring-up, controlled
  workload, and SMP lock contention diagnostics. These start secondary cores
  and prove primitive contention, but they do not run the scheduler on those
  cores.
- src/target/rpi5.rs mirrors the single-core timer-preemption diagnostic on
  physical hardware. Its IRQ handler records timer/tick/preemption-request
  state and exits; diagnostic kernel-thread code later masks local IRQs,
  mutates the local scheduler, and switches contexts.
- src/target/rpi5.rs also owns PSCI secondary startup, cacheable-MMU handoff,
  and the accepted Pi 5 SMP lock/cache-coherence proof. That proof validates
  the primitive and cache regime, not shared scheduler queues.

Accepted documents and evidence inspected:

- docs/src/architecture/scheduler.md
- docs/src/project/phase4-closeout-checkpoint.md
- docs/src/project/phase5-tty-stdio-closeout-checkpoint.md
- docs/src/project/phase5-diagnostic-command-channel-closeout-checkpoint.md
- docs/src/project/phase6-secondary-core-bringup-closeout-checkpoint.md
- docs/src/project/phase6-smp-safe-primitives-source-inventory.md
- docs/src/project/phase6-secondary-cacheable-mmu-handoff-source-inventory.md
- docs/src/project/phase6-smp-safe-primitives-closeout-checkpoint.md
- tasks/evidence/2026-05-25-pi5-smp-lock-cache-coherence-final-proof/summary.md

## Selected First Slice

The next implementation slice should add a CPU-local scheduler ownership
boundary before any shared queue or migration behavior exists.

The selected shape is:

- keep each runnable queue owned by one logical CPU;
- introduce a per-core scheduler state wrapper that records the owning logical
  CPU and contains that CPU's local SingleCoreScheduler;
- keep boot-time task creation and initial runnable placement on the boot CPU
  until a later task accepts explicit per-core task placement;
- require local IRQ masking around mutations of the current CPU's scheduler
  state when the mutation can race local timer/preemption handling;
- do not take an SMP lock for a purely CPU-local runnable queue in this first
  slice;
- reserve `SpinLock<T>` for later shared scheduler metadata, global task tables,
  cross-core wake lists, or migration queues that are explicitly planned and
  proven.

This first slice should be durable without hardware evidence because it is a
local ownership refactor and QEMU/substitute contract. Hardware proof becomes
relevant only when secondary cores actually participate in production
scheduling, when cross-core wakeup/IPI paths are added, or when shared
scheduler state is mutated by more than one core.

## Boundaries

Per-core run queue ownership:

- a run queue has exactly one logical CPU owner;
- local timer preemption may inspect and mutate only that CPU's local queue;
- the first implementation may keep only CPU 0 populated;
- secondary-core scheduler activation remains deferred until a later task
  defines how tasks are placed on those cores.

Shared scheduler state:

- global task-id allocation, global task lookup, migration state, and shared
  wake lists remain deferred;
- any later shared scheduler structure must name its protecting lock and
  ordering rule before implementation;
- shared state must not be accessed from IRQ context while holding a lock that
  can allocate, format, print, block, sleep, or run callbacks.

Lock ordering and IRQ masking:

- local IRQ mask first when protecting data that can also be touched by the
  same CPU's timer/preemption path;
- acquire an SMP lock only after the local IRQ state is saved and masked;
- release the SMP lock before restoring the saved local IRQ state;
- do not hold scheduler locks across talos_aarch64_context_switch;
- do not print, allocate, poll UART input, run diagnostic command dispatch, or
  call arbitrary callbacks while a scheduler lock is held.

Cross-core wakeups and IPIs:

- no cross-core wakeup path exists in this slice;
- no IPI controller path is selected in this slice;
- waking a task owned by another CPU, stealing work, rebalancing load, and
  forcing remote reschedule are later tasks that must define target selection,
  interrupt routing, acknowledgement, and memory-ordering evidence.

Per-core timer/preemption policy:

- the accepted timer IRQ handler contract remains: acknowledge/classify the
  interrupt, account for the tick or bounded preemption request, reprogram the
  compare value, EOI, and return;
- scheduler mutation still happens outside the IRQ hot path;
- the first per-core ownership implementation may reuse the current QEMU
  timer-preemption smoke as substitute evidence for CPU 0 only;
- per-core timer routing and secondary-core preemption remain deferred until
  secondary scheduler participation is explicitly planned.

Boot-CPU-only state:

- initial production scheduler state remains boot-CPU-only;
- runtime console output, TTY/diagnostic command state, descriptor state,
  process owner hooks, filesystem state, networking state, RP1/PCIe state, and
  DMA/cache policy remain outside the scheduler migration slice.

## Follow-Up Tasks

Implementation task: phase6-per-core-scheduler-state-core-20260525.

- Add the smallest Rust surface for CPU-local scheduler ownership.
- Preserve the existing SingleCoreScheduler behavior and tests.
- Keep CPU 0 as the only production owner initially.
- Add focused no_std tests for owner identity, local queue access, and rejected
  cross-owner mutation if that API exists.
- Do not implement shared run queues, migration, IPIs, cross-core wakeups,
  userspace, descriptors, filesystem, networking, SSH, shell, RP1/PCIe, UART
  interrupt ownership, or DMA/cache-coherent driver policy.

Substitute evidence task: phase6-qemu-per-core-scheduler-ownership-smoke-20260525.

- Exercise the CPU-local scheduler ownership shape under QEMU.
- Keep the proof on the boot CPU unless the implementation task explicitly
  creates secondary scheduler participation.
- Reuse existing timer/preemption and context-switch gates where they cover
  the changed boundary.

Inventory task: phase6-cross-core-wakeup-ipi-source-inventory-20260525.

- Run only after the per-core ownership boundary and QEMU evidence are
  accepted.
- Define cross-core wakeups, IPI routing, remote reschedule semantics, lock
  ordering, and validation before any implementation.

Checkpoint task: phase6-scheduler-migration-slice-checkpoint-20260525.

- Reconcile the first Milestone 6.3 scheduler migration slice before broader
  migration, shared queues, or later roadmap work starts.

## Explicit Deferrals

Deferred by name: shared run queues, global task lookup, task migration, load
balancing, work stealing, cross-core wakeups, IPIs, remote reschedule,
secondary-core production scheduling, per-core timer routing on secondary
cores, cross-core preemption, sleep queues, wait queues, process address
spaces, EL0, syscalls, descriptor tables, filesystem behavior, networking,
SSH, shell behavior, concurrent runtime-console ownership, UART interrupt
ownership, RP1/PCIe behavior, and DMA/cache-coherent driver policy.

## Validation

- static inspection: git status --short was clean before documentation edits.
- static review: inspected src/scheduler.rs, src/smp.rs, src/smp_sync.rs,
  src/arch/aarch64/mod.rs, src/target/qemu_virt.rs, src/target/rpi5.rs,
  relevant architecture docs, Phase 4 and 5 closeout docs, Phase 6.1 and 6.2
  closeout docs, and accepted Pi 5 SMP lock evidence.
- fmt/lint/typecheck: git diff --check passed after documentation edits.
- static inspection: mdbook was unavailable in the container, so mdBook build
  was not run.
- Rust fmt/tests were not required because this task changed only Markdown
  documentation and durable task state.
