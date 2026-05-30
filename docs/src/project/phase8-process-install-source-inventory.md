# Phase 8 Process Install Source Inventory

Status: accepted as the documentation-only Milestone 8.3 process-install
source inventory. This follows the accepted
[Phase 8 Program Loader Closeout Checkpoint](phase8-program-loader-closeout-checkpoint.md).
It adds no Rust behavior, assembly behavior, QEMU execution, Pi 5 hardware
run, boot archive publication, hardware-lock use, process creation,
lower-EL launch, argv/envp implementation, exec/spawn/wait, shell, writable
filesystem, networking, SSH, RP1/PCIe, UART interrupt ownership, or
DMA/cache-driver policy.

The accepted loader frontier now validates immutable /bin/init bytes and
produces a ProgramImagePlan. This inventory maps the source owners and gaps
between that image-plan-only output and a future process-owned address-space
installation. It deliberately keeps image validation separate from allocating
frames, building page tables, installing process metadata, constructing the
initial user frame and stack, inheriting descriptors, or handing a runnable
task to the scheduler.

## Accepted Inputs

- src/program_loader.rs owns the image-plan input: source path, fixture
  identity, digest, entry point, ordered UserText/UserData segments,
  file-copy ranges, zero-fill ranges, rounded memory footprint, and
  deterministic loader errors.
- src/initramfs.rs owns the immutable read-only initramfs/VFS fixture and
  ReadOnlyInitramfs::regular_file_bytes() source boundary for /bin/init.
- src/posix.rs owns the current user-address range, null-guard,
  UserMappingPermissions, UserMapping, validate_user_memory_access(),
  copy_from_user(), copy_to_user(), DescriptorTable, and
  ProcessDescriptorStore vocabulary.
- src/scheduler.rs owns TaskId, Task, TaskState, KernelStack, ContextFrame,
  and the placeholder ProcessOwnerId that can be attached to a scheduler task.
- src/arch/aarch64/exceptions.rs owns the saved ExceptionFrame, lower AArch64
  vector names, and accepted lower-EL syscall routing vocabulary.
- src/memory_map/layout.rs, src/memory_map/page_frames.rs, and
  src/memory_map/translation.rs own early memory discovery, early bootstrap
  frame ownership, and early EL2 identity translation-table construction.
  They do not yet own per-process user tables.
- src/target/qemu_virt.rs and src/target/rpi5.rs contain scenario-local
  lower-EL/user-memory proof code and retained-output conventions. They are
  evidence producers, not process-install owners.

## Address-Space Ownership Gap

The loader can prove that an image requests canonical user virtual ranges and
that each segment belongs to UserText or UserData. No source module yet owns a
process address-space object, process page-table root, per-process translation
lifecycle, or switchable TTBR/TCR policy.

The next contract must define the owner for:

- process address-space identity and lifetime;
- page-table root allocation and teardown;
- translating each PlannedUserSegment into leaf mappings without expanding
  permissions beyond the image plan;
- W^X preservation, null-guard preservation, and user/kernel split
  enforcement; and
- unwind behavior if any segment allocation, copy, zero-fill, or mapping step
  fails.

The early memory-map code can provide vocabulary for page size, frame
ownership, and translation attributes, but it is bootstrap-oriented. It must
not be treated as an accepted process allocator or user page-table
implementation without a separate contract.

## User-Frame And Segment Install Gap

ProgramImagePlan records enough metadata to install bytes later, but no code
currently allocates user frames, copies file bytes into those frames, zeros BSS
tail ranges, records per-segment provenance, or releases partially installed
state.

The next contract should choose:

- whether the first installer is target-independent metadata-only, a
  QEMU/substitute physical-page install, or a more complete address-space
  mutation;
- the exact frame source and ownership tags for UserText, UserData, and future
  UserStack frames;
- all-or-nothing install ordering for file copy, zero-fill, page-table
  insertion, and cleanup;
- deterministic errors for allocation exhaustion, bad plan invariants, copy
  failures, and teardown failures; and
- evidence proving that rejected plans leave no process object, frame lease,
  page table, descriptor table, lower-EL frame, or runnable task behind.

## Initial Lower-EL Frame Gap

The accepted lower-EL work proves diagnostic trap and syscall routes, plus
saved-frame capture through ExceptionFrame. It does not define a reusable
initial user-return frame for a loaded program.

Before launch, a later contract must bind:

- ELR to the validated loader entry point inside UserText;
- user SP to a valid UserStack mapping with 16-byte ABI alignment;
- SPSR/PSTATE to an accepted lower-EL runtime value;
- x0 through x5 and other argument registers to either an empty startup ABI or
  a documented /bin/init bootstrap convention; and
- fault classification if entry, stack, or PSTATE validation fails after image
  installation.

This inventory does not choose argv/envp, auxiliary vectors, TLS, libc
startup, or shell behavior.

## User Stack And Argv/Envp Gap

src/posix.rs has range and copy helper vocabulary, but no stack builder, guard
page, argv/envp byte layout, auxiliary vector policy, or string-table
ownership. The accepted program-loader format contract deliberately deferred
the first argv/envp policy.

The next process-install contract may keep argv/envp out of scope and require
a separate stack contract. If it includes stack work, it must specify stack
size, guard placement, alignment, initial register values, byte layout,
ownership, cleanup, and deterministic overflow or bad-pointer behavior.

## Process And Scheduler Gap

src/scheduler.rs::ProcessOwnerId is only a placeholder attached to a scheduler
Task. There is no process table, PID allocator, executable image ownership
record, current process lookup, exit status, parent/child relation, wait
target, signal policy, credentials, current working directory, root directory,
or process-owned address-space pointer.

The scheduler can run kernel tasks and carry a process-owner token. It cannot
yet create a process, install a loaded image into it, enqueue it as a user
task, switch process address spaces, or tear it down on exit. Any contract
that touches scheduler handoff must preserve the split between scheduler-local
TaskId and future POSIX process identity.

## Descriptor Inheritance Gap

src/posix.rs::DescriptorTable and ProcessDescriptorStore already model
inherited stdio, close, dup, and fixed stdin/read surfaces. They do not define
exec inheritance, close-on-exec enforcement during image replacement,
filesystem-backed descriptor lifetime, open-file-description final release, or
descriptor table ownership inside a process object.

The process-install contract should treat descriptor inheritance as an input
or explicitly defer it. It must not imply descriptor-backed filesystem
syscalls or current-working-directory behavior from the read-only initramfs
fixture.

## Boundary Table

| Area | Accepted input | Missing before process install |
| --- | --- | --- |
| Image validation | ProgramImagePlan with digest, entry, ordered segments, zero-fill, and errors. | Process install owner, frame allocation, page-table mutation, all-or-nothing cleanup, and install evidence. |
| Address space | User virtual range, null guard, permission vocabulary, and lower-EL mapping names. | Process address-space object, page-table root, TTBR switch policy, teardown, and user/kernel split enforcement at runtime. |
| User frames | Bootstrap frame ownership vocabulary and early allocator shapes. | Per-process frame source, lease/release API, zero-fill ownership, OOM policy, and partial-install rollback. |
| Entry frame | Lower-AArch64 trap and syscall proof frame capture. | Initial ELR/SP/SPSR/register construction for a loaded image and launch-time fault classification. |
| User stack | Copy helper and user-range validation primitives. | Stack mapping, guard, argv/envp/auxv/TLS policy, alignment, overflow handling, and cleanup. |
| Process identity | ProcessOwnerId placeholder and scheduler task vocabulary. | Process table, PID, current process, image/address-space owner, exit/wait state, and scheduler handoff. |
| Descriptors | DescriptorTable and ProcessDescriptorStore inherited-stdio vocabulary. | Exec inheritance policy, close-on-exec enforcement, open-file lifetime, cwd/root ownership, and descriptor release on failure/exit. |
| Evidence | QEMU/substitute PASS/classification conventions. | Process-install success/failure lines, no-partial-install proof, and conditional lower-EL launch deferral. |

## Recommended Next Task

The next bounded task should be
phase8-process-install-contract-20260530, documentation-only under Milestone
8.3.

That contract should define the first accepted process-install boundary from a
validated ProgramImagePlan to either a target-independent installation plan or
a minimal process-owned address-space install. It should choose the address
space owner, frame source, mapping and zero-fill order, rollback rules, error
matrix, descriptor-inheritance deferral, initial-frame deferral, and exact
evidence needed before any Rust implementation.

It should keep lower-EL launch of the loaded image, argv/envp bytes,
exec/spawn/wait, shell behavior, descriptor-backed filesystem syscalls, Pi 5
hardware proof, writable filesystems, persistent storage, networking, SSH,
RP1/PCIe, UART interrupt ownership, and DMA/cache-driver policy blocked until
later explicit tasks accept their gates.

## Validation

- static inspection: git status --short before edits was clean.
- static source/documentation review: inspected the accepted Phase 8 program
  loader source inventory, format contract, QEMU/substitute smoke plan,
  program-loader core task record, QEMU/substitute smoke task record,
  program-loader closeout checkpoint, retained smoke evidence, Phase 7
  EL0/address-space contract, Phase 7 descriptor-table contract,
  src/program_loader.rs, src/initramfs.rs, src/posix.rs, src/scheduler.rs,
  src/syscall.rs, src/arch/aarch64/exceptions.rs,
  src/memory_map/layout.rs, src/memory_map/page_frames.rs,
  src/memory_map/translation.rs, src/target/qemu_virt.rs, src/target/rpi5.rs,
  roadmap, SUMMARY, and ADR index.
- static inspection: git diff --check passed.
- documentation: mdbook build passed.
- staged whitespace inspection: git diff --cached --check passed before
  commit.
- hardware: no Pi 5 archive publication, hardware-lock acquisition, power
  cycle, or serial observation was performed by this inventory.
