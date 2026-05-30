# Phase 8 Process Page-Table Materialization Source Inventory

Status: accepted as the documentation-only Milestone 8.3 process page-table
materialization source inventory. This document follows the accepted
[Phase 8 Process Address-Space Closeout Checkpoint](phase8-process-address-space-closeout-checkpoint.md).
It does not add Rust behavior, assembly behavior, QEMU execution, Pi 5
hardware runs, archive publishing, hardware-lock use, physical page-table
mutation, TTBR/TCR switching, lower-EL launch, argv/envp, process creation,
exec/spawn/wait, shell behavior, descriptor-backed filesystem syscalls,
writable filesystems, persistent storage, networking, SSH, RP1/PCIe, UART
interrupt ownership, or DMA/cache-driver policy.

The accepted process address-space frontier is still target-independent. A
validated ProgramImagePlan can become a ProcessImageInstallPlan and then a
ProcessAddressSpace model with explicit root/table/user-frame leases, ordered
mapping records, copy/zero accounting, rollback, and teardown. No accepted
source yet materializes those records into real architecture-owned page-table
pages, descriptor entries, frame contents, ASIDs, TTBR registers, or runnable
lower-EL state.

## Accepted Inputs And Source Owners

- src/program_loader.rs owns ProgramImagePlan, PlannedUserSegment,
  UserSegmentKind, LOADER_PAGE_SIZE, MAX_LOAD_SEGMENTS, and
  PHASE8_PROGRAM_LOADER_FIXTURE_IDENTITY. It validates the immutable /bin/init
  ELF64/AArch64 image and exposes source path, digest, entry point, load
  segment ranges, file offsets, file sizes, zero-fill ranges, and rounded
  memory footprint. It does not allocate frames, own page tables, or provide
  descriptor attributes.
- src/process_install.rs owns ProcessImageInstallPlan,
  ProcessImagePageInstallRecord, ProcessInstallAction::AllocateCopyZeroMap,
  ProcessInstallSideEffects::NONE, PROCESS_INSTALL_BOUNDARY_IDENTITY, and
  MAX_PROCESS_INSTALL_PAGES. It derives page-sized UserText/UserData install
  records, file-copy ranges, zero-fill ranges, deterministic POSIX-shaped
  errors, and lower_el_launch_blocked=true. Its accepted side effects remain
  frames_allocated=0, mappings_installed=0, process_created=false,
  descriptors_mutated=false, lower_el_frame=false, and runnable=false.
- src/process_address_space.rs owns the accepted ProcessAddressSpace model:
  ProcessAddressSpaceId, ProcessAddressSpaceLeaseSource,
  PageTableRootLease, TablePageLease, UserFrameLease, ProcessUserMapping,
  ProcessAddressSpaceSideEffects, ProcessAddressSpaceTeardownReport, and
  install_process_address_space(). It proves model root/table/user-frame
  leases, ordered mappings, permission preservation, all-or-nothing rollback,
  and idempotent teardown, but every lease token is model-only and no
  descriptor entry or physical byte is installed.
- src/posix.rs owns USER_NULL_GUARD_END, USER_ADDRESS_SPACE_END,
  UserMappingPermissions, UserRange, UserMapping, UserAccessKind,
  validate_user_memory_access(), copy_from_user(), copy_to_user(),
  DescriptorTable, and ProcessDescriptorStore. These provide accepted
  user-range, permission, copy-boundary, and descriptor vocabulary. They do
  not yet authorize live page-table lookup, descriptor-backed filesystem
  syscalls for a loaded program, or copy authority from hardware mappings.
- src/memory_map/page_frames.rs owns early page-frame vocabulary:
  EarlyPageFrameSeed, EarlyBootstrapPageReservation,
  EarlyBootstrapAllocatorPlan, EarlyPageFrameOwnershipContract,
  EarlyHeapExpansionPolicy, and EarlyPageFrameReuseAllocator. It names
  bootstrap-reserved frames, translation-table frames, allocator-owned spans,
  deferred frames, metadata bounds, free-frame errors, and recoverable/fatal
  OOM policy. It is not yet a process user-frame allocator or process
  page-table-page allocator.
- src/memory_map/translation.rs owns EL2 bootstrap translation table layout,
  population, table descriptors, 2 MiB block descriptors, MAIR/TCR register
  plans, and low/MMIO bootstrap mappings. It does not own EL1 process roots,
  4 KiB user leaf descriptors, per-process kernel-map sharing, ASID tagging,
  TTBR0_EL1/TTBR1_EL1 activation, or TLB maintenance for address-space
  switches.
- src/arch/aarch64/mod.rs owns architecture-level EL2 translation helpers and
  AArch64 context-switch entry points for the accepted kernel runtime. It does
  not yet provide a process page-table materializer, descriptor flush API,
  EL1 user-address-space activation API, or process ASID lifecycle.
- src/scheduler.rs owns TaskId, ProcessOwnerId, Task, KernelStack,
  ContextFrame, runnable queues, CPU-local schedulers, shared metadata, and
  production dispatch state. ProcessOwnerId is an owner label accepted by the
  ProcessAddressSpace model, but no Task stores a process address-space root,
  ASID, userspace stack, executable entry, exit state, wait state, or current
  process table pointer.
- src/target/qemu_virt.rs owns retained QEMU/substitute evidence producers for
  the accepted program-loader, process-install, and process-address-space
  smokes. The retained process-address-space evidence path is
  tasks/evidence/2026-05-30-qemu-process-address-space-smoke-core/qemu-process-address-space-smoke.log.
  It proves target-independent model behavior, not real descriptor
  materialization.
- src/target/rpi5.rs owns serialized Pi 5 proof scenarios and scenario-local
  EL0 trap/syscall tables. Those tables demonstrate proof-specific lower-EL
  mappings and register observations, but they are fixed diagnostic fixtures,
  not reusable process page-table materialization.

## Proven Target-Independent Behavior

The accepted Phase 8.3 address-space slice proves only this capability:

- immutable /bin/init bytes can be validated as a static ELF64/AArch64
  ProgramImagePlan with source digest 0x3892eed223900c65 and fixture identity
  phase8-program-loader-elf64-aarch64-v1;
- the ProgramImagePlan can be converted into a ProcessImageInstallPlan with
  process-install boundary phase8-process-install-plan-v1, ordered page
  records, UserText R-X and UserData RW- permissions, file-copy ranges,
  zero-fill ranges, and lower_el_launch_blocked=true;
- the ProcessImageInstallPlan can be installed into a ProcessAddressSpace
  model with address-space boundary phase8-process-address-space-model-v1,
  one model root lease, one model table-page lease, one user-frame lease per
  installed page, ordered ProcessUserMapping records, copy/zero accounting,
  publication state, deterministic rollback, and idempotent teardown;
- retained QEMU/substitute evidence reports
  qemu-process-address-space-smoke-complete and
  qemu-process-address-space-smoke: PASS; and
- deterministic rejection evidence covers malformed install plans,
  null-guard/user-kernel split violations, overlap, permission widening,
  lease exhaustion, copy/zero model failure, no partial install, no leaked
  leases, and teardown idempotence.

No accepted source yet turns a PageTableRootLease into physical root-table
memory, a TablePageLease into zeroed table pages, a UserFrameLease into
physical user memory with copied bytes, or a ProcessUserMapping into AArch64
leaf descriptors.

## Materialization Gap Map

| Area | Accepted input | Missing contract before materialization |
| --- | --- | --- |
| Physical user frames | UserFrameLease records virtual page, kind, permissions, copy/zero counts, and release status. | Real frame source, owner tag, zero-before-copy implementation, source-byte copy path, BSS zero path, frame cacheability expectations, OOM mapping, and release/scrub policy. |
| Page-table root | PageTableRootLease records one model root token. | Physical root-table allocation, alignment, zeroing, table-level shape, kernel/user split, root lifetime, root release, and whether the root can ever be loaded into TTBR0_EL1. |
| Table pages | TablePageLease proves model table-page capacity and rollback. | Number and levels of real table pages for current user VA ranges, zeroing, descriptor ownership, intermediate descriptor attributes, and release order. |
| User leaf descriptors | ProcessUserMapping preserves kind, permissions, EL0 user access, W^X, normal-memory intent, and kernel/device deny intent. | Exact AP/PXN/UXN/AF/shareability/attribute-index bits for UserText and UserData, physical address masks, normal-memory attributes, descriptor validity, and null/kernel/device denial checks before write. |
| Kernel mapping sharing | Bootstrap translation helpers own EL2 low/MMIO mappings and proof scenarios own fixed EL1 tables. | Whether per-process EL1 roots replicate kernel mappings, split TTBR0/TTBR1, or preserve an always-accessible kernel half; how VBAR, stack, UART/MMIO, and exception paths remain reachable during future activation. |
| ASID and TTBR policy | No accepted ASID or process TTBR lifecycle exists. | ASID allocation/reuse, TTBR0_EL1/TTBR1_EL1 ownership, TCR/MAIR compatibility, TLB invalidation scope, barrier sequencing, and context-switch integration. |
| Rollback | ProcessAddressSpace rollback releases model mappings, frames, table pages, and root tokens. | Reverse-order removal of descriptors, table pages, copied physical frames, owner records, cache/TLB cleanup, and proof that failed materialization leaves no reachable partial mapping. |
| Teardown | ProcessAddressSpace::destroy() is idempotent in the model. | Live unmap/destroy ordering, in-use rejection, ASID invalidation, table/frame release, zero/scrub requirements, descriptor poisoning, and double-destroy behavior for real resources. |
| Launch blockers | ProcessAddressSpace stores entry-adjacent mapping metadata and owner label. | Lower-EL frame construction, user stack, argv/envp/auxv/TLS, scheduler runnable task state, process table, PID, wait/exit, descriptor inheritance, and ERET. |
| Evidence | QEMU/substitute address-space smoke proves model behavior. | Materialization success/rejection vocabulary, retained evidence path, descriptor/page ownership observations, rollback/no-leak observations, teardown observations, and conditional regression gates. |

## Unaccepted Mutation Surfaces

The next contract must keep these surfaces unaccepted until it names exact
ownership and evidence:

- real physical user-frame allocation, zeroing, byte copy, release, and scrub;
- physical page-table root and table-page allocation, zeroing, population, and
  release;
- AArch64 table, block, or page descriptor installation for user mappings;
- kernel mapping sharing or split TTBR policy for per-process roots;
- MAIR, TCR, SCTLR, TTBR0_EL1, TTBR1_EL1, ASID, TLB, DSB, and ISB sequencing;
- process address-space activation on a CPU or during scheduler context
  switch;
- lower-EL exception-frame construction, ELR/SP/SPSR/x0..x30 state, ERET, and
  launch-time trap classification;
- user stack, guard page, argv/envp, auxv, TLS, libc startup, exec/spawn/wait,
  process table, PID allocation, parent/child relation, wait/exit state,
  credentials, and signals;
- descriptor inheritance, close-on-exec, current/root directory,
  open-file-description lifetime, and descriptor-backed filesystem syscalls;
- Pi 5 hardware proof, boot archive publication, TFTP evidence, power-cycle,
  serial observation, or hardwareTestLock acquisition; and
- writable filesystem, persistent storage, shell, networking, SSH, RP1/PCIe,
  UART interrupt ownership, and DMA/cache-driver policy.

## Smallest Objective Contract Boundary

The next bounded documentation task should be
phase8-process-page-table-materialization-contract-20260530. It is
mechanically objective because the accepted ProcessAddressSpace model already
identifies the next missing boundary: materializing model leases and mappings
into architecture-owned frame/table resources while still staying below
TTBR activation and lower-EL launch.

That contract should define:

- exact inputs from ProgramImagePlan, ProcessImageInstallPlan, and
  ProcessAddressSpace records;
- whether the first implementation uses real early-frame reuse, a modeled
  architecture materializer, or another explicitly named frame/table source;
- ownership and release policy for user frames, root tables, and table pages;
- UserText/UserData descriptor bit policy, normal-memory attributes, EL0
  access rules, W^X, PXN/UXN, null-guard/user-kernel split checks, and
  kernel/device deny checks;
- kernel mapping sharing, ASID, TTBR, TCR, MAIR, TLB, and barrier boundaries,
  including what remains explicitly blocked;
- all-or-nothing rollback, teardown, and deterministic POSIX-shaped error
  mapping; and
- the next QEMU/substitute smoke-plan task and required evidence vocabulary.

Implementation remains blocked until that contract and its QEMU/substitute
smoke plan are accepted. The contract must not accept lower-EL launch,
argv/envp, process lifecycle, exec/spawn/wait, shell, descriptor-backed
filesystem syscalls, hardware proof, networking, or SSH.

## Reviewed Materials

- docs/src/project/phase8-process-address-space-source-inventory.md
- docs/src/project/phase8-process-address-space-contract.md
- docs/src/project/phase8-qemu-process-address-space-smoke-plan.md
- docs/src/project/phase8-process-address-space-closeout-checkpoint.md
- src/program_loader.rs
- src/process_install.rs
- src/process_address_space.rs
- src/posix.rs
- src/memory_map/page_frames.rs
- src/memory_map/translation.rs
- src/arch/aarch64/mod.rs
- src/scheduler.rs
- src/target/qemu_virt.rs
- src/target/rpi5.rs
- tasks/2026-05-30-phase8-process-address-space-core.md
- tasks/2026-05-30-phase8-qemu-process-address-space-smoke-core.md
- tasks/evidence/2026-05-30-qemu-process-address-space-smoke-core/qemu-process-address-space-smoke.log
- docs/src/roadmap.md
- docs/src/decisions/README.md

## Validation

- static inspection: git status --short before edits was clean.
- static documentation/source review: inspected accepted process
  address-space docs, retained QEMU/substitute evidence, process-loader,
  process-install, ProcessAddressSpace model, POSIX user-memory helpers,
  frame ownership vocabulary, translation descriptors/register helpers,
  scheduler process-owner placeholders, QEMU evidence producer, Pi 5
  proof-local lower-EL translation fixtures, roadmap, SUMMARY, and ADR index.
- hardware: no Pi 5 archive publication, hardware-lock acquisition, power
  cycle, or serial observation was performed by this documentation-only
  inventory.
