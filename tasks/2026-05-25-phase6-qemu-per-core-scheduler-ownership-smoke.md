# Phase 6 QEMU Per-Core Scheduler Ownership Smoke

Task: phase6-qemu-per-core-scheduler-ownership-smoke-20260525
Status: accepted

## Goal

Prove under QEMU substitute evidence that logical CPUs 0 through 3 can each own
and report a bounded per-core scheduler state snapshot without enabling task
migration, shared run queues, IPIs, cross-core wakeups, or secondary-core
production scheduling.

## Implementation

- Added `TALOS_QEMU_PER_CORE_SCHEDULER_OWNERSHIP_SMOKE` as a gated QEMU
  diagnostic path in `build.rs`, `src/arch/aarch64/boot.S`, and `src/main.rs`.
- Added `scripts/qemu-per-core-scheduler-ownership-smoke.sh`, which builds the
  gated image, runs QEMU virt with four CPUs, and checks the PASS
  classification plus per-core report invariants.
- Added QEMU diagnostic logic in `src/target/qemu_virt.rs` that starts
  secondary cores through PSCI, builds per-core `PerCoreScheduler` ownership
  reports, exercises bounded local runnable/progress accounting, checks the
  accepted IRQ-mask probe, and reports final classification outside hot paths.
- Updated `docs/src/architecture/scheduler.md` with the accepted QEMU
  substitute evidence boundary.

## Evidence

- Changed scheduler/SMP files: `src/target/qemu_virt.rs`,
  `src/arch/aarch64/boot.S`, `src/main.rs`, `build.rs`, and
  `scripts/qemu-per-core-scheduler-ownership-smoke.sh`.
- QEMU/substitute transcript:
  `target/qemu-per-core-scheduler-ownership-smoke.log`.
- QEMU/substitute classification:
  `qemu-per-core-scheduler-ownership-complete`.
- Per-core summary:
  - logical 0: owner 0, role `boot-production`, progress 4, transitions 4,
    errors 0, ok true.
  - logical 1: owner 1, role `secondary-deferred`, progress 4, transitions 4,
    dispatch deferred true, errors 0, ok true.
  - logical 2: owner 2, role `secondary-deferred`, progress 4, transitions 4,
    dispatch deferred true, errors 0, ok true.
  - logical 3: owner 3, role `secondary-deferred`, progress 4, transitions 4,
    dispatch deferred true, errors 0, ok true.
- Final summary: participants 4, expected 4, errors 0, lock available true,
  IRQ-mask probe ok true, PASS.

## Validation

- fmt/lint: `cargo fmt --all -- --check` passed.
- unit tests: `cargo -Zjson-target-spec test` passed with 108 no_std tests.
- QEMU/substitute: `scripts/qemu-smoke.sh` passed.
- QEMU/substitute: `scripts/qemu-per-core-scheduler-ownership-smoke.sh` passed.
- QEMU/substitute: `scripts/qemu-secondary-core-workload-smoke.sh` passed.
- QEMU/substitute: `scripts/qemu-smp-lock-contention-smoke.sh` passed.
- whitespace inspection: `git diff --check` passed.
- whitespace inspection: `git diff --cached --check` passed.

## Acceptance

Accepted as the QEMU substitute proof for the first Phase 6.3 per-core
scheduler ownership boundary. This task does not authorize shared run queues,
task migration, IPIs, cross-core wakeups, or secondary-core production
scheduler dispatch.
