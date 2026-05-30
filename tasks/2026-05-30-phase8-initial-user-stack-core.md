# Phase 8 Initial User Stack Core Task

Task: phase8-initial-user-stack-core-20260530

Status: accepted

## Scope

Implemented the target-independent Milestone 8.3 InitialUserStackPlan boundary
selected by the accepted initial user stack contract and QEMU/substitute smoke
plan.

Changed files:

- src/initial_user_stack.rs
- src/initial_process_launch.rs
- src/main.rs
- docs/src/roadmap.md
- docs/src/decisions/README.md
- tasks/2026-05-30-phase8-initial-user-stack-core.md

Non-goals honored: no TTBR/TCR/MAIR/SCTLR writes, no ASID allocation, no live
TLB invalidation, no lower-EL ERET, no scheduler runnable publication, no PID
or process lifecycle, no descriptor-backed filesystem syscall behavior, no
broad argv/envp/auxv/TLS ABI, no QEMU smoke execution, no Pi 5 hardware run,
no boot archive publication, no hardwareTestLock acquisition, no networking,
no SSH, no RP1/PCIe, no UART interrupt ownership, and no DMA/cache-driver
policy.

## Outcome

The core adds a model-only InitialUserStackPlan with boundary identity:

    phase8-initial-user-stack-plan-v1

Accepted success behavior:

- copied image/install/address-space/materialization/launch lineage from the
  accepted ProgramImagePlan, ProcessImageInstallPlan, ProcessAddressSpace,
  ProcessPageTableMaterialization, and InitialProcessLaunchPlan records;
- stack top and initial SP 0x0000_8000_0000_0000 with 16-byte alignment;
- usable range [0x0000_7fff_ffff_c000, 0x0000_8000_0000_0000);
- guard range [0x0000_7fff_ffff_b000, 0x0000_7fff_ffff_c000);
- four stack-owned USER_DATA page leases, zeroed_before_copy=true,
  copied_bytes=0, and zeroed_bytes=0x1000 per page;
- aggregate copied_bytes=0 and zeroed_bytes=0x4000;
- one unmapped guard page with no frame lease and no descriptor;
- minimal-empty-argc0 startup metadata with argv/envp NULL and auxv/TLS still
  blocked-pending-startup-abi;
- launch binding to model-only-initial-user-stack-ready with saved-frame
  SP_EL0 intent equal to the stack initial SP; and
- idempotent teardown that releases only stack-owned leases.

Deterministic rejection tests cover unsupported live-launch requests,
already-stack-ready launch input, bad stack range, executable-stack
permissions, image overlap, and stack-frame capacity exhaustion with partial
stack leases released.

## Evidence

- changed source/test paths:
  - src/initial_user_stack.rs
  - src/initial_process_launch.rs
  - src/main.rs
- task record path:
  tasks/2026-05-30-phase8-initial-user-stack-core.md.
- unit tests added:
  - initial_user_stack::tests::builds_initial_stack_plan_with_fixed_layout_and_accounting
  - initial_user_stack::tests::records_empty_startup_payload_and_launch_binding_without_live_side_effects
  - initial_user_stack::tests::teardown_releases_only_stack_owned_leases_and_is_idempotent
  - initial_user_stack::tests::rejects_live_launch_requests_without_partial_launch_or_publication
  - initial_user_stack::tests::rejects_already_stack_ready_launch_input_without_leasing
  - initial_user_stack::tests::rejects_bad_stack_range_and_executable_permissions_without_leasing
  - initial_user_stack::tests::rejects_image_overlap_without_leasing
  - initial_user_stack::tests::capacity_exhaustion_releases_partial_stack_leases

## Validation

- static inspection: git status --short before edits was clean.
- fmt/lint: cargo fmt --all -- --check passed.
- unit tests/QEMU-substitute: cargo -Zjson-target-spec test passed with 306
  no_std tests.
- static inspection: git diff --check passed.
- documentation: mdbook build passed.
- staged whitespace inspection: git diff --cached --check passed before
  commit.
- hardware: no Pi 5 archive publication, hardware-lock acquisition, power
  cycle, or serial observation was performed.

## Residual Blocked Surfaces

QEMU smoke evidence for qemu_initial_user_stack_smoke, live TTBR activation,
ASID/TLB sequencing, lower-EL ERET, scheduler runnable publication, process
lifecycle, exec/spawn/wait, broad argv/envp/auxv/TLS ABI, shell,
descriptor-backed filesystem syscalls, Pi 5 hardware proof, writable
filesystems, persistent storage, networking, SSH, RP1/PCIe, UART interrupt
ownership, and DMA/cache-driver policy remain blocked until later explicit
tasks accept their contracts and evidence gates.

## Commit

Recorded in durable supervisor state after acceptance.
