# Phase 8 Process Address-Space Source Inventory

Status: accepted as the documentation-only Milestone 8.3 process address-space
source inventory. This document follows the accepted
[Phase 8 Process Install Closeout Checkpoint](phase8-process-install-closeout-checkpoint.md).
It does not add Rust behavior, assembly behavior, QEMU execution, Pi 5
hardware runs, archive publishing, hardware-lock use, frame allocation,
physical byte copy, page-table mutation, TTBR/TCR switching, lower-EL launch,
argv/envp, exec/spawn/wait, shell behavior, descriptor-backed filesystem
syscalls, writable filesystems, persistent storage, networking, SSH,
RP1/PCIe, UART interrupt ownership, or DMA/cache-driver policy.

The accepted process-install frontier can derive a metadata-only
ProcessImageInstallPlan from a validated ProgramImagePlan and prove that
derivation through QEMU/substitute evidence. This inventory maps the source
owners and gaps for the next frontier: turning those page-install records into
a process-owned address-space installation boundary without assuming the
implementation shape.

## Accepted Inputs And Source Owners

- src/process_install.rs owns ProcessImageInstallPlan,
  ProcessImagePageInstallRecord, ProcessInstallAction,
  ProcessInstallSideEffects::NONE, PROCESS_INSTALL_BOUNDARY_IDENTITY,
  MAX_PROCESS_INSTALL_PAGES, MAX_PROCESS_INSTALL_FOOTPRINT, and
  plan_process_image_install(). It proves ordered page metadata, clipped
  file-copy ranges, explicit zero-fill ranges, UserText/UserData permission
  preservation, deterministic POSIX-shaped errors, zero frame/page-table
  side effects, and lower_el_launch_blocked=true.
- src/program_loader.rs owns ProgramImagePlan, PlannedUserSegment,
  UserSegmentKind, LOADER_PAGE_SIZE, MAX_LOAD_SEGMENTS,
  PHASE8_PROGRAM_LOADER_FIXTURE_IDENTITY, and plan_phase8_init_image(). It
  validates the immutable /bin/init image shape and provides source path,
  digest, entry, segment ranges, file offsets, zero-fill ranges, and rounded
  footprint. It does not allocate process memory or own mappings.
- src/posix.rs owns USER_ADDRESS_SPACE_END, USER_NULL_GUARD_END,
  DEFAULT_USER_COPY_LIMIT, UserRange, UserMappingPermissions, UserMapping,
  UserAccessKind, validate_user_memory_access(), copy_from_user(),
  copy_to_user(), DescriptorTable, and ProcessDescriptorStore. These are
  accepted user-range, permission, copy-boundary, and descriptor vocabularies,
  not process address-space ownership.
- src/memory_map/page_frames.rs owns the early page-frame vocabulary:
  EarlyPageFrameSeed, EarlyBootstrapPageReservation,
  EarlyBootstrapAllocatorPlan, EarlyPageFrameOwnershipContract,
  EarlyHeapExpansionPolicy, and EarlyPageFrameReuseAllocator. The vocabulary
  names bootstrap-reserved frames, translation-table frames, allocator-owned
  frames, deferred frames, and recoverable/fatal OOM policy. It is not yet a
  per-process user-frame lease API.
- src/memory_map/translation.rs and src/arch/aarch64/mod.rs own early
  translation-table layout and EL2 stage-1 register programming helpers for
  kernel/bootstrap mappings. They do not yet own switchable per-process roots,
  user leaf descriptor policy, EL1 TTBR0/TTBR1 install, or teardown.
- src/scheduler.rs owns TaskId, ProcessOwnerId, Task, KernelStack,
  ContextFrame, SchedulerTaskSnapshot, runnable queues, CPU-local schedulers,
  shared metadata, and production dispatch state. ProcessOwnerId is only an
  explicit future extension point; there is no process table, PID allocator,
  process address-space pointer, exit status, or wait relationship.
- src/arch/aarch64/exceptions.rs owns ExceptionFrame and lower-AArch64
  synchronous trap routing for accepted EL0/syscall proof scenarios. It can
  capture trap state, but it is not the initial user-frame constructor for a
  loaded image and does not switch process address spaces.
- src/target/qemu_virt.rs owns QEMU/substitute evidence producers for the
  accepted loader and process-install smokes. The retained process-install
  smoke log is
  tasks/evidence/2026-05-30-qemu-process-install-smoke-core/qemu-process-install-smoke.log.
  It proves metadata-only install-plan derivation and no-partial-install
  rejections, not physical mapping.
- src/target/rpi5.rs owns serialized Pi 5 proof scenarios and scenario-local
  EL0 trap/syscall page-table probes. Those tables are proof fixtures, not a
  reusable process address-space abstraction.

## Proven Metadata-Only Behavior

The accepted Phase 8.3 process-install slice proves only the following
capability:

- immutable /bin/init can be planned as an ELF64 AArch64 ProgramImagePlan;
- that image plan can be converted into a ProcessImageInstallPlan with exact
  fixture identity, source digest, entry, rounded footprint, UserText R-X and
  UserData RW- page records, file-copy ranges, zero-fill ranges, and
  allocate/copy/zero/map action vocabulary;
- malformed plan invariants, overlaps, permission widening, bad entry, budget
  overflow, and invalid source ranges fail before any partial install;
- the accepted implementation records frames_allocated=0,
  mappings_installed=0, process_created=false, descriptors_mutated=false,
  lower_el_frame=false, runnable=false, and lower_el_launch_blocked=true;
- retained QEMU/substitute evidence reports
  qemu-process-install-smoke-complete and qemu-process-install-smoke: PASS.

No accepted source yet turns a ProcessImagePageInstallRecord into a physical
frame, bytes in memory, page-table entry, process object, scheduler task, or
lower-EL return state.

## Unaccepted Mutation Surfaces

The next contract must keep these unaccepted until it names exact ownership
and evidence:

- allocation of UserText, UserData, UserHeap, or UserStack frames;
- physical byte copy from loader bytes into user frames;
- zeroing of BSS, guard pages, stack pages, or padding in physical frames;
- process-owned page-table root allocation and population;
- TTBR0/TTBR1, TCR, MAIR, SCTLR, ASID, TLB, and barrier sequencing;
- rollback after allocation, copy, zero, or map failure;
- process object creation, PID allocation, current-process lookup, scheduler
  handoff, and runnable user task creation;
- initial user frame construction, validated ELR/SP/SPSR/x0..x30 state, and
  ERET to /bin/init;
- argv/envp/auxv/TLS stack construction;
- descriptor table inheritance and close-on-exec policy;
- descriptor-backed VFS/filesystem syscalls, shell, hardware proof,
  networking, SSH, RP1/PCIe, UART interrupt ownership, and DMA/cache-driver
  policy.

## Gap Map

| Area | Accepted input | Missing contract before mutation |
| --- | --- | --- |
| Address-space identity | ProcessImageInstallPlan names source image and ordered page records. | ProcessAddressSpace identity, lifetime, owner id, relationship to ProcessOwnerId and TaskId, and when it becomes visible. |
| Frame source | Early page-frame ownership vocabulary names bootstrap and allocator-owned frame spans. | User-frame lease API, owner tags, zeroing guarantees, release path, OOM mapping, high-memory/deferred-frame policy, and interaction with bootstrap-reserved/table frames. |
| Page-table root | Early translation helpers own bootstrap EL2 maps and proof-local tables. | Process page-table root allocation, level layout, user leaf descriptor attributes, kernel mapping sharing policy, ASID/TTBR/TCR policy, and root teardown. |
| Mapping order | ProcessImagePageInstallRecord names allocate/copy/zero/map action order. | Whether each page is allocated, filled, and mapped one-at-a-time or staged all-at-once; which state is visible after each failure point. |
| Permission preservation | UserText R-X and UserData RW- vocabulary is accepted. | Descriptor-bit mapping for EL0 read/write/execute, W^X enforcement, UXN/PXN policy, device/kernel deny rules, and null-guard split checks in real page tables. |
| Rollback | Process-install errors prove no partial metadata install. | All-or-nothing cleanup of frames, copied bytes, zeroed pages, page-table leaves, table pages, owner records, and scheduler/process metadata. |
| Teardown | No process object or installed address space exists. | Destroy/unmap/release order, idempotence, in-use rejection, and evidence that no frame/table/page lease leaks. |
| Switching | Phase 7 lower-EL contracts name ELR/SP validity and blocked TTBR/TCR work. | When the installed address space can be activated, how kernel mappings remain accessible, which register writes and barriers are required, and whether QEMU evidence is required before hardware. |
| Evidence | QEMU/substitute process-install smoke proves metadata-only behavior. | Success and rejection/no-partial-install observations for real address-space mutation, retained evidence path, classification/PASS vocabulary, and conditional regression gates. |

## Smallest Objective Contract Boundary

The next bounded documentation task should be
phase8-process-address-space-contract-20260530. It is mechanically objective
because the accepted inputs already identify the missing owner boundary:
ProcessImageInstallPlan is complete metadata, while no accepted module owns
process address-space identity, frame leases, page-table roots, mapping order,
rollback, or teardown.

That contract should define the first mutation boundary before implementation:

- the owner module and record name for the first process address-space object
  or the smaller prerequisite if an address-space object remains premature;
- frame source, lease, release, zeroing, and OOM vocabulary;
- page-table root ownership and user leaf permission policy;
- deterministic mapping order and all-or-nothing rollback rules;
- null-guard and user/kernel split checks for actual mappings;
- deterministic POSIX-shaped error mapping for kernel-side malformed input,
  access-denied mapping policy, and no-memory failures;
- evidence level and conditional QEMU/substitute smoke requirements.

Implementation remains blocked until that contract is accepted. The contract
must not accept lower-EL launch, argv/envp, exec/spawn/wait, shell, descriptor
inheritance, filesystem syscalls, hardware proof, networking, or SSH.

## Reviewed Materials

- docs/src/project/phase8-process-install-source-inventory.md
- docs/src/project/phase8-process-install-contract.md
- docs/src/project/phase8-qemu-process-install-smoke-plan.md
- docs/src/project/phase8-process-install-closeout-checkpoint.md
- docs/src/project/phase8-program-loader-source-inventory.md
- docs/src/project/phase8-program-loader-format-contract.md
- docs/src/project/phase7-el0-trap-address-space-contract.md
- docs/src/project/phase7-copyin-copyout-helper-contract.md
- src/process_install.rs
- src/program_loader.rs
- src/posix.rs
- src/scheduler.rs
- src/arch/aarch64/exceptions.rs
- src/arch/aarch64/mod.rs
- src/memory_map/page_frames.rs
- src/memory_map/translation.rs
- src/target/qemu_virt.rs
- src/target/rpi5.rs
- tasks/2026-05-30-phase8-process-install-core.md
- tasks/2026-05-30-phase8-qemu-process-install-smoke-core.md
- tasks/evidence/2026-05-30-qemu-process-install-smoke-core/qemu-process-install-smoke.log

## Validation

- static inspection: git status --short before edits was clean.
- static documentation/source review: inspected accepted process-install docs,
  loader docs, Phase 7 lower-EL and copy contracts, process-install and
  loader source, POSIX user-memory helpers, scheduler placeholders, memory-map
  frame/translation code, architecture translation helpers, QEMU/Pi 5 target
  evidence producers, and retained process-install smoke evidence.
- hardware: no Pi 5 archive publication, hardware-lock acquisition, power
  cycle, or serial observation was performed by this documentation-only
  inventory.
