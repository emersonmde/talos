# Phase 7 Descriptor Lifetime And Close Source Inventory

Status: accepted as the documentation-only Milestone 7.4 descriptor lifetime
and close-semantics source inventory after the accepted
[Phase 7 Process Descriptor Table Closeout Checkpoint](phase7-process-descriptor-table-closeout-checkpoint.md).
This task adds no Rust behavior, assembly behavior, QEMU run, Pi 5 hardware
run, boot archive publication, hardware-lock acquisition, close/dup/read
syscall contract, process loading, VFS/filesystem behavior, shell behavior,
networking, SSH, RP1/PCIe, UART interrupt ownership, or DMA/cache-driver
policy.

This inventory maps the current table-level close and dup primitives, process
descriptor ownership surface, retained evidence, and missing tests before any
close/dup/read syscall contract. It preserves the current boundary: inherited
stdio descriptor tables and talos_write fd 1/fd 2 are accepted through
runtime-console0; descriptor lifetime, close, dup, and read remain blocked at
the syscall and live-process layers until explicit follow-up tasks accept their
contracts and gates.

## Source Ownership

### Descriptor Table Data Model

src/posix.rs owns the target-independent descriptor table vocabulary:

- STDIN_FD, STDOUT_FD, and STDERR_FD reserve descriptors 0, 1, and 2.
- DescriptorFlags accepts only EMPTY and CLOSE_ON_EXEC; close-on-exec is stored
  vocabulary only because exec is not accepted.
- DescriptorAccess distinguishes read-only, write-only, and read/write entries,
  and maps access mismatch to PosixError::BadDescriptor.
- DescriptorObjectKind reserves stdio, file, directory, pipe, socket, device,
  and generic kernel-object tags. Only stdio tags are behaviorally integrated.
- DescriptorObject stores a target-independent kind plus reference value. The
  reference is not an open-file-description reference count, kernel object
  registry index, inode handle, pipe endpoint owner, or hardware address.
- DescriptorEntry owns one table entry's access mode, flags, and object.
  require_readable(), require_writable(), and require_tty() are validation
  helpers; unsupported_operation() and unsupported_kind_operation() preserve
  deterministic future error vocabulary.
- DescriptorTable&lt;CAPACITY&gt; owns process-local descriptor slots as
  [Option&lt;DescriptorEntry&gt;; CAPACITY].

The accepted table operations are:

- DescriptorTable::new_empty() creates an empty fixed-capacity table.
- DescriptorTable::with_inherited_stdio() installs fd 0 as read-only
  StdioInput and fd 1/fd 2 as write-only StdioOutput.
- DescriptorTable::get() returns an occupied entry or BadDescriptor.
- DescriptorTable::allocate() uses the lowest free descriptor slot.
- DescriptorTable::allocate_at() installs an exact free descriptor slot or
  returns InvalidArgument.
- DescriptorTable::close() removes one occupied table entry with take() and
  returns the removed DescriptorEntry; invalid, absent, or already closed
  descriptors return BadDescriptor.
- DescriptorTable::dup() copies the source entry into the lowest free slot.
  This preserves a shared DescriptorObject value, but it does not yet model
  open-file-description reference counts, final object release, dup2, dup3, or
  fcntl.

Accepted close semantics are therefore table-local only: one descriptor slot is
cleared, duplicated entries remain separately addressable, and double close is
BadDescriptor. No syscall ABI number, lower-EL frame mutation, object
finalizer, process teardown hook, filesystem close, pipe close, socket close,
or TTY detach behavior is accepted.

### Process Descriptor Ownership

src/posix.rs::ProcessDescriptorOwner and src/posix.rs::ProcessDescriptorStore
own the first process-owned descriptor table boundary:

- ProcessDescriptorOwner::with_inherited_stdio() binds one ProcessOwnerId to
  one inherited-stdio DescriptorTable.
- ProcessDescriptorOwner::descriptor_table() and descriptor_table_mut() expose
  borrowed table access only while the owner record is alive.
- ProcessDescriptorStore::new_empty() creates a bounded owner store.
- ProcessDescriptorStore::create_owner_with_inherited_stdio() installs one
  owner, rejects duplicate owners with InvalidArgument, and maps store-full to
  TooManyOpenFiles.
- descriptor_table() and descriptor_table_mut() look up an owner id.
- current_descriptor_table() and current_descriptor_table_mut() translate a
  missing current owner or unknown owner into BadDescriptor.

The store has no teardown, owner removal, process exit, wait, fork, spawn, exec,
close-on-exec application, descriptor-table clone, address-space binding,
credential/session model, or cross-CPU process registry. Those gaps are
intentional until a lifetime contract accepts them.

### Syscall And Console Surfaces

src/syscall.rs owns the stable syscall vocabulary currently accepted:

- TALOS_NOP_SYSCALL = 0.
- TALOS_WRITE_SYSCALL = 1.
- dispatch() returns NotSupported for talos_write without a descriptor table.
- dispatch_descriptor_write() and its private dispatch_talos_write() route only
  talos_write through a caller-provided descriptor table, user mappings,
  scratch buffer, and runtime-console backend.

No close syscall number, dup syscall number, read syscall number, fd-returning
open syscall, per-thread errno storage, restart policy, blocking policy, or
lower-EL close/dup argument contract exists.

src/posix.rs::write_descriptor_to_runtime_console() currently accepts only fd 1
and fd 2 when their entries are write-capable StdioOutput descriptors. It
copies bytes from accepted user memory with copy_from_user() and writes them
through runtime_console::write_default_console_bytes(). It treats fd 0, invalid
descriptors, closed descriptors, excessive length, undersized scratch, copy
faults, unsupported object kind, and backend failure deterministically, but it
does not implement stdin/read, partial writes, nonblocking behavior, TTY
readiness, EOF, signals, or descriptor object lifetime.

src/runtime_console.rs owns runtime-console0 output and input facade
vocabulary. It does not own POSIX descriptor lifetime, object reference counts,
close finalization, or TTY controlling-session policy.

src/scheduler.rs::ProcessOwnerId, Task::attach_process_owner(),
Task::process_owner(), and SchedulerTaskSnapshot::process_owner() preserve the
owner identity that later syscall paths can use for descriptor lookup. The
scheduler still does not own a process table, PID allocator, exit lifecycle,
descriptor teardown, or userspace loader.

## Documentation And Evidence Matrix

| Surface | Current evidence | Accepted | Missing before close/dup/read contract |
| --- | --- | --- | --- |
| Descriptor table contract | docs/src/project/phase7-descriptor-table-contract.md | Table-local allocation, lookup, close, dup, inherited stdio, deterministic errors | Syscall numbers, live process lifetime, object finalizers |
| Descriptor table core | tasks/2026-05-28-phase7-descriptor-table-core.md; src/posix.rs unit tests | close() removes one slot; double close is BadDescriptor; dup() preserves object value and separate descriptor slot | Open-file-description reference counts, final close, dup2/dup3/fcntl |
| Process descriptor table contract/core | docs/src/project/phase7-process-descriptor-table-contract.md; tasks/2026-05-29-phase7-process-descriptor-table-core.md | One ProcessOwnerId owns one inherited-stdio table in ProcessDescriptorStore | Owner teardown, table removal, descriptor inheritance across process lifetime |
| QEMU process descriptor stdio smoke | tasks/evidence/2026-05-29-qemu-process-descriptor-stdio-smoke-core/qemu-process-descriptor-stdio-smoke.log | lower-AArch64 talos_write fd 1/fd 2 resolves through process-owned inherited stdio to runtime-console0 | QEMU close/dup/read syscall observations |
| Pi 5 descriptor-write proof | tasks/evidence/2026-05-29-pi5-descriptor-write-proof/local3-candidate-rerun/rerun-proof-lines.txt | Physical talos_write fd 1/fd 2 through inherited stdio and runtime-console0 | Physical process-owned descriptor-table proof, close/dup/read physical proof |

The retained accepted evidence remains scoped. It proves descriptor-write and
process-owned inherited stdio lookup only at the named evidence levels. It does
not prove descriptor lifetime, syscall close, syscall dup, syscall read, final
object release, process teardown, VFS close, pipe close, socket close, or full
POSIX descriptor readiness.

## Unit-Test Coverage

Existing no_std unit tests in src/posix.rs cover:

- descriptor_flags_accept_known_bits_and_reject_unknown_bits;
- descriptor_object_kind_names_cover_reserved_future_kinds;
- inherited_stdio_populates_process_local_reserved_descriptors;
- inherited_stdio_requires_room_for_descriptors_zero_one_and_two;
- descriptor_allocate_uses_lowest_available_slot;
- descriptor_allocate_at_rejects_invalid_or_occupied_target;
- descriptor_get_close_and_double_close_use_ebadf;
- descriptor_dup_preserves_object_reference_and_separate_lifetime;
- descriptor_full_table_maps_allocate_and_dup_to_emfile;
- descriptor_access_mismatch_maps_to_ebadf;
- descriptor_reserved_operation_errors_are_deterministic;
- process_descriptor_owner_initializes_inherited_stdio_for_owner;
- process_descriptor_store_resolves_current_owner_table;
- process_descriptor_lookup_failures_map_to_ebadf;
- process_descriptor_store_preserves_owner_and_table_errors.

These tests are enough to support an inventory-level claim that table-local
close and dup primitives exist and are deterministic. Before accepting a
close-semantics implementation beyond the existing data model, the next task
should add or reaffirm focused unit evidence for:

- closing fd 1 or fd 2 in a ProcessDescriptorStore current-owner table makes
  subsequent descriptor-write lookup return BadDescriptor;
- double close through the process-owned mutable lookup remains BadDescriptor;
- descriptor reuse after close uses the lowest available slot and preserves
  deterministic table state;
- dup followed by close of the original leaves the duplicate usable;
- closing the last duplicate is recorded as a table-local removal only, because
  object finalization is still blocked;
- duplicate owner/table lookup failures still map to BadDescriptor,
  InvalidArgument, or TooManyOpenFiles as already contracted;
- future syscall-facing close/dup argument validation and return conventions
  remain absent until a separate contract accepts syscall numbers and evidence.

## Deferred Surfaces

This inventory explicitly keeps the following blocked: close syscall, dup
syscall, read syscall, process loading, fork/spawn/exec, descriptor inheritance
across exec, close-on-exec enforcement, process exit teardown, wait/exit
status, open-file-description reference counting, object finalizers, VFS and
filesystem lookup, regular files, directories, pipes, sockets, device
registries, stdin behavior, TTY blocking/readiness, EOF, nonblocking flags,
wait queues, signals, restart semantics, path copying, argv/envp loading,
per-thread errno storage, libc/Rust std stdio, shell behavior, networking, SSH,
RP1/PCIe, UART interrupt ownership, DMA/cache-driver policy, Pi 5 physical
close/dup/read claims, and full POSIX descriptor readiness.

## Recommended Next Task

The next bounded Milestone 7.4 task should be
phase7-descriptor-lifetime-close-contract-20260529, documentation-only.

That contract should define the first descriptor lifetime and close-semantics
boundary without adding syscall routing yet. It should name:

- table-local close behavior through DescriptorTable::close();
- process-owned mutable lookup through ProcessDescriptorStore;
- the exact unit tests required for close, double-close, descriptor reuse, and
  dup interaction;
- the deferred open-file-description reference-count model and final object
  release semantics;
- why syscall-facing close/dup/read numbers and lower-EL proof remain blocked
  until later tasks.

The contract should not implement Rust behavior, run QEMU, acquire
hardwareTestLock, publish a boot archive, run Pi 5 hardware, or advance process
loading, VFS/filesystem, shell, networking, SSH, RP1/PCIe, UART interrupt
ownership, DMA/cache-driver policy, or a phase transition.

## Validation

- static inspection: git status --short before documentation edits was clean.
- static source review: inspected src/posix.rs, src/syscall.rs,
  src/runtime_console.rs, src/scheduler.rs, src/target/qemu_virt.rs,
  src/target/rpi5.rs, accepted Phase 7 descriptor-table/process-descriptor
  docs, task records, and retained QEMU/Pi 5 descriptor evidence references.
- whitespace inspection: git diff --check passed.
- documentation: mdbook build passed.
- Rust fmt/tests, QEMU runs, and Pi 5 hardware runs were not required because
  this task changes only Markdown documentation and durable worker state.
