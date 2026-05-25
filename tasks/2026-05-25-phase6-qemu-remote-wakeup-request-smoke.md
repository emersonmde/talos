# Phase 6 QEMU Remote Wake-Request Smoke

Task ID: phase6-qemu-remote-wakeup-request-smoke-20260525

## Status

Accepted.

## Goal

Prove the first scheduler-facing remote wake-request path under QEMU without
enabling broad scheduler migration.

## Scope

- Added a bounded `RemoteWakeQueue` model in `src/scheduler.rs` for
  target-owned wake requests over scheduler-local `TaskId` values.
- Added QEMU-only remote wake-request diagnostic wiring behind
  `TALOS_QEMU_REMOTE_WAKEUP_REQUEST_SMOKE` and
  `scripts/qemu-remote-wakeup-request-smoke.sh`.
- Proves CPU 0 request publication, duplicate coalescing, SGI INTID 1
  signaling, target-side IPI observation/EOI, and target-owned request
  consumption on logical CPUs 1, 2, and 3.
- Keeps CPU 0 as the only production scheduler owner; secondary CPUs are
  diagnostic owners for request consumption counters only.

## Non-Goals

No shared run queues, task migration, load balancing, work stealing,
production secondary-core scheduling, Pi 5 scheduler wakeup claim, Phase 7,
userspace, descriptors, filesystem, networking, SSH, shell behavior,
RP1/PCIe, UART interrupt ownership, or DMA behavior.

## Evidence

- Static inspection: `git status --short` before edits showed a clean Talos
  worktree.
- Unit tests: `cargo -Zjson-target-spec test` passed 113 no_std tests,
  including remote wake queue duplicate, self-target, wrong-target, overflow,
  and cross-owner-consumption rejection coverage.
- QEMU/substitute transcript:
  `target/qemu-remote-wakeup-request-smoke.log`.
- QEMU/substitute classification:
  `qemu-remote-wakeup-request-complete`.
- QEMU/substitute evidence: CPU 0 published requests for targets 1, 2, and 3;
  target 1 duplicate publication was coalesced; SGI INTID 1 was sent with
  target-list bits 0x02, 0x04, and 0x08; each target observed one SGI, EOId
  once, consumed its own task ID, drained to queue length 0, rejected
  cross-owner local scheduler mutation, and kept production dispatch deferred.
- Retained QEMU gates: `scripts/qemu-smoke.sh`,
  `scripts/qemu-cross-core-ipi-delivery-smoke.sh`, and
  `scripts/qemu-per-core-scheduler-ownership-smoke.sh` passed.
- Formatting/static checks: `cargo fmt --all -- --check` and
  `git diff --check` passed.
- Static inspection: `mdbook` is unavailable in the container; mdBook build
  was not run.

## Acceptance

Accepted as a QEMU-only remote wake-request proof. The implementation proves
bounded request publication, SGI signaling, target-owned post-IPI
observation/consumption, duplicate coalescing, and preserved local scheduler
ownership. It does not authorize shared run queues, task migration, production
secondary-core scheduling, Pi 5 scheduler wakeup claims, Phase 7 behavior,
filesystem, networking, SSH, shell behavior, RP1/PCIe, UART interrupt
ownership, or DMA behavior.
