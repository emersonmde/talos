# Phase 8 Process Address-Space Core Task

Task: phase8-process-address-space-core-20260530
Status: accepted

## Scope

Implemented the accepted target-independent process address-space installation
boundary selected by docs/src/project/phase8-process-address-space-contract.md.
This task adds process-owned address-space metadata, model root/table/user-frame
leases, ordered mappings, rollback accounting, and idempotent teardown. It does
not add hardware page tables, TTBR/TCR switching, lower-EL launch, scheduler
handoff, descriptors, shell, filesystem syscalls, Pi 5 hardware evidence, or
boot archive publication.

## Changed Files

- src/main.rs
- src/process_address_space.rs
- src/process_install.rs
- tasks/2026-05-30-phase8-process-address-space-core.md

## Accepted Boundary

- ProcessAddressSpaceId is distinct from TaskId, ProcessOwnerId, PID, and the
  loader fixture identity.
- install_process_address_space() consumes an accepted ProcessImageInstallPlan
  and a caller-provided ProcessAddressSpaceLeaseSource.
- The published ProcessAddressSpace owns one model page-table root token, one
  table-page lease record, one user-frame lease per installed page, ordered
  ProcessUserMapping records, side-effect counters, and teardown state.
- UserText preserves R-X and UserData preserves RW-. The model records EL0
  access intent, W^X preservation, normal-memory intent, and kernel/device deny
  intent without claiming AArch64 descriptor installation.
- Validation rejects malformed plans, missing page slots, null-guard or kernel
  split violations, overlaps, permission widening, bad entry, byte-accounting
  errors, and lease/model exhaustion with deterministic PosixError values.
- Failure before leasing leaves the lease source unchanged. Failure after root,
  table, user-frame, copy/zero, or mapping work rolls back all owned model
  leases and mapping slots. Teardown releases mappings, user-frame leases,
  table-page leases, and root token, then reports already-destroyed on repeat.

src/process_install.rs gained cfg(test)-only unchecked constructors for
malformed install-plan fixtures. These are test support only and do not change
production ProcessImageInstallPlan behavior.

## Unit Coverage

Added focused process_address_space tests:

- installs_process_address_space_with_preserved_permissions_and_leases
- teardown_releases_owned_leases_in_order_and_is_idempotent
- rejects_bad_install_plan_before_leasing
- rejects_null_guard_or_kernel_split_without_leasing
- rejects_overlap_and_permission_widening_without_leasing
- rolls_back_root_table_user_frame_mapping_and_copy_zero_failures

Existing ProcessImageInstallPlan and ProgramImagePlan unit coverage also passed
in the full cargo test gate.

## Validation

- static inspection: git status --short before edits showed an existing
  in-progress src/process_install.rs test-support edit from this active task;
  the worker preserved and completed it.
- fmt/lint/typecheck: cargo fmt --all -- --check passed.
- unit tests: cargo -Zjson-target-spec test passed, 285 tests.
- QEMU/substitute smoke: not run for this core task because the accepted smoke
  plan keeps retained qemu_process_address_space_smoke evidence in the later
  phase8-qemu-process-address-space-smoke-core-20260530 task. This
  implementation did not touch target boot routing or existing boot-scenario
  output owners.
- documentation: mdbook build passed; no docs/src files were touched by this
  task.
- static inspection: git diff --check passed.
- staged whitespace inspection: git diff --cached --check passed before commit.

## Deferred Surfaces

Deferred exactly as required by the accepted contract: AArch64 descriptor
construction, TTBR0/TTBR1 switching, TCR/MAIR/SCTLR policy, ASIDs, TLB and
barriers, lower-EL launch, argv/envp/auxv/TLS, process table/PID/wait/exit,
scheduler handoff, descriptor inheritance, descriptor-backed filesystem
syscalls, writable filesystem, persistent storage, Pi 5 hardware proof, boot
archive publication, networking, SSH, RP1/PCIe, UART interrupt ownership, and
DMA/cache-driver policy.
