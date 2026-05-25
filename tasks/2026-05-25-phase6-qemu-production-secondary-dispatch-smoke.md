# Phase 6 QEMU Production Secondary Dispatch Smoke

Task ID: phase6-qemu-production-secondary-dispatch-smoke-20260525
Status: accepted

## Goal

Prove production secondary dispatch on QEMU with CPU-local diagnostic kernel
threads and no shared scheduler mutation.

## Scope

- Added `TALOS_QEMU_PRODUCTION_SECONDARY_DISPATCH_SMOKE` and
  `scripts/qemu-production-secondary-dispatch-smoke.sh`.
- Added a QEMU-only secondary-core diagnostic that starts logical CPUs 1, 2,
  and 3 through the accepted PSCI path.
- Each secondary uses `SecondaryProductionDiagnostic` to seed and dispatch
  bounded CPU-local diagnostic kernel threads.
- The transcript records per-core current task, local queue state, dispatch
  counters, and rejected cross-owner local queue/dispatch attempts.

## Non-Goals

No Pi 5 hardware claim, shared run queue, global task registry, remote enqueue
queue, task migration, load balancing, work stealing, secondary timer
preemption, Phase 7, userspace, descriptors, filesystem, networking, SSH,
shell behavior, RP1/PCIe, UART interrupt ownership, or DMA/cache-coherent
driver policy.

## Evidence

- Static inspection: `git status --short` before edits showed a clean Talos
  worktree.
- QEMU/substitute transcript:
  `target/qemu-production-secondary-dispatch-smoke.log`.
- QEMU/substitute classification:
  `qemu-production-secondary-dispatch-complete`.
- QEMU/substitute evidence: logical CPUs 1, 2, and 3 reported
  `role=secondary-production-diagnostic`, `production=true`, three bounded
  local dispatches each, queue length 0 after dispatch, stable current task
  IDs 203/303/403, rejected cross-owner local queue mutation, rejected
  cross-owner production dispatch, and `ok=true`.
- Validation gates passed: `cargo fmt --all -- --check`,
  `cargo -Zjson-target-spec test`, the focused QEMU smoke,
  `scripts/qemu-smoke.sh`, retained QEMU per-core scheduler ownership and
  remote wake-to-local-runnable smokes, `mdbook build`, and `git diff --check`.

## Acceptance

Accepted as QEMU substitute evidence for the first production secondary
dispatch slice. Serialized Pi 5 hardware proof, shared run queues, global task
lookup, task migration, load balancing, multi-core preemption, Phase 7 behavior,
filesystem, networking, SSH, shell behavior, RP1/PCIe, UART interrupt
ownership, and DMA behavior remain deferred.
