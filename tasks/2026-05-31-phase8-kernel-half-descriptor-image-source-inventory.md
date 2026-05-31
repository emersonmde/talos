# Phase 8 Kernel-Half Descriptor-Image Source Inventory Task

Task: phase8-kernel-half-descriptor-image-source-inventory-20260531

Status: accepted

## Scope

Documentation-only Milestone 8.3 inventory of the kernel-half descriptor-image
construction frontier after accepted kernel-half reachability closeout. No Rust
behavior, assembly behavior, QEMU execution, Pi 5 hardware run, boot archive
publication, hardwareTestLock acquisition, live TTBR/TCR/MAIR/SCTLR mutation,
ASID allocation, TLB mutation, activation DSB/ISB, lower-EL ERET, scheduler
runnable publication, process lifecycle, shell behavior, descriptor-backed
filesystem syscalls, writable filesystem, networking, SSH, RP1/PCIe, UART
interrupt ownership, or DMA/cache-driver policy was added.

Changed files:

- docs/src/project/phase8-kernel-half-descriptor-image-source-inventory.md
- docs/src/SUMMARY.md
- docs/src/roadmap.md
- docs/src/decisions/README.md
- tasks/2026-05-31-phase8-kernel-half-descriptor-image-source-inventory.md

## Outcome

The inventory maps the accepted KernelHalfReachabilityPlan frontier to the
exact remaining blocked-no-kernel-half-descriptor-image boundary.

It distinguishes descriptor-image construction from live TTBR/TCR/MAIR/SCTLR
mutation, ASID/TLB/barrier activation, lower-EL ERET, scheduler runnable
publication, and process lifecycle work.

It identifies the source owners for the next contract:

- src/kernel_half_reachability.rs for accepted policy identity, required
  kernel reachability entries, permission expectations, and current descriptor
  image blocker;
- src/process_page_table_materialization.rs for the closest model descriptor
  image precedent, rollback, teardown, and no-partial construction pattern;
- src/memory_map/translation.rs for stage-1 descriptor vocabulary and
  normal/device MAIR attribute intent;
- linker.ld and linker-rpi5.ld for kernel range source-of-truth;
- memory/page-frame, exception/vector, UART/MMIO diagnostic, scheduler, and
  live activation owners for reachability and side-effect boundaries.

The recommended next bounded task is the already queued documentation-only
phase8-kernel-half-descriptor-image-contract-20260531, if dependencies remain
satisfied.

## Evidence

- Inventory document:
  docs/src/project/phase8-kernel-half-descriptor-image-source-inventory.md.
- Accepted input closeout commit reviewed:
  e2b91b87f12199838571f4e46277c09f8f998068.
- Reviewed accepted input commits:
  kernel-half reachability source inventory
  1eda202e085d91759ad8b9e884772416516766bb; contract
  7d95e0a7ae2041ff00a19ff89515cde9b11cb99f; smoke plan
  c1645d5851e27b99f7aeee100782738bfdb4b093; core
  a4294f5f6a89a44c5fcee41028e916d9655767b0; smoke core
  edda81d340d919962b6856575451e201344abd29.
- Retained evidence reviewed:
  tasks/evidence/2026-05-31-qemu-kernel-half-reachability-smoke-core/qemu-kernel-half-reachability-smoke.log.
- Recommendation: phase8-kernel-half-descriptor-image-contract-20260531 as the
  next bounded documentation-only task if dependencies remain satisfied.

## Validation

- static inspection: git status --short before edits was clean.
- static documentation/source inspection: reviewed accepted kernel-half
  reachability docs, task records, retained QEMU/substitute evidence, process
  materialization docs, source modules for kernel reachability,
  process-page-table materialization, AArch64 translation descriptors, linker
  layout, memory architecture notes, lower-EL architecture notes, roadmap,
  SUMMARY, and ADR index.
- static inspection: git diff --check passed.
- documentation: mdbook build passed.
- staged whitespace inspection: git diff --cached --check passed before
  commit.

## Commit

Recorded in durable supervisor state after acceptance.
