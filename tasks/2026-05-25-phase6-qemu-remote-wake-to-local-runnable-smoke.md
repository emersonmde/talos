# Phase 6 QEMU Remote Wake To Local Runnable Smoke

Task ID: phase6-qemu-remote-wake-to-local-runnable-smoke-20260525
Status: accepted

## Goal

Prove under QEMU that a target CPU can consume a remote wake request and
perform only a target-owned local blocked-to-runnable transition.

## Scope

- Added `PerCoreScheduler::wake_blocked_local_task_from_remote_request()` as
  the narrow target-owned local wake boundary after request drain.
- Added unit coverage for successful blocked-to-runnable wake consumption,
  cross-owner rejection, task mismatch rejection, and duplicate local runnable
  rejection.
- Added `scripts/qemu-remote-wake-to-local-runnable-smoke.sh`, a QEMU-only
  diagnostic that extends the accepted remote wake-request path.
- Kept SGI INTID 1 IPI handling limited to acknowledge/classify/record/EOI;
  local runnable mutation happens after the target has left IPI context and
  drained its owned request queue.

## Non-Goals

No Pi 5 hardware claim, shared run queue, global task registry, remote enqueue
queue, task migration, load balancing, work stealing, production secondary
scheduler dispatch, multi-core preemption, Phase 7, userspace, descriptors,
filesystem, networking, SSH, shell behavior, RP1/PCIe, UART interrupt
ownership, or DMA/cache-coherent driver policy.

## Evidence

- Static inspection: `git status --short` before edits showed a clean Talos
  worktree.
- Unit tests: `cargo -Zjson-target-spec test` passed 116 no_std tests with the
  QEMU tool path configured.
- QEMU/substitute transcript:
  `target/qemu-remote-wake-to-local-runnable-smoke.log`.
- QEMU/substitute classification:
  `qemu-remote-wake-to-local-runnable-complete`.
- QEMU/substitute evidence: CPU 0 published requests for targets 1, 2, and 3;
  target 1 duplicate publication was coalesced; SGI INTID 1 was sent with
  target-list bits 0x02, 0x04, and 0x08; each target observed and EOId one
  SGI, consumed one target-owned request, drained its queue to length 0,
  transitioned one local diagnostic task from `Blocked` to `Runnable`, rejected
  duplicate local enqueue, rejected cross-owner scheduler mutation, and kept
  production dispatch deferred.
- Retained QEMU gates passed:
  `scripts/qemu-remote-wakeup-request-smoke.sh`,
  `scripts/qemu-cross-core-ipi-delivery-smoke.sh`,
  `scripts/qemu-per-core-scheduler-ownership-smoke.sh`, and
  `scripts/qemu-smoke.sh`.
- Formatting/static checks: `cargo fmt --all -- --check` and
  `git diff --check` passed.

## Acceptance

Accepted as a QEMU-only target-owned wake-consumption proof. This accepts the
bounded local `Blocked -> Runnable` transition only for a target-owned
diagnostic task after target-owned request drain. Pi 5 hardware proof, shared
run queues, global task lookup, task migration, production secondary scheduler
dispatch, multi-core preemption, Phase 7 behavior, filesystem, networking,
SSH, shell behavior, RP1/PCIe, UART interrupt ownership, and DMA behavior
remain deferred.
