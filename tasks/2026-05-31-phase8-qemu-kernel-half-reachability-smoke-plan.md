# Phase 8 QEMU Kernel-Half Reachability Smoke Plan Task

Task: phase8-qemu-kernel-half-reachability-smoke-plan-20260531

Status: accepted

## Scope

Documentation-only Milestone 8.3 QEMU/substitute smoke plan after the accepted
kernel-half reachability contract. The task fixed the retained evidence
boundary for the future KernelHalfReachabilityPlan implementation.

Changed files:

- docs/src/project/phase8-qemu-kernel-half-reachability-smoke-plan.md
- docs/src/SUMMARY.md
- docs/src/roadmap.md
- docs/src/decisions/README.md
- tasks/2026-05-31-phase8-qemu-kernel-half-reachability-smoke-plan.md

Non-goals honored: no Rust or assembly behavior changes, no QEMU execution, no
Pi 5 hardware run, no boot archive publication, no hardwareTestLock
acquisition, no live TTBR0_EL1/TTBR1_EL1, TCR_EL1, MAIR_EL1, SCTLR_EL1, ASID,
TLBI, DSB, or ISB mutation, no lower-EL ERET, no scheduler runnable
publication, no process lifecycle, no shell, no descriptor-backed filesystem
syscalls, no writable filesystem, no persistent storage, no networking, no
SSH, no RP1/PCIe, no UART interrupt ownership, and no DMA/cache-driver policy.

## Outcome

The smoke plan selects qemu_kernel_half_reachability_smoke as a
QEMU/substitute scenario for the accepted kernel-half reachability boundary.
It requires retained evidence at:

    tasks/evidence/2026-05-31-qemu-kernel-half-reachability-smoke-core/qemu-kernel-half-reachability-smoke.log

The required final vocabulary is:

    qemu-kernel-half-reachability-smoke: final participants=16 expected=16 errors=0 classification=qemu-kernel-half-reachability-smoke-complete
    qemu-kernel-half-reachability-smoke: PASS

The plan defines success observations for copied accepted input lineage,
kernel-half boundary phase8-kernel-half-reachability-plan-v1, kernel-half
policy preflight-ttbr1-shared-kernel-root-reachability-v1, TTBR0_EL1 root
provenance, blocked TTBR1_EL1 descriptor-image construction, required kernel
reachability, privileged-only kernel permissions, TCR/MAIR compatibility,
blocked SCTLR/ASID/TLB/barrier states, deterministic no-partial rejection
cases, idempotent plan-local teardown, and zero live side effects.

Pi 5 hardware proof, live translation-register mutation, live descriptor-image
installation, lower-EL launch, scheduler runnable publication, process
lifecycle, broad argv/envp/auxv/TLS ABI, filesystem syscalls, networking, and
SSH remain blocked.

## Evidence

- smoke plan document:
  docs/src/project/phase8-qemu-kernel-half-reachability-smoke-plan.md.
- reviewed accepted kernel-half docs:
  - docs/src/project/phase8-kernel-half-reachability-contract.md
  - docs/src/project/phase8-kernel-half-reachability-source-inventory.md
- reviewed adjacent smoke-plan pattern:
  docs/src/project/phase8-qemu-live-address-space-activation-smoke-plan.md.
- selected QEMU/substitute scenario:
  qemu_kernel_half_reachability_smoke.
- retained evidence path for the later smoke core:
  tasks/evidence/2026-05-31-qemu-kernel-half-reachability-smoke-core/qemu-kernel-half-reachability-smoke.log.
- required classification/PASS vocabulary:
  qemu-kernel-half-reachability-smoke-complete and
  qemu-kernel-half-reachability-smoke: PASS.
- next bounded task:
  phase8-kernel-half-reachability-core-20260531.

## Validation

- static inspection: git status --short before edits was clean.
- static documentation review: inspected the accepted kernel-half reachability
  contract and source inventory, adjacent Phase 8 contracts, existing
  QEMU/substitute smoke-plan pattern, roadmap, SUMMARY, and ADR index.
- static inspection: git diff --check passed.
- documentation: mdbook build passed.
- staged whitespace inspection: git diff --cached --check passed before
  commit.

## Residual Blocked Surfaces

Kernel-half descriptor-image construction for a live root, live
TTBR/TCR/MAIR/SCTLR mutation, ASID/TLB sequencing, live DSB/ISB activation
ordering, lower-EL ERET, scheduler runnable publication, process lifecycle,
exec/spawn/wait, broad argv/envp/auxv/TLS ABI, shell, descriptor-backed
filesystem syscalls, Pi 5 hardware proof, writable filesystems, persistent
storage, networking, SSH, RP1/PCIe, UART interrupt ownership, and
DMA/cache-driver policy remain blocked until later explicit tasks accept their
contracts and evidence gates.

## Commit

Recorded in durable supervisor state after acceptance.
