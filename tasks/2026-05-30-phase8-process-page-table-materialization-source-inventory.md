# Phase 8 Process Page-Table Materialization Source Inventory Task

Task: phase8-process-page-table-materialization-source-inventory-20260530

Status: accepted

## Scope

Documentation-only Milestone 8.3 source inventory after the accepted process
address-space closeout. The task mapped source owners, accepted inputs, and
gaps for turning target-independent ProcessAddressSpace lease/mapping records
into a future architecture-owned process page-table materialization boundary.

Non-goals honored: no Rust or assembly behavior changes, no QEMU execution, no
Pi 5 hardware run, no boot archive publication, no hardwareTestLock
acquisition, no page-table descriptor installation, no physical frame
allocation, no TTBR/TCR/MAIR/SCTLR/ASID/TLB policy changes, no lower-EL
launch, no argv/envp, no process creation, no exec/spawn/wait, no shell, no
descriptor-backed filesystem syscall, no writable filesystem, no persistent
storage, no networking, no SSH, no RP1/PCIe, no UART interrupt ownership, and
no DMA/cache-driver policy.

## Evidence

- inventory document:
  docs/src/project/phase8-process-page-table-materialization-source-inventory.md.
- reviewed docs:
  - docs/src/project/phase8-process-address-space-source-inventory.md
  - docs/src/project/phase8-process-address-space-contract.md
  - docs/src/project/phase8-qemu-process-address-space-smoke-plan.md
  - docs/src/project/phase8-process-address-space-closeout-checkpoint.md
- reviewed source owners:
  src/program_loader.rs, src/process_install.rs, src/process_address_space.rs,
  src/posix.rs, src/memory_map/page_frames.rs,
  src/memory_map/translation.rs, src/arch/aarch64/mod.rs, src/scheduler.rs,
  src/target/qemu_virt.rs, and src/target/rpi5.rs.
- reviewed task/evidence records:
  - tasks/2026-05-30-phase8-process-address-space-core.md
  - tasks/2026-05-30-phase8-qemu-process-address-space-smoke-core.md
  - tasks/evidence/2026-05-30-qemu-process-address-space-smoke-core/qemu-process-address-space-smoke.log
- gap summary: missing real user-frame source, process page-table root/table
  page allocation, descriptor bit policy, kernel mapping sharing, ASID/TTBR
  policy, TCR/MAIR/TLB/barrier boundaries, rollback, teardown, and
  materialization evidence vocabulary.
- next bounded task recommendation:
  phase8-process-page-table-materialization-contract-20260530.

## Outcome

The inventory separates accepted target-independent ProcessAddressSpace model
behavior from unaccepted architecture-specific materialization. The next
contract boundary is now documented as page-table materialization below TTBR
activation and below lower-EL launch. No physical page table, descriptor,
frame, address-space activation, lower-EL launch, process lifecycle,
descriptor inheritance, hardware, networking, or SSH capability is accepted by
this task.

## Validation

- static inspection: git status --short before edits was clean.
- static inspection: git diff --check passed.
- documentation: mdbook build passed.
- staged whitespace inspection: git diff --cached --check passed before
  commit.

## Commit

Recorded in durable supervisor state after acceptance.
