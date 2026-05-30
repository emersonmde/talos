# Phase 8 QEMU Process Page-Table Materialization Smoke Core Task

Task: phase8-qemu-process-page-table-materialization-smoke-core-20260530

Status: accepted

## Scope

Milestone 8.3 QEMU/substitute evidence for the accepted non-activating process
page-table materialization boundary.

Changed files:

- build.rs
- src/main.rs
- src/process_address_space.rs
- src/process_install.rs
- src/process_page_table_materialization.rs
- src/target/qemu_virt.rs
- scripts/qemu-process-page-table-materialization-smoke.sh
- docs/src/roadmap.md
- docs/src/decisions/README.md
- tasks/2026-05-30-phase8-qemu-process-page-table-materialization-smoke-core.md
- tasks/evidence/2026-05-30-qemu-process-page-table-materialization-smoke-core/qemu-process-page-table-materialization-smoke.log

Non-goals honored: no Pi 5 hardware run, boot archive publication,
hardwareTestLock acquisition, TTBR/TCR/MAIR/SCTLR writes, live TLB mutation,
ASID allocation, lower-EL launch, argv/envp setup, process lifecycle, scheduler
publication, shell behavior, descriptor-backed filesystem syscalls, writable
filesystem, networking, SSH, RP1/PCIe, UART interrupt ownership, or
DMA/cache-driver policy.

## Outcome

The task added qemu_process_page_table_materialization_smoke as a QEMU-only
boot scenario and script. The scenario consumes the accepted /bin/init
ProgramImagePlan, ProcessImageInstallPlan, and ProcessAddressSpace chain, then
materializes phase8-process-page-table-materialization-v1 without activation.

The retained smoke log proves:

- success publication with one root descriptor-image page, three table pages,
  three user frames, three descriptors, activation_blocked=true, and
  kernel_mapping_policy=activation-blocked-no-kernel-half;
- UserText/UserData frame copy/zero accounting, zero-before-copy, source-page,
  physical-frame, and scrub-required evidence;
- UserText/UserData descriptor permission preservation, AP, PXN/UXN, AF,
  normal-inner-shareable, and W^X evidence;
- no TTBR/TLB/scheduler/lower-EL/runnable side effects;
- first teardown releases descriptors, table pages, user frames, and root page;
- second teardown is already-destroyed without double release; and
- deterministic bad-address-space, forbidden-range, permission-widening,
  resource-exhaustion, unsupported-topology, copy-zero-mismatch, and
  activation-request rejections with no partial materialization and no leaked
  leases.

## Evidence

- QEMU/substitute smoke log:
  tasks/evidence/2026-05-30-qemu-process-page-table-materialization-smoke-core/qemu-process-page-table-materialization-smoke.log
- Classification:
  qemu-process-page-table-materialization-smoke: final participants=12
  expected=12 errors=0
  classification=qemu-process-page-table-materialization-smoke-complete
- PASS line:
  qemu-process-page-table-materialization-smoke: PASS
- Conditional QEMU/substitute regressions:
  scripts/qemu-process-address-space-smoke.sh passed, and
  scripts/qemu-process-install-smoke.sh passed.

## Validation

- static inspection: git status --short before edits was clean.
- fmt/lint/typecheck: cargo fmt --all -- --check passed.
- unit tests: cargo -Zjson-target-spec test passed with 291 no_std tests.
- QEMU/substitute materialization smoke:
  scripts/qemu-process-page-table-materialization-smoke.sh passed and retained
  the required log.
- conditional QEMU/substitute regressions:
  scripts/qemu-process-address-space-smoke.sh passed and
  scripts/qemu-process-install-smoke.sh passed because this task touched
  boot-scenario routing and the loader/install/address-space fixture chain.
- static inspection: git diff --check passed.
- documentation: mdbook build passed with the existing search-index size
  warning only.
- staged whitespace inspection: git diff --cached --check passed before
  commit.

## Residual Blocked Surfaces

TTBR activation, kernel-half mapping policy, ASID/TLB sequencing,
lower-EL launch, argv/envp, process lifecycle, exec/spawn/wait, shell,
descriptor-backed filesystem syscalls, Pi 5 hardware proof, writable
filesystems, persistent storage, networking, SSH, RP1/PCIe, UART interrupt
ownership, and DMA/cache-driver policy remain blocked until later explicit
tasks accept their contracts and evidence gates.

## Commit

Recorded in durable supervisor state after acceptance.
