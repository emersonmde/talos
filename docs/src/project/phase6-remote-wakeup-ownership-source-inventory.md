# Phase 6 Remote Wakeup Ownership Source Inventory

Task ID: phase6-remote-wakeup-ownership-source-inventory-20260525

## Status

Accepted.

## Goal

Define the first scheduler-facing remote wake-request ownership model before
any implementation can publish cross-core scheduler work or depend on IPIs for
task wakeups.

## Source Inventory

Current scheduler facts:

- `src/scheduler.rs` still owns fixed FIFO `RunnableQueue` instances through
  `SingleCoreScheduler`; there is no global task lookup, shared run queue,
  migration queue, sleep queue, wait queue, or production secondary dispatch.
- `PerCoreScheduler` records a logical CPU owner, `SchedulerCoreRole`, local
  scheduler state, and a current-task slot. `local_scheduler_mut()` rejects
  the wrong owner, while `production_scheduler_mut()` still rejects deferred
  secondary owners.
- CPU 0 remains the only production scheduler owner. Secondary per-core
  scheduler state is diagnostic and deferred from production dispatch.
- `TaskId` is scheduler-local. It names a scheduler task, not a POSIX process,
  descriptor, userspace thread, or global kernel object.

Current synchronization facts:

- `src/smp_sync.rs` provides `SpinLock<T>` with acquire ordering on lock
  acquisition and release ordering on unlock.
- The AArch64 `lock_irqsave()` composition records the required ordering for
  shared scheduler state: mask local IRQs first, acquire the SMP lock second,
  release the lock first, then restore the saved IRQ mask state.
- `smp_full_barrier()` names the first shared-memory barrier boundary for
  places that must publish data before sending a cross-core signal.
- Cache maintenance is not hidden inside the generic lock. Accepted Pi 5 SMP
  evidence requires participating cores to share the cacheable EL2 stage-1
  regime before generic shared-memory lock claims are valid.

Current IPI facts:

- `docs/src/architecture/interrupts-timers.md` records the raw GICv2 SGI
  contract. QEMU and Pi 5 both use GICv2/GIC-400 surfaces with GICC_IAR,
  GICC_EOIR, and GICD_SGIR.
- `src/arch/aarch64/gicv2.rs` exposes distributor and CPU-interface enablement,
  SGI priority setup, target-list and all-except-self SGIR writes, acknowledge,
  EOI, pending/active inspection, and highest-pending reads.
- `phase6-qemu-cross-core-ipi-delivery-smoke-20260525` accepted QEMU raw SGI
  delivery for SGI INTID 1 to logical CPUs 1, 2, and 3.
- `phase6-pi5-cross-core-ipi-delivery-proof-20260525` accepted serialized Pi 5
  raw SGI delivery after the Pi 5 IRQ dispatcher included the cross-core IPI
  proof path. The accepted hardware run used SGIR all-except-self, receivers
  1, 2, and 3 each reported `receive-count=1 eoi-count=1 intid=1`, and the
  final classification was `pi5-cross-core-ipi-delivery-complete`.

## Selected Model

The first remote wakeup shape is a bounded per-target remote wake-request list,
not direct remote enqueue and not a shared run queue.

Each target logical CPU owns a small `RemoteWakeQueue`-style structure for
requests addressed to that CPU. A remote requester may publish a request into
the target's queue while holding the queue lock, then send SGI INTID 1 to the
target after the request is visible. The target CPU is the only CPU allowed to
consume its remote wake requests and decide what local scheduler action, if
any, follows.

The first implementation proof is diagnostic-only. It may prove that a request
for a `TaskId` is published, signaled, observed, and consumed by the target CPU
without mutating another CPU's local runnable queue. It must not make secondary
production dispatch available, migrate tasks, or place a task on a remote
`RunnableQueue`.

This model is selected over direct remote enqueue because the accepted
`PerCoreScheduler` boundary deliberately keeps runnable queues CPU-local.
Mutating another CPU's local queue from a remote sender would bypass that
ownership boundary and would force a global task-state policy before Talos has
sleep queues, a task registry, migration rules, or production secondary
scheduler dispatch.

## Ownership Rules

- Wake requests target scheduler `TaskId` values and target logical CPUs.
- A self-targeted wake request should use the current CPU's local wake path in
  a later task; it does not need an IPI.
- A remote sender owns only the act of publishing a bounded request and sending
  the SGI after publication.
- The target CPU owns request consumption and any future transition from
  `Blocked` to `Runnable`.
- The IPI handler owns only acknowledge, classify, bounded per-core
  wake-pending accounting, EOI, and return.
- Request consumption must happen after IPI context. It may be polled by the
  focused QEMU diagnostic or later reached from scheduler-return code, but it
  must not run inside the IRQ hot path.
- CPU 0 remains the only production scheduler owner in this slice. A focused
  QEMU proof may use secondary CPUs as diagnostic owners for remote request
  consumption counters, not for production runnable-queue dispatch.

## Lock And Memory Ordering

The request publication path must use this order:

1. Mask local IRQs.
2. Acquire the target remote-wake queue's `SpinLock<T>`.
3. Insert or coalesce the request.
4. Release the spin lock.
5. Restore the saved local IRQ mask state.
6. Execute the named shared-memory publish barrier if the implementation needs
   one beyond the lock release boundary.
7. Send SGI INTID 1 to the target CPU.

The target consumption path must use this order:

1. Run outside IPI context.
2. Mask local IRQs.
3. Acquire the current CPU's remote-wake queue lock.
4. Drain or snapshot bounded requests into target-owned diagnostic or scheduler
   state.
5. Release the lock.
6. Restore the saved local IRQ mask state.

The implementation must not hold a remote-wake queue lock across
`talos_aarch64_context_switch`, serial output, allocation, formatting, TTY
polling, diagnostic command dispatch, blocking, sleeping, task migration, or
arbitrary callbacks.

## Duplicate And Error Semantics

The first implementation should treat duplicate pending requests for the same
target `TaskId` as a coalesced wake:

- the original request remains pending;
- the duplicate does not consume another queue slot;
- a duplicate counter may be incremented for evidence;
- the duplicate is not a fatal error.

Queue-full conditions are explicit errors for the caller and must be counted in
the focused proof. Invalid target CPU IDs, zero/invalid `TaskId` values, and
self-targeting through the remote path should be rejected or routed to a local
path by the implementation task; they must not silently mutate scheduler state.

## Deferrals

The inventory does not accept:

- direct remote enqueue into another CPU's local runnable queue;
- shared run queues, global task lookup, task migration, load balancing, or
  work stealing;
- production secondary scheduler dispatch;
- sleeping locks or blocking in scheduler-locked paths;
- lower-EL task state, POSIX processes, descriptors, filesystems, networking,
  SSH, shell behavior, RP1/PCIe, UART interrupt ownership, or DMA behavior;
- a Pi 5 scheduler-facing remote wakeup proof. The accepted Pi 5 SGI proof is
  raw interrupt-delivery evidence only.

## Next Bounded Task

The next implementation proof is
`phase6-qemu-remote-wakeup-request-smoke-20260525`.

That task should add only the selected bounded request-list model and a focused
QEMU transcript proving request publication, SGI signaling, target-side
observation, target-owned consumption, duplicate behavior, and zero unexpected
cross-owner runnable-queue mutation.

## Validation

- Static inspection: `git status --short` before edits showed a clean Talos
  worktree.
- Static review: scheduler, SMP sync, GICv2, interrupts/timers, scheduler
  architecture docs, accepted QEMU raw IPI task, accepted Pi 5 raw IPI task,
  and Pi 5 evidence summary were reviewed.
- Whitespace inspection: `git diff --check` passed.
- mdBook: `mdbook` was unavailable in the container, so `mdbook build` was not
  run.
