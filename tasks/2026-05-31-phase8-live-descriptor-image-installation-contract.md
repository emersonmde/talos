# Phase 8 Live Descriptor-Image Installation Contract Task

Task: phase8-live-descriptor-image-installation-contract-20260531

Status: accepted

## Scope

Documentation-only Milestone 8.3 contract for the live descriptor-image
installation frontier. No Rust behavior, assembly behavior, QEMU execution,
Pi 5 hardware run, boot archive publication, hardwareTestLock acquisition,
live TTBR0_EL1/TTBR1_EL1/TCR_EL1/MAIR_EL1/SCTLR_EL1 mutation, ASID
allocation, TLB mutation, activation DSB/ISB, lower-EL ERET, scheduler
runnable publication, process lifecycle, shell behavior, descriptor-backed
filesystem syscalls, writable filesystem, networking, SSH, RP1/PCIe, UART
interrupt ownership, or DMA/cache-driver policy was added.

Changed files:

- docs/src/project/phase8-live-descriptor-image-installation-contract.md
- docs/src/SUMMARY.md
- docs/src/roadmap.md
- docs/src/decisions/README.md
- tasks/2026-05-31-phase8-live-descriptor-image-installation-contract.md

## Outcome

The contract selects a target-independent
KernelHalfDescriptorImageInstallation boundary, or equivalent activation
extension record, for the next implementation slice.

The selected boundary consumes the accepted non-installed
KernelHalfDescriptorImage and the accepted LiveAddressSpaceActivationPlan,
copies their Phase 8 lineage, and produces only a model-level
installation-ready activation binding below TTBR activation.

Required contract facts:

- boundary identity phase8-live-descriptor-image-installation-v1;
- selected policy
  model-installed-ttbr1-descriptor-image-below-live-registers-v1;
- copied descriptor-image, activation, reachability, TTBR0 materialization,
  loader/install/address-space/materialization/launch/stack, source digest,
  entry point, and initial SP identities;
- preserved kernel-half coverage, privileged-only permissions, device MMIO
  attributes, exception-vector reachability, UART/MMIO diagnostics, runtime
  console, scheduler, and panic/fault reporting prerequisites;
- model-owned installed binding state that changes only installation record
  state from non-installed descriptor-image evidence to
  installation-ready-activation-binding;
- deterministic rejection for stale/destroyed/wrong-identity inputs,
  mismatched lineage, already-installed inputs, forbidden EL0 kernel access,
  diagnostic reachability loss, capacity exhaustion, and live-register
  requests; and
- rollback/teardown that affects only installation-record-local state and
  records zero live activation side effects.

The contract explicitly preserves the distinction between model installation
and live architectural installation. TTBR1_EL1 writes, active-root descriptor
copy, TCR/MAIR/SCTLR mutation, ASID/TLB/barrier sequencing, lower-EL ERET,
scheduler publication, process lifecycle, filesystem syscall behavior, Pi 5
hardware proof, shell behavior, networking, and SSH remain blocked.

The recommended next bounded task is
phase8-qemu-live-descriptor-image-installation-smoke-plan-20260531, if queued
dependencies remain satisfied.

## Evidence

- Contract document:
  docs/src/project/phase8-live-descriptor-image-installation-contract.md.
- Accepted source inventory reviewed:
  docs/src/project/phase8-live-descriptor-image-installation-source-inventory.md.
- Accepted input closeout commit reviewed:
  448a95dac8fb24bc8d99c07c4fb056df7ea06d79.
- Retained descriptor-image smoke evidence reviewed:
  tasks/evidence/2026-05-31-qemu-kernel-half-descriptor-image-smoke-core/qemu-kernel-half-descriptor-image-smoke.log.
- Recommended next task:
  phase8-qemu-live-descriptor-image-installation-smoke-plan-20260531.
- Hardware lock: hardwareTestLock remained unlocked/restored and unused.

## Validation

- static inspection: git status --short before edits was clean except durable
  supervisor state promotion outside the Talos repo.
- static documentation/source inspection: reviewed accepted live descriptor
  installation source inventory, kernel-half descriptor-image docs/task
  records/smoke evidence, live activation docs, reachability and
  materialization contracts, relevant source owners, linker scripts,
  architecture notes, roadmap, SUMMARY, and ADR index.
- static inspection: git diff --check passed.
- documentation: mdbook build passed.
- staged whitespace inspection: git diff --cached --check passed before
  commit.
- hardware: no Pi 5 archive publication, hardware-lock acquisition, power
  cycle, TFTP action, or serial observation was performed.

## Commit

Recorded in durable supervisor state after acceptance.
