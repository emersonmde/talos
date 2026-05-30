# Phase 8 Initial Process Launch Contract

Status: accepted documentation-only contract for
phase8-initial-process-launch-contract-20260530.

This contract follows the accepted
[Phase 8 Initial Process Launch Source Inventory](phase8-initial-process-launch-source-inventory.md).
It selects the first launch-preparation boundary after non-activating
page-table materialization. It adds no Rust behavior, assembly behavior, QEMU
execution, Pi 5 hardware run, boot archive publication, hardware-lock
acquisition, TTBR/TCR/MAIR/SCTLR writes, ASID allocation, live TLB
invalidation, lower-EL ERET to /bin/init, initial user stack implementation,
argv/envp/auxv/TLS setup, process table, PID/wait/exit state, scheduler
runnable publication, shell behavior, descriptor-backed filesystem syscalls,
writable filesystems, persistent storage, networking, SSH, RP1/PCIe, UART
interrupt ownership, or DMA/cache-driver policy.

## Selected Boundary

The first implementation boundary should be a target-independent
InitialProcessLaunchPlan record. It consumes the accepted image, install,
address-space, and materialization records and produces a launch-preparation
record that can be inspected and rejected deterministically, but cannot yet be
committed as runnable lower-EL state.

Accepted inputs:

- ProgramImagePlan from src/program_loader.rs, including fixture identity,
  source digest, entry point, segment kind, segment permissions, and rounded
  footprint;
- ProcessImageInstallPlan from src/process_install.rs, including install
  boundary identity, entry, page order, copied/zeroed byte accounting, and
  lower_el_launch_blocked=true;
- ProcessAddressSpace from src/process_address_space.rs, including
  ProcessAddressSpaceId, optional ProcessOwnerId, publication state, mapping
  records, teardown state, and side-effect counters;
- ProcessPageTableMaterialization from
  src/process_page_table_materialization.rs, including boundary identity
  phase8-process-page-table-materialization-v1, descriptor records, user-frame
  records, activation_blocked=true, teardown state, and
  kernel_mapping_policy=activation-blocked-no-kernel-half; and
- POSIX user-range, permission, and error vocabulary from src/posix.rs.

Accepted output:

- one InitialProcessLaunchPlan whose boundary identity should be
  phase8-initial-process-launch-plan-v1;
- copied identities for the image, install plan, address-space record, and
  materialization record;
- an entry_pc copied from ProgramImagePlan.entry() only after it is proven to
  match the install, address-space, and materialization UserText mapping;
- an explicit user_sp_state value of blocked-missing-initial-user-stack;
- an explicit activation_state value of blocked-no-ttbr-activation;
- a saved-frame intent record that names ELR, SP_EL0, SPSR, x0..x5, DAIF, and
  current address-space prerequisites without writing registers; and
- side-effect counters showing no TTBR/TCR/MAIR/SCTLR writes, no ASID
  allocation, no live TLB invalidation, no lower-EL ERET, no scheduler
  publication, no process-table mutation, and no descriptor-table mutation.

The selected boundary is deliberately launch preparation, not process launch.
It makes the entry, stack, activation, frame, and scheduler blockers concrete
before any task can attempt a QEMU-only ERET or physical Pi 5 proof.

## Entry And Stack Ownership

Entry ownership is accepted only as provenance:

- ProgramImagePlan remains the source of entry_pc.
- ProcessImageInstallPlan must preserve the same entry.
- ProcessAddressSpace must contain a UserText mapping covering entry_pc.
- ProcessPageTableMaterialization must contain an EL0 executable descriptor
  record for the same virtual page.
- Any identity, digest, entry, mapping, descriptor, or permission disagreement
  returns EINVAL or ENOEXEC before any launch plan is published.

Initial user stack ownership remains blocked. The first launch-preparation
record must not fabricate a stack address from USER_ADDRESS_SPACE_END, reuse a
Phase 7 proof fixture stack, or map a stack frame implicitly. It records
blocked-missing-initial-user-stack until a later explicit stack contract
defines:

- stack virtual range and guard-page policy;
- frame ownership, zeroing, copy, and teardown behavior;
- initial SP alignment and top-of-stack provenance;
- argc/argv/envp/auxv/TLS layout; and
- failure cleanup and evidence vocabulary.

Because no initial stack is accepted, the plan may describe SP_EL0 only as a
missing prerequisite. Any caller request to commit runnable state or ERET with
this plan must return ENOSYS and no-partial-launch=true.

## Activation Preconditions

The launch-preparation record may inspect the accepted non-activating
descriptor image, but it must not activate it. It records these preconditions
as blocked:

- TTBR0_EL1 root selection for the process materialization;
- TTBR1_EL1 or kernel-half policy;
- TCR_EL1 and MAIR_EL1 compatibility with generated descriptors;
- SCTLR_EL1 constraints before and after enabling translation;
- ASID allocation, reuse, and ownership;
- live TLB invalidation and DSB/ISB sequencing; and
- VBAR_EL1, kernel stack, UART/MMIO diagnostics, exception-vector, and
  scheduler reachability after activation.

The plan must keep activation_state=blocked-no-ttbr-activation and
activation_blocked=true. Requests to write translation registers, invalidate
live TLBs, change SCTLR, or publish an active address space return ENOSYS
without side effects.

## Saved Frame And Register Intent

The first implementation should add only a target-independent saved-frame
intent record. It must not call talos_aarch64_enter_el1_then_el0, write
ELR_EL1, write SP_EL0, write SPSR_EL1, or modify exception-frame state.

Required saved-frame intent fields:

| Field | Required value for this boundary |
| --- | --- |
| ELR | entry_pc from the validated ProgramImagePlan lineage. |
| SP_EL0 | blocked-missing-initial-user-stack. |
| SPSR | blocked-pending-lower-el-pstate-policy. |
| x0..x5 | blocked-pending-startup-ABI. |
| x6..x30 | zero-or-unspecified policy remains blocked until the startup ABI contract. |
| DAIF | blocked-pending-interrupt-mask-policy. |
| Address-space token | ProcessAddressSpaceId plus materialization identity only; not a live TTBR value. |

The record may name these fields so a later QEMU/substitute smoke can prove
that the kernel refuses to launch without stack and activation prerequisites.
It must not imply that the saved frame is usable by the scheduler or exception
return path.

## Scheduler And Publication Blockers

The launch-preparation boundary accepts no process lifecycle. It must not
allocate a PID, create a process table entry, attach descriptor inheritance,
mark a Task runnable, set a current-process pointer, or enqueue scheduler
state.

Required no-partial-publication behavior:

- validation failure returns no InitialProcessLaunchPlan;
- launch-plan construction failure leaves no scheduler, process-table,
  descriptor, address-space, materialization, or frame mutation;
- commit-to-runnable requests return ENOSYS with no-runnable-publication=true;
  and
- teardown of the already accepted ProcessAddressSpace or materialization
  records remains owned by their existing modules.

TaskId, ProcessOwnerId, and ProcessDescriptorStore remain metadata or future
inputs only. They are not PIDs, current-process handles, inherited descriptor
tables, or runnable authority in this boundary.

## Error Matrix

| Condition | Required error |
| --- | --- |
| Missing input, wrong boundary identity, destroyed/unpublished address space, destroyed materialization, or mismatched fixture identity/digest | EINVAL |
| Entry mismatch between image, install, address-space mapping, and descriptor record | ENOEXEC |
| Entry not covered by UserText or descriptor lacks EL0 execute permission | ENOEXEC |
| Entry range wraps, crosses null guard, enters kernel/device space, or violates accepted user range | EACCES |
| Materialization has activation_blocked=false or unsupported kernel mapping policy | EINVAL |
| Initial stack prerequisite is required for commit-to-launch | ENOSYS |
| TTBR/TCR/MAIR/SCTLR, ASID, live TLB, lower-EL ERET, scheduler publication, process-table mutation, descriptor inheritance, argv/envp/auxv/TLS, or runnable-state request | ENOSYS |
| Plan-record capacity or deterministic fixture resource exhaustion | ENOMEM |

Errors must be deterministic and must leave no visible partial launch state.

## Deferred Surfaces

This contract explicitly defers:

- initial user stack allocation, guard-page mapping, argv/envp, auxv, TLS, and
  libc startup;
- TTBR0_EL1/TTBR1_EL1 writes, TCR_EL1/MAIR_EL1/SCTLR_EL1 policy, ASID
  lifecycle, TLB invalidation, and barrier sequencing;
- lower-EL ERET to /bin/init, trap return, syscall routing for loaded
  programs, and launch-time fault classification;
- process table, PID allocation, parent/child relation, wait/exit state,
  signal policy, credentials, and current-process lookup;
- scheduler runnable publication and context-switch integration for
  process-backed tasks;
- descriptor inheritance, close-on-exec, cwd/root, open-file-description
  lifetime, and descriptor-backed filesystem syscalls;
- Pi 5 hardware proof, boot archive publication, and hardwareTestLock use; and
- writable filesystem, persistent storage, shell, networking, SSH, RP1/PCIe,
  UART interrupt ownership, and DMA/cache-driver policy.

## Next Evidence Boundary

The mechanically next documentation-only task should be
phase8-qemu-initial-process-launch-smoke-plan-20260530, if queued
dependencies remain satisfied.

That smoke plan should define QEMU/substitute evidence for this selected
launch-preparation boundary:

- exact scenario or substitute command identity;
- retained evidence path;
- boundary identity phase8-initial-process-launch-plan-v1;
- classification line and PASS line;
- fixture, install, address-space, and materialization identities inherited
  from accepted Phase 8 records;
- success observations for entry provenance, descriptor coverage,
  blocked-missing-initial-user-stack, blocked-no-ttbr-activation,
  saved-frame-intent fields, and zero launch side effects;
- deterministic rejection observations for mismatched identities, bad entry,
  missing UserText descriptor, destroyed inputs, activation requests,
  stack-required launch requests, and scheduler publication requests; and
- conditional regression gates for process-install, process-address-space, and
  process-page-table-materialization smokes if shared owners are touched.

Implementation remains blocked until this contract and the QEMU/substitute
smoke plan are both accepted. Pi 5 hardware proof remains blocked until a
later explicit hardware-proof plan exists.

## Reviewed Inputs

- docs/src/project/phase8-initial-process-launch-source-inventory.md
- docs/src/project/phase8-process-page-table-materialization-contract.md
- docs/src/project/phase8-process-page-table-materialization-closeout-checkpoint.md
- docs/src/project/phase8-process-address-space-contract.md
- docs/src/project/phase8-process-install-contract.md
- src/program_loader.rs
- src/process_install.rs
- src/process_address_space.rs
- src/process_page_table_materialization.rs
- src/posix.rs
- src/arch/aarch64/mod.rs
- src/arch/aarch64/exceptions.rs
- src/memory_map/translation.rs
- src/scheduler.rs
- docs/src/roadmap.md
- docs/src/decisions/README.md

## Validation

- static inspection: git status --short before edits was clean.
- static documentation/source review: inspected the accepted initial process
  launch source inventory, page-table materialization contract and closeout,
  address-space and process-install contracts, loader, install, address-space,
  materialization, POSIX, AArch64 lower-EL and exception helpers, translation
  register vocabulary, scheduler owner placeholders, roadmap, SUMMARY, and
  ADR index.
- static inspection: git diff --check passed.
- documentation: mdbook build passed.
- staged whitespace inspection: git diff --cached --check passed before
  commit.
- hardware: no Pi 5 archive publication, hardware-lock acquisition, power
  cycle, or serial observation was performed by this contract.
