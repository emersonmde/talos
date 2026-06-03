# Phase 8 Process Address-Space Contract

Status: accepted documentation-only contract for
phase8-process-address-space-contract-20260530.

This contract follows the accepted
[Phase 8 Process Address-Space Source Inventory](phase8-process-address-space-source-inventory.md).
It selects the first process-owned address-space installation boundary after
the metadata-only ProcessImageInstallPlan frontier. It adds no Rust behavior,
assembly behavior, QEMU execution, Pi 5 hardware run, boot archive
publication, hardware-lock acquisition, lower-EL launch, argv/envp,
exec/spawn/wait, shell behavior, descriptor-backed filesystem syscalls,
writable filesystems, persistent storage, networking, SSH, RP1/PCIe, UART
interrupt ownership, or DMA/cache-driver policy.

## Selected Boundary

The first implementation boundary should be target-independent process
address-space installation metadata with explicit lease and rollback
accounting. A new narrow owner, expected to live in a process-address-space
module, should consume a validated ProcessImageInstallPlan and produce one
installed ProcessAddressSpace record. The record is process-owned state for a
future lower-EL image, but it is not an active hardware translation context.

Accepted inputs:

- a ProcessImageInstallPlan from src/process_install.rs;
- the Phase 7 user range, null-guard, user/kernel split, permission, and
  PosixError vocabulary from src/posix.rs;
- the early frame ownership vocabulary from src/memory_map/page_frames.rs as
  naming for future frame sources, without reusing bootstrap-reserved or
  translation-table frames directly;
- an opaque ProcessAddressSpaceId and optional ProcessOwnerId label for
  ownership metadata only; and
- a caller-provided target-independent lease source used by tests and
  QEMU/substitute evidence to model user frames and page-table pages.

Accepted output:

- one ProcessAddressSpace record that owns an identity, owner label,
  page-table root token, user-frame lease records, page-table lease records,
  ordered user mappings, and teardown status;
- one user mapping per ProcessImagePageInstallRecord, preserving virtual page
  range, UserText/UserData kind, permissions, file-copy range, zero-fill
  range, and source page ordinal;
- explicit side-effect counters for frames leased, table pages leased,
  mappings installed, copied bytes, zeroed bytes, and rollback releases; and
- no Task, PID, descriptor table mutation, lower-EL frame, TTBR/TCR switch, or
  runnable state.

The selected boundary is deliberately not a hardware page-table installer.
Real AArch64 leaf descriptors, ASIDs, TTBR0_EL1/TTBR1_EL1 writes, TCR/MAIR
policy, TLB invalidation, exception-vector reachability after switching, and
Pi 5 proof remain later contracts. The first boundary exists to make
ownership, lease lifetime, mapping order, and all-or-nothing cleanup testable
before those architecture-specific steps.

## Identity And Lifetime

ProcessAddressSpace identity is distinct from TaskId, ProcessOwnerId, PID, and
the loader fixture identity.

Required lifecycle:

1. allocate a fresh ProcessAddressSpaceId or accept one from a test fixture;
2. create an unpublished in-progress installation record;
3. lease the page-table root token and any table-page metadata required by the
   target-independent mapping model;
4. lease every user frame required by the install plan;
5. copy and zero page payload bytes into the owned install image or model
   buffers;
6. append mapping records with exact user permissions;
7. publish the ProcessAddressSpace record only after every prior step
   succeeds; and
8. make teardown idempotently release all owned leases and reject use after
   destroy.

An in-progress installation must not be visible through scheduler state,
current-process lookup, descriptor tables, lower-EL exception state, or boot
scenario routing. Published means only that the returned ProcessAddressSpace
record is complete and teardown-capable.

## Frame And Table Lease Policy

The first implementation must use a target-independent lease vocabulary rather
than raw hardware allocation:

| Lease | Required contract |
| --- | --- |
| User frame | One lease per installed user page. The lease records owner id, virtual page, page kind, permissions, zeroed-before-copy state, copied byte count, zeroed byte count, and release status. |
| Page-table root | One root token per ProcessAddressSpace. It is a model-owned root, not TTBR-ready physical memory. |
| Page-table pages | Table-page leases are allowed only as model records needed to prove root ownership, mapping capacity, and rollback. They do not encode AArch64 descriptors yet. |

Frame sources must never draw from bootstrap-reserved frames, early
translation-table pages, kernel image pages, kernel stacks, DTB memory, MMIO
ranges, or device memory. If the early frame vocabulary cannot name a safe
future source for a requested lease, the mutator must return ENOMEM or remain
unimplemented under supervisor planning; it must not borrow an existing
bootstrap owner by assumption.

Every leased user frame is zeroed before copying file bytes. Zero-fill ranges
from the ProcessImageInstallPlan are then applied explicitly. Copy and zero
counts are evidence fields, not implicit behavior.

## Mapping And Permission Rules

The installation must preserve the accepted process-install permissions:

- UserText maps readable and executable, never writable.
- UserData maps preserve the process-install permissions exactly: readable and
  writable for R+W pages, or read-only for R-only pages; they are never
  executable.
- No mapping may grant permissions broader than the page install record.
- No mapping may cover the null guard, canonical user limit, kernel space,
  kernel image, kernel stacks, bootstrap page tables, DTB memory, MMIO, or
  device memory.
- No two mappings may overlap.
- Mapping order is deterministic and follows the ordered
  ProcessImageInstallPlan page records.

The target-independent mapping record must name the future architecture policy
it is preserving: EL0 user access, W^X, UXN/PXN intent, normal-memory intent,
and device/kernel deny rules. It must not claim that descriptor bits have been
installed until an architecture-specific page-table contract accepts them.

## Rollback And Teardown

The first implementation must be all-or-nothing:

- validation failure returns no ProcessAddressSpace and performs no leases;
- root/table lease failure releases any prior table leases;
- user-frame lease failure releases all prior table and user leases;
- copy or zero failure scrubs or releases every leased user frame before
  returning;
- mapping failure removes inserted mapping records in reverse order and
  releases all user and table leases; and
- no failure leaves a visible address-space record, scheduler owner,
  descriptor mutation, lower-EL frame, runnable task, or leaked lease.

Teardown must be deterministic and idempotent. Destroying a published
ProcessAddressSpace releases mapping records first, then user-frame leases,
then table-page leases, then the root token. A second teardown call must report
already-destroyed state without double-freeing. In-use rejection may be
modeled, but no scheduler integration is accepted by this boundary.

## Error Matrix

The first implementation should use deterministic POSIX-shaped errors without
exposing exec/spawn/wait syscalls.

| Failure class | Error |
| --- | --- |
| Missing install plan, impossible page count, missing page slot, unordered pages, copied/zeroed byte accounting mismatch, or other kernel-side malformed input | EINVAL |
| Page range or byte range wraps arithmetic | EINVAL |
| Entry is outside installed UserText after address-space validation | ENOEXEC |
| Mapping crosses the null guard, canonical user limit, kernel split, kernel/device memory, or reserved bootstrap/table ranges | EACCES |
| Mapping overlaps another installed page | EACCES |
| Permission widening, W+X, executable data, writable text, or device/kernel access request | EACCES |
| User frame, table-page, root-token, mapping-slot, or model-buffer exhaustion | ENOMEM |
| Copy or zero operation cannot be represented by the model after validation | EINVAL |
| Teardown of a still-borrowed future scheduler binding, if modeled without scheduler integration | EBUSY |

The implementation must not fabricate hardware translation failures. Any
future AArch64 descriptor insertion, TTBR/TCR switch, TLB, or EL0 activation
failure needs a later architecture-specific contract.

## Deferred Surfaces

This contract explicitly defers:

- AArch64 page-table descriptor construction, TTBR0_EL1/TTBR1_EL1 switching,
  TCR/MAIR/SCTLR policy, ASIDs, TLB invalidation, and barrier sequencing;
- initial lower-EL frame construction, ELR/SP/SPSR/x0..x30 setup, ERET, and
  launch-time fault classification;
- initial user stack, guard page, argv/envp, auxv, TLS, and libc startup;
- process table, PID allocation, parent/child relation, wait/exit state,
  current-process lookup, credentials, and signal policy;
- scheduler handoff, runnable user tasks, address-space activation, and
  context-switch integration;
- descriptor inheritance, close-on-exec, current/root directory, open-file
  description lifetime, and descriptor-backed filesystem syscalls;
- Pi 5 hardware proof, boot archive publication, and hardwareTestLock use; and
- writable filesystem, persistent storage, shell, networking, SSH, RP1/PCIe,
  UART interrupt ownership, and DMA/cache-driver policy.

## Evidence And Next Task

The next bounded task should be
phase8-qemu-process-address-space-smoke-plan-20260530, documentation-only
under Milestone 8.3, if durable queue dependencies remain satisfied.

That smoke plan should define QEMU/substitute evidence for this selected
target-independent boundary:

- exact scenario or substitute command name;
- retained evidence path, classification line, and PASS line;
- fixture identity inherited from the accepted ProgramImagePlan and
  ProcessImageInstallPlan;
- success observations for address-space identity, root/table leases,
  user-frame leases, copied/zeroed byte counts, ordered mappings, permission
  preservation, and published status;
- deterministic rejection observations for malformed input, null-guard/user
  split violations, overlap, permission widening, lease exhaustion, copy/zero
  representation failure, and teardown idempotence; and
- no-partial-install observations proving failed installs leave no visible
  address space and no unreleased leases.

Implementation remains blocked until this contract and the QEMU/substitute
smoke plan are both accepted, unless a later supervisor-owned task explicitly
records a narrower dependency order.

## Reviewed Inputs

- docs/src/project/phase8-process-address-space-source-inventory.md
- docs/src/project/phase8-process-install-contract.md
- docs/src/project/phase8-process-install-closeout-checkpoint.md
- docs/src/project/phase8-qemu-process-install-smoke-plan.md
- docs/src/project/phase7-el0-trap-address-space-contract.md
- docs/src/project/phase7-copyin-copyout-helper-contract.md
- src/process_install.rs
- src/program_loader.rs
- src/posix.rs
- src/scheduler.rs
- src/memory_map/page_frames.rs
- src/memory_map/translation.rs
- src/arch/aarch64/mod.rs
- docs/src/roadmap.md
- docs/src/decisions/README.md

## Validation

- static inspection: git status --short before edits was clean.
- static documentation/source review: inspected the accepted process
  address-space source inventory, process-install contract and closeout,
  QEMU/substitute process-install smoke plan, Phase 7 lower-EL and copy
  contracts, process-install and loader sources, POSIX user-memory helpers,
  scheduler process-owner placeholders, frame ownership vocabulary,
  translation helpers, roadmap, SUMMARY, and ADR index.
- static inspection: git diff --check passed.
- documentation: mdbook build passed.
- staged whitespace inspection: git diff --cached --check passed before
  commit.
- hardware: no Pi 5 archive publication, hardware-lock acquisition, power
  cycle, or serial observation was performed by this contract.
