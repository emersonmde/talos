# Phase 8 Initial Process Launch Source Inventory Task

Task: phase8-initial-process-launch-source-inventory-20260530

Status: accepted

## Scope

Documentation-only Milestone 8.3 inventory after the accepted process
page-table materialization closeout.

Changed files:

- docs/src/project/phase8-initial-process-launch-source-inventory.md
- docs/src/SUMMARY.md
- docs/src/roadmap.md
- docs/src/decisions/README.md
- tasks/2026-05-30-phase8-initial-process-launch-source-inventory.md

Non-goals honored: no Rust or assembly behavior changes, no TTBR/TCR/MAIR/SCTLR
writes, no ASID allocation, no live TLB invalidation, no lower-EL ERET to
/bin/init, no initial user stack implementation, no argv/envp/auxv/TLS setup,
no process table, no PID/wait/exit state, no scheduler runnable publication,
no shell, no descriptor-backed filesystem syscalls, no QEMU run, no Pi 5
hardware run, no boot archive publication, no hardwareTestLock acquisition,
no networking, no SSH, no RP1/PCIe, no UART interrupt ownership, and no
DMA/cache-driver policy.

## Outcome

The inventory maps the source owners and gaps for the first launch boundary
after accepted non-activating page-table materialization:

- accepted ProgramImagePlan entry and image metadata;
- accepted ProcessImageInstallPlan lower_el_launch_blocked metadata;
- accepted ProcessAddressSpace model leases, mappings, rollback, and teardown;
- accepted ProcessPageTableMaterialization descriptor-image/user-frame
  evidence with activation_blocked=true;
- AArch64 lower-EL entry, exception-frame, TTBR/TCR/MAIR/SCTLR, ASID/TLB, and
  barrier surfaces;
- scheduler ProcessOwnerId/Task/runnable-publication blockers; and
- QEMU/substitute and Pi 5 proof-local evidence producers.

The document recommends exactly one next bounded task:
phase8-initial-process-launch-contract-20260530. Implementation, QEMU
execution, lower-EL launch, process lifecycle, filesystem syscalls, hardware
proof, shell, networking, and SSH remain blocked.

## Evidence

- inventory document:
  docs/src/project/phase8-initial-process-launch-source-inventory.md
- reviewed accepted commits and evidence:
  - materialization closeout: 526646b1a76d4dedd8bac03828039dc3590d71b3.
  - materialization source inventory: 8de8472.
  - materialization contract: 7d8c0ce.
  - materialization smoke plan: 4ee01e8.
  - materialization core: 54d519e.
  - materialization smoke core: 6169783.
  - retained QEMU/substitute log:
    tasks/evidence/2026-05-30-qemu-process-page-table-materialization-smoke-core/qemu-process-page-table-materialization-smoke.log.
- next contract recommendation:
  phase8-initial-process-launch-contract-20260530.

## Validation

- static inspection: git status --short before edits was clean.
- static documentation/source review: inspected accepted launch-adjacent docs,
  task records, retained QEMU/substitute evidence, loader/install/address-space
  materialization source owners, AArch64 lower-EL helpers, exception routing,
  memory-map owners, scheduler owner placeholders, QEMU/Pi 5 evidence
  producers, roadmap, SUMMARY, and ADR index.
- static inspection: git diff --check passed.
- documentation: mdbook build passed.
- staged whitespace inspection: git diff --cached --check passed before
  commit.

## Residual Blocked Surfaces

TTBR/TCR/MAIR/SCTLR mutation, ASID/TLB sequencing, lower-EL launch, user stack,
argv/envp/auxv/TLS, process lifecycle, exec/spawn/wait, scheduler runnable
publication, shell, descriptor-backed filesystem syscalls, Pi 5 hardware
proof, writable filesystems, persistent storage, networking, SSH, RP1/PCIe,
UART interrupt ownership, and DMA/cache-driver policy remain blocked until
later explicit tasks accept their contracts and evidence gates.

## Commit

Recorded in durable supervisor state after acceptance.
