# Phase 8 Process Page-Table Materialization Core Task

Task: phase8-process-page-table-materialization-core-20260530

Status: accepted

## Scope

Milestone 8.3 implementation of the accepted non-activating process
page-table materialization boundary.

Changed files:

- src/main.rs
- src/memory_map/mod.rs
- src/memory_map/translation.rs
- src/process_page_table_materialization.rs
- docs/src/roadmap.md
- docs/src/decisions/README.md
- tasks/2026-05-30-phase8-process-page-table-materialization-core.md

Non-goals honored: no TTBR/TCR/MAIR/SCTLR writes, no ASID allocation, no live
TLB invalidation, no scheduler publication, no lower-EL launch, no argv/envp
setup, no process lifecycle, no descriptor-backed filesystem syscalls, no
shell, no QEMU evidence run for the new boundary, no Pi 5 hardware run, no boot
archive publication, no hardwareTestLock acquisition, no networking, no SSH,
no RP1/PCIe, no UART interrupt ownership, and no DMA/cache-driver policy.

## Outcome

The implementation adds
phase8-process-page-table-materialization-v1 as descriptor-image evidence
only. It consumes the accepted ProgramImagePlan, ProcessImageInstallPlan, and
ProcessAddressSpace records and produces:

- one owned root descriptor-image lease;
- the minimum first-contract table-page lease set for one L0/L1/L2 path;
- one owned user-frame evidence lease per accepted UserFrameLease;
- ordered AArch64 EL0 user descriptor records for UserText and UserData pages;
- side-effect counters for leased resources, populated frames, descriptors,
  copied/zeroed bytes, rollback releases, teardown releases, and
  activation_blocked=true; and
- kernel_mapping_policy=activation-blocked-no-kernel-half.

Deterministic errors reject bad address-space/install input, forbidden or
malformed mappings, permission widening, activation requests, unsupported
topology, resource exhaustion, and copy/zero population failure without
visible partial materialization.

## Evidence

- unit tests: cargo -Zjson-target-spec test passed with 291 tests.
- covered success path: materializes descriptor image with preserved
  UserText/UserData permissions, descriptor bits, owned leases, copied/zeroed
  byte accounting, activation_blocked=true, and blocked kernel mapping policy.
- covered deterministic rejection: activation request returns ENOSYS,
  bad address-space/install mismatch returns EINVAL, unsupported topology
  returns ENOTSUP.
- covered rollback/no-leak: root/table/user-frame/descriptor capacity failures
  and copy/zero population failure release all acquired leases.
- covered teardown: first destroy clears descriptors and releases resources;
  second destroy reports already_destroyed without double release.
- conditional QEMU/substitute rationale: the accepted materialization smoke
  command did not exist before this core task; retained QEMU evidence is the
  explicit next queued smoke-core task. The implementation did not change
  ProcessAddressSpace generation, process-install generation, loader fixture
  bytes, read-only initramfs/VFS, syscall dispatch, descriptor tables,
  user-copy helpers, lower-EL routing, or boot-scenario routing.

## Validation

- static inspection: git status --short before edits showed one in-scope
  partial module-wire edit in src/main.rs from the active task.
- fmt/lint/typecheck: cargo fmt --all -- --check passed.
- unit tests: cargo -Zjson-target-spec test passed with 291 tests.
- QEMU/substitute: not run for this task; accepted materialization evidence
  remains queued as phase8-qemu-process-page-table-materialization-smoke-core-20260530.
- static inspection: git diff --check passed.
- documentation: mdbook build passed.
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
