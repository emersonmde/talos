# Phase 8 QEMU Initial User Stack Smoke Plan Task

Task: phase8-qemu-initial-user-stack-smoke-plan-20260530

Status: accepted

## Scope

Documentation-only Milestone 8.3 QEMU/substitute smoke plan after the accepted
initial user stack contract. The task fixed the retained evidence boundary for
the future InitialUserStackPlan implementation.

Changed files:

- docs/src/project/phase8-qemu-initial-user-stack-smoke-plan.md
- docs/src/SUMMARY.md
- docs/src/roadmap.md
- docs/src/decisions/README.md
- tasks/2026-05-30-phase8-qemu-initial-user-stack-smoke-plan.md

Non-goals honored: no Rust or assembly behavior changes, no QEMU execution, no
Pi 5 hardware run, no boot archive publication, no hardwareTestLock
acquisition, no initial user stack implementation, no TTBR activation, no
lower-EL ERET, no scheduler runnable publication, no process lifecycle, no
shell, no descriptor-backed filesystem syscalls, no networking, no SSH, no
RP1/PCIe, no UART interrupt ownership, and no DMA/cache-driver policy.

## Outcome

The smoke plan selects qemu_initial_user_stack_smoke as a QEMU/substitute
scenario for the accepted stack-preparation boundary. It requires retained
evidence at:

    tasks/evidence/2026-05-30-qemu-initial-user-stack-smoke-core/qemu-initial-user-stack-smoke.log

The required final vocabulary is:

    qemu-initial-user-stack-smoke: final participants=13 expected=13 errors=0 classification=qemu-initial-user-stack-smoke-complete
    qemu-initial-user-stack-smoke: PASS

The plan defines success observations for:

- accepted ProgramImagePlan, ProcessImageInstallPlan, ProcessAddressSpace,
  ProcessPageTableMaterialization, and InitialProcessLaunchPlan identity
  lineage;
- stack boundary phase8-initial-user-stack-plan-v1;
- stack top and initial SP 0x0000_8000_0000_0000 with 16-byte alignment;
- usable range [0x0000_7fff_ffff_c000, 0x0000_8000_0000_0000) and guard
  range [0x0000_7fff_ffff_b000, 0x0000_7fff_ffff_c000);
- four USER_DATA stack-owned usable pages, one unmapped guard page,
  copied_bytes=0, and zeroed_bytes=0x4000;
- minimal-empty-argc0 startup placeholder with argv/envp NULL and auxv/TLS
  still blocked;
- launch binding that changes only model user_sp_state and saved-frame SP_EL0
  intent; and
- no TTBR/TCR/MAIR/SCTLR, ASID, TLB, lower-EL ERET, scheduler,
  process-table, or descriptor-table side effects.

It defines deterministic rejection evidence for mismatched identities, stack
range faults, image overlap, executable-stack permission requests, exhausted
stack-page capacity, already-stack-ready launch inputs, and unsupported
live-launch requests. Pi 5 hardware proof, live TTBR activation, lower-EL
launch, process lifecycle, broad argv/envp/auxv/TLS ABI, filesystem syscalls,
networking, and SSH remain blocked.

## Evidence

- smoke plan document:
  docs/src/project/phase8-qemu-initial-user-stack-smoke-plan.md.
- reviewed accepted stack docs:
  - docs/src/project/phase8-initial-user-stack-contract.md
  - docs/src/project/phase8-initial-user-stack-source-inventory.md
- reviewed smoke-plan pattern:
  - docs/src/project/phase8-qemu-initial-process-launch-smoke-plan.md
- selected QEMU/substitute scenario:
  qemu_initial_user_stack_smoke.
- retained evidence path for the later smoke core:
  tasks/evidence/2026-05-30-qemu-initial-user-stack-smoke-core/qemu-initial-user-stack-smoke.log.
- required classification/PASS vocabulary:
  qemu-initial-user-stack-smoke-complete and
  qemu-initial-user-stack-smoke: PASS.
- next bounded task:
  phase8-initial-user-stack-core-20260530.

## Validation

- static inspection: git status --short before edits was clean.
- static documentation/source review: inspected the accepted initial user
  stack contract and source inventory, the QEMU initial process launch
  smoke-plan pattern, adjacent Phase 8 contracts, roadmap, SUMMARY, and ADR
  index.
- static inspection: git diff --check passed.
- documentation: mdbook build passed.
- staged whitespace inspection: git diff --cached --check passed before
  commit.

## Residual Blocked Surfaces

TTBR/TCR/MAIR/SCTLR mutation, ASID/TLB sequencing, lower-EL launch,
scheduler runnable publication, process lifecycle, exec/spawn/wait, broad
argv/envp/auxv/TLS ABI, shell, descriptor-backed filesystem syscalls, Pi 5
hardware proof, writable filesystems, persistent storage, networking, SSH,
RP1/PCIe, UART interrupt ownership, and DMA/cache-driver policy remain
blocked until later explicit tasks accept their contracts and evidence gates.

## Commit

Recorded in durable supervisor state after acceptance.
