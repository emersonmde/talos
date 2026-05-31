# Phase 8 Kernel-Half Descriptor-Image Core Task

Task: phase8-kernel-half-descriptor-image-core-20260531
Status: accepted

## Scope

Implemented the target-independent, non-installed KernelHalfDescriptorImage
boundary selected by the accepted descriptor-image contract and
QEMU/substitute smoke plan. The implementation adds
src/kernel_half_descriptor_image.rs and wires it into src/main.rs for test
ownership only.

## Accepted Boundary

- boundary identity: phase8-kernel-half-descriptor-image-v1
- policy identity: ttbr1-shared-privileged-kernel-root-descriptor-image-v1
- TTBR0 intent: materialized process root provenance only; no write
- TTBR1 intent: model-owned shared privileged kernel-root image; no write
- descriptor image: descriptor-image-ready record only; not installed
- ownership: model-owned root/table leases with idempotent teardown
- coverage: kernel text, rodata, data, bss, vectors, active stack, heap,
  page-frame metadata, UART/MMIO diagnostics, scheduler code/data, runtime
  console, and panic/fault reporting
- blockers: live register sequence, ASID allocation, live TLBI, activation
  barriers, lower-EL ERET, scheduler publication, process-table mutation, and
  descriptor-table mutation

The image consumes the accepted KernelHalfReachabilityPlan and
ProcessPageTableMaterialization provenance. It publishes only inspectable
records and deterministic teardown/rollback behavior.

## Validation

- static inspection: git status --short before edits showed only the stale
  previous worker's partial src/main.rs module declaration for this same task;
  no unrelated Talos working-tree conflicts were present.
- fmt/lint: cargo fmt --all -- --check passed after formatting.
- unit tests: cargo -Zjson-target-spec test passed with 324 no_std tests.
- QEMU/substitute smoke: not yet applicable in this core task because retained
  qemu-kernel-half-descriptor-image smoke evidence belongs to the queued
  phase8-qemu-kernel-half-descriptor-image-smoke-core-20260531 task.
- conditional regressions: no adjacent QEMU/substitute smokes were rerun
  because this task added only a new target-independent descriptor-image model
  and module wiring; it did not change existing loader, install,
  address-space, materialization, launch, stack, live-activation, or
  reachability owners.
- static inspection: git diff --check passed.
- documentation: mdbook build passed.
- staged whitespace inspection: git diff --cached --check passed before
  commit.

## Evidence

- changed source: src/kernel_half_descriptor_image.rs
- changed module wiring: src/main.rs
- task record: tasks/2026-05-31-phase8-kernel-half-descriptor-image-core.md
- roadmap update: docs/src/roadmap.md
- decision log update: docs/src/decisions/README.md

## Deferred

QEMU/substitute retained smoke evidence, live TTBR0_EL1/TTBR1_EL1/TCR_EL1/
MAIR_EL1/SCTLR_EL1 mutation, ASID allocation, live TLB invalidation, live
DSB/ISB activation sequencing, lower-EL ERET, scheduler runnable publication,
process lifecycle, startup ABI expansion, descriptor-backed filesystem
syscalls, Pi 5 hardware proof, boot archive publication, shell behavior,
networking, SSH, RP1/PCIe, UART interrupt ownership, and DMA/cache-driver
policy remain blocked.

## Next Action

Next queued task: phase8-qemu-kernel-half-descriptor-image-smoke-core-20260531.
Promote it on the next worker wake if this task is committed, the accepted
smoke plan still matches the implementation boundary, and hardwareTestLock
remains unlocked/restored.
