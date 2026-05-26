# Phase 6 Secondary Scheduler Service Loop Core

Status: accepted.

Task id: phase6-secondary-scheduler-service-loop-core-20260526

## Goal

Implement the target-independent secondary scheduler service-loop adapter
selected by the accepted source inventory.

## Scope

- Added a narrow normal-control-flow secondary adapter around
  `CpuLocalSchedulerService::run_cycle`.
- Kept scheduler mutation owner-local: the requester must be the local
  secondary owner, boot CPU use is rejected, and deferred secondary roles
  remain rejected.
- Preserved `SecondaryProductionDiagnostic` as the only accepted secondary
  production role for this slice.
- Added focused unit coverage for wrong-owner, deferred-role, no-work,
  remote-wake, timer-preemption, dispatch, and metadata-refresh outcomes.

## Non-Goals

- No shared run queues, remote enqueue queues, task migration, load balancing,
  work stealing, or multi-core preemption.
- No Pi 5 hardware claim.
- No descriptor, syscall, userspace, filesystem, networking, SSH, or shell
  behavior.
- No RP1/PCIe, UART interrupt ownership, or DMA/cache-driver policy.

## Evidence

- Implementation: `src/scheduler.rs`.
- Architecture update: `docs/src/architecture/scheduler.md`.

## Validation

- `git status --short` before edits: clean.
- fmt/lint: `cargo fmt --all -- --check` passed.
- unit tests: `cargo -Zjson-target-spec test` passed, 134 no_std tests.
- QEMU/substitute: `scripts/qemu-smoke.sh` passed with
  `talos: qemu smoke PASS`.
- whitespace inspection: `git diff --check` passed.
- documentation: `mdbook build` passed.

## Result

Accepted. Commit hash recorded in durable supervisor state.
