# Phase 4 Closeout Checkpoint

Status: accepted for Phase 4 interrupts, timers, and single-core scheduler
preemption. Phase 5 planning may start with the queued local console/device
model source inventory; Phase 5 implementation must remain gated by that
bounded inventory task.

## Scope

This checkpoint reconciles the accepted Phase 4 evidence before Talos leaves
interrupt, timer, and scheduler bring-up. It does not add new kernel code,
boot-image behavior, hardware execution, console/TTY implementation, userspace,
file descriptors, filesystems, networking, SSH, SMP, or lower-EL support.

## Accepted Capabilities

- QEMU virt and Raspberry Pi 5 both use the EL2 hypervisor physical timer path:
  `CNTHP_*_EL2` plus GICv2/GIC-400 PPI 10 / INTID 26.
- The current-EL IRQ frame path preserves the interrupted register frame,
  calls the Rust IRQ handler with vector, ELR, SPSR, and frame state, restores
  the frame, and returns with `ERET`.
- The timer IRQ hot path acknowledges with `GICC_IAR`, classifies INTID 26,
  records bounded tick/request state, reprograms `CNTHP_CVAL_EL2`, writes
  `GICC_EOIR`, and returns. It must not allocate, format, print, block, sleep,
  call scheduler dispatch, walk runnable queues, or perform context switches.
- Monotonic tick accounting reaches the four-tick periodic proof target on QEMU
  and Pi 5 while preserving post-tick workload progress.
- `single_core_irq_mask_save()` / `single_core_irq_restore()` protect short
  boot-CPU mutation windows. They are not SMP locks, blocking locks, sleepable
  locks, lower-EL interrupt policy, or general scheduler locks.
- The scheduler has a single-core, kernel-thread-first data model with
  scheduler-local task IDs, fixed runnable queue, task states, per-task kernel
  stacks, and EL2 cooperative switch frames.
- QEMU proves cooperative context switching, voluntary-yield dispatch, and
  timer-driven scheduler handoff between two kernel threads.
- Pi 5 hardware proves the same timer-driven preemption handoff shape through
  the physical GIC-400 and EL2 timer path.

## Evidence

- Phase 4 source inventory: commit `0fb6260`; architecture note
  `docs/src/architecture/interrupts-timers.md`.
- IRQ frame foundation: commit `de40482`.
- QEMU EL2 timer IRQ smoke: commit `bce215d`; task record
  `tasks/2026-05-24-phase4-qemu-el2-timer-irq-smoke.md`; QEMU log
  `target/qemu-timer-irq-smoke.log`.
- Pi 5 EL2 timer IRQ smoke: commit `966d453`; task record
  `tasks/2026-05-24-phase4-pi5-el2-timer-irq-smoke.md`; serialized hardware
  evidence in `tasks/evidence/2026-05-24-pi5-el2-timer-irq-smoke/`.
- Timer-smoke checkpoint: commit `957bbc8`; checkpoint
  `docs/src/project/phase4-timer-smoke-checkpoint.md`.
- Monotonic tick accounting: commit `54d2075`; task record
  `tasks/2026-05-24-phase4-monotonic-tick-accounting.md`; serialized hardware
  evidence in `tasks/evidence/2026-05-24-pi5-monotonic-tick-accounting/`.
- Single-core IRQ mask policy: commit `1bbfec6`; task record
  `tasks/2026-05-24-phase4-interrupt-mask-critical-section-policy.md`.
- Phase 4 pre-scheduler closeout: commit `68e3529`; checkpoint
  `docs/src/project/phase4-prescheduler-closeout.md`.
- Scheduler shape: commit `37ce658`; architecture note
  `docs/src/architecture/scheduler.md`.
- Scheduler structs and runnable queue: commit `7ce1a91`; task record
  `tasks/2026-05-24-phase4-scheduler-structs-runnable-queue.md`.
- Cooperative context-switch contract: commit `988ea31`; task record
  `tasks/2026-05-24-phase4-context-switch-contract.md`.
- Cooperative context-switch QEMU smoke: commit `6f24076`; task record
  `tasks/2026-05-24-phase4-cooperative-context-switch-qemu-smoke.md`.
- Voluntary-yield dispatch: commit `24c25c6`; task record
  `tasks/2026-05-24-phase4-voluntary-yield-dispatch.md`.
- Preemption-entry policy checkpoint: commit `8134e7c`; checkpoint
  `docs/src/project/phase4-preemption-entry-checkpoint.md`.
- QEMU timer-preemption smoke: commit `2cf0e64`; task record
  `tasks/2026-05-24-phase4-timer-preemption-qemu-smoke.md`; QEMU log
  `target/qemu-timer-preemption-smoke.log`.
- Pi 5 timer-preemption hardware proof: commit `9e53676`; task record
  `tasks/2026-05-24-phase4-pi5-timer-preemption-hardware-proof.md`;
  serialized hardware evidence in
  `tasks/evidence/2026-05-24-pi5-timer-preemption-hardware-proof/`.
- Scheduler/preemption contract consolidation: commit `f1e0cd2`; task record
  `tasks/2026-05-24-phase4-scheduler-preemption-contract-consolidation.md`;
  architecture note `docs/src/architecture/scheduler.md`.

Hardware proof quality:

- The Pi 5 EL2 timer IRQ smoke captured TFTP proof for an 86,429-byte
  `kernel_2712.img`, serial output with `irq-count=1`, `intid=26`,
  `unexpected=0`, post-IRQ progress, and `PASS`, then restored the pre-run
  snapshot.
- The Pi 5 monotonic tick proof captured accepted evidence on a later
  serialized run after an initial firmware-only serial timeout. The accepted
  run fetched the 86,661-byte kernel image, reported `tick-count=4 target=4`,
  `intid=26`, `unexpected=0`, and `PASS`, then restored the pre-run snapshot.
- The Pi 5 timer-preemption proof captured TFTP proof for a 103,152-byte
  `kernel_2712.img`; a follow-up serial observe after fresh TFTP evidence
  reported `task1=3`, `task2=3`, `ticks=6`, `requests=6`, `handled=6`,
  `timer-preemptions=6`, `dispatch-switches=6`, `voluntary-yields=0`,
  `intid=26`, `unexpected=0`, and `PASS`, then restored the pre-run snapshot.

## Deferred Work

- Real quantum policy, fairness policy, preemption-disable counters, sleeping,
  blocking waits, timer-driven wakeups, and scheduler wait queues.
- Asynchronous exception-frame task switching from arbitrary interrupted
  caller-saved state; the accepted handoff leaves IRQ context before scheduler
  dispatch and context switch.
- SMP, secondary-core startup, per-core timer state, interrupt routing,
  cross-core wakeups, scheduler locking, task migration, memory-ordering rules,
  and interrupt-safe lock hierarchy.
- Lower-EL execution, user address spaces, syscall ABI, process IDs, process
  lifetime, descriptor tables, pipes, TTY-backed stdio, VFS, filesystem,
  program loading, libc/Rust std, and local shell.
- UART interrupts, runtime console implementation, input path, line discipline,
  PTYs, local diagnostic command channel, networking, SSH, RP1/PCIe, DMA, MSI,
  IOMMU, cache-coherent driver policy, and real POSIX clocks.

## Risks

- Phase 4 scheduler proofs are diagnostic boot surfaces, not stable kernel
  interfaces. They should be removed or replaced once a runtime diagnostic
  command channel can report equivalent state.
- The scheduler is single-core only. Reusing the current IRQ-mask primitive as
  a future SMP lock would be incorrect.
- Timer-driven preemption is proven as a bounded handoff, not as a complete
  fair scheduler with sleep, wakeup, quantum accounting, or process resource
  ownership.
- The console path is still early logging. Phase 5 must first inventory current
  early serial/printing ownership before adding a runtime console abstraction.

## Go/No-Go Decision

Go for Phase 5 planning. The next task is
`phase5-console-device-model-source-inventory-20260524`.

That task may inventory source code and document the early/runtime console
ownership boundary. It must not implement console/TTY behavior, descriptor
tables, userspace, file systems, networking, SSH, or shell behavior. If that
inventory finds a Phase 4 regression or missing interrupt/timer evidence, the
worker should create or request a narrow remaining Phase 4 task instead of
broadening Phase 5.

## Validation

- static inspection: `git status --short` was clean before checkpoint edits.
- fmt/lint/typecheck: `git diff --check` passed for this checkpoint.
- fmt/lint/typecheck: `git diff --cached --check` passed for this checkpoint.
- static inspection: `mdbook` was unavailable in the container, so the mdBook
  build was not run.
