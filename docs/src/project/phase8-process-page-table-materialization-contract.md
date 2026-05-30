# Phase 8 Process Page-Table Materialization Contract

Status: accepted documentation-only contract for
phase8-process-page-table-materialization-contract-20260530.

This contract follows the accepted
[Phase 8 Process Page-Table Materialization Source Inventory](phase8-process-page-table-materialization-source-inventory.md).
It selects the first bounded materialization boundary that turns the accepted
target-independent ProcessAddressSpace model into architecture-shaped owned
resources. It adds no Rust behavior, assembly behavior, QEMU execution, Pi 5
hardware run, boot archive publication, hardware-lock acquisition, TTBR/TCR
switching, ASID lifecycle, TLB invalidation, lower-EL launch, argv/envp,
process lifecycle, shell behavior, descriptor-backed filesystem syscalls,
writable filesystems, persistent storage, networking, SSH, RP1/PCIe, UART
interrupt ownership, or DMA/cache-driver policy.

## Selected Boundary

The first implementation boundary should be a non-activating AArch64 process
page-table materialization record. It consumes the accepted image, install, and
address-space records and produces an owned descriptor image plus owned
user-frame byte images. The result is inspectable and teardown-capable, but it
is not loaded into TTBR0_EL1 or TTBR1_EL1 and cannot become runnable user
state.

Accepted inputs:

- ProgramImagePlan from src/program_loader.rs, including fixture identity,
  source path, source digest, entry point, segment kinds, segment permissions,
  file offsets, file sizes, zero-fill ranges, and rounded footprint;
- ProcessImageInstallPlan and ProcessImagePageInstallRecord from
  src/process_install.rs, including page order, virtual ranges,
  UserText/UserData kind, copy page offset, copy file offset, copy length,
  zero length, source page ordinal, and
  ProcessInstallAction::AllocateCopyZeroMap;
- ProcessAddressSpace from src/process_address_space.rs, including
  ProcessAddressSpaceId, optional ProcessOwnerId, PageTableRootLease,
  TablePageLease, UserFrameLease, ProcessUserMapping, publication state,
  side-effect counters, and teardown behavior; and
- POSIX user-range and permission vocabulary from src/posix.rs, plus the early
  frame ownership vocabulary from src/memory_map/page_frames.rs as the source
  of owner names and forbidden bootstrap/device spans.

Accepted output:

- one ProcessPageTableMaterialization record whose boundary identity should be
  phase8-process-page-table-materialization-v1;
- one owned root page-table page lease and the minimum owned table-page leases
  needed to represent the accepted user mappings as 4 KiB AArch64 stage-1
  user leaf descriptors;
- one owned user-frame lease per accepted UserFrameLease, with zeroed-before
  copy state, copied byte count, zeroed byte count, source page ordinal,
  virtual page, physical frame identity, release state, and scrub requirement;
- ordered descriptor records tying each ProcessUserMapping to a user-frame
  physical address and descriptor value; and
- side-effect counters for root pages leased, table pages leased, user frames
  leased, user frames populated, descriptors installed in the owned image,
  copied bytes, zeroed bytes, rollback releases, teardown releases, and
  activation_blocked=true.

The selected boundary deliberately remains below live address-space
activation. It may build an owned descriptor image and prove descriptor values,
but it must not write CPU translation registers, invalidate live TLB state,
change SCTLR, publish scheduler state, create a lower-EL frame, install a
runnable task, or claim that /bin/init can execute.

## Allocation Ownership

The first implementation must use an explicit materialization lease source
rather than the live early bootstrap allocator. The lease source may be backed
by deterministic test fixtures or a later real allocator, but the accepted
contract is the same:

| Resource | Required contract |
| --- | --- |
| User frame | One owned 4 KiB frame per ProcessUserMapping. The frame is zeroed before copy, receives exactly the page's copy bytes at copy_page_offset, receives explicit zero-fill for zero_len, records owner id and source page ordinal, and is scrubbed or marked scrub-required on release. |
| Root page table | One 4 KiB zeroed root image owned by the materialization record. It is a descriptor image, not a TTBR-ready live root. |
| Table pages | The minimum zeroed 4 KiB table images needed for the accepted virtual ranges. The first implementation may reject unsupported shapes with ENOTSUP, but must not silently widen the mapping or borrow bootstrap translation tables. |

Materialization must never allocate from bootstrap-reserved frames, early
translation-table frames, kernel image memory, kernel stacks, DTB memory, MMIO
ranges, device memory, the null guard, or any range at or above
USER_ADDRESS_SPACE_END. A missing safe resource maps to ENOMEM. An unsupported
page-table topology maps to ENOTSUP. A malformed accepted model maps to EINVAL
or EACCES according to the error matrix below.

## Descriptor Policy

All accepted user mappings are 4 KiB normal-memory EL0 mappings. The
implementation should use descriptor constants adjacent to the existing stage-1
translation helpers, but it must keep the process-materialization descriptor
builder separate from the EL2 bootstrap block-map builder.

Required descriptor properties:

- Valid bit is set and the leaf is a level-3 page descriptor.
- AttrIndx uses the accepted normal-memory attribute index, currently
  EARLY_TRANSLATION_NORMAL_ATTR_INDEX.
- Access flag is set.
- Shareability is inner shareable.
- PXN is set for every user page so privileged execution is denied.
- UserText maps readable and EL0 executable, never writable:
  AP=EL0 read-only, UXN clear, PXN set.
- UserData maps readable and writable, never executable:
  AP=EL0 read-write, UXN set, PXN set.
- W^X is mandatory. Any mapping that requests both write and execute returns
  EACCES.
- Kernel, device, null-guard, unaligned, overflowed, overlapping, or
  non-normal-memory mappings are rejected before any descriptor is committed.

The descriptor image must preserve the accepted ProcessUserMapping order for
evidence, but descriptor writes inside the owned table image may use canonical
virtual-address table order. The implementation must report both the mapping
ordinal and descriptor slot so QEMU/substitute evidence can prove ordering
without assuming a live MMU switch.

## Kernel Mapping And Activation Boundary

The first materialization record is user-root evidence only. It does not accept
one of the future activation policies: replicated kernel half, split
TTBR0_EL1/TTBR1_EL1, shared kernel root, or per-process kernel/user combined
root. The record must include a kernel_mapping_policy field or equivalent
observation with the value activation-blocked-no-kernel-half.

Future activation remains blocked on a separate contract that defines:

- whether kernel mappings live in TTBR1_EL1 or a shared upper half;
- VBAR_EL1, kernel stack, UART/MMIO, exception-vector, and scheduler
  reachability after activation;
- ASID allocation and reuse;
- TTBR0_EL1 and TTBR1_EL1 ownership;
- TCR_EL1 and MAIR_EL1 compatibility with the generated descriptors; and
- DSB, ISB, TLB invalidation, and context-switch sequencing.

Because those surfaces are blocked, the materializer must expose
activation_blocked=true and must reject any caller request to produce runnable
lower-EL state with ENOSYS.

## Rollback And Teardown

Materialization is all-or-nothing. No published record is returned until every
resource has been leased, every user frame has been populated, every descriptor
has been constructed, and every safety check has passed.

On failure, rollback order is:

1. clear any descriptor slots written in the owned table images;
2. release table-page leases in reverse allocation order;
3. scrub or mark scrub-required and release user-frame leases in reverse
   mapping order;
4. clear and release the root page-table lease; and
5. report rollback counters that match every acquired resource.

Teardown for a published record is idempotent. The first teardown clears
descriptor records, releases table pages, scrubs or marks user frames for
scrub, releases the root, sets published=false, and records
already_destroyed=false. Later teardown calls return zero releases and
already_destroyed=true.

## Error Matrix

| Condition | Required error |
| --- | --- |
| ProcessAddressSpace is unpublished, destroyed, has wrong boundary identity, or disagrees with the install plan | EINVAL |
| ProgramImagePlan or ProcessImageInstallPlan fixture identity/digest/page records do not match the accepted address-space mappings | EINVAL |
| Null-guard, kernel-space, device/MMIO, unaligned, overflowed, or overlapping mapping | EACCES |
| Permission widening, writable text, executable data, or write+execute mapping | EACCES |
| Missing root, table-page, user-frame, or descriptor-record capacity | ENOMEM |
| Unsupported table topology below this first contract | ENOTSUP |
| Copy/zero source bounds mismatch or impossible byte population | EINVAL |
| Caller asks for TTBR activation, lower-EL launch, runnable task state, or scheduler publication | ENOSYS |

Errors must be deterministic and must leave no visible partial materialization.

## Next Evidence Boundary

The mechanically next documentation-only task should be
phase8-qemu-process-page-table-materialization-smoke-plan-20260530, if the
queued dependencies remain satisfied.

That smoke plan should define QEMU/substitute evidence for this selected
materialization boundary:

- exact scenario or substitute command identity;
- retained evidence path;
- boundary identity phase8-process-page-table-materialization-v1;
- classification line and PASS line;
- fixture identity inherited from /bin/init, ProcessImageInstallPlan, and
  ProcessAddressSpace;
- success observations for root/table/user-frame ownership, descriptor bit
  policy, permission preservation, copied/zeroed byte accounting,
  activation_blocked=true, rollback, no leaked leases, and idempotent
  teardown;
- deterministic rejection observations for malformed inputs, forbidden ranges,
  permission widening, resource exhaustion, unsupported topology,
  copy/zero mismatch, and activation requests; and
- conditional regression gates for process-install and process-address-space
  smoke evidence if shared owners are touched.

Implementation remains blocked until this contract and the QEMU/substitute
smoke plan are both accepted. Pi 5 hardware proof remains blocked until a
later explicit hardware-proof plan exists.

## Reviewed Inputs

- docs/src/project/phase8-process-page-table-materialization-source-inventory.md
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
- docs/src/roadmap.md
- docs/src/decisions/README.md

## Validation

- static inspection: git status --short before edits was clean.
- static documentation/source review: inspected the accepted page-table
  materialization source inventory, process address-space contract, QEMU
  process address-space smoke plan, process-address-space closeout, loader,
  process-install, ProcessAddressSpace model, POSIX user-range/permission
  vocabulary, early frame ownership vocabulary, translation descriptor
  helpers, architecture register boundaries, scheduler owner placeholders,
  roadmap, SUMMARY, and ADR index.
- static inspection: git diff --check passed.
- documentation: mdbook build passed.
- staged whitespace inspection: git diff --cached --check passed before
  commit.
- hardware: no Pi 5 archive publication, hardware-lock acquisition, power
  cycle, or serial observation was performed by this contract.
