# Phase 8 Initial Process Launch Contract Task

Task: phase8-initial-process-launch-contract-20260530

Status: accepted

## Scope

Documentation-only Milestone 8.3 contract after the accepted initial process
launch source inventory.

Changed files:

- docs/src/project/phase8-initial-process-launch-contract.md
- docs/src/SUMMARY.md
- docs/src/roadmap.md
- docs/src/decisions/README.md
- tasks/2026-05-30-phase8-initial-process-launch-contract.md

Non-goals honored: no Rust or assembly behavior changes, no TTBR/TCR/MAIR/SCTLR
writes, no ASID allocation, no live TLB invalidation, no lower-EL ERET to
/bin/init, no initial user stack implementation, no argv/envp/auxv/TLS setup,
no process table, no PID/wait/exit state, no scheduler runnable publication,
no shell, no descriptor-backed filesystem syscalls, no QEMU run, no Pi 5
hardware run, no boot archive publication, no hardwareTestLock acquisition,
no networking, no SSH, no RP1/PCIe, no UART interrupt ownership, and no
DMA/cache-driver policy.

## Outcome

The contract selects a target-independent InitialProcessLaunchPlan boundary
with identity phase8-initial-process-launch-plan-v1. The selected boundary
copies and cross-checks entry provenance from ProgramImagePlan,
ProcessImageInstallPlan, ProcessAddressSpace, and
ProcessPageTableMaterialization, then records the remaining blockers instead
of launching:

- user_sp_state=blocked-missing-initial-user-stack;
- activation_state=blocked-no-ttbr-activation;
- saved-frame intent for ELR, SP_EL0, SPSR, x0..x5, DAIF, and address-space
  token without register writes;
- no-runnable-publication and no-partial-launch behavior for commit requests;
  and
- deterministic POSIX-shaped errors for mismatched inputs, bad entry,
  activation requests, stack-required launch requests, and scheduler
  publication requests.

The contract names exactly one next bounded task:
phase8-qemu-initial-process-launch-smoke-plan-20260530. Implementation, QEMU
execution, lower-EL launch, initial user stack implementation, process
lifecycle, filesystem syscalls, hardware proof, shell, networking, and SSH
remain blocked.

## Evidence

- contract document:
  docs/src/project/phase8-initial-process-launch-contract.md
- reviewed accepted docs and sources:
  - docs/src/project/phase8-initial-process-launch-source-inventory.md.
  - docs/src/project/phase8-process-page-table-materialization-contract.md.
  - docs/src/project/phase8-process-page-table-materialization-closeout-checkpoint.md.
  - docs/src/project/phase8-process-address-space-contract.md.
  - docs/src/project/phase8-process-install-contract.md.
  - src/program_loader.rs.
  - src/process_install.rs.
  - src/process_address_space.rs.
  - src/process_page_table_materialization.rs.
  - src/posix.rs.
  - src/arch/aarch64/mod.rs.
  - src/arch/aarch64/exceptions.rs.
  - src/memory_map/translation.rs.
  - src/scheduler.rs.
- next QEMU/substitute smoke-plan task:
  phase8-qemu-initial-process-launch-smoke-plan-20260530.

## Validation

- static inspection: git status --short before edits was clean.
- static documentation/source review: inspected accepted launch-adjacent docs,
  loader/install/address-space/materialization source owners, POSIX user-range
  and error vocabulary, AArch64 lower-EL and exception helpers, translation
  register vocabulary, scheduler owner placeholders, roadmap, SUMMARY, and ADR
  index.
- static inspection: git diff --check passed.
- documentation: mdbook build passed.
- staged whitespace inspection: git diff --cached --check passed before
  commit.

## Residual Blocked Surfaces

TTBR/TCR/MAIR/SCTLR mutation, ASID/TLB sequencing, lower-EL launch, initial
user stack implementation, argv/envp/auxv/TLS, process lifecycle,
exec/spawn/wait, scheduler runnable publication, shell, descriptor-backed
filesystem syscalls, Pi 5 hardware proof, writable filesystems, persistent
storage, networking, SSH, RP1/PCIe, UART interrupt ownership, and
DMA/cache-driver policy remain blocked until later explicit tasks accept their
contracts and evidence gates.

## Commit

Recorded in durable supervisor state after acceptance.
