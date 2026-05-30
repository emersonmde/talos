# Phase 8 QEMU Initial Process Launch Smoke Plan Task

Task: phase8-qemu-initial-process-launch-smoke-plan-20260530

Status: accepted

## Scope

Documentation-only Milestone 8.3 QEMU/substitute smoke plan after the accepted
initial process launch contract. The task fixed the retained evidence boundary
for the future InitialProcessLaunchPlan implementation.

Changed files:

- docs/src/project/phase8-qemu-initial-process-launch-smoke-plan.md
- docs/src/SUMMARY.md
- docs/src/roadmap.md
- docs/src/decisions/README.md
- tasks/2026-05-30-phase8-qemu-initial-process-launch-smoke-plan.md

Non-goals honored: no Rust or assembly behavior changes, no QEMU execution, no
Pi 5 hardware run, no boot archive publication, no hardwareTestLock
acquisition, no initial process launch implementation, no TTBR activation, no
lower-EL ERET, no initial user stack implementation, no argv/envp/auxv/TLS, no
process lifecycle, no scheduler runnable publication, no shell, no
descriptor-backed filesystem syscalls, no networking, no SSH, no RP1/PCIe, no
UART interrupt ownership, and no DMA/cache-driver policy.

## Outcome

The smoke plan selects qemu_initial_process_launch_smoke as a QEMU/substitute
scenario for the accepted launch-preparation boundary. It requires retained
evidence at:

    tasks/evidence/2026-05-30-qemu-initial-process-launch-smoke-core/qemu-initial-process-launch-smoke.log

The required final vocabulary is:

    qemu-initial-process-launch-smoke: final participants=11 expected=11 errors=0 classification=qemu-initial-process-launch-smoke-complete
    qemu-initial-process-launch-smoke: PASS

The plan defines success observations for:

- accepted ProgramImagePlan, ProcessImageInstallPlan, ProcessAddressSpace, and
  ProcessPageTableMaterialization identity lineage;
- launch boundary phase8-initial-process-launch-plan-v1;
- entry provenance through image, install, UserText mapping, and
  EL0-executable descriptor coverage;
- user_sp_state=blocked-missing-initial-user-stack;
- activation_state=blocked-no-ttbr-activation;
- saved-frame intent without architectural register writes; and
- no TTBR/TCR/MAIR/SCTLR, ASID, TLB, lower-EL ERET, scheduler,
  process-table, or descriptor-table side effects.

It defines deterministic rejection evidence for mismatched identities, bad
entry provenance, missing UserText descriptor coverage, forbidden entry range,
destroyed inputs, activation requests, stack-required launch requests, and
scheduler publication requests. Pi 5 hardware proof, live TTBR activation,
initial user stack implementation, lower-EL launch, process lifecycle,
filesystem syscalls, networking, and SSH remain blocked.

## Evidence

- smoke plan document:
  docs/src/project/phase8-qemu-initial-process-launch-smoke-plan.md.
- reviewed accepted launch docs:
  - docs/src/project/phase8-initial-process-launch-contract.md
  - docs/src/project/phase8-initial-process-launch-source-inventory.md
- reviewed smoke-plan pattern:
  - docs/src/project/phase8-qemu-process-page-table-materialization-smoke-plan.md
- selected QEMU/substitute scenario:
  qemu_initial_process_launch_smoke.
- retained evidence path for the later smoke core:
  tasks/evidence/2026-05-30-qemu-initial-process-launch-smoke-core/qemu-initial-process-launch-smoke.log.
- required classification/PASS vocabulary:
  qemu-initial-process-launch-smoke-complete and
  qemu-initial-process-launch-smoke: PASS.
- next bounded task:
  phase8-initial-process-launch-core-20260530.

## Validation

- static inspection: git status --short before edits was clean.
- static documentation/source review: inspected the accepted initial process
  launch contract and source inventory, the QEMU process page-table
  materialization smoke-plan pattern, adjacent Phase 8 contracts, roadmap,
  SUMMARY, and ADR index.
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
