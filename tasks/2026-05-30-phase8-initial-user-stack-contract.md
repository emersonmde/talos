# Phase 8 Initial User Stack Contract Task

Task: phase8-initial-user-stack-contract-20260530

Status: accepted

## Scope

Completed a documentation-only contract for the first target-independent
initial user stack construction boundary after the accepted source inventory.

Non-goals honored: no Rust or assembly behavior changes, no QEMU execution,
no Pi 5 hardware run, no boot archive publication, no hardwareTestLock
acquisition, no TTBR/TCR/MAIR/SCTLR writes, no ASID allocation, no live TLB
invalidation, no lower-EL ERET, no scheduler runnable publication, no process
lifecycle, no shell, no descriptor-backed filesystem syscalls, no networking,
no SSH, no RP1/PCIe, no UART interrupt ownership, and no DMA/cache-driver
policy.

## Evidence

- contract document:
  docs/src/project/phase8-initial-user-stack-contract.md.
- reviewed source/docs:
  - docs/src/project/phase8-initial-user-stack-source-inventory.md.
  - docs/src/project/phase8-initial-process-launch-contract.md.
  - docs/src/project/phase8-initial-process-launch-closeout-checkpoint.md.
  - src/posix.rs.
  - src/program_loader.rs.
  - src/process_install.rs.
  - src/process_address_space.rs.
  - src/process_page_table_materialization.rs.
  - src/initial_process_launch.rs.
  - docs/src/architecture/lower-el-userspace.md.
  - docs/src/roadmap.md.
  - docs/src/decisions/README.md.
- accepted contract result: InitialUserStackPlan should be a model-only
  target-independent record with stack top 0x0000_8000_0000_0000, usable
  range [0x0000_7fff_ffff_c000, 0x0000_8000_0000_0000), guard range
  [0x0000_7fff_ffff_b000, 0x0000_7fff_ffff_c000), 16-byte initial SP
  alignment, USER_DATA usable pages, copied_bytes=0, zeroed_bytes=0x4000,
  idempotent stack teardown, deterministic errors, and no-partial-stack /
  no-partial-launch guarantees.
- launch integration result: the stack binding may change only model
  user_sp_state from blocked-missing-initial-user-stack to
  model-only-initial-user-stack-ready and saved-frame SP intent to the stack
  top. Activation, ERET, scheduler publication, process table mutation,
  descriptor table mutation, filesystem syscalls, Pi 5 proof, networking, and
  SSH remain blocked.
- recommended next task:
  phase8-qemu-initial-user-stack-smoke-plan-20260530.

## Validation

- static inspection: git status --short before edits was clean.
- static documentation/source review: inspected accepted inventory and
  launch-preparation docs plus source owners for POSIX, loader,
  process-install, address-space, materialization, and initial launch records.
- static inspection: git diff --check passed.
- documentation: mdbook build passed.
- staged whitespace inspection: git diff --cached --check passed before
  commit.

## Result

Accepted as a documentation-only contract. It names the queued
phase8-qemu-initial-user-stack-smoke-plan-20260530 task as the next bounded
documentation-only step and keeps implementation, live translation
activation, lower-EL launch, scheduler publication, process lifecycle,
filesystem syscalls, hardware proof, networking, and SSH blocked.

Commit: recorded in durable supervisor state after acceptance.
