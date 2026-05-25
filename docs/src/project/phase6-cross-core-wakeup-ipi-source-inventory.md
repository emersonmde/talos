# Phase 6 Cross-Core Wakeup and IPI Source Inventory

Status: accepted as the Milestone 6.3 source inventory for cross-core
wakeup and IPI work. This checkpoint is documentation-only. It does not
implement SGI delivery, mutate scheduler code, publish a boot archive, acquire
the hardware lock, power-cycle hardware, add shared run queues, migrate tasks,
add userspace, descriptors, filesystem behavior, networking, SSH, shell
behavior, RP1/PCIe behavior, UART interrupt ownership, or DMA/cache-coherent
driver policy.

## Source Inventory

Accepted scheduler state:

- src/scheduler.rs now has LogicalCpuId, SchedulerCoreRole, and
  PerCoreScheduler. Each local scheduler is owned by exactly one logical CPU.
  CPU 0 remains the only production scheduler owner; secondary schedulers are
  diagnostic/deferred owners.
- SingleCoreScheduler remains a local FIFO runnable-queue model. It has no
  remote enqueue path, shared run queue, migration policy, sleep queue, wait
  queue, global task lookup, IPI state, or wake-list lock.
- The accepted QEMU per-core scheduler ownership smoke proves that QEMU logical
  CPUs 0 through 3 can each publish distinct per-core scheduler snapshots, with
  secondary production dispatch still deferred.

Accepted SMP and secondary-core state:

- src/smp.rs owns PSCI secondary-core startup, MPIDR/logical identity,
  secondary stack ownership, lifecycle publication, and diagnostic-only
  workloads. It is not a scheduler wakeup or IPI layer.
- src/smp_sync.rs owns the accepted SpinLock<T>, SpinLockGuard, AArch64
  lock_irqsave() composition, and smp_full_barrier(). The accepted
  lock-ordering rule remains local IRQ mask first, then SMP lock acquisition;
  release the SMP lock before restoring local IRQ state.
- The Pi 5 SMP lock/cache-coherence proof accepts generic shared cached
  atomics only after all participating cores enter the same cacheable EL2
  stage-1 regime. It does not prove scheduler wakeups or interrupt delivery.

Interrupt-controller facts:

- QEMU virt currently uses GICv2 with distributor base 0x0800_0000 and CPU
  interface base 0x0801_0000, as recorded in src/target/qemu_virt.rs.
- Raspberry Pi 5 currently uses GIC-400/GICv2 with distributor base
  0x10_7fff_9000 and CPU interface base 0x10_7fff_a000, as recorded in
  src/target/rpi5.rs.
- src/arch/aarch64/gicv2.rs currently supports enabling PPI/SPI INTIDs,
  reading GICC_IAR, writing GICC_EOIR, inspecting enable/pending/active bits,
  and reading GICC_HPPIR. It does not yet expose SGI generation.
- GICv2 SGIs are INTIDs 0 through 15. The GICv2 distributor
  software-generated interrupt register is GICD_SGIR at offset 0xf00. The SGIR
  encoding includes an SGI ID in bits 3:0, a CPU target list in bits 23:16,
  and a target list filter in bits 25:24.
- The existing PPI/SGI enable register bank is shared for INTIDs below 32. A
  focused SGI smoke should enable the chosen SGI ID in that bank on receiving
  cores, acknowledge it with GICC_IAR, and EOI the exact IAR value with
  GICC_EOIR.

Current uncertainties:

- The mapping from Talos LogicalCpuId to GICv2 SGIR target-list bits has not
  been proven on QEMU or Pi 5. The first proof must report MPIDR/logical CPU,
  target-list bit, observed INTID, and sender/receiver identity.
- Pi 5 GIC-400 SGI delivery to secondary cores has not been captured with
  serialized hardware evidence. QEMU evidence is not sufficient for accepting
  Pi 5 scheduler wakeups.
- Talos has not decided how remote wake requests are queued. A remote core must
  not mutate another CPU's local runnable queue directly until a later task
  accepts the wake-list or remote-enqueue ownership model.

Accepted documents and evidence inspected:

- docs/src/architecture/interrupts-timers.md
- docs/src/architecture/scheduler.md
- docs/src/project/phase6-scheduler-migration-readiness-source-inventory.md
- tasks/2026-05-25-phase6-qemu-per-core-scheduler-ownership-smoke.md
- tasks/2026-05-25-phase6-per-core-scheduler-state-core.md
- tasks/evidence/2026-05-25-pi5-smp-lock-cache-coherence-final-proof/summary.md

## Wakeup and IPI Contract

The next boundary is raw cross-core signal delivery, not scheduler migration.
The first IPI diagnostic may prove that one core can send a selected SGI to
another core and that the receiver can acknowledge, classify, count, and EOI
that SGI. It must not wake a scheduler task, migrate a task, use a shared run
queue, or run production scheduler dispatch on secondary cores.

The first scheduler-facing contract remains:

- wakeups target tasks, not processes;
- a same-core wake may eventually enqueue a task on the owning CPU's local
  queue under that CPU's local scheduler rules;
- a remote wake may only publish a bounded wake request plus send an IPI after
  a later task accepts the exact wake-list ownership, lock ordering, and memory
  ordering;
- an IPI handler must acknowledge/classify the SGI, record bounded per-core
  evidence or wake-pending state, EOI, and return;
- IPI context must not allocate, format, print to serial, poll UART input,
  dispatch diagnostic commands, block, sleep, take long locks, walk arbitrary
  scheduler queues, migrate tasks, or cross the context-switch boundary.

If a later task introduces a shared wake list or remote enqueue queue, its lock
ordering must be local IRQ mask first, then SMP lock acquisition; release the
SMP lock before restoring local IRQ state. Scheduler locks must not be held
across talos_aarch64_context_switch.

## Selected Proof Strategy

Split the work into two tasks:

1. QEMU first: add a focused
   phase6-qemu-cross-core-ipi-delivery-smoke-20260525 task. It should extend
   the GICv2 wrapper with the minimal SGI surface, start QEMU virt with four
   CPUs through the accepted PSCI path, send a diagnostic SGI from CPU 0 to
   each secondary logical CPU, and report sender, receiver, SGI INTID,
   target-list bit, acknowledgement/EOI, per-core counts, errors, and a PASS
   classification. This remains QEMU/substitute evidence.
2. Pi 5 follow-up: after QEMU SGI delivery is accepted, plan a serialized
   phase6-pi5-cross-core-ipi-delivery-proof task before any production
   scheduler wakeup or remote enqueue uses SGIs on hardware. It must capture
   archive digest, kernel size/hash, TFTP fetch evidence, serial output,
   hardware lock acquisition/release, and restore evidence.

Only after those delivery proofs should the supervisor plan a scheduler wakeup
implementation task. That later task must name the remote wake-list or
remote-enqueue ownership model before it mutates scheduler state across cores.

## Hazards and Deferrals

- Concurrent console output from multiple cores remains deferred. IPI and
  wakeup diagnostics should publish per-core counters and print from a bounded
  owner path rather than printing directly from every handler.
- Allocation, formatting, diagnostic command dispatch, UART polling, blocking
  I/O, and sleep/wait behavior remain invalid in IPI context and while holding
  scheduler locks.
- Shared run queues, task migration, load balancing, work stealing, remote
  reschedule, multi-core timer preemption, sleep queues, wait queues, process
  address spaces, EL0, syscalls, descriptors, filesystem, networking, SSH,
  shell behavior, runtime-console concurrency, UART interrupt ownership,
  RP1/PCIe, and DMA/cache-coherent driver policy remain deferred.
- QEMU SGI proof cannot be treated as Pi 5 GIC-400 proof. Pi 5 acceptance
  requires serialized hardware evidence.

## Validation

- git status --short was clean before edits.
- Static review inspected interrupt/timer docs, scheduler docs, GICv2 source,
  QEMU and Pi 5 target sources, the accepted scheduler migration readiness
  inventory, the per-core scheduler state task, and the QEMU per-core scheduler
  ownership task.
- git diff --check passed.
- mdbook is unavailable in the container, so mdBook build was not run.
- Rust fmt/tests were not required because this task changed only Markdown
  documentation and durable task state.
