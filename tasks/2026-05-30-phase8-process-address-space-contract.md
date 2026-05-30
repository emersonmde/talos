# Phase 8 Process Address-Space Contract Task

Task: phase8-process-address-space-contract-20260530

Status: accepted

## Scope

Documentation-only Milestone 8.3 contract after the accepted process
address-space source inventory. The task selected the first process-owned
address-space installation boundary, owner vocabulary, lease lifecycle,
mapping order, rollback rules, deterministic errors, deferred surfaces, and
next QEMU/substitute evidence task.

Non-goals honored: no Rust or assembly behavior changes, no QEMU execution, no
Pi 5 hardware run, no boot archive publication, no hardwareTestLock
acquisition, no lower-EL launch, no argv/envp, no exec/spawn/wait, no shell,
no descriptor-backed filesystem syscall, no writable filesystem, no persistent
storage, no networking, no SSH, no RP1/PCIe, no UART interrupt ownership, and
no DMA/cache-driver policy.

## Evidence

- contract document:
  docs/src/project/phase8-process-address-space-contract.md.
- reviewed docs:
  - docs/src/project/phase8-process-address-space-source-inventory.md
  - docs/src/project/phase8-process-install-contract.md
  - docs/src/project/phase8-process-install-closeout-checkpoint.md
  - docs/src/project/phase8-qemu-process-install-smoke-plan.md
  - docs/src/project/phase7-el0-trap-address-space-contract.md
  - docs/src/project/phase7-copyin-copyout-helper-contract.md
- reviewed source owners:
  src/process_install.rs, src/program_loader.rs, src/posix.rs,
  src/scheduler.rs, src/memory_map/page_frames.rs,
  src/memory_map/translation.rs, and src/arch/aarch64/mod.rs.
- selected boundary: a target-independent ProcessAddressSpace record with
  explicit identity, owner label, model root/table leases, user-frame leases,
  ordered user mappings, copy/zero accounting, all-or-nothing rollback, and
  idempotent teardown; no active hardware translation context is accepted.
- blocked surfaces: AArch64 descriptor construction, TTBR/TCR switching,
  ASIDs/TLB/barriers, lower-EL launch, argv/envp, scheduler handoff,
  process table/PID/wait/exit, descriptor inheritance, filesystem syscalls,
  Pi 5 hardware proof, networking, and SSH.
- next bounded task recommendation:
  phase8-qemu-process-address-space-smoke-plan-20260530.

## Outcome

The contract makes the next implementation boundary mechanical but keeps
implementation blocked until the QEMU/substitute smoke plan is accepted. The
accepted boundary is process-owned address-space metadata plus lease/mapping
accounting and cleanup rules, not a hardware page-table installer or runnable
user process.

## Validation

- static inspection: git status --short before edits was clean.
- static inspection: git diff --check passed.
- documentation: mdbook build passed.
- staged whitespace inspection: git diff --cached --check passed before
  commit.

## Commit

Recorded in durable supervisor state after acceptance.
