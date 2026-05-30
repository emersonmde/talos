# Phase 8 Initial Process Launch Source Inventory

Status: accepted as the documentation-only Milestone 8.3 initial process
launch source inventory. This document follows the accepted
[Phase 8 Process Page-Table Materialization Closeout Checkpoint](phase8-process-page-table-materialization-closeout-checkpoint.md).
It does not add Rust behavior, assembly behavior, QEMU execution, Pi 5
hardware runs, archive publishing, hardware-lock use, TTBR/TCR/MAIR/SCTLR
writes, ASID allocation, live TLB invalidation, lower-EL ERET to /bin/init,
initial user stack implementation, argv/envp/auxv/TLS setup, process table,
PID/wait/exit state, scheduler runnable publication, shell behavior,
descriptor-backed filesystem syscalls, writable filesystems, persistent
storage, networking, SSH, RP1/PCIe, UART interrupt ownership, or
DMA/cache-driver policy.

The accepted Phase 8 frontier can validate immutable /bin/init as a
ProgramImagePlan, derive a ProcessImageInstallPlan, install that plan into a
target-independent ProcessAddressSpace model, and materialize a non-activating
AArch64 descriptor image with user-frame evidence. No accepted source yet
activates those descriptors in TTBR0_EL1, constructs a launch frame, creates
an initial user stack, publishes a runnable process, or ERETs to the loaded
image.

## Accepted Inputs And Source Owners

- src/program_loader.rs owns ProgramImagePlan, PlannedUserSegment,
  UserSegmentKind, LOADER_PAGE_SIZE, MAX_LOAD_SEGMENTS, and
  PHASE8_PROGRAM_LOADER_FIXTURE_IDENTITY. It proves the immutable /bin/init
  ELF64/AArch64 fixture identity, source path, source digest, entry point,
  UserText/UserData segment ranges, file-copy ranges, zero-fill ranges, and
  rounded memory footprint. The entry point is validated as an image-plan
  field only; it is not yet loaded into ELR_EL1 for a process launch.
- src/process_install.rs owns ProcessImageInstallPlan,
  ProcessImagePageInstallRecord, ProcessInstallAction::AllocateCopyZeroMap,
  ProcessInstallSideEffects::NONE, PROCESS_INSTALL_BOUNDARY_IDENTITY, and
  lower_el_launch_blocked=true. It preserves the ProgramImagePlan entry,
  UserText/UserData page records, copy/zero ranges, and deterministic
  POSIX-shaped errors. It deliberately records no lower-EL frame, runnable
  state, scheduler handoff, descriptor inheritance, or process creation.
- src/process_address_space.rs owns ProcessAddressSpace,
  ProcessAddressSpaceId, ProcessAddressSpaceLeaseSource, PageTableRootLease,
  TablePageLease, UserFrameLease, ProcessUserMapping, teardown reports, and
  PROCESS_ADDRESS_SPACE_BOUNDARY_IDENTITY. It models an owner label, root and
  table leases, user-frame leases, ordered mappings, copy/zero accounting,
  rollback, and idempotent teardown. It does not own TTBR activation,
  launch-frame construction, process table state, or runnable scheduler
  publication.
- src/process_page_table_materialization.rs owns
  ProcessPageTableMaterialization,
  ProcessPageTableMaterializationLeaseSource,
  ProcessPageDescriptorRecord, MaterializedRootPageLease,
  MaterializedTablePageLease, MaterializedUserFrameLease,
  PROCESS_PAGE_TABLE_MATERIALIZATION_BOUNDARY_IDENTITY, and
  PROCESS_PAGE_TABLE_KERNEL_MAPPING_POLICY. It creates a descriptor-image
  record with one root page, three table pages, user frames, ordered EL0 leaf
  descriptor records, copied/zeroed byte accounting, rollback/no-leak
  behavior, idempotent teardown, activation_blocked=true, and
  kernel_mapping_policy=activation-blocked-no-kernel-half. It rejects
  ProcessMaterializationRequest::RunnableLowerElState with ENOSYS and performs
  no register writes.
- src/posix.rs owns USER_NULL_GUARD_END, USER_ADDRESS_SPACE_END,
  UserRange, UserMappingPermissions, UserMapping, UserAccessKind,
  validate_user_memory_access(), copy_from_user(), copy_to_user(),
  DescriptorTable, and ProcessDescriptorStore. These define accepted user
  range, permission, copy-boundary, and descriptor vocabulary. They do not
  provide launch authority, argv/envp layout, process file-descriptor
  inheritance, or hardware-backed copy authority for a loaded process.
- src/arch/aarch64/mod.rs owns current_el(), current_vbar(), EL2
  translation/cache helpers, cooperative_context_switch(), and the unsafe
  enter_el1_then_el0() wrapper over talos_aarch64_enter_el1_then_el0. Those
  symbols are source material for launch-frame and register-boundary design,
  but no accepted Phase 8 process-launch contract may call them without first
  fixing the entry, SP, SPSR, address-space activation, rollback, and evidence
  rules.
- src/arch/aarch64/exceptions.rs owns ExceptionVector, ExceptionFrame,
  lower-AArch64 SVC routing, saved register accessors, ESR decoding, and
  unexpected IRQ snapshots. It can describe trap return observations after a
  lower-EL launch attempt, but it is not an initial process-frame constructor.
- src/memory_map/translation.rs and src/memory_map/page_frames.rs own the
  early translation register vocabulary, descriptor bits, bootstrap table
  layout, and early frame ownership policy. They do not yet own per-process
  TTBR0_EL1 roots, TTBR1_EL1 kernel-half sharing, TCR/MAIR compatibility for
  live EL1 process roots, ASID allocation/reuse, or TLB maintenance.
- src/scheduler.rs owns TaskId, ProcessOwnerId, Task, KernelStack,
  ContextFrame, CPU-local runnable queues, shared run-queue transfer, remote
  wake requests, and production dispatch state. A Task can carry an optional
  ProcessOwnerId, but no accepted scheduler record stores a
  ProcessAddressSpace, materialized root, ASID, user entry, user SP,
  lower-EL saved frame, process table pointer, exit status, or wait state.
- src/target/qemu_virt.rs owns the QEMU/substitute evidence producers for the
  accepted loader, process-install, process-address-space, and
  page-table-materialization smokes. The retained materialization evidence is
  tasks/evidence/2026-05-30-qemu-process-page-table-materialization-smoke-core/qemu-process-page-table-materialization-smoke.log.
  Existing QEMU lower-EL and syscall smoke paths are diagnostic source
  material, not a loaded /bin/init launch.
- src/target/rpi5.rs owns serialized Pi 5 proof scenarios with proof-local
  EL0 trap/syscall tables, fixed UserText/UserStack ranges, pre-ERET register
  observations, and hardware classifications. Those scenarios are physical
  evidence for bounded Phase 7 paths, not reusable Phase 8 process-launch
  ownership.

## Accepted Frontier

The current accepted capability is still pre-launch:

- ProgramImagePlan validates immutable /bin/init as static ELF64/AArch64 and
  records the source digest, entry, segment permissions, and footprint.
- ProcessImageInstallPlan derives ordered metadata for allocate/copy/zero/map
  page work, while lower_el_launch_blocked=true and all side effects remain
  false or zero.
- ProcessAddressSpace models process-owned root/table/user-frame leases,
  mappings, copied/zeroed byte counts, publication state, rollback, and
  teardown, but leases remain target-independent model tokens.
- ProcessPageTableMaterialization turns the accepted model into a
  non-activating descriptor image and user-frame evidence record with
  activation_blocked=true and no TTBR/TLB/scheduler/lower-EL/runnable side
  effects.
- Retained QEMU/substitute materialization evidence reports
  qemu-process-page-table-materialization-smoke-complete and
  qemu-process-page-table-materialization-smoke: PASS.

This frontier does not prove that /bin/init can execute, trap, return through
SVC, inherit descriptors, receive argv/envp, exit, be waited on, or run on
Pi 5 hardware.

## Launch Gap Map

| Area | Accepted input | Missing contract before launch |
| --- | --- | --- |
| Entry provenance | ProgramImagePlan.entry() is validated against loaded executable ranges and preserved through install/address-space records. | Which module owns the launch entry, how entry is copied into ELR_EL1, what alignment/canonical-range checks are repeated, and how entry mismatch fails without partial launch state. |
| User SP provenance | Phase 7 proof fixtures have fixed diagnostic UserStack ranges; ProcessAddressSpace has only image mappings. | Initial user stack range, guard mapping, top-of-stack value, ownership source, stack-zeroing policy, and whether stack creation belongs to the launch boundary or a prerequisite task. |
| Launch frame | ExceptionFrame records lower-EL trap state and arch/mod.rs exposes enter_el1_then_el0(). | A process launch frame type with x0..x30/SP/ELR/SPSR ownership, DAIF/SPSR policy, argument register values, error classes, and no-use-before-address-space-activation rules. |
| Address-space activation | ProcessPageTableMaterialization records root/table/user-frame descriptor-image evidence with activation_blocked=true. | TTBR0_EL1 root selection, TTBR1_EL1 or kernel-half policy, TCR_EL1/MAIR_EL1 compatibility, SCTLR_EL1 constraints, ASID allocation, TLB invalidation, DSB/ISB sequencing, and rollback if activation preconditions fail. |
| Kernel reachability | Existing boot maps and Phase 7 proof tables keep exception paths reachable in bounded scenarios. | Whether a launched process uses a split TTBR, replicated kernel mappings, or no activation yet; how VBAR, kernel stack, UART/MMIO diagnostics, and exception routing remain reachable. |
| Scheduler publication | Task can carry ProcessOwnerId and runnable queues can publish kernel tasks. | Process object creation, Task linkage to address space/materialization/ASID/user frame, when a task becomes runnable, no-partial-publication failure behavior, and current-process lookup. |
| Trap and return evidence | Phase 7 QEMU/Pi 5 proofs show fixed lower-EL SVC/syscall trap routes. | Loaded /bin/init launch evidence vocabulary, distinction between launch-preparation and actual ERET, expected PASS/classification lines, and deterministic rejection/no-runnable-publication cases. |
| Descriptor/userland state | Phase 7 descriptor and stdin/stdout syscalls are accepted separately; Phase 8 loader has no process descriptors. | Descriptor inheritance, cwd/root, close-on-exec, argv/envp/auxv/TLS, libc startup, exit/wait, and filesystem syscall behavior. |

## Unaccepted Mutation Surfaces

The next contract must keep these unaccepted until it names exact ownership
and evidence:

- TTBR0_EL1, TTBR1_EL1, TCR_EL1, MAIR_EL1, SCTLR_EL1, ASID, live TLB, DSB,
  and ISB mutation for a process address space;
- lower-EL ERET to /bin/init or any loaded program;
- construction of an initial user stack, guard page, argv/envp, auxv, TLS,
  argc/argv registers, or libc startup frame;
- persistent process table, PID allocation, parent/child relation, exit
  status, wait state, signal, credential, or current-process lookup;
- scheduler runnable publication of a process-backed task;
- descriptor inheritance, close-on-exec, cwd/root, and descriptor-backed
  filesystem syscall behavior for loaded programs;
- Pi 5 hardware proof, boot archive publication, TFTP evidence, power cycle,
  serial observation, or hardwareTestLock acquisition; and
- writable filesystem, persistent storage, shell, networking, SSH, RP1/PCIe,
  UART interrupt ownership, and DMA/cache-driver policy.

## Smallest Objective Contract Boundary

The next bounded documentation task should be
phase8-initial-process-launch-contract-20260530. It is mechanically objective
because the accepted materialization boundary now identifies the next missing
frontier: launch preparation for a materialized /bin/init without yet claiming
argv/envp expansion, process lifecycle, shell behavior, filesystem syscalls,
or Pi 5 hardware.

That contract should select the first launch-preparation boundary and define:

- exact inputs from ProgramImagePlan, ProcessImageInstallPlan,
  ProcessAddressSpace, and ProcessPageTableMaterialization;
- whether the boundary creates only a launch-preparation record or also
  permits a QEMU-only ERET attempt;
- entry PC and user SP provenance, including whether initial user stack
  creation is in scope or explicitly blocked;
- saved lower-EL frame/register ownership, SPSR/DAIF policy, and initial
  argument-register values;
- address-space activation preconditions and explicitly blocked TTBR/TCR/MAIR,
  SCTLR, ASID, TLB, and barrier mutation;
- scheduler publication blockers and no-runnable-publication rollback rules;
- deterministic POSIX-shaped or launch-specific errors; and
- the next QEMU/substitute smoke-plan task and required evidence vocabulary.

Implementation remains blocked until that contract and its QEMU/substitute
smoke plan are accepted. The contract must not accept broad exec/spawn/wait,
argv/envp expansion, shell behavior, descriptor-backed filesystem syscalls,
Pi 5 hardware proof, networking, or SSH.

## Reviewed Materials

- docs/src/project/phase8-process-page-table-materialization-closeout-checkpoint.md
- docs/src/project/phase8-process-page-table-materialization-source-inventory.md
- docs/src/project/phase8-process-page-table-materialization-contract.md
- docs/src/project/phase8-qemu-process-page-table-materialization-smoke-plan.md
- tasks/2026-05-30-phase8-process-page-table-materialization-core.md
- tasks/2026-05-30-phase8-qemu-process-page-table-materialization-smoke-core.md
- tasks/evidence/2026-05-30-qemu-process-page-table-materialization-smoke-core/qemu-process-page-table-materialization-smoke.log
- src/program_loader.rs
- src/process_install.rs
- src/process_address_space.rs
- src/process_page_table_materialization.rs
- src/posix.rs
- src/arch/aarch64/mod.rs
- src/arch/aarch64/exceptions.rs
- src/memory_map/translation.rs
- src/memory_map/page_frames.rs
- src/scheduler.rs
- src/target/qemu_virt.rs
- src/target/rpi5.rs
- docs/src/architecture/lower-el-userspace.md
- docs/src/roadmap.md
- docs/src/decisions/README.md

## Validation

- static inspection: git status --short before edits was clean.
- static documentation/source review: inspected accepted Phase 8 loader,
  process-install, process-address-space, page-table materialization,
  retained QEMU/substitute evidence, lower-EL readiness, AArch64 exception and
  register helpers, scheduler process-owner placeholders, QEMU evidence
  producers, Pi 5 proof-local lower-EL fixtures, roadmap, SUMMARY, and ADR
  index.
- hardware: no Pi 5 archive publication, hardware-lock acquisition, power
  cycle, or serial observation was performed by this documentation-only
  inventory.
