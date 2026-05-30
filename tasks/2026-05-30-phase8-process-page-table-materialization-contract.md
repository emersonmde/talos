# Phase 8 Process Page-Table Materialization Contract Task

Task: phase8-process-page-table-materialization-contract-20260530

Status: accepted

## Scope

Documentation-only Milestone 8.3 contract after the accepted process
page-table materialization source inventory. The task selected the first
bounded process page-table materialization boundary below TTBR activation and
below lower-EL launch.

Non-goals honored: no Rust or assembly behavior changes, no QEMU execution, no
Pi 5 hardware run, no boot archive publication, no hardwareTestLock
acquisition, no TTBR/TCR/MAIR/SCTLR/ASID/TLB policy change, no lower-EL
launch, no argv/envp, no process lifecycle, no shell, no descriptor-backed
filesystem syscall, no writable filesystem, no persistent storage, no
networking, no SSH, no RP1/PCIe, no UART interrupt ownership, and no
DMA/cache-driver policy.

## Evidence

- contract document:
  docs/src/project/phase8-process-page-table-materialization-contract.md.
- reviewed inventory and docs:
  - docs/src/project/phase8-process-page-table-materialization-source-inventory.md
  - docs/src/project/phase8-process-address-space-contract.md
  - docs/src/project/phase8-qemu-process-address-space-smoke-plan.md
  - docs/src/project/phase8-process-address-space-closeout-checkpoint.md
- reviewed source owners:
  src/program_loader.rs, src/process_install.rs, src/process_address_space.rs,
  src/posix.rs, src/memory_map/page_frames.rs,
  src/memory_map/translation.rs, src/arch/aarch64/mod.rs, and
  src/scheduler.rs.
- selected boundary:
  phase8-process-page-table-materialization-v1, a non-activating AArch64
  descriptor-image and user-frame materialization record.
- next bounded task:
  phase8-qemu-process-page-table-materialization-smoke-plan-20260530.

## Outcome

The contract fixes exact inputs from ProgramImagePlan,
ProcessImageInstallPlan, and ProcessAddressSpace records. It defines
user-frame, root-table, and table-page ownership; UserText/UserData descriptor
policy; kernel mapping and activation boundaries; ASID/TTBR/TCR/TLB blocked
surfaces; rollback; teardown; and deterministic error mapping.

Lower-EL launch, argv/envp, process lifecycle, filesystem syscalls, Pi 5
hardware proof, networking, and SSH remain blocked until later explicit tasks
accept their contracts and gates.

## Validation

- static inspection: git status --short before edits was clean.
- static inspection: git diff --check passed.
- documentation: mdbook build passed.
- staged whitespace inspection: git diff --cached --check passed before
  commit.

## Commit

Recorded in durable supervisor state after acceptance.
