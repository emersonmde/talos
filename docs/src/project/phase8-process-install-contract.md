# Phase 8 Process Install Contract

Status: accepted documentation-only contract for
phase8-process-install-contract-20260530.

## Scope

This contract follows the accepted
[Phase 8 Process Install Source Inventory](phase8-process-install-source-inventory.md).
It defines the first process-install boundary from a validated
ProgramImagePlan before any address-space mutation, user-frame allocation,
page-table installation, lower-EL launch, argv/envp stack construction,
exec/spawn/wait, shell, descriptor-backed filesystem syscall, writable
filesystem, networking, SSH, RP1/PCIe, UART interrupt ownership, or
DMA/cache-driver policy.

The first accepted implementation boundary is target-independent and
metadata-only: it converts an accepted ProgramImagePlan into an ordered
ProcessImageInstallPlan. That install plan is evidence for the future
process-owned address-space mutator. It does not allocate frames, copy bytes
into physical memory, create page tables, attach a scheduler task, build a
lower-EL frame, or make the loaded image runnable.

## Selected Boundary

The first process-install core should add a small target-independent owner
module for ProcessImageInstallPlan records. The owner may live near the loader
or in a new process-install module, but its contract is independent of target
MMU code.

Inputs:

- a validated ProgramImagePlan from src/program_loader.rs;
- the accepted Phase 7 user-address range, null guard, UserText/UserData
  permission vocabulary, and PosixError names from src/posix.rs;
- an optional future process owner label, such as ProcessOwnerId, used only as
  metadata when the implementation needs one; and
- fixed page-size vocabulary matching the loader's rounded segment ranges.

Output:

- one ProcessImageInstallPlan with fixture identity, source digest, entry,
  total rounded footprint, ordered user-page install records, and blocked
  lower-EL launch metadata;
- one install record per page touched by a planned UserText or UserData
  segment;
- exact file-copy and zero-fill ranges clipped to each page; and
- no scheduler task, PID, descriptor table, frame lease, page-table root, or
  runnable state.

This chooses metadata-only first because the accepted source inventory found no
process address-space object, per-process allocator, page-table root, or TTBR
switching owner. A QEMU/substitute address-space mutator would be useful later,
but accepting it before the metadata contract would hide several kernel
responsibilities behind a test fixture.

## Address-Space Ownership

For this slice, ownership is deliberately split:

| Owner | Accepted responsibility in this slice | Deferred responsibility |
| --- | --- | --- |
| ProgramImagePlan | Provides validated entry, segment ranges, permissions, file ranges, and zero-fill ranges. | Does not own process state or install cleanup. |
| ProcessImageInstallPlan | Owns ordered metadata for pages a later mutator must allocate, fill, map, and unwind. | Does not allocate frames or mutate page tables. |
| Future ProcessAddressSpace | Not implemented in this slice. | Will own page-table root, frame leases, mappings, TTBR/TCR policy, and teardown. |
| Scheduler Task/ProcessOwnerId | May provide an opaque label only if needed by tests. | Does not create a task, PID, runnable user thread, wait state, or exit state. |

The first implementation must not treat ProcessOwnerId as a POSIX PID, current
process handle, process table index, or authority to switch address spaces.

## Page And Permission Rules

The install plan preserves the loader's segment permissions exactly:

- UserText pages are readable and executable, never writable.
- UserData pages preserve the loader's data permissions exactly: readable and
  writable for R+W segments, or read-only for R-only segments; they are never
  executable.
- No page record may merge incompatible segment permissions.
- No rounded page range may cross the null guard, user/kernel split,
  kernel mapping, MMIO, bootstrap table, kernel stack, DTB, or another segment.
- The install plan may reject a ProgramImagePlan as a bad kernel-side input if
  it violates invariants the loader is already supposed to guarantee.

Each page record must include:

- virtual page start and exclusive end;
- segment kind and permissions;
- source segment index or stable segment ordinal;
- file-copy offset and length for bytes that come from immutable fixture data;
- zero-fill offset and length for BSS or rounded page tail bytes; and
- an explicit later-action classification: allocate, copy, zero, then map.

The metadata-only plan must prove that the later mutator can preserve W^X and
never grant broader permissions than the original ProgramImagePlan requested.

## Ordering And Rollback

The accepted future mutation order is:

1. validate the complete ProgramImagePlan and derive every page record;
2. allocate all required user frames under future ProcessAddressSpace
   ownership;
3. copy file bytes into allocated frames;
4. zero BSS and rounded page tail bytes;
5. install page-table leaves with exact user permissions; and
6. publish the installed address-space metadata only after all prior steps
   succeed.

The first metadata-only implementation performs only step 1 and records the
later steps as ordered page actions. It must still model rollback rules so the
future mutator has a stable contract:

- if validation fails, no ProcessImageInstallPlan is returned;
- if a later allocation fails, all previously allocated frames are released;
- if a later copy or zero-fill fails, all allocated frames are scrubbed or
  released before returning;
- if a later mapping fails, inserted mappings are removed in reverse order and
  their frame leases are released; and
- no failure may leave a process object, frame/page lease, mapping, descriptor
  mutation, lower-EL frame, or runnable task beyond the accepted boundary.

## Error Matrix

The first implementation should use deterministic POSIX-shaped errors without
exposing a public exec syscall ABI.

| Failure class | Error |
| --- | --- |
| Null ProgramImagePlan-equivalent request, impossible segment count, missing segment slot, unsorted pages, or other kernel-side malformed input | EINVAL |
| Segment or rounded page range wraps arithmetic | EINVAL |
| Entry is not inside UserText after install-plan derivation | ENOEXEC |
| Requested user range crosses the null guard or canonical user limit | EACCES |
| Requested mapping overlaps another install record | EACCES |
| Segment permissions are incompatible with UserText/UserData policy | EACCES |
| Page count, copied byte count, zero-fill byte count, or memory footprint exceeds the accepted implementation budget | ENOMEM |
| Future frame allocation failure | ENOMEM |
| Future copy/zero-fill fault from kernel-side malformed source or destination | EINVAL |
| Future page-table insertion failure that is not allocation-sized | EIO or EINVAL, chosen by the later mutator contract |

The metadata-only core must test the deterministic errors it can produce
without fabricating target MMU failures.

## Deferred Surfaces

This contract explicitly defers:

- descriptor inheritance, close-on-exec, current working directory, root
  directory, open-file-description lifetime, and descriptor-backed filesystem
  syscalls;
- initial user stack, guard page, argv/envp, auxiliary vectors, TLS, and libc
  startup;
- initial lower-EL frame construction, SPSR/PSTATE choice, x0 through x5
  startup ABI, ERET, trap return, and launch-time fault classification;
- process table, PID allocation, parent/child relation, wait/exit state,
  signal policy, credentials, and current process lookup;
- scheduler handoff, runnable user tasks, address-space switching, TTBR/TCR
  policy, and physical page-table mutation;
- Pi 5 hardware proof, boot archive publication, and hardwareTestLock use; and
- writable filesystem, persistent storage, shell, networking, SSH, RP1/PCIe,
  UART interrupt ownership, and DMA/cache-driver policy.

Descriptor inheritance is out of scope for the first implementation. The
future process object may later attach a ProcessDescriptorStore, but this
contract accepts no descriptor mutation during image installation.

## Evidence And Next Task

The next bounded task should be
phase8-qemu-process-install-smoke-plan-20260530, documentation-only under
Milestone 8.3, if durable queue dependencies remain satisfied.

That smoke plan should define the smallest QEMU/substitute evidence for the
metadata-only ProcessImageInstallPlan boundary:

- exact fixture identity and source digest inherited from the accepted
  ProgramImagePlan;
- success observations for page record order, UserText/UserData permissions,
  file-copy and zero-fill clipping, entry preservation, and total footprint;
- deterministic rejection observations for bad plan invariants, overlapping
  rounded pages, permission widening, and budget overflow;
- no-partial-install observations proving failure returns no plan; and
- exact retained log path, classification line, PASS line, and conditional
  regression gates.

Implementation remains blocked until this contract and the QEMU/substitute
smoke plan are both accepted, unless a later supervisor-owned task explicitly
records a narrower dependency order.

## Reviewed Inputs

- docs/src/project/phase8-process-install-source-inventory.md
- docs/src/project/phase8-program-loader-format-contract.md
- docs/src/project/phase8-program-loader-closeout-checkpoint.md
- docs/src/project/phase7-el0-trap-address-space-contract.md
- docs/src/project/phase7-descriptor-table-contract.md
- src/program_loader.rs
- src/posix.rs
- src/scheduler.rs
- docs/src/roadmap.md
- docs/src/decisions/README.md

## Validation

- static inspection: git status --short before edits was clean.
- static documentation/source review: inspected the accepted process-install
  source inventory, loader format contract, loader closeout, Phase 7
  EL0/address-space and descriptor-table contracts, src/program_loader.rs,
  src/posix.rs, src/scheduler.rs, roadmap, SUMMARY, and ADR index.
- static inspection: git diff --check passed.
- documentation: mdbook build passed.
- staged whitespace inspection: git diff --cached --check passed before
  commit.
- hardware: no Pi 5 archive publication, hardware-lock acquisition, power
  cycle, or serial observation was performed by this contract.
