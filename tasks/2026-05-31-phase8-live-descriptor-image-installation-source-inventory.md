# Phase 8 Live Descriptor-Image Installation Source Inventory Task

Task: phase8-live-descriptor-image-installation-source-inventory-20260531

Status: accepted

## Scope

Documentation-only Milestone 8.3 inventory of the live descriptor-image
installation frontier after accepted kernel-half descriptor-image closeout. No
Rust behavior, assembly behavior, QEMU execution, Pi 5 hardware run, boot
archive publication, hardwareTestLock acquisition, live TTBR/TCR/MAIR/SCTLR
mutation, ASID allocation, TLB mutation, activation DSB/ISB, lower-EL ERET,
scheduler runnable publication, process lifecycle, shell behavior,
descriptor-backed filesystem syscalls, writable filesystem, networking, SSH,
RP1/PCIe, UART interrupt ownership, or DMA/cache-driver policy was added.

Changed files:

- docs/src/project/phase8-live-descriptor-image-installation-source-inventory.md
- docs/src/SUMMARY.md
- docs/src/roadmap.md
- docs/src/decisions/README.md
- tasks/2026-05-31-phase8-live-descriptor-image-installation-source-inventory.md

## Outcome

The inventory maps the accepted non-installed KernelHalfDescriptorImage
frontier to the next descriptor-image installation handoff.

It distinguishes accepted non-installed evidence from a proposed
installation-ready activation binding: current evidence proves published=true
and installed=false for the descriptor image, while the next contract should
define only a target-independent model installation below TTBR activation.

It identifies source owners for the next contract:

- src/kernel_half_descriptor_image.rs for descriptor-image identity, policy,
  TTBR0 provenance, TTBR1 model-owned root/table leases, coverage, attributes,
  teardown, and zero side effects;
- src/live_address_space_activation.rs for activation plan identity, TTBR1
  blocker vocabulary, model-only activation state, no-partial behavior, and
  commit rejections;
- src/kernel_half_reachability.rs for the TTBR1 shared privileged kernel-root
  policy and reachability prerequisites;
- src/process_page_table_materialization.rs for TTBR0 materialized root
  provenance and descriptor-image precedent;
- src/memory_map/translation.rs, linker scripts, exception/vector owners,
  UART/MMIO diagnostics, runtime console, and scheduler owners for descriptor,
  reachability, fault-reporting, and side-effect boundaries.

The recommended next bounded task is the already queued documentation-only
phase8-live-descriptor-image-installation-contract-20260531, if dependencies
remain satisfied.

## Evidence

- Inventory document:
  docs/src/project/phase8-live-descriptor-image-installation-source-inventory.md.
- Accepted input closeout commit reviewed:
  448a95dac8fb24bc8d99c07c4fb056df7ea06d79.
- Reviewed accepted input commits:
  live activation inventory 7f645691ea423bdc38e9bf04a27f75ce967984a5;
  live activation contract 89c624b95f39739e67168c4e6465a61ee18f345d;
  live activation core 129337734011004297da0b2768a3a802063c3293;
  live activation smoke core 1c441c301387ed75e24db7f9788301126f1f5a72;
  kernel-half reachability closeout e2b91b87f12199838571f4e46277c09f8f998068;
  kernel-half descriptor-image source inventory
  6cafdd8fc7673955adab8a91f9195e1a4a4da770; contract
  a3bc1610975027f5377c276ef45de345d6bbc83b; smoke plan
  ddaebb3897da014cadc3d4c76ddea79bd9709ba5; core
  3e0e83652e07581447cdb7f06c54a454e052ab83; smoke core
  424c1f3f754462f8496feeba6684bf3a4ae6738a.
- Retained evidence reviewed:
  tasks/evidence/2026-05-31-qemu-kernel-half-descriptor-image-smoke-core/qemu-kernel-half-descriptor-image-smoke.log.
- Recommendation: phase8-live-descriptor-image-installation-contract-20260531
  as the next bounded documentation-only task if dependencies remain
  satisfied.

## Validation

- static inspection: git status --short before edits was clean.
- static documentation/source inspection: reviewed accepted live activation,
  kernel-half reachability, kernel-half descriptor-image docs and task records,
  retained QEMU/substitute descriptor-image evidence, relevant source owners,
  linker scripts, roadmap, SUMMARY, and ADR index.
- static inspection: git diff --check passed.
- documentation: mdbook build passed.
- staged whitespace inspection: git diff --cached --check passed before
  commit.
- hardware: no Pi 5 archive publication, hardware-lock acquisition, power
  cycle, TFTP action, or serial observation was performed.

## Commit

Recorded in durable supervisor state after acceptance.
