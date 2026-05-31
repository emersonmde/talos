# Phase 8 QEMU Live Translation-Register Activation Smoke Plan Task

Task: phase8-qemu-live-translation-register-activation-smoke-plan-20260531

Status: accepted

## Scope

Documentation-only Milestone 8.3 QEMU/substitute smoke plan for the live
translation-register activation frontier. No Rust behavior, assembly behavior,
QEMU execution, Pi 5 hardware run, boot archive publication, hardwareTestLock
acquisition, live TTBR0_EL1/TTBR1_EL1/TCR_EL1/MAIR_EL1/SCTLR_EL1 mutation,
active-root descriptor copy, ASID allocation, TLB mutation, activation
DSB/ISB, lower-EL ERET, scheduler runnable publication, process lifecycle,
shell behavior, descriptor-backed filesystem syscalls, writable filesystem,
networking, SSH, RP1/PCIe, UART interrupt ownership, or DMA/cache-driver
policy was added.

Changed files:

- docs/src/project/phase8-qemu-live-translation-register-activation-smoke-plan.md
- docs/src/SUMMARY.md
- docs/src/roadmap.md
- docs/src/decisions/README.md
- tasks/2026-05-31-phase8-qemu-live-translation-register-activation-smoke-plan.md

## Outcome

The plan defines the first QEMU/substitute evidence boundary for the accepted
model-level live translation-register activation contract.

Required smoke facts:

- scenario qemu_live_translation_register_activation_smoke;
- retained evidence path
  tasks/evidence/2026-05-31-qemu-live-translation-register-activation-smoke-core/qemu-live-translation-register-activation-smoke.log;
- boundary identity phase8-live-translation-register-activation-v1;
- policy model-ttbr0-ttbr1-activation-commit-below-live-registers-v1;
- copied loader/install/address-space/materialization/launch/stack/
  activation-plan/reachability/descriptor-image/installation lineage;
- TTBR0 materialized-root provenance and TTBR1 descriptor-image kernel-root
  provenance without register writes;
- TCR/MAIR compatibility records, blocked SCTLR/ASID/TLB/barrier state,
  active-root nonmutation, and kernel-owned diagnostic reachability;
- model-only activation-commit intent state;
- deterministic rejection for missing/destroyed/mismatched/stale inputs,
  already-consumed installation, forbidden EL0 kernel access, diagnostic
  reachability loss, live-register requests, active-root-copy requests,
  lower-EL launch requests, scheduler publication requests, filesystem
  requests, and resource exhaustion;
- rollback/teardown that affects only activation-record-local state; and
- final classification
  qemu-live-translation-register-activation-smoke-complete and PASS after zero
  live side-effect evidence.

The plan explicitly keeps live TTBR/TCR/MAIR/SCTLR mutation, active-root
descriptor copy, ASID/TLB/barrier activation, lower-EL ERET, scheduler
publication, process lifecycle, filesystem syscall behavior, Pi 5 hardware
proof, boot archive publication, shell behavior, networking, and SSH blocked.

The recommended next bounded task is
phase8-live-translation-register-activation-core-20260531, if queued
dependencies remain satisfied.

## Evidence

- Smoke-plan document:
  docs/src/project/phase8-qemu-live-translation-register-activation-smoke-plan.md.
- Accepted contract reviewed:
  docs/src/project/phase8-live-translation-register-activation-contract.md.
- Accepted source inventory reviewed:
  docs/src/project/phase8-live-translation-register-activation-source-inventory.md.
- Existing QEMU/substitute smoke-plan patterns reviewed:
  docs/src/project/phase8-qemu-live-descriptor-image-installation-smoke-plan.md
  and docs/src/project/phase8-qemu-live-address-space-activation-smoke-plan.md.
- Recommended next task:
  phase8-live-translation-register-activation-core-20260531.
- Hardware lock: hardwareTestLock remained unlocked/restored and unused.

## Validation

- static inspection: git status --short before edits was clean except durable
  supervisor state promotion outside the Talos repo.
- static documentation review: inspected the accepted live
  translation-register activation contract/source inventory, adjacent Phase 8
  contracts and QEMU/substitute smoke-plan patterns, roadmap, SUMMARY, and ADR
  index.
- static inspection: git diff --check passed.
- documentation: mdbook build passed.
- staged whitespace inspection: git diff --cached --check passed before
  commit.
- hardware: no Pi 5 archive publication, hardware-lock acquisition, power
  cycle, TFTP action, or serial observation was performed.

## Commit

Recorded in durable supervisor state after acceptance.
