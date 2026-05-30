# Phase 8 Live Address-Space Activation Contract Task

Task: phase8-live-address-space-activation-contract-20260530

Status: accepted

## Scope

Completed a documentation-only contract for the first live address-space
activation boundary below live TTBR mutation, lower-EL launch, and scheduler
runnable publication.

Non-goals honored: no Rust or assembly behavior changes, no QEMU execution,
no Pi 5 hardware run, no boot archive publication, no hardwareTestLock
acquisition, no TTBR/TCR/MAIR/SCTLR writes, no ASID allocation, no live TLB
invalidation, no lower-EL ERET, no scheduler runnable publication, no process
lifecycle, no shell, no descriptor-backed filesystem syscalls, no writable
filesystem, no networking, no SSH, no RP1/PCIe, no UART interrupt ownership,
and no DMA/cache-driver policy.

## Evidence

- contract document:
  docs/src/project/phase8-live-address-space-activation-contract.md.
- reviewed source/docs:
  - docs/src/project/phase8-live-address-space-activation-source-inventory.md.
  - docs/src/project/phase8-initial-user-stack-closeout-checkpoint.md.
  - docs/src/project/phase8-initial-user-stack-contract.md.
  - docs/src/project/phase8-initial-process-launch-contract.md.
  - docs/src/project/phase8-process-page-table-materialization-contract.md.
  - docs/src/architecture/lower-el-userspace.md.
  - src/program_loader.rs.
  - src/process_install.rs.
  - src/process_address_space.rs.
  - src/process_page_table_materialization.rs.
  - src/initial_process_launch.rs.
  - src/initial_user_stack.rs.
  - src/posix.rs.
  - src/memory_map/translation.rs.
  - src/arch/aarch64/mod.rs.
  - src/arch/aarch64/exceptions.rs.
  - src/scheduler.rs.
  - docs/src/roadmap.md.
  - docs/src/decisions/README.md.
- accepted contract result: the selected boundary is a
  LiveAddressSpaceActivationPlan with identity
  phase8-live-address-space-activation-plan-v1 and policy
  preflight-split-user-ttbr0-kernel-reachability-blocked-v1.
- selected boundary identity and blocked surfaces recorded: TTBR0 root
  provenance is tied to the materialized root lease, while TTBR1/kernel-half
  mapping, live register mutation, ASID allocation, TLBI, lower-EL ERET,
  scheduler runnable publication, process lifecycle, startup ABI expansion,
  filesystem syscalls, Pi 5 hardware proof, networking, and SSH remain blocked.
- deterministic behavior recorded: activation preflight is all-or-nothing,
  returns stable errors/blockers, and leaves no partial activation, no live
  translation side effect, and no runnable publication on failure.
- recommended next task:
  phase8-qemu-live-address-space-activation-smoke-plan-20260530.

## Validation

- static inspection: git status --short before edits was clean.
- static documentation/source review: inspected accepted activation inventory,
  launch/stack/materialization contracts, lower-EL architecture notes, source
  owners, roadmap, SUMMARY, and ADR index.
- static inspection: git diff --check passed.
- documentation: mdbook build passed.
- staged whitespace inspection: git diff --cached --check passed before
  commit.

## Result

Accepted as a documentation-only contract. It names the queued
phase8-qemu-live-address-space-activation-smoke-plan-20260530 task as the next
bounded documentation-only step and keeps implementation, live register
mutation, lower-EL launch, scheduler runnable publication, process lifecycle,
filesystem syscalls, hardware proof, networking, and SSH blocked.

Commit: recorded in durable supervisor state after acceptance.
