# Phase 8 Kernel-Half Descriptor-Image Contract Task

Task: phase8-kernel-half-descriptor-image-contract-20260531

Status: accepted

## Scope

Completed a documentation-only contract for the first non-installed
kernel-half descriptor-image boundary below live translation-register
mutation, lower-EL launch, scheduler publication, process lifecycle, Pi 5
hardware proof, shell behavior, filesystem syscall expansion, networking, and
SSH.

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
  docs/src/project/phase8-kernel-half-descriptor-image-contract.md.
- accepted inventory commit reviewed:
  6cafdd8fc7673955adab8a91f9195e1a4a4da770.
- reviewed source/docs:
  - docs/src/project/phase8-kernel-half-descriptor-image-source-inventory.md.
  - docs/src/project/phase8-kernel-half-reachability-closeout-checkpoint.md.
  - docs/src/project/phase8-kernel-half-reachability-contract.md.
  - docs/src/project/phase8-qemu-kernel-half-reachability-smoke-plan.md.
  - docs/src/project/phase8-process-page-table-materialization-contract.md.
  - docs/src/project/phase8-live-address-space-activation-contract.md.
  - docs/src/project/phase8-live-address-space-activation-closeout-checkpoint.md.
  - tasks/evidence/2026-05-31-qemu-kernel-half-reachability-smoke-core/qemu-kernel-half-reachability-smoke.log.
  - src/kernel_half_reachability.rs.
  - src/process_page_table_materialization.rs.
  - src/memory_map/translation.rs.
  - src/memory_map/layout.rs.
  - src/memory_map/page_frames.rs.
  - src/arch/aarch64/exceptions.rs.
  - src/arch/aarch64/vectors.S.
  - src/mmio.rs.
  - src/pl011.rs.
  - src/runtime_console.rs.
  - src/scheduler.rs.
  - linker.ld.
  - linker-rpi5.ld.
  - docs/src/architecture/memory.md.
  - docs/src/architecture/lower-el-userspace.md.
  - docs/src/architecture/exceptions.md.
  - docs/src/roadmap.md.
  - docs/src/decisions/README.md.
- selected boundary identity:
  phase8-kernel-half-descriptor-image-v1.
- selected policy:
  ttbr1-shared-privileged-kernel-root-descriptor-image-v1.
- deterministic behavior recorded: construction is all-or-nothing, returns
  stable errors/blockers, leaves no partial KernelHalfDescriptorImage, and
  leaves no live translation-register, TLB, lower-EL, scheduler, process, or
  descriptor-table side effect.
- recommended next task:
  phase8-qemu-kernel-half-descriptor-image-smoke-plan-20260531.

## Validation

- static inspection: git status --short before edits was clean.
- static documentation/source review: inspected accepted descriptor-image
  inventory, kernel-half reachability closeout/contract/smoke plan, retained
  reachability smoke evidence, process page-table materialization precedent,
  live activation docs, translation/linker/memory/vector/UART/MMIO/scheduler
  source owners, architecture notes, roadmap, SUMMARY, and ADR index.
- static inspection: git diff --check passed.
- documentation: mdbook build passed.
- staged whitespace inspection: git diff --cached --check passed before
  commit.
- hardware: hardwareTestLock remained unlocked/restored and unused.

## Result

Accepted as a documentation-only contract. It names the queued
phase8-qemu-kernel-half-descriptor-image-smoke-plan-20260531 task as the next
bounded documentation-only step and keeps implementation, live register
mutation, ASID/TLB/barrier activation, lower-EL launch, scheduler runnable
publication, process lifecycle, descriptor-backed filesystem syscalls, Pi 5
hardware proof, shell behavior, networking, SSH, RP1/PCIe, UART interrupt
ownership, and DMA/cache-driver policy blocked.

Commit: recorded in durable supervisor state after acceptance.
