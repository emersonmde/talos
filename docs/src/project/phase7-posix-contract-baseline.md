# Phase 7 POSIX Contract Baseline

Status: accepted as the documentation-only Phase 7.1 POSIX baseline
contract. This document defines Talos vocabulary and invariants for errno,
paths, process lifetime, descriptors, stdio inheritance, and early loader
shape before implementation tasks add target-independent cores. The descriptor
portion is narrowed by the accepted
[Phase 7 Descriptor Table Contract](phase7-descriptor-table-contract.md). The
accepted Phase 7.1 path/error model and descriptor-table core now implement the
first target-independent test seams under this contract. This baseline itself
does not add boot scenarios, QEMU runs, Pi 5 hardware runs, EL0 entry,
SVC/syscall ABI, VFS, filesystem objects, program loading, networking, SSH,
shell behavior, RP1/PCIe, UART interrupt ownership, or DMA/cache-driver
policy.

This baseline expands the early POSIX shape note and follows the accepted
Phase 7 POSIX contract source inventory. Its purpose is to stop convenient
diagnostic surfaces from hardening into user/kernel contracts.

## Contract Invariants

- The scheduler schedules tasks. A scheduler TaskId is not a POSIX process ID,
  and kernel threads may continue to have no process owner.
- A process is the future resource-owning container for an address space,
  descriptor table, current working directory, root or namespace handle,
  credentials, exit status, wait state, and one or more user threads.
- stdin, stdout, and stderr are future process-local descriptors 0, 1, and 2.
  They attach to console or TTY objects through descriptor-owned handles, not
  by calling QEMU or Pi 5 target backends directly.
- runtime-console0, the TTY line discipline, runtime-console write results,
  console input polling results, and diagnostic command errors are internal
  kernel contracts. They are not errno values, syscall results, shell exit
  statuses, or descriptor readiness semantics.
- The diagnostic command channel is a kernel diagnostic client of the accepted
  TTY path. It is not a shell grammar, syscall path, program loader,
  filesystem command interface, spawn path, exec path, or environment model.
- The current EL2 identity map is only a kernel bring-up map. It is not a
  lower-EL isolation contract and does not permit untrusted payloads.

## Error And Errno Vocabulary

Talos keeps rich internal errors inside kernel subsystems and translates them
at explicit boundaries. The first target-independent implementation should
define an internal POSIX-facing error enum whose variants map to stable
errno-style names. Numeric syscall return values remain deferred until the
SVC/syscall ABI task, but the names below are contractual for Phase 7.1 tests
and later ABI mapping:

| Name | Meaning in the first contract |
| --- | --- |
| EPERM | operation not permitted by the current kernel contract |
| ENOENT | empty path for lookup, or path component not found once lookup exists |
| EINTR | operation interrupted after signal delivery exists |
| EIO | backend I/O failure that cannot be classified more narrowly |
| ENOEXEC | image format not executable once loading exists |
| EBADF | invalid, closed, or inappropriate descriptor number |
| ECHILD | no waitable child once wait exists |
| EAGAIN | operation would block or resource is temporarily unavailable |
| ENOMEM | allocation failed |
| EACCES | access denied by permissions once credentials exist |
| EFAULT | invalid userspace pointer once copy-in/copy-out exists |
| EBUSY | resource is busy |
| EEXIST | object already exists |
| ENODEV | no such device |
| ENOTDIR | non-directory component used as a directory |
| EISDIR | directory used where a non-directory object is required |
| EINVAL | invalid argument or flag combination |
| EMFILE | process descriptor table is full |
| ENOTTY | descriptor is not a TTY for a TTY-only operation |
| ENOSPC | no space left in the target object or namespace |
| EPIPE | write to a pipe-like object with no reader once pipes exist |
| ERANGE | result or input exceeds a representable range |
| ENAMETOOLONG | full path or one component exceeds configured limits |
| ENOSYS | syscall or operation is not implemented |
| ENOTEMPTY | directory is not empty |
| ENOTSUP | operation is known but not supported for this object |

Implementation tasks may add internal variants only when they also define
whether the variant maps to one of these names or extends the baseline with a
documented reason. Diagnostic labels such as line-complete, unknown-command,
backend-error, or timeout must not leak as errno names.

Before syscall ABI work, a host-side API may return Result<T, PosixError>.
When the syscall ABI exists, successful calls should return non-negative
values and failures should return a conventional negative errno value or an
equivalent documented machine ABI. That return convention is deliberately not
implemented by this contract.

## Path Contract

Paths are byte strings with slash as the separator and no embedded NUL bytes.
The first path core is lexical and target-independent. It does not perform VFS
lookup, mount traversal, symlink expansion, permission checks, filesystem I/O,
or current-working-directory storage.

The normalized representation should preserve:

- whether resolution starts at the process root or the process current working
  directory;
- an ordered list of normalized components;
- whether a trailing slash requires the final object to be a directory once
  lookup exists;
- deterministic errors for empty input, NUL bytes, full-path length limits,
  component length limits, and component-count limits.

Absolute paths start at the process root. Relative paths start at the process
current working directory. The process owns both handles later; a shell or
diagnostic singleton does not own them.

Lexical normalization rules:

- repeated slash separators collapse to one separator;
- dot components are removed;
- dot-dot cancels the previous normal component when one exists;
- absolute paths clamp attempts to walk above root, so slash-dot-dot
  normalizes to root;
- relative paths retain leading dot-dot components that cannot be cancelled
  without a current working directory, so dot-dot/a remains relative with
  dot-dot then a;
- an empty path is invalid for lookup and maps to ENOENT;
- root is the root path with zero components;
- dot is a relative current-directory path with zero components;
- a trailing slash on any non-root path sets the directory-required flag.

The first tests must cover empty input, root, repeated separators, dot and
dot-dot, absolute parent clamping, relative leading parents, trailing slash,
embedded NUL rejection, full-path limits, component limits, and component-count
limits.

Symlink-aware semantics are deferred. Once symlinks exist, lookup may need to
interleave normalization and object traversal instead of relying only on a
lexical normalized string.

## Process Lifetime Vocabulary

Phase 7.1 defines names without implementing a process table:

- Task: a schedulable execution context owned by the scheduler.
- Kernel thread: a task running only in kernel address space.
- User thread: a task executing in a process address space at EL0 after later
  EL0 work.
- Process: a resource-owning container for address space, descriptor table,
  current working directory, root or namespace handle, credentials, exit
  status, wait state, and one or more user threads.
- PID: a future process-table identifier, separate from TaskId.
- Parent and child: process lifetime relation used by spawn, exit, and wait.
- Zombie: a future exited process whose status remains waitable by its parent.
- Reaped: a future exited process whose waitable state has been consumed.

spawn means creating a new process from a kernel-selected image and inherited
resources. exec means replacing the current process image while preserving
process identity and selected inherited resources. exit means recording a
status and terminating process execution. wait means observing a child process
transition and consuming the waitable status.

The scheduler may already block or wake tasks for kernel purposes. POSIX wait,
process exit, signals, process groups, sessions, credentials, and controlling
terminal ownership remain deferred and must not be inferred from scheduler
task state or diagnostic command labels.

## Descriptor Contract

File descriptors are process-local integer handles. Descriptor numbers are
indexes into a process descriptor table. Descriptor table entries reference an
open file description or a kernel object handle; entries must not embed target
UART addresses, scheduler task IDs, diagnostic command state, or filesystem
implementation details.

The first descriptor vocabulary is:

- open: later create a descriptor from a path and flags through VFS lookup;
- read: transfer bytes from a readable descriptor into a caller buffer;
- write: transfer bytes from a caller buffer to a writable descriptor;
- close: release one descriptor table entry;
- dup: allocate a second descriptor entry that references the same underlying
  open description or kernel object;
- pipe: later create paired descriptors for byte-stream IPC;
- socket: later create network descriptors after networking phases.

Deterministic descriptor errors for the first table core:

- invalid descriptor number: EBADF;
- closed descriptor: EBADF;
- operation not supported by descriptor kind: ENOTSUP;
- read on write-only descriptor or write on read-only descriptor: EBADF;
- descriptor table full during allocation or dup: EMFILE;
- invalid flags or dup target: EINVAL;
- unimplemented operation: ENOSYS.

The accepted Phase 7 descriptor-table contract further fixes the first table
core boundary: descriptor entries are process-local table slots, dup creates a
new descriptor number referencing the same underlying open description or
kernel object handle, close releases one table entry without closing every
duplicate, and inherited stdio is represented by reserved descriptor-owned
handles rather than direct target backend calls.

Descriptors 0, 1, and 2 are reserved for inherited stdio when a process is
created:

- fd 0, stdin, is readable and attaches to the controlling TTY input side only
  after descriptor lifetime, readiness or blocking policy, and scheduler
  sleep/wakeup policy exist;
- fd 1, stdout, is writable and attaches through a descriptor-owned handle to
  the normal output side of the controlling TTY or runtime-console0;
- fd 2, stderr, is a separate writable descriptor identity that may initially
  share the same console object as stdout.

The first target-independent descriptor table core exists for table semantics
only. Kernel diagnostics may keep calling runtime console, TTY, and
diagnostic-command code directly until a later descriptor I/O integration task
exists. Those calls are not POSIX read, write, isatty, readiness polling, EOF,
partial I/O, or nonblocking behavior.

## Early Loader, Arguments, And Environment

The early loader vocabulary is intentionally small:

- Program image: a kernel-selected executable byte sequence or built-in image
  accepted by a later loader task.
- Loader: kernel code that validates an image, creates mappings, builds the
  initial user stack, and prepares entry state.
- Entry point: the first userspace instruction address after later EL0 work.
- argv: an ordered list of byte strings, with argv[0] naming the program image
  by convention.
- envp: an ordered list of KEY=VALUE byte strings inherited or provided by the
  process creator.
- Auxiliary data: optional loader-provided records for later libc or runtime
  support.

The first process creation path may use a kernel-selected image and empty
environment. It must not use diagnostic commands as a program namespace, must
not implement shell PATH lookup, and must not treat command-line parsing as
loader policy. Shell syntax, scripts, globbing, redirection, pipelines,
environment expansion, job control, sessions, and signals remain later work.

The loader ABI cannot be implemented until user address-space layout,
copy-in/copy-out, lower-EL trap return, executable page permissions, and fault
policy are accepted.

## Current Versus Deferred

Contractual now:

- errno-style names and translation boundary vocabulary;
- target-independent lexical path normalization semantics, implementation, and
  edge-case tests;
- process, PID, parent/child, spawn, exec, exit, wait, zombie, and reaped
  vocabulary;
- process-local descriptor entry vocabulary plus target-independent
  descriptor-table allocation, lookup, close, dup, inherited stdio, reserved
  object kind, access-check, and deterministic error tests;
- stdio inheritance shape through descriptor-owned handles;
- early loader, argv, and envp vocabulary;
- the diagnostic command channel remains outside shell, syscall, loader, VFS,
  and descriptor contracts.

Deferred:

- Rust implementation beyond the accepted target-independent path/error and
  descriptor-table cores;
- PID allocator, process table, parent/child storage, exit status storage, wait
  queues, signals, process groups, sessions, credentials, and controlling TTY;
- process address spaces, user stacks, EL0 entry, lower-EL vector routing,
  SVC/syscall ABI, user pointer validation, and copy-in/copy-out;
- descriptor I/O integration, VFS lookup, filesystem objects, program loading,
  pipes, sockets, readiness polling, blocking I/O, and partial I/O;
- filesystem-backed commands, local shell, networking, SSH, RP1/PCIe, UART
  interrupt ownership, and DMA/cache-driver policy.

## Required Test Seams

The next implementation tasks must keep their first tests target-independent.

Path/error tests should instantiate the path normalizer and errno mapping
without booting QEMU or touching hardware. They must cover the path cases named
above and prove that invalid inputs map to the accepted POSIX error names.

Descriptor-table tests should instantiate a process-local table with stub
kernel objects. They must cover inherited stdio entries, invalid descriptor
numbers, double close, close plus reuse, dup aliasing, table-full allocation,
invalid flags, and unsupported operations. These are table semantics only; they
must not call runtime console, TTY polling, VFS, syscall, EL0, QEMU, or Pi 5
hardware paths.

Milestone 7.1 required those target-independent tests for path normalization
and descriptor-table edge cases; they are now accepted and reconciled by the
Phase 7.1 closeout checkpoint.

## Next Implementation Slice

The accepted next implementation slices were
phase7-path-error-model-core-20260528 and
phase7-descriptor-table-core-20260528. No task may use this baseline to start
EL0 entry, SVC/syscall ABI, descriptor I/O integration, VFS, filesystem,
program loading, networking, SSH, shell behavior, RP1/PCIe, UART interrupt
ownership, or DMA/cache-driver policy without a later explicit task.

## Validation

- static inspection: reconciled accepted scheduler, runtime console, TTY,
  diagnostic command, lower-EL readiness, early POSIX shape, and Phase 7 source
  inventory contracts.
- whitespace inspection: git diff --check passed.
- documentation: mdbook build passed.
