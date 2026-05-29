# Phase 7 Process Descriptor Table Contract

Status: accepted as the documentation-only Milestone 7.4 process-owned
descriptor-table contract after the accepted
[Phase 7 File Descriptor Table Source Inventory](phase7-file-descriptor-table-source-inventory.md).
This task adds no Rust behavior, assembly behavior, QEMU run, Pi 5 hardware
run, boot archive publication, hardware-lock acquisition, process loading,
VFS/filesystem behavior, shell behavior, networking, SSH, RP1/PCIe, UART
interrupt ownership, or DMA/cache-driver policy.

This contract defines the first process-owned descriptor-table slice. It turns
the accepted proof-owned inherited-stdio descriptor model into an explicit
future-process ownership boundary while preserving the current implementation
frontier: descriptor data-model behavior, talos_write fd 1/fd 2 evidence,
copy helper behavior, and runtime-console0 are accepted; process loading,
close/dup/read syscalls, VFS/filesystem, shell, networking, and full POSIX
descriptor claims remain deferred.

## Ownership Model

The first process descriptor table is owned by a process owner identity, not by
a CPU, target UART, syscall number, runtime-console backend, or diagnostic
scenario. The accepted scheduler vocabulary for this contract is
`ProcessOwnerId`: it is a stable owner token that may be attached to scheduler
`Task` metadata, but it is not yet a PID, process table index, fork/exec
handle, address-space owner, credential, session, or wait target.

The implementation task should introduce only a target-independent owner record
that can hold one `DescriptorTable` for a single process owner. The descriptor
table remains the authority for descriptor entries and deterministic
`PosixError` results. The owner record may expose creation, immutable lookup,
and mutable lookup operations, but it must not allocate global PIDs, spawn
tasks, publish cross-CPU process registries, or change scheduler ownership
rules.

The owner lifetime for this slice is static and explicit:

- a process owner is created by kernel code for the first user-process proof or
  substitute path;
- that owner receives exactly one descriptor table initialized with inherited
  stdio;
- table lookup is valid only while the owner object is borrowed by the caller;
- owner teardown, exit/wait, descriptor reference counting, and object close
  finalization are vocabulary only until later contracts accept them.

## Inherited Stdio

The first process-owned table must install descriptors 0, 1, and 2 using the
accepted `DescriptorTable::with_inherited_stdio()` shape:

- fd 0 is `StdioInput`, read-only, and reserved for a future TTY-backed read
  contract. It exists so inherited stdio shape is stable, but read behavior,
  blocking, readiness, EOF, line discipline, signal interruption, and scheduler
  wait/wakeup remain blocked.
- fd 1 is `StdioOutput`, write-only, and backed by the accepted
  runtime-console0 output identity.
- fd 2 is `StdioOutput`, write-only, and is a distinct descriptor identity
  that may initially share the same runtime-console0 backing object as fd 1.

The first backing identity is runtime-console0 through the existing
`runtime_console` facade. The process-owned table must not call QEMU or Pi 5
PL011 backends directly, create a device registry, or reinterpret diagnostic
command I/O as POSIX stdio. The descriptor object reference values for fd 0, fd
1, and fd 2 remain target-independent handles, not hardware addresses or
open-file-description reference counts.

## Current Process Lookup

The stable syscall path needs one explicit lookup boundary before descriptor
I/O can stop using proof-owned tables. For this slice, the current descriptor
table is found by:

1. taking the current task metadata from the scheduler or a narrow
   target-independent substitute fixture;
2. requiring that the task has a `ProcessOwnerId`;
3. resolving that owner id against the bounded process-descriptor owner store;
4. borrowing that owner's descriptor table for the duration of the syscall
   dispatch operation.

Lookup failure must be deterministic. A missing current task, missing
`ProcessOwnerId`, unknown owner id, or missing descriptor table maps to
`PosixError::BadDescriptor` for descriptor syscalls in this first slice,
because the syscall cannot locate the process descriptor namespace. This does
not define task exit, kill, wait, or process-not-found behavior.

The implementation may use an explicit test fixture for current-owner lookup
until a live process table exists. That fixture must use the same owner id and
table operations as the production-shaped path so QEMU/substitute evidence can
distinguish process-owned descriptor lookup from the earlier proof-owned
descriptor table.

## Operations Carried Forward

The next implementation task should carry forward only target-independent
operations already accepted by the descriptor-table core:

- create an owner with inherited stdio;
- attach or expose a `ProcessOwnerId` for a scheduler task or substitute
  current-task fixture;
- resolve the current owner to a descriptor table;
- get descriptor entries and preserve EBADF for invalid or closed descriptors;
- preserve EMFILE for table-full allocation paths and EINVAL for invalid
  explicit descriptor slots or unsupported descriptor flags in unit tests.

The process-owned slice does not add close, dup, read, open, pipe, socket,
ioctl, poll/select, fcntl, fork, spawn, exec, close-on-exec enforcement,
blocking/readiness, per-thread errno storage, or path copying. Existing close
and dup table methods remain target-independent primitives until syscall
contracts accept them.

## Error And Evidence Contract

Process-owner lookup must not weaken accepted descriptor errors:

- current owner resolved, fd 1/fd 2 write path: later QEMU/substitute evidence
  may prove talos_write routes through the process-owned inherited stdio table;
- fd 0 through talos_write: -EBADF remains the accepted write result;
- invalid descriptor: -EBADF remains stable;
- invalid user range: -EFAULT remains owned by copy_from_user();
- nonzero reserved syscall registers or excessive length: -EINVAL remains
  stable;
- unknown syscall: -ENOSYS remains stable.

The first implementation acceptance can rely on static inspection, fmt, and
unit tests because the owner model is target-independent. The later
QEMU/substitute smoke must prove that lower-AArch64 talos_write fd 1/fd 2 uses
a process-owned inherited stdio table rather than a proof-owned ad hoc table.
Pi 5 physical proof remains blocked until after that QEMU/substitute boundary
is accepted and a separate hardware plan names exact evidence requirements.

## Deferred Surfaces

This contract keeps the following blocked: PID allocation, process table
lifetime, fork/spawn/exec, process loading, process-owned address spaces,
close/dup/read syscalls, stdin behavior, TTY blocking/readiness, EOF,
nonblocking flags, wait queues, signals, restart semantics, open-file
description reference counting, VFS/filesystem lookup, regular files,
directories, pipes, sockets, device registries, shell behavior, libc/Rust std
stdio, networking, SSH, RP1/PCIe, UART interrupt ownership, DMA/cache-driver
policy, and any phase transition.

## Recommended Next Task

The next bounded task should be
`phase7-process-descriptor-table-core-20260529`.

That implementation task should add the smallest target-independent
process-owned descriptor-table owner/attachment/lookup surface, focused unit
tests for inherited stdio and current-owner resolution, and a task record
preserving the later QEMU/substitute evidence boundary. It should not acquire
hardwareTestLock, publish a boot archive, run Pi 5 hardware, or implement
close/dup/read syscalls.

## Validation

- static inspection: reviewed the accepted file descriptor table source
  inventory, descriptor table contract/core, descriptor syscall contract,
  `src/posix.rs`, `src/syscall.rs`, `src/runtime_console.rs`, and
  `src/scheduler.rs`.
- whitespace inspection: git diff --check passed.
- documentation: mdbook build passed.
- Rust fmt/tests, QEMU runs, and Pi 5 hardware runs were not required because
  this task changes only Markdown documentation and durable worker state.
