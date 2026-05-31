# Phase 8 QEMU Kernel-Half Descriptor-Image Smoke Plan Task

Task: phase8-qemu-kernel-half-descriptor-image-smoke-plan-20260531

Status: accepted

## Scope

Completed a documentation-only QEMU/substitute smoke plan for the accepted
non-installed KernelHalfDescriptorImage boundary. The plan names the future
scenario, retained evidence path, exact classification/PASS vocabulary,
success observations, deterministic negative cases, no-partial-image
requirements, teardown behavior, zero live activation side effects, and
conditional regression gates.

Non-goals honored: no Rust or assembly behavior changes, no QEMU execution,
no Pi 5 hardware run, no boot archive publication, no hardwareTestLock
acquisition, no TTBR0_EL1/TTBR1_EL1/TCR_EL1/MAIR_EL1/SCTLR_EL1 writes, no
ASID allocation, no live TLB invalidation, no activation DSB/ISB, no lower-EL
ERET, no scheduler runnable publication, no process lifecycle, no shell, no
descriptor-backed filesystem syscalls, no writable filesystem, no networking,
no SSH, no RP1/PCIe, no UART interrupt ownership, and no DMA/cache-driver
policy.

## Evidence

- smoke plan document:
  docs/src/project/phase8-qemu-kernel-half-descriptor-image-smoke-plan.md.
- accepted contract commit reviewed:
  a3bc1610975027f5377c276ef45de345d6bbc83b.
- selected future scenario:
  qemu_kernel_half_descriptor_image_smoke.
- retained future evidence path:
  tasks/evidence/2026-05-31-qemu-kernel-half-descriptor-image-smoke-core/qemu-kernel-half-descriptor-image-smoke.log.
- classification and PASS vocabulary:
  qemu-kernel-half-descriptor-image-smoke-complete and PASS.
- selected boundary identity:
  phase8-kernel-half-descriptor-image-v1.
- selected policy:
  ttbr1-shared-privileged-kernel-root-descriptor-image-v1.
- deterministic behavior recorded: success must prove accepted input lineage,
  coverage records, privileged-only descriptor attributes, model-owned
  root/table leases, no-partial rollback, idempotent teardown, deterministic
  rejection, and zero live TTBR/TCR/MAIR/SCTLR/TLB/lower-EL/scheduler/
  process/descriptor-table side effects.
- recommended next task:
  phase8-kernel-half-descriptor-image-core-20260531.

## Validation

- static inspection: git status --short before edits was clean.
- static documentation review: inspected accepted descriptor-image contract
  and source inventory, kernel-half reachability smoke-plan pattern,
  process-page-table materialization smoke-plan pattern, adjacent Phase 8
  contracts, roadmap, SUMMARY, and ADR index.
- static inspection: git diff --check passed.
- documentation: mdbook build passed.
- staged whitespace inspection: git diff --cached --check passed before
  commit.
- hardware: hardwareTestLock remained unlocked/restored and unused.

## Result

Accepted as a documentation-only QEMU/substitute smoke plan. It names the
queued phase8-kernel-half-descriptor-image-core-20260531 task as the next
bounded implementation step and keeps live register mutation, ASID/TLB/barrier
activation, lower-EL launch, scheduler runnable publication, process
lifecycle, descriptor-backed filesystem syscalls, Pi 5 hardware proof, shell
behavior, networking, SSH, RP1/PCIe, UART interrupt ownership, and
DMA/cache-driver policy blocked.

Commit: recorded in durable supervisor state after acceptance.
