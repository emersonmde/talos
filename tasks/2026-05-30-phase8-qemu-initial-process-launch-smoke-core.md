# Phase 8 QEMU Initial Process Launch Smoke Core Task

Task: phase8-qemu-initial-process-launch-smoke-core-20260530

Status: accepted

## Scope

Milestone 8.3 QEMU/substitute evidence for the accepted target-independent
initial process launch-preparation boundary.

Changed files:

- build.rs
- src/main.rs
- src/initial_process_launch.rs
- src/process_install.rs
- src/process_page_table_materialization.rs
- src/program_loader.rs
- src/target/qemu_virt.rs
- scripts/qemu-initial-process-launch-smoke.sh
- docs/src/roadmap.md
- docs/src/decisions/README.md
- tasks/2026-05-30-phase8-qemu-initial-process-launch-smoke-core.md
- tasks/evidence/2026-05-30-qemu-initial-process-launch-smoke-core/qemu-initial-process-launch-smoke.log

Non-goals honored: no Pi 5 hardware run, boot archive publication,
hardwareTestLock acquisition, physical serial observation, lower-EL ERET,
TTBR/TCR/MAIR/SCTLR writes, ASID allocation, live TLB mutation, initial user
stack implementation, argv/envp expansion, process lifecycle, scheduler
runnable publication, shell behavior, descriptor-backed filesystem syscalls,
writable filesystem, networking, SSH, RP1/PCIe, UART interrupt ownership, or
DMA/cache-driver policy.

## Outcome

The task added qemu_initial_process_launch_smoke as a QEMU-only boot scenario
and retained script. The scenario consumes the accepted /bin/init
ProgramImagePlan, ProcessImageInstallPlan, ProcessAddressSpace, and
ProcessPageTableMaterialization chain, then constructs the accepted
phase8-initial-process-launch-plan-v1 launch-preparation record without
publishing runnable lower-EL state.

The retained smoke log proves:

- accepted fixture, install, address-space, materialization, and launch-plan
  identities;
- InitialProcessLaunchPlan success publication with entry copied from the
  accepted image lineage;
- UserText mapping and EL0-executable descriptor provenance for entry_pc;
- blocked-missing-initial-user-stack and blocked-no-ttbr-activation states;
- saved-frame intent without architectural register writes;
- no TTBR/TCR/MAIR/SCTLR, ASID, TLB, lower-EL ERET, scheduler, process-table,
  or descriptor-table side effects;
- ENOSYS runnable commit rejection with no-partial-launch and
  no-runnable-publication evidence; and
- deterministic identity, entry, descriptor, forbidden-range, destroyed-input,
  activation, stack-required, and scheduler-publication rejection cases.

## Evidence

- QEMU/substitute smoke log:
  tasks/evidence/2026-05-30-qemu-initial-process-launch-smoke-core/qemu-initial-process-launch-smoke.log
- Classification:
  qemu-initial-process-launch-smoke: final participants=11 expected=11
  errors=0 classification=qemu-initial-process-launch-smoke-complete
- PASS line:
  qemu-initial-process-launch-smoke: PASS
- Conditional QEMU/substitute regressions:
  scripts/qemu-process-page-table-materialization-smoke.sh,
  scripts/qemu-process-address-space-smoke.sh,
  scripts/qemu-process-install-smoke.sh, and
  scripts/qemu-program-loader-smoke.sh passed.

## Validation

- static inspection: git status --short before edits was clean.
- fmt/lint/typecheck: cargo fmt --all -- --check passed.
- unit tests: cargo -Zjson-target-spec test passed with 298 no_std tests.
- QEMU/substitute initial-process-launch smoke:
  scripts/qemu-initial-process-launch-smoke.sh passed and retained the
  required log.
- conditional QEMU/substitute regressions:
  scripts/qemu-process-page-table-materialization-smoke.sh,
  scripts/qemu-process-address-space-smoke.sh,
  scripts/qemu-process-install-smoke.sh, and
  scripts/qemu-program-loader-smoke.sh passed because this task touched
  target boot-scenario routing and the loader/install/address-space/
  materialization fixture chain.
- static inspection: git diff --check passed.
- documentation: mdbook build passed with the existing search-index size
  warning only.
- staged whitespace inspection: git diff --cached --check passed before
  commit.

## Residual Blocked Surfaces

Initial user stack, TTBR activation, ASID/TLB sequencing, lower-EL ERET,
argv/envp, process lifecycle, exec/spawn/wait, shell, descriptor-backed
filesystem syscalls, Pi 5 hardware proof, writable filesystems, persistent
storage, networking, SSH, RP1/PCIe, UART interrupt ownership, and
DMA/cache-driver policy remain blocked until later explicit tasks accept their
contracts and evidence gates.

## Commit

Recorded in durable supervisor state after acceptance.
