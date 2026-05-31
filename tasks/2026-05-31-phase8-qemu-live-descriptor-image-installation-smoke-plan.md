# Phase 8 QEMU Live Descriptor-Image Installation Smoke Plan Task

Task: phase8-qemu-live-descriptor-image-installation-smoke-plan-20260531

Status: accepted

## Scope

Documentation-only Milestone 8.3 QEMU/substitute smoke plan for the live
descriptor-image installation frontier. No Rust behavior, assembly behavior,
QEMU execution, Pi 5 hardware run, boot archive publication, hardwareTestLock
acquisition, live TTBR0_EL1/TTBR1_EL1/TCR_EL1/MAIR_EL1/SCTLR_EL1 mutation,
ASID allocation, TLB mutation, activation DSB/ISB, lower-EL ERET, scheduler
runnable publication, process lifecycle, shell behavior, descriptor-backed
filesystem syscalls, writable filesystem, networking, SSH, RP1/PCIe, UART
interrupt ownership, or DMA/cache-driver policy was added.

Changed files:

- docs/src/project/phase8-qemu-live-descriptor-image-installation-smoke-plan.md
- docs/src/SUMMARY.md
- docs/src/roadmap.md
- docs/src/decisions/README.md
- tasks/2026-05-31-phase8-qemu-live-descriptor-image-installation-smoke-plan.md

## Outcome

The plan defines the first QEMU/substitute evidence boundary for the accepted
model-level live descriptor-image installation contract.

Required smoke facts:

- scenario qemu_live_descriptor_image_installation_smoke;
- retained evidence path
  tasks/evidence/2026-05-31-qemu-live-descriptor-image-installation-smoke-core/qemu-live-descriptor-image-installation-smoke.log;
- boundary identity phase8-live-descriptor-image-installation-v1;
- policy model-installed-ttbr1-descriptor-image-below-live-registers-v1;
- copied loader/install/address-space/materialization/launch/stack/
  activation/reachability/descriptor-image lineage;
- TTBR0 materialized-root provenance and TTBR1 descriptor-image root
  provenance without register writes;
- preserved kernel-half coverage, privileged-only permissions, device MMIO
  attributes, exception-vector reachability, UART/MMIO diagnostics, runtime
  console, scheduler, and panic/fault reporting prerequisites;
- installation-ready activation binding state only;
- deterministic rejection for missing/destroyed/mismatched/already-installed
  inputs, forbidden EL0 kernel access, diagnostic reachability loss, resource
  exhaustion, and live-register requests;
- rollback/teardown that affects only installation-record-local state; and
- final classification qemu-live-descriptor-image-installation-smoke-complete
  and PASS after zero live side-effect evidence.

The plan explicitly keeps live TTBR/TCR/MAIR/SCTLR mutation, active-root
descriptor copy, ASID/TLB/barrier activation, lower-EL ERET, scheduler
publication, process lifecycle, filesystem syscall behavior, Pi 5 hardware
proof, shell behavior, networking, and SSH blocked.

The recommended next bounded task is
phase8-live-descriptor-image-installation-core-20260531, if queued
dependencies remain satisfied.

## Evidence

- Smoke-plan document:
  docs/src/project/phase8-qemu-live-descriptor-image-installation-smoke-plan.md.
- Accepted contract reviewed:
  docs/src/project/phase8-live-descriptor-image-installation-contract.md.
- Accepted source inventory reviewed:
  docs/src/project/phase8-live-descriptor-image-installation-source-inventory.md.
- Existing QEMU/substitute smoke-plan patterns reviewed:
  docs/src/project/phase8-qemu-kernel-half-descriptor-image-smoke-plan.md and
  docs/src/project/phase8-qemu-live-address-space-activation-smoke-plan.md.
- Recommended next task:
  phase8-live-descriptor-image-installation-core-20260531.
- Hardware lock: hardwareTestLock remained unlocked/restored and unused.

## Validation

- static inspection: git status --short before edits was clean except durable
  supervisor state promotion outside the Talos repo.
- static documentation review: inspected the accepted live descriptor-image
  installation contract/source inventory, adjacent Phase 8 contracts and
  QEMU/substitute smoke-plan patterns, roadmap, SUMMARY, and ADR index.
- static inspection: git diff --check passed.
- documentation: mdbook build passed.
- staged whitespace inspection: git diff --cached --check passed before
  commit.
- hardware: no Pi 5 archive publication, hardware-lock acquisition, power
  cycle, TFTP action, or serial observation was performed.

## Commit

Recorded in durable supervisor state after acceptance.
