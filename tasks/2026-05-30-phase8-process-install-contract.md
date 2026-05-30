# Phase 8 Process Install Contract Task

Task: phase8-process-install-contract-20260530

Status: accepted

## Scope

Documentation-only contract for the first process-install boundary from an
accepted ProgramImagePlan. The selected boundary is target-independent and
metadata-only: a ProcessImageInstallPlan with ordered page records, exact
permissions, copy/zero-fill ranges, rollback rules, deterministic errors, and
explicit deferrals.

No Rust or assembly behavior changed. No QEMU execution, Pi 5 hardware run,
boot archive publication, hardwareTestLock acquisition, lower-EL launch,
argv/envp construction, exec/spawn/wait, shell, descriptor-backed filesystem
syscall, writable filesystem, persistent storage, networking, SSH, RP1/PCIe,
UART interrupt ownership, or DMA/cache-driver policy was added.

## Reviewed Inputs

- docs/src/project/phase8-process-install-source-inventory.md
- docs/src/project/phase8-program-loader-format-contract.md
- docs/src/project/phase8-program-loader-closeout-checkpoint.md
- docs/src/project/phase7-el0-trap-address-space-contract.md
- docs/src/project/phase7-descriptor-table-contract.md
- src/program_loader.rs
- src/posix.rs
- src/scheduler.rs
- docs/src/roadmap.md
- docs/src/decisions/README.md

## Accepted Decision

- First implementation boundary: metadata-only ProcessImageInstallPlan.
- Address-space owner: future ProcessAddressSpace; not implemented here.
- Frame/page ownership: future user-frame leases; represented only as ordered
  allocate/copy/zero/map metadata.
- Ordering: validate complete plan, allocate all frames, copy file bytes,
  zero BSS and rounded tails, install exact-permission leaves, publish only
  after success.
- Rollback: no plan on metadata failure; future mutator must release frames
  and remove mappings in reverse order on partial failure.
- Deferrals: descriptor inheritance, initial lower-EL frame, user stack,
  argv/envp, process table/PID, scheduler handoff, lower-EL launch, hardware,
  shell, filesystem syscall, networking, and driver policy.

## Evidence

- documentation: added
  docs/src/project/phase8-process-install-contract.md.
- documentation: added this task record.
- documentation: updated docs/src/SUMMARY.md, docs/src/roadmap.md, and
  docs/src/decisions/README.md.
- static inspection: git status --short before edits was clean.
- static inspection: git diff --check passed.
- documentation: mdbook build passed.
- staged whitespace inspection: git diff --cached --check passed before
  commit.

## Next Action

The next bounded task should be
phase8-qemu-process-install-smoke-plan-20260530, documentation-only, if
dependencies remain satisfied. It should specify the QEMU/substitute evidence
for the metadata-only ProcessImageInstallPlan boundary and keep implementation,
hardware, and lower-EL launch blocked.
