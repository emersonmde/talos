# Phase 8 Kernel-Half Reachability Contract Task

Task: phase8-kernel-half-reachability-contract-20260531

Status: accepted

## Scope

Completed a documentation-only contract for the first kernel-half reachability
boundary below descriptor-image construction, live register mutation,
lower-EL launch, and scheduler runnable publication.

Non-goals honored: no Rust or assembly behavior changes, no QEMU execution,
no Pi 5 hardware run, no boot archive publication, no hardwareTestLock
acquisition, no TTBR0_EL1/TTBR1_EL1/TCR_EL1/MAIR_EL1/SCTLR_EL1 writes, no
ASID allocation, no live TLB invalidation, no activation DSB/ISB, no lower-EL
ERET, no scheduler runnable publication, no process lifecycle, no shell, no
descriptor-backed filesystem syscalls, no writable filesystem, no networking,
no SSH, no RP1/PCIe, no UART interrupt ownership, and no DMA/cache-driver
policy.

## Evidence

- contract document:
  docs/src/project/phase8-kernel-half-reachability-contract.md.
- reviewed source/docs:
  - docs/src/project/phase8-kernel-half-reachability-source-inventory.md.
  - docs/src/project/phase8-live-address-space-activation-closeout-checkpoint.md.
  - docs/src/project/phase8-live-address-space-activation-contract.md.
  - docs/src/project/phase8-qemu-live-address-space-activation-smoke-plan.md.
  - docs/src/project/phase8-process-page-table-materialization-contract.md.
  - docs/src/project/phase8-initial-process-launch-contract.md.
  - docs/src/project/phase8-initial-user-stack-contract.md.
  - src/live_address_space_activation.rs.
  - src/process_page_table_materialization.rs.
  - src/memory_map/layout.rs.
  - src/memory_map/page_frames.rs.
  - src/memory_map/translation.rs.
  - src/arch/aarch64/exceptions.rs.
  - src/arch/aarch64/vectors.S.
  - src/mmio.rs.
  - src/pl011.rs.
  - src/runtime_console.rs.
  - src/scheduler.rs.
  - docs/src/architecture/memory.md.
  - docs/src/architecture/lower-el-userspace.md.
  - docs/src/architecture/exceptions.md.
  - docs/src/roadmap.md.
  - docs/src/decisions/README.md.
- selected boundary: KernelHalfReachabilityPlan with identity
  phase8-kernel-half-reachability-plan-v1.
- selected policy:
  preflight-ttbr1-shared-kernel-root-reachability-v1, a preflight-only
  TTBR1 shared-kernel-root policy below descriptor-image construction and live
  register mutation.
- deterministic behavior recorded: construction is all-or-nothing, returns
  stable errors/blockers, and leaves no partial descriptor image, no live
  translation side effect, no lower-EL launch, and no runnable publication on
  failure.
- recommended next task:
  phase8-qemu-kernel-half-reachability-smoke-plan-20260531.

## Validation

- static inspection: git status --short before edits was clean.
- static documentation/source review: inspected accepted kernel-half
  inventory, live activation docs, adjacent Phase 8 contracts, source owners
  for translation/vector/UART/MMIO/scheduler/fault-reporting boundaries,
  architecture notes, roadmap, SUMMARY, and ADR index.
- static inspection: git diff --check passed.
- documentation: mdbook build passed.
- staged whitespace inspection: git diff --cached --check passed before
  commit.
- hardware: hardwareTestLock remained unlocked/restored and unused.

## Result

Accepted as a documentation-only contract. It names the queued
phase8-qemu-kernel-half-reachability-smoke-plan-20260531 task as the next
bounded documentation-only step and keeps descriptor-image construction,
implementation, live register mutation, lower-EL launch, scheduler runnable
publication, process lifecycle, filesystem syscalls, hardware proof,
networking, and SSH blocked.

Commit: recorded in durable supervisor state after acceptance.
