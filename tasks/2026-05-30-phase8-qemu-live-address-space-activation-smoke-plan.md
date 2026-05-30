# Phase 8 QEMU Live Address-Space Activation Smoke Plan Task

Task: phase8-qemu-live-address-space-activation-smoke-plan-20260530

Status: accepted

## Scope

Documentation-only Milestone 8.3 QEMU/substitute smoke plan after the accepted
live address-space activation contract. The task fixed the retained evidence
boundary for the future LiveAddressSpaceActivationPlan implementation.

Changed files:

- docs/src/project/phase8-qemu-live-address-space-activation-smoke-plan.md
- docs/src/SUMMARY.md
- docs/src/roadmap.md
- docs/src/decisions/README.md
- tasks/2026-05-30-phase8-qemu-live-address-space-activation-smoke-plan.md

Non-goals honored: no Rust or assembly behavior changes, no QEMU execution, no
Pi 5 hardware run, no boot archive publication, no hardwareTestLock
acquisition, no live TTBR/TCR/MAIR/SCTLR mutation, no ASID allocation, no live
TLB invalidation, no lower-EL ERET, no scheduler runnable publication, no
process lifecycle, no shell, no descriptor-backed filesystem syscalls, no
writable filesystem, no persistent storage, no networking, no SSH, no
RP1/PCIe, no UART interrupt ownership, and no DMA/cache-driver policy.

## Outcome

The smoke plan selects qemu_live_address_space_activation_smoke as a
QEMU/substitute scenario for the accepted activation-preflight boundary. It
requires retained evidence at:

    tasks/evidence/2026-05-30-qemu-live-address-space-activation-smoke-core/qemu-live-address-space-activation-smoke.log

The required final vocabulary is:

    qemu-live-address-space-activation-smoke: final participants=15 expected=15 errors=0 classification=qemu-live-address-space-activation-smoke-complete
    qemu-live-address-space-activation-smoke: PASS

The plan defines success observations for:

- accepted ProgramImagePlan, ProcessImageInstallPlan, ProcessAddressSpace,
  ProcessPageTableMaterialization, InitialProcessLaunchPlan, and
  InitialUserStackPlan identity lineage;
- activation boundary phase8-live-address-space-activation-plan-v1;
- activation policy
  preflight-split-user-ttbr0-kernel-reachability-blocked-v1;
- TTBR0_EL1 root provenance from the materialized process root lease with no
  TTBR0_EL1 write;
- TTBR1_EL1/kernel-half policy blocked-no-accepted-kernel-half-map;
- TCR_EL1 and MAIR_EL1 compatibility records and SCTLR_EL1 mutation-blocked;
- ASID/TLB/barrier states blocked from live use;
- kernel-reachability checklist for VBAR_EL1, exception vectors, active stack,
  kernel text/data, allocator, UART/MMIO diagnostics, scheduler code/data, and
  panic/fault reporting;
- model-only activation-preflight-ready launch binding; and
- zero TTBR/TCR/MAIR/SCTLR, ASID, TLB, DSB/ISB, lower-EL, scheduler,
  process-table, and descriptor-table side effects.

It defines deterministic rejection evidence for identity mismatch,
entry/stack/descriptor disagreement, forbidden kernel/device/user-range
requests, missing kernel-reachability prerequisites, live-register requests,
scheduler publication requests, lower-EL launch requests, and deterministic
resource exhaustion. Pi 5 hardware proof, live translation-register mutation,
lower-EL launch, scheduler runnable publication, process lifecycle, broad
argv/envp/auxv/TLS ABI, filesystem syscalls, networking, and SSH remain
blocked.

## Evidence

- smoke plan document:
  docs/src/project/phase8-qemu-live-address-space-activation-smoke-plan.md.
- reviewed accepted activation docs:
  - docs/src/project/phase8-live-address-space-activation-contract.md
  - docs/src/project/phase8-live-address-space-activation-source-inventory.md
- reviewed smoke-plan patterns:
  - docs/src/project/phase8-qemu-initial-user-stack-smoke-plan.md
  - docs/src/project/phase8-qemu-process-page-table-materialization-smoke-plan.md
- selected QEMU/substitute scenario:
  qemu_live_address_space_activation_smoke.
- retained evidence path for the later smoke core:
  tasks/evidence/2026-05-30-qemu-live-address-space-activation-smoke-core/qemu-live-address-space-activation-smoke.log.
- required classification/PASS vocabulary:
  qemu-live-address-space-activation-smoke-complete and
  qemu-live-address-space-activation-smoke: PASS.
- next bounded task:
  phase8-live-address-space-activation-core-20260530.

## Validation

- static inspection: git status --short before edits was clean.
- static documentation/source review: inspected the accepted live
  address-space activation contract and source inventory, adjacent Phase 8
  contracts, existing QEMU smoke-plan patterns, roadmap, SUMMARY, and ADR
  index.
- static inspection: git diff --check passed.
- documentation: mdbook build passed.
- staged whitespace inspection: git diff --cached --check passed before
  commit.

## Residual Blocked Surfaces

Live TTBR/TCR/MAIR/SCTLR mutation, ASID/TLB sequencing, live DSB/ISB
activation ordering, lower-EL ERET, scheduler runnable publication, process
lifecycle, exec/spawn/wait, broad argv/envp/auxv/TLS ABI, shell,
descriptor-backed filesystem syscalls, Pi 5 hardware proof, writable
filesystems, persistent storage, networking, SSH, RP1/PCIe, UART interrupt
ownership, and DMA/cache-driver policy remain blocked until later explicit
tasks accept their contracts and evidence gates.

## Commit

Recorded in durable supervisor state after acceptance.
