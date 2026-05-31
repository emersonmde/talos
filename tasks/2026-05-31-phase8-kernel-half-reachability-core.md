# Phase 8 Kernel-Half Reachability Core Task

Task: phase8-kernel-half-reachability-core-20260531
Status: accepted

## Scope

Implemented the target-independent KernelHalfReachabilityPlan preflight
boundary selected by the accepted contract and QEMU/substitute smoke plan. The
implementation adds src/kernel_half_reachability.rs and wires it into
src/main.rs for test ownership only.

## Accepted Boundary

- boundary identity: phase8-kernel-half-reachability-plan-v1
- policy identity: preflight-ttbr1-shared-kernel-root-reachability-v1
- TTBR0 intent: copied materialized-process-root provenance; no write
- TTBR1 intent: shared-privileged-kernel-root policy only
- descriptor image: blocked-no-kernel-half-descriptor-image
- compatibility: split TCR record-only and normal/device MAIR record-only
- blockers: live register sequence, ASID allocation, live TLBI, activation
  barriers, lower-EL ERET, runnable publication, process lifecycle, startup
  ABI expansion, filesystem syscalls, and Pi 5 hardware proof

The plan consumes accepted loader, process install, address-space,
page-table-materialization, initial launch, initial user stack, and
live-address-space activation records. It publishes only an inspectable model
record and an idempotent plan-local teardown path.

## Validation

- static inspection: git status --short before edits was clean.
- fmt/lint: cargo fmt --all -- --check passed after formatting.
- unit tests: cargo -Zjson-target-spec test passed with 318 no_std tests.
- QEMU/substitute smoke: not yet applicable in this core task because no
  runnable qemu-kernel-half-reachability smoke script is introduced here; the
  queued smoke-core task owns retained QEMU evidence.
- conditional regressions: no existing loader, install, address-space,
  materialization, launch, stack, or live-activation smoke scripts were rerun
  because those shared owners were not behaviorally changed.
- static inspection: git diff --check passed.
- documentation: mdbook build passed.
- staged whitespace inspection: git diff --cached --check passed before
  commit.

## Evidence

- changed source: src/kernel_half_reachability.rs
- changed module wiring: src/main.rs
- task record: tasks/2026-05-31-phase8-kernel-half-reachability-core.md
- roadmap update: docs/src/roadmap.md
- decision log update: docs/src/decisions/README.md

## Deferred

Kernel-half descriptor-image construction, live TTBR0_EL1/TTBR1_EL1,
TCR_EL1, MAIR_EL1, SCTLR_EL1, ASID, TLBI, DSB, or ISB mutation, lower-EL
ERET, scheduler runnable publication, process lifecycle, descriptor-backed
filesystem syscalls, Pi 5 hardware proof, shell, networking, SSH, RP1/PCIe,
UART interrupt ownership, and DMA/cache-driver policy remain blocked.

## Next Action

Next queued task: phase8-qemu-kernel-half-reachability-smoke-core-20260531.
Promote it on the next worker wake if this task is committed, retained evidence
dependencies remain satisfied, and hardwareTestLock remains unlocked/restored.
