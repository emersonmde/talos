# Phase 8 Initial User Stack Source Inventory Task

Task: phase8-initial-user-stack-source-inventory-20260530

Status: accepted

## Scope

Completed a documentation-only source inventory for the first initial user
stack construction boundary after the accepted initial process launch
closeout.

Non-goals honored: no Rust or assembly behavior changes, no stack
implementation, no argv/envp/auxv/TLS construction, no QEMU execution, no
Pi 5 hardware run, no boot archive publication, no hardwareTestLock
acquisition, no TTBR/TCR/MAIR/SCTLR writes, no ASID allocation, no live TLB
invalidation, no lower-EL ERET, no scheduler runnable publication, no process
lifecycle, no shell, no descriptor-backed filesystem syscalls, no networking,
no SSH, no RP1/PCIe, no UART interrupt ownership, and no DMA/cache-driver
policy.

## Evidence

- inventory document:
  docs/src/project/phase8-initial-user-stack-source-inventory.md.
- reviewed source/docs:
  - docs/src/project/phase8-initial-process-launch-closeout-checkpoint.md.
  - docs/src/project/phase8-initial-process-launch-source-inventory.md.
  - docs/src/project/phase8-initial-process-launch-contract.md.
  - tasks/2026-05-30-phase8-initial-process-launch-core.md.
  - tasks/2026-05-30-phase8-qemu-initial-process-launch-smoke-core.md.
  - tasks/evidence/2026-05-30-qemu-initial-process-launch-smoke-core/qemu-initial-process-launch-smoke.log.
  - src/posix.rs.
  - src/program_loader.rs.
  - src/process_install.rs.
  - src/process_address_space.rs.
  - src/process_page_table_materialization.rs.
  - src/initial_process_launch.rs.
  - src/arch/aarch64/mod.rs.
  - src/arch/aarch64/exceptions.rs.
  - src/scheduler.rs.
  - src/target/qemu_virt.rs.
  - src/target/rpi5.rs.
  - docs/src/architecture/lower-el-userspace.md.
  - docs/src/roadmap.md.
  - docs/src/decisions/README.md.
- accepted frontier reviewed: InitialProcessLaunchPlan remains
  target-independent launch preparation with user_sp_state
  blocked-missing-initial-user-stack, activation_state
  blocked-no-ttbr-activation, saved-frame intent only, and zero live-launch
  side effects.
- inventory result: initial stack ownership is not yet accepted; the smallest
  next boundary is a target-independent stack record contract that defines
  range, guard, alignment, frame/page ownership, zero/copy accounting,
  teardown, deterministic errors, and launch-plan stack-state integration.
- recommended next task:
  phase8-initial-user-stack-contract-20260530.

## Validation

- static inspection: git status --short before edits was clean.
- static documentation/source review: inspected accepted launch preparation
  docs, task records, retained QEMU/substitute evidence, source owners, lower
  EL architecture notes, roadmap, SUMMARY, and ADR index.
- static inspection: git diff --check passed.
- documentation: mdbook build passed.
- staged whitespace inspection: git diff --cached --check passed before
  commit.

## Result

Accepted as a documentation-only inventory. It recommends the queued
phase8-initial-user-stack-contract-20260530 task as the next bounded
documentation-only step. Initial stack construction, live translation
activation, lower-EL launch, scheduler publication, process lifecycle,
filesystem syscalls, hardware proof, networking, and SSH remain blocked.

Commit: recorded in durable supervisor state after acceptance.
