# Phase 6 QEMU Secondary Scheduler Service Loop Smoke

Status: accepted.

Task id: phase6-qemu-secondary-scheduler-service-loop-smoke-20260526

## Goal

Prove the secondary scheduler service loop on the QEMU virt substitute before
any physical Pi 5 claim.

## Scope

- Add a focused QEMU smoke for the accepted secondary scheduler service-loop
  adapter.
- Start secondary cores through the accepted PSCI/QEMU flow.
- Exercise owner-local service-loop execution, remote wake drain, local
  dispatch, metadata refresh, and retained explicit deferrals.
- Keep the transcript deterministic with a bounded PASS/classification line.

## Non-Goals

- No Pi 5 hardware claim.
- No shared run queues, task migration, load balancing, work stealing,
  multi-core preemption, or remote enqueue queues.
- No descriptor, syscall, userspace, filesystem, networking, SSH, or shell
  behavior.

## Evidence

- Implementation: src/target/qemu_virt.rs, src/main.rs, build.rs.
- Retained gate: scripts/qemu-secondary-scheduler-service-loop-smoke.sh.
- Architecture update: docs/src/architecture/scheduler.md.
- QEMU transcript: target/qemu-secondary-scheduler-service-loop-smoke.log.

## Validation

- git status --short before edits: clean.
- fmt/lint: cargo fmt --all -- --check passed.
- unit tests: cargo -Zjson-target-spec test passed, 134 no_std tests.
- QEMU/substitute baseline: scripts/qemu-smoke.sh passed with
  talos: qemu smoke PASS.
- focused QEMU/substitute: scripts/qemu-secondary-scheduler-service-loop-smoke.sh
  passed with classification qemu-secondary-scheduler-service-loop-complete.
- whitespace inspection: git diff --check passed.
- documentation: mdbook build passed.

## Result

Accepted. The QEMU transcript shows all three secondary cores reached
workload-complete through the PSCI/QEMU path and reported owner-local
service-loop execution: remote wake drain, local dispatch, no-work metadata
refresh, cross-owner rejection, deferred-role rejection, queue length zero,
and final PASS/classification output. No physical hardware claim is made.
