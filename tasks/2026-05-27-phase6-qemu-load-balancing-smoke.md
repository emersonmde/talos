# Phase 6 QEMU Load-Balancing Smoke

## Task

- Title: Phase 6 QEMU load-balancing smoke
- Owner: worker
- Date: 2026-05-27
- Milestone: Phase 6.3, Multi-Core Preemptive Scheduler
- Scope: deterministic QEMU substitute proof for the accepted load-balancing
  core

## Goal

Prove the implemented load-balancing core in QEMU without making a physical
Pi 5 claim.

## Acceptance Criteria

- QEMU transcript proves the named load-balancing invariant through the
  implemented core.
- Failure and final classification strings are deterministic and recorded.
- No physical Pi 5 behavior is claimed.
- Accepted QEMU proof is committed before any Pi 5 proof task starts.

## Context

The target-independent load-balancing core was accepted in
`tasks/2026-05-27-phase6-load-balancing-core.md`. This task adds only a
focused QEMU diagnostic scenario and runner that drive
`LoadBalancingPolicy::plan_front_runnable`,
`LoadBalancingPolicy::publish_front_runnable`, and the accepted
`SharedRunQueue::consume_for_destination` path.

## Work Performed

- Added the `qemu_load_balancing_smoke` boot scenario.
- Added `scripts/qemu-load-balancing-smoke.sh`.
- Added `target::qemu_virt::run_load_balancing_smoke()`.
- The diagnostic builds a source owner, destination owner, shared metadata
  table, shared run queue, and runnable task. It proves deterministic
  front-runnable selection, source-local removal, shared handoff, destination
  enqueue, metadata owner refresh, and final PASS classification.
- Updated scheduler architecture, roadmap status, and decision log entries for
  the accepted QEMU substitute proof.

## Evidence

- static inspection: changes are bounded to `build.rs`, `src/main.rs`,
  `src/target/qemu_virt.rs`, `scripts/qemu-load-balancing-smoke.sh`,
  scheduler/roadmap/decision docs, and this task record.
- git status: `git status --short` was clean before edits.
- QEMU/substitute: `scripts/qemu-load-balancing-smoke.sh` passed and emitted
  `classification=qemu-load-balancing-smoke-complete`.
- QEMU transcript: `target/qemu-load-balancing-smoke.log` shows source owner 0
  selecting task 109 for destination owner 1, plan generation 1 matching
  registered generation 1, source queue length changing 1 -> 0, shared queue
  length changing 1 -> 0, destination queue length becoming 1 with front task
  109, metadata owner changing to 1, metadata generation changing to 2, and
  PASS.
- fmt/lint: `cargo fmt --all -- --check` passed.
- unit tests: `cargo -Zjson-target-spec test` passed.
- QEMU/substitute: `scripts/qemu-smoke.sh` passed.
- QEMU/substitute: `scripts/qemu-shared-runqueue-migration-smoke.sh` passed.
- QEMU/substitute: `scripts/qemu-secondary-scheduler-service-loop-smoke.sh`
  passed, preserving the accepted owner-local secondary service-loop gate.
- whitespace inspection: `git diff --check` passed.
- documentation: `mdbook build` passed.

## Diagnostic Surface Notes

`qemu_load_balancing_smoke` and
`scripts/qemu-load-balancing-smoke.sh` are retained Phase 6.3 QEMU substitute
proof surfaces until a later load-balancing closeout either keeps them as
regression gates or explicitly retires/replaces them. They do not add Pi 5
staging, work stealing, running-task migration, multi-core preemption,
secondary runtime policy, Phase 7, filesystem, networking, SSH, shell
behavior, RP1/PCIe, UART interrupt ownership, or DMA/cache-driver behavior.

## Review

- Pre-hardware review findings: hardware is not required; this task makes no
  physical claim.
- Hardware test evidence, if required: not required.
- Post-hardware review findings: not applicable.

## Result

Accepted as QEMU substitute evidence for the load-balancing core. The next
bounded task may be the serialized Pi 5 load-balancing proof only after
supervisor ready-marking and hardware lock availability.
