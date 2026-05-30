# Phase 8 Initial User Stack Source Inventory

Status: accepted as the documentation-only Milestone 8.3 initial user stack
source inventory. This document follows the accepted
[Phase 8 Initial Process Launch Closeout Checkpoint](phase8-initial-process-launch-closeout-checkpoint.md).
It does not add Rust behavior, assembly behavior, QEMU execution, Pi 5
hardware runs, archive publishing, hardware-lock use, TTBR/TCR/MAIR/SCTLR
writes, ASID allocation, live TLB invalidation, lower-EL ERET to /bin/init,
scheduler runnable publication, process lifecycle, argv/envp/auxv/TLS layout,
descriptor-backed filesystem syscalls, shell behavior, writable filesystems,
networking, SSH, RP1/PCIe, UART interrupt ownership, or DMA/cache-driver
policy.

The accepted Phase 8 frontier can validate immutable /bin/init as a
ProgramImagePlan, derive a ProcessImageInstallPlan, model a
ProcessAddressSpace, materialize a non-activating AArch64 descriptor image,
and produce an InitialProcessLaunchPlan with entry provenance and saved-frame
intent. That launch plan deliberately records
user_sp_state=blocked-missing-initial-user-stack and
activation_state=blocked-no-ttbr-activation. No accepted source yet constructs
a process-owned user stack record, allocates or materializes a stack frame,
sets SP_EL0 to a usable address, publishes runnable state, or ERETs to the
loaded image.

## Current Owners

- src/posix.rs owns the accepted user-address vocabulary:
  USER_NULL_GUARD_END=0x0000_0000_0001_0000,
  USER_ADDRESS_SPACE_END=0x0000_8000_0000_0000, UserRange,
  UserMappingPermissions, UserMapping, validate_user_memory_access(),
  copy_from_user(), copy_to_user(), DescriptorTable, and
  ProcessDescriptorStore. It can validate a non-guard user range and data
  permissions, but it does not select an initial stack top, reserve a guard
  page, own frame leases, or define argv/envp/auxv/TLS layout.
- src/program_loader.rs owns ProgramImagePlan, PlannedUserSegment,
  UserSegmentKind, LOADER_PAGE_SIZE, MAX_LOAD_SEGMENTS, and
  PHASE8_PROGRAM_LOADER_FIXTURE_IDENTITY. It validates immutable /bin/init
  executable text/data ranges and entry, but explicitly does not allocate user
  frames, build user stacks, or launch lower-EL code.
- src/process_install.rs owns ProcessImageInstallPlan and
  ProcessImagePageInstallRecord. It preserves page-copy and zero-fill metadata
  for image segments while lower_el_launch_blocked=true and no
  lower-EL-frame, descriptor, process, or runnable side effect is accepted.
  It has no stack page, stack guard, SP, or startup payload record.
- src/process_address_space.rs owns ProcessAddressSpace,
  ProcessAddressSpaceId, ProcessAddressSpaceLeaseSource,
  PageTableRootLease, TablePageLease, UserFrameLease, ProcessUserMapping,
  publication state, rollback, and idempotent teardown for image mappings. It
  provides the nearest target-independent lease/release pattern for stack
  ownership, but the accepted implementation sizes its model from the
  ProcessImageInstallPlan page count and contains no stack-specific mapping or
  guard record.
- src/process_page_table_materialization.rs owns
  ProcessPageTableMaterialization,
  ProcessPageTableMaterializationLeaseSource, MaterializedUserFrameLease,
  ProcessPageDescriptorRecord, activation_blocked=true, rollback/no-leak
  behavior, and idempotent teardown for the image-derived mappings. It
  provides the descriptor and materialized-frame vocabulary a stack contract
  can reuse, but it currently materializes only the accepted address-space
  mappings and does not install a stack descriptor.
- src/initial_process_launch.rs owns InitialProcessLaunchPlan,
  INITIAL_PROCESS_LAUNCH_BOUNDARY_IDENTITY,
  INITIAL_USER_SP_BLOCKED=blocked-missing-initial-user-stack,
  INITIAL_ACTIVATION_BLOCKED=blocked-no-ttbr-activation, saved-frame intent,
  zero launch side-effect counters, and ENOSYS no-partial-launch rejection for
  activation, stack-required launch, and scheduler publication requests. It is
  the direct consumer that a future stack record must unblock, but it must not
  fabricate a stack address.
- src/arch/aarch64/mod.rs and src/arch/aarch64/exceptions.rs own lower-EL
  entry helpers, saved exception-frame vocabulary, ELR/SP/SPSR register
  context, and trap observations. They remain source material only; this
  inventory does not authorize register writes or lower-EL ERET.
- src/scheduler.rs owns TaskId, ProcessOwnerId, Task, KernelStack,
  ContextFrame, runnable queues, and production dispatch state. A Task can
  carry an optional ProcessOwnerId, but no scheduler record owns a user stack,
  ProcessAddressSpace, materialized root, ASID, user SP, process table entry,
  exit status, or wait state.
- src/target/qemu_virt.rs owns QEMU/substitute evidence producers and the
  earlier Phase 7 fixed diagnostic UserStack range. Those diagnostic lower-EL
  stacks prove bounded trap/syscall paths, not reusable Phase 8 process-stack
  ownership for loaded /bin/init.
- src/target/rpi5.rs owns serialized Pi 5 proof scenarios with proof-local
  fixed UserStack ranges and pre-ERET observations. Those hardware scenarios
  are evidence for accepted Phase 7 proof paths and must not be promoted into
  the Phase 8 launch contract without a new explicit hardware-proof plan.
- docs/src/architecture/lower-el-userspace.md owns the accepted lower-EL
  vocabulary for user text/data/heap/stack/guard mappings, validated user
  ELR/SP/SPSR state, and stack-fault classes. It keeps EL0 entry, process
  isolation, production fault recovery, and user stack mapping creation behind
  explicit later tasks.

## Stack Gap Map

| Area | Accepted input | Missing stack contract |
| --- | --- | --- |
| User range policy | POSIX accepts a non-guard user range below 0x0000_8000_0000_0000 and a null guard below 0x0000_0000_0001_0000. | Exact initial stack virtual range, top address, downward-growth convention, and whether the top is derived from USER_ADDRESS_SPACE_END or a narrower process-layout constant. |
| Guard policy | Lower-EL readiness names user stack/guard vocabulary; UserRange rejects the null guard. | Stack-adjacent guard-page size, whether the guard is unmapped or explicitly denied, and how guard overlap with image mappings fails. |
| Alignment and SP provenance | InitialProcessLaunchPlan records SP_EL0 as blocked-missing-initial-user-stack. | Required stack-top alignment, initial SP value, and how the stack record updates launch saved-frame intent without writing SP_EL0. |
| Frame and page ownership | ProcessAddressSpace and materialization modules own target-independent leases, user-frame evidence, rollback, and teardown for image mappings. | Whether stack construction extends ProcessAddressSpace before materialization, appends a separate stack record beside it, or creates a planned stack lease to be materialized later. |
| Zero/copy accounting | ProcessImageInstallPlan and ProcessAddressSpace track copied/zeroed bytes for image pages. | Stack pages should start zeroed with no file-copy bytes, with deterministic accounting and no argv/envp payload copy in the first slice unless explicitly contracted. |
| Teardown | ProcessAddressSpace and ProcessPageTableMaterialization teardown are idempotent and release accepted image leases/descriptors. | Stack teardown ownership, idempotence, and rollback ordering relative to address-space and materialization teardown. |
| Startup payload | InitialProcessLaunchPlan blocks x0..x5 pending startup ABI; POSIX copy helpers can validate user memory. | Whether this slice permits only argc=0 and empty argv/envp placeholders, or keeps all argv/envp/auxv/TLS bytes blocked for a later ABI task. |
| Launch integration | InitialProcessLaunchPlan has blocked stack and activation states with no side effects. | How a stack record changes user_sp_state from blocked to model-only-ready while still keeping TTBR activation, ERET, runnable publication, and process table mutation blocked. |
| Evidence | Existing QEMU/substitute launch evidence proves blocked stack state and no-partial-launch rejection. | PASS/classification vocabulary for constructing a stack record, rejecting overlap/resource errors, preserving zero live-launch side effects, and proving launch-plan stack-state integration. |

## Smallest Target-Independent Boundary

The smallest objective follow-up is a documentation-only contract for
phase8-initial-user-stack-contract-20260530. That contract should select a
target-independent initial user stack record below live launch. The record
should be inspectable and deterministic, not a runnable process or a hardware
claim.

The next contract should decide:

- exact stack virtual range, top-of-stack, page size, and alignment;
- guard-page size and rejection rules for overlap with existing image
  mappings, null guard, kernel/device space, or wraparound;
- whether stack frame ownership is represented as an extension to
  ProcessAddressSpace, a companion stack plan, or a materialization-pending
  stack record;
- zero/copy accounting, including zeroed stack pages and no file-copy bytes;
- teardown and rollback ordering, including no leaks and idempotent release;
- launch-plan integration, specifically how the stack record changes the
  InitialProcessLaunchPlan stack state without allowing TTBR activation,
  ERET, scheduler publication, process-table mutation, or descriptor mutation;
- deterministic errors, likely EINVAL for bad identities/state, EFAULT or
  EACCES for forbidden user ranges/permissions, ENOMEM for lease or capacity
  exhaustion, and ENOSYS for live activation/runnable/process-lifecycle
  requests; and
- evidence vocabulary for the later QEMU/substitute smoke plan.

Implementation should remain blocked until that contract and its
QEMU/substitute smoke plan are accepted.

## Deferred Surfaces

The initial user stack inventory keeps these surfaces blocked:

- TTBR0_EL1, TTBR1_EL1, TCR_EL1, MAIR_EL1, SCTLR_EL1, ASID, live TLB, DSB,
  and ISB mutation;
- lower-EL ERET, architectural SP_EL0/ELR_EL1/SPSR_EL1 writes, and live
  trap-return behavior;
- scheduler runnable publication, process table/PID/current-process state,
  parent/child lifecycle, exit status, wait state, signals, and credentials;
- broad argv/envp/auxv/TLS ABI, libc startup, dynamic stack growth, signal
  stack, guard-fault handling, copy-on-write, and demand paging;
- descriptor inheritance, close-on-exec, cwd/root, filesystem syscalls, and
  shell behavior;
- QEMU execution for this slice, Pi 5 hardware proof, boot archive
  publication, TFTP/serial evidence, and hardwareTestLock acquisition; and
- writable filesystem, persistent storage, networking, SSH, RP1/PCIe, UART
  interrupt ownership, and DMA/cache-driver policy.

## Recommendation

The mechanically next task should be
phase8-initial-user-stack-contract-20260530, if queued dependencies remain
satisfied. The accepted inventory identifies the first missing launch
prerequisite as a target-independent stack record that can update the
InitialProcessLaunchPlan from blocked-missing-initial-user-stack to an
inspectable stack-ready state, while keeping activation, ERET, scheduler
publication, process lifecycle, filesystem syscalls, Pi 5 proof, networking,
and SSH blocked.

## Reviewed Materials

- docs/src/project/phase8-initial-process-launch-closeout-checkpoint.md
- docs/src/project/phase8-initial-process-launch-source-inventory.md
- docs/src/project/phase8-initial-process-launch-contract.md
- tasks/2026-05-30-phase8-initial-process-launch-core.md
- tasks/2026-05-30-phase8-qemu-initial-process-launch-smoke-core.md
- tasks/evidence/2026-05-30-qemu-initial-process-launch-smoke-core/qemu-initial-process-launch-smoke.log
- src/posix.rs
- src/program_loader.rs
- src/process_install.rs
- src/process_address_space.rs
- src/process_page_table_materialization.rs
- src/initial_process_launch.rs
- src/arch/aarch64/mod.rs
- src/arch/aarch64/exceptions.rs
- src/scheduler.rs
- src/target/qemu_virt.rs
- src/target/rpi5.rs
- docs/src/architecture/lower-el-userspace.md
- docs/src/roadmap.md
- docs/src/decisions/README.md

## Validation

- static inspection: git status --short before edits was clean.
- static documentation/source review: inspected accepted initial-process
  launch docs and retained QEMU/substitute evidence; POSIX user range, copy,
  and descriptor owners; loader, process-install, ProcessAddressSpace,
  process page-table materialization, and InitialProcessLaunchPlan source
  owners; AArch64 lower-EL helpers; scheduler placeholders; QEMU/Pi 5
  proof-local stack fixtures; lower-EL architecture notes; roadmap; SUMMARY;
  and ADR index.
- static inspection: git diff --check passed.
- documentation: mdbook build passed.
- staged whitespace inspection: git diff --cached --check passed before
  commit.
