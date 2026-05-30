# Phase 8 Initial Process Launch Core Task

Task: phase8-initial-process-launch-core-20260530

Status: accepted

## Scope

Milestone 8.3 implementation of the accepted target-independent initial
process launch-preparation boundary.

Changed files:

- src/main.rs
- src/initial_process_launch.rs
- docs/src/roadmap.md
- docs/src/decisions/README.md
- tasks/2026-05-30-phase8-initial-process-launch-core.md

Non-goals honored: no shell, no argv/envp/auxv/TLS expansion, no initial user
stack implementation, no TTBR/TCR/MAIR/SCTLR writes, no ASID allocation, no
live TLB invalidation, no lower-EL ERET, no process table/PID/wait/exit, no
scheduler runnable publication, no descriptor-backed filesystem syscalls, no
QEMU evidence run for the new boundary, no Pi 5 hardware run, no boot archive
publication, no hardwareTestLock acquisition, no networking, no SSH, no
RP1/PCIe, no UART interrupt ownership, and no DMA/cache-driver policy.

## Outcome

The implementation adds phase8-initial-process-launch-plan-v1 as a
launch-preparation record only. It consumes the accepted ProgramImagePlan,
ProcessImageInstallPlan, ProcessAddressSpace, and
ProcessPageTableMaterialization records and produces:

- copied loader, install, address-space, materialization, and launch boundary
  identities;
- entry_pc copied from the validated ProgramImagePlan lineage only after
  install, UserText mapping, and EL0-executable descriptor provenance checks;
- user_sp_state=blocked-missing-initial-user-stack;
- activation_state=blocked-no-ttbr-activation;
- saved-frame intent for ELR, SP_EL0, SPSR, x0..x5, DAIF, and model-only
  address-space token state without architectural register writes; and
- zero TTBR/TCR/MAIR/SCTLR, ASID, live TLB, lower-EL ERET, scheduler,
  process-table, and descriptor-table side effects.

Runnable commit, activation, stack-required launch, and scheduler publication
requests remain blocked with ENOSYS, no-partial-launch=true, and
no-runnable-publication=true semantics.

## Evidence

- changed files: src/main.rs, src/initial_process_launch.rs,
  docs/src/roadmap.md, docs/src/decisions/README.md, and
  tasks/2026-05-30-phase8-initial-process-launch-core.md.
- unit tests: cargo -Zjson-target-spec test passed with 298 tests.
- covered success path: creates InitialProcessLaunchPlan with accepted
  identity lineage, source digest/path, entry_pc, blocked user SP,
  blocked activation, saved-frame intent, published=true, and zero side
  effects.
- covered deterministic rejection: bad fixture identity returns EINVAL, entry
  mismatch returns ENOEXEC, forbidden entry range returns EACCES, destroyed
  address-space input returns EINVAL, and activation/stack/scheduler requests
  return ENOSYS.
- covered no unintended runnable publication: runnable commit rejection
  reports no-partial-launch=true and no-runnable-publication=true while
  scheduler_published remains false.
- conditional QEMU/substitute rationale: no existing runtime, target routing,
  loader, process-install, process-address-space, materialization, lower-EL,
  or boot-scenario owner behavior was changed. Retained QEMU/substitute
  evidence for this new boundary is the explicit next queued smoke-core task.

## Validation

- static inspection: git status --short before edits was clean.
- fmt/lint/typecheck: cargo fmt --all -- --check passed.
- unit tests: cargo -Zjson-target-spec test passed with 298 tests.
- QEMU/substitute: not run for this task; accepted launch evidence remains
  queued as phase8-qemu-initial-process-launch-smoke-core-20260530.
- static inspection: git diff --check passed.
- documentation: mdbook build passed.
- staged whitespace inspection: git diff --cached --check passed before
  commit.

## Residual Blocked Surfaces

Initial user stack allocation, argv/envp/auxv/TLS layout, TTBR activation,
ASID/TLB sequencing, lower-EL ERET, process lifecycle, scheduler runnable
publication, descriptor inheritance, shell, descriptor-backed filesystem
syscalls, Pi 5 hardware proof, writable filesystems, persistent storage,
networking, SSH, RP1/PCIe, UART interrupt ownership, and DMA/cache-driver
policy remain blocked until later explicit tasks accept their contracts and
evidence gates.

## Commit

Recorded in durable supervisor state after acceptance.
