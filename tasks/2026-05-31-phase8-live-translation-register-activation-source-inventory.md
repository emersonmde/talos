# Phase 8 Live Translation-Register Activation Source Inventory Task

Task: phase8-live-translation-register-activation-source-inventory-20260531

Status: accepted

## Scope

Documentation-only Milestone 8.3 inventory of the live translation-register
activation frontier after accepted live descriptor-image installation closeout.
No Rust behavior, assembly behavior, QEMU execution, Pi 5 hardware run, boot
archive publication, hardwareTestLock acquisition, live TTBR/TCR/MAIR/SCTLR
mutation, active-root descriptor copy, ASID allocation, TLB invalidation,
activation DSB/ISB, lower-EL ERET, scheduler runnable publication, process
lifecycle, shell behavior, descriptor-backed filesystem syscalls, writable
filesystem, networking, SSH, RP1/PCIe, UART interrupt ownership, or
DMA/cache-driver policy was added.

Changed files:

- docs/src/project/phase8-live-translation-register-activation-source-inventory.md
- docs/src/SUMMARY.md
- docs/src/roadmap.md
- docs/src/decisions/README.md
- tasks/2026-05-31-phase8-live-translation-register-activation-source-inventory.md

## Outcome

The inventory maps the accepted model-only
KernelHalfDescriptorImageInstallation frontier to the next translation-register
activation handoff.

It distinguishes accepted model-only evidence from unaccepted live behavior:
current evidence proves TTBR0 materialized-root provenance, TTBR1
descriptor-image kernel-root provenance, compatibility-only TCR/MAIR records,
blocked SCTLR mutation, blocked ASID/TLB, planned-only no-live DSB/ISB, and
zero side effects, while TTBR0_EL1/TTBR1_EL1/TCR_EL1/MAIR_EL1/SCTLR_EL1
mutation, active-root copy, ASID allocation, live TLB invalidation, live
barrier sequencing, lower-EL launch, scheduler publication, process lifecycle,
filesystem syscalls, and Pi 5 proof remain unaccepted.

It identifies source owners for the next contract:

- src/live_descriptor_image_installation.rs for installation-ready activation
  binding, copied TTBR0/TTBR1 provenance, blocked live-register states,
  teardown, and zero side effects;
- src/live_address_space_activation.rs for TTBR0 provenance, TCR/MAIR/SCTLR/
  ASID/TLB/barrier vocabulary, kernel reachability checklist, and commit
  rejections;
- src/kernel_half_descriptor_image.rs and src/kernel_half_reachability.rs for
  TTBR1 descriptor-image provenance, coverage, permissions, vector/stack/
  UART/MMIO/fault-reporting reachability, and rollback prerequisites;
- src/process_page_table_materialization.rs for TTBR0 root leases and
  materialized process-root provenance;
- src/memory_map/translation.rs, linker scripts, exception/vector owners,
  UART/MMIO diagnostics, runtime console, and scheduler owners for register,
  descriptor, diagnostic, and side-effect boundaries.

The recommended next bounded task is the already queued documentation-only
phase8-live-translation-register-activation-contract-20260531, if dependencies
remain satisfied.

## Evidence

- Inventory document:
  docs/src/project/phase8-live-translation-register-activation-source-inventory.md.
- Accepted input closeout commit reviewed:
  9d1dbec78d4fd7704ea90eb3313cdad2b6067f87.
- Reviewed accepted input commits:
  live activation inventory 7f645691ea423bdc38e9bf04a27f75ce967984a5;
  live activation contract 89c624b95f39739e67168c4e6465a61ee18f345d;
  live activation core 129337734011004297da0b2768a3a802063c3293;
  live activation smoke core 1c441c301387ed75e24db7f9788301126f1f5a72;
  kernel-half reachability closeout e2b91b87f12199838571f4e46277c09f8f998068;
  kernel-half descriptor-image closeout
  448a95dac8fb24bc8d99c07c4fb056df7ea06d79; live descriptor-image
  installation source inventory 19b824a3f6b6249204b3b7ca8129c051cfefcc05;
  contract e58ecebd5a4ce339b21d79e9029ecef70cc3d109; core
  ea264b234a2a68c89dc49d91d8adfa9c266148bd; smoke core
  5ef41854f6789dc829f4c4dfc984536c7104e559.
- Retained evidence reviewed:
  tasks/evidence/2026-05-31-qemu-live-descriptor-image-installation-smoke-core/qemu-live-descriptor-image-installation-smoke.log.
- Recommendation:
  phase8-live-translation-register-activation-contract-20260531 as the next
  bounded documentation-only task if dependencies remain satisfied.

## Validation

- static inspection: git status --short before edits was clean.
- static documentation/source inspection: reviewed accepted live activation,
  kernel-half reachability, kernel-half descriptor-image, and live
  descriptor-image installation docs and task records, retained QEMU/substitute
  descriptor-image installation evidence, relevant source owners, linker
  scripts, roadmap, SUMMARY, and ADR index.
- static inspection: git diff --check passed.
- documentation: mdbook build passed.
- staged whitespace inspection: git diff --cached --check passed before
  commit.
- hardware: no Pi 5 archive publication, hardware-lock acquisition, power
  cycle, TFTP action, or serial observation was performed.

## Commit

Recorded in durable supervisor state after acceptance.
