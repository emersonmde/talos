# Phase 8 Live Translation-Register Activation Contract Task

Task: phase8-live-translation-register-activation-contract-20260531

Status: accepted

## Scope

Documentation-only Milestone 8.3 contract for the live translation-register
activation frontier after the accepted source inventory. No Rust behavior,
assembly behavior, QEMU execution, Pi 5 hardware run, boot archive
publication, hardwareTestLock acquisition, live TTBR/TCR/MAIR/SCTLR mutation,
active-root descriptor copy, ASID allocation, TLB invalidation, activation
DSB/ISB, lower-EL ERET, scheduler runnable publication, process lifecycle,
shell behavior, descriptor-backed filesystem syscalls, writable filesystem,
networking, SSH, RP1/PCIe, UART interrupt ownership, or DMA/cache-driver
policy was added.

Changed files:

- docs/src/project/phase8-live-translation-register-activation-contract.md
- docs/src/SUMMARY.md
- docs/src/roadmap.md
- docs/src/decisions/README.md
- tasks/2026-05-31-phase8-live-translation-register-activation-contract.md

## Outcome

The contract selects a target-independent
LiveTranslationRegisterActivation model record boundary with identity
phase8-live-translation-register-activation-v1 and policy
model-ttbr0-ttbr1-activation-commit-below-live-registers-v1.

The selected boundary is still model/substitute-only. It consumes the accepted
KernelHalfDescriptorImageInstallation and copied Phase 8 lineage, verifies
TTBR0 materialized-root provenance, TTBR1 descriptor-image kernel-root
provenance, TCR/MAIR compatibility records, blocked SCTLR/ASID/TLB/barrier
states, active-root nonmutation, and kernel-owned fault-reporting
reachability, then publishes only an activation-commit intent with zero live
side effects.

The contract defines deterministic rejection vocabulary for stale, destroyed,
unpublished, mismatched, already-consumed, forbidden EL0/kernel/device,
diagnostic-reachability-loss, capacity, and live-register/active-root/
lower-EL/scheduler/filesystem/hardware requests. It preserves explicit
blockers for live register sequencing, active-root copy, ASID allocation, live
TLBI, live DSB/ISB, lower-EL ERET, scheduler publication, process lifecycle,
startup ABI expansion, filesystem syscalls, and Pi 5 hardware proof.

The mechanically next documentation-only task is
phase8-qemu-live-translation-register-activation-smoke-plan-20260531, if
dependencies remain satisfied.

## Evidence

- Contract document:
  docs/src/project/phase8-live-translation-register-activation-contract.md.
- Accepted source inventory:
  docs/src/project/phase8-live-translation-register-activation-source-inventory.md.
- Retained input evidence reviewed:
  tasks/evidence/2026-05-31-qemu-live-descriptor-image-installation-smoke-core/qemu-live-descriptor-image-installation-smoke.log.
- Selected boundary:
  phase8-live-translation-register-activation-v1.
- Selected policy:
  model-ttbr0-ttbr1-activation-commit-below-live-registers-v1.
- Recommended next task:
  phase8-qemu-live-translation-register-activation-smoke-plan-20260531.

## Validation

- static inspection: git status --short before edits was clean in the Talos
  repo.
- static documentation/source inspection: reviewed accepted source inventory,
  adjacent live activation, kernel-half reachability, kernel-half
  descriptor-image, live descriptor-image installation docs/task records,
  retained QEMU/substitute descriptor-image installation evidence, roadmap,
  SUMMARY, and ADR index.
- static inspection: git diff --check passed.
- documentation: mdbook build passed.
- staged whitespace inspection: git diff --cached --check passed before
  commit.
- hardware: no Pi 5 archive publication, hardware-lock acquisition, power
  cycle, TFTP action, or serial observation was performed.

## Commit

Recorded in durable supervisor state after acceptance.
