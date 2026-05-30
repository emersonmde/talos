# Phase 8 QEMU Process Page-Table Materialization Smoke Plan Task

Task: phase8-qemu-process-page-table-materialization-smoke-plan-20260530

Status: accepted

## Scope

Documentation-only Milestone 8.3 QEMU/substitute smoke plan after the accepted
process page-table materialization contract. The task fixed the retained
evidence boundary for the future non-activating descriptor-image and
user-frame materialization implementation.

Non-goals honored: no Rust or assembly behavior changes, no QEMU execution, no
Pi 5 hardware run, no boot archive publication, no hardwareTestLock
acquisition, no TTBR activation, no ASID/TLB policy change, no lower-EL
launch, no argv/envp, no process lifecycle, no shell, no descriptor-backed
filesystem syscall, no writable filesystem, no persistent storage, no
networking, no SSH, no RP1/PCIe, no UART interrupt ownership, and no
DMA/cache-driver policy.

## Evidence

- smoke plan document:
  docs/src/project/phase8-qemu-process-page-table-materialization-smoke-plan.md.
- reviewed accepted materialization docs:
  - docs/src/project/phase8-process-page-table-materialization-contract.md
  - docs/src/project/phase8-process-page-table-materialization-source-inventory.md
- reviewed smoke-plan patterns:
  - docs/src/project/phase8-qemu-process-address-space-smoke-plan.md
  - docs/src/project/phase8-qemu-process-install-smoke-plan.md
- selected QEMU/substitute scenario:
  qemu_process_page_table_materialization_smoke.
- retained evidence path for the later smoke core:
  tasks/evidence/2026-05-30-qemu-process-page-table-materialization-smoke-core/qemu-process-page-table-materialization-smoke.log.
- required classification/PASS vocabulary:
  qemu-process-page-table-materialization-smoke-complete and
  qemu-process-page-table-materialization-smoke: PASS.
- next bounded task:
  phase8-process-page-table-materialization-core-20260530.

## Outcome

The smoke plan defines success observations for the accepted /bin/init fixture
chain, ProcessPageTableMaterialization identity, owned root/table/user-frame
records, UserText/UserData descriptor permissions, copy/zero accounting,
activation_blocked=true, no live TTBR/TLB/scheduler mutation, rollback,
no-leak rejection, and idempotent teardown.

It defines deterministic rejection evidence for bad address-space inputs,
forbidden ranges, permission widening, resource exhaustion, unsupported
topology, copy/zero mismatch, and activation requests. Pi 5 hardware proof,
TTBR activation, lower-EL launch, argv/envp, process lifecycle, filesystem
syscalls, networking, and SSH remain blocked until later explicit tasks accept
their contracts and gates.

## Validation

- static inspection: git status --short before edits was clean.
- static inspection: git diff --check passed.
- documentation: mdbook build passed.
- staged whitespace inspection: git diff --cached --check passed before
  commit.

## Commit

Recorded in durable supervisor state after acceptance.
