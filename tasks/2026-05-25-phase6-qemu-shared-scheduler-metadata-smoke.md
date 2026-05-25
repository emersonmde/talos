# Phase 6 QEMU Shared Scheduler Metadata Smoke

Task ID: phase6-qemu-shared-scheduler-metadata-smoke-20260525

## Goal

Prove the shared scheduler metadata invariant under QEMU SMP before any
physical hardware claim or migration work.

## Scope

- Added a focused QEMU diagnostic flag and script:
  TALOS_QEMU_SHARED_SCHEDULER_METADATA_SMOKE and
  scripts/qemu-shared-scheduler-metadata-smoke.sh.
- Started four QEMU logical CPUs through the accepted PSCI path.
- Had CPU-local diagnostic tasks on logical CPUs 0 through 3 publish/query the
  shared metadata boundary.
- Reported owner CPU, local task ID, metadata generation, lookup results,
  rejected cross-owner scheduler mutation, rejected cross-owner metadata
  publication, retained local runnable ownership, errors, and PASS
  classification.

## Non-Goals

No Pi 5 hardware run, shared run queue, task migration, load balancing,
production task movement beyond explicitly seeded CPU-local diagnostic tasks,
remote enqueue queue, multi-core preemption, Phase 7, filesystem, networking,
SSH, shell behavior, RP1/PCIe, UART interrupt ownership, or DMA behavior was
added.

## Implementation Summary

- build.rs: registered the new QEMU cfg and assembly define.
- src/arch/aarch64/boot.S: included the new QEMU proof flag in the secondary
  stack and entry stub surface.
- src/main.rs: routed the new QEMU diagnostic to
  target::qemu_virt::run_shared_scheduler_metadata_smoke.
- src/target/qemu_virt.rs: added the shared metadata smoke state, shared
  metadata lock/table, per-core report construction, secondary runner, and
  final QEMU classification.
- scripts/qemu-shared-scheduler-metadata-smoke.sh: builds and checks the
  focused QEMU transcript.

## Evidence

- QEMU/substitute: scripts/qemu-shared-scheduler-metadata-smoke.sh passed. The
  transcript is target/qemu-shared-scheduler-metadata-smoke.log and reports
  classification=qemu-shared-scheduler-metadata-complete.
- The focused transcript reports logical CPUs 0 through 3 publishing task IDs
  101, 201, 301, and 401, read-only lookup success, boot-task lookup success,
  rejected cross-owner local scheduler mutation, rejected cross-owner metadata
  publication, preserved local runnable queues, final-metadata-len=4, errors=0,
  and PASS.
- Unit tests: cargo -Zjson-target-spec test passed with 125 no_std tests.
- QEMU/substitute: scripts/qemu-smoke.sh passed.
- QEMU/substitute: scripts/qemu-production-secondary-dispatch-smoke.sh passed
  with classification=qemu-production-secondary-dispatch-complete.
- QEMU/substitute: scripts/qemu-remote-wake-to-local-runnable-smoke.sh passed
  with classification=qemu-remote-wake-to-local-runnable-complete.
- Formatting/docs: cargo fmt --all -- --check, git diff --check, and mdbook
  build passed.

## Acceptance

Accepted as QEMU substitute evidence for the shared scheduler metadata
invariant. The next queued task is the serialized Pi 5 shared scheduler
metadata proof, not shared run queues, task migration, load balancing,
multi-core preemption, or later roadmap work.
