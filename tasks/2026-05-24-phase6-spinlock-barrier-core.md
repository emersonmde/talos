# Phase 6 Spinlock/Barrier Core

Status: accepted.

Task: `phase6-spinlock-barrier-core-20260524`

## Scope

Implemented the first narrow Milestone 6.2 synchronization primitive core:

- added `src/smp_sync.rs` with `SpinLock<T>`, `SpinLockGuard`,
  AArch64 `lock_irqsave()`, and `smp_full_barrier()`;
- registered the module from `src/main.rs`;
- documented the implementation boundary in the accepted Phase 6.2 project
  note, scheduler architecture note, roadmap, and decision log.

Non-goals stayed intact: no scheduler migration, shared run queue, cross-core
wakeup, IPI, userspace, descriptor, filesystem, networking, SSH, shell, UART
interrupt, RP1/PCIe, DMA, hardware publish, or hardware test behavior.

## Static Inspection

Unsafe and architecture-specific boundaries are limited to:

- `SpinLock<T>` mutable access through `UnsafeCell<T>`;
- `unsafe impl<T: Send> Sync/Send for SpinLock<T>`;
- `SpinLockGuard`'s CPU-local marker, which prevents treating a held guard as
  a cross-core transfer token;
- `IrqSpinLockGuard` manual drop ordering so the lock releases before DAIF is
  restored;
- AArch64 `dmb ish` inside `smp_full_barrier()`.

The lock uses acquire ordering on successful compare-exchange acquisition,
release ordering on unlock, relaxed failure/load polling while spinning, and a
non-recursive `try_lock()` result for tests and misuse detection.

## Validation

- static inspection: `git status --short` was clean before implementation
  edits.
- fmt/lint/typecheck: `cargo fmt --all -- --check` passed.
- unit tests: `cargo -Zjson-target-spec test` passed with 102 no_std tests.
- QEMU/substitute: `scripts/qemu-smoke.sh` passed.
- static inspection: `git diff --check` passed.
- static inspection: `mdbook` was unavailable in the container, so mdBook
  build was not run.

## Result

Accepted as the first SMP-safe primitive implementation. The next queued task
is `phase6-qemu-smp-lock-contention-smoke-20260524`, which must prove the
primitive under bounded QEMU multi-core contention before any Pi 5 primitive
proof or scheduler sharing work starts.
