# Phase 7 Descriptor Table Contract

Status: accepted as the documentation-only Phase 7.1 descriptor-table
contract. This document narrows the descriptor portion of the accepted
[Phase 7 POSIX Contract Baseline](phase7-posix-contract-baseline.md) before
any descriptor-table implementation, syscall ABI, EL0, VFS, filesystem, pipe,
socket, shell, networking, SSH, RP1/PCIe, UART interrupt ownership, or
DMA/cache-driver policy work.

The contract binds future stdin, stdout, and stderr descriptors to the
accepted runtime-console and TTY direction without turning kernel diagnostic
calls into POSIX I/O. It is a process-local data-model contract, not a target
backend or userspace ABI.

## Contract Invariants

- A descriptor table belongs to one future process. Descriptor numbers are
  process-local indexes; they are not global handles, scheduler TaskIds, UART
  identifiers, diagnostic command tokens, or filesystem object IDs.
- A descriptor table entry references a shared open description or a reserved
  kernel object handle. The descriptor entry owns per-descriptor flags and
  lifetime; the referenced object owns object state.
- Closing one descriptor table entry releases that entry. It must not imply
  that every duplicated descriptor or inherited descriptor for the same
  referenced object is closed.
- Duplicating a descriptor allocates a new descriptor number that references
  the same underlying open description or kernel object handle.
- stdin, stdout, and stderr are ordinary reserved descriptor entries 0, 1,
  and 2 at process creation. They are not privileged target backdoors.
- Descriptor-facing stdio must attach through runtime-console0 and TTY kernel
  objects by handle. Descriptor read/write paths must not call QEMU or Pi 5
  PL011 backends directly.
- Diagnostic command input and output may continue to call the accepted TTY
  and runtime-console paths directly until process descriptors exist. Those
  calls are not POSIX read, write, close, dup, shell I/O, EOF, readiness, or
  nonblocking behavior.

## Descriptor Numbers And Table Shape

The first implementation should use a small fixed-capacity table suitable for
host-side unit tests. The capacity is a kernel configuration detail for now,
not a user-visible limit constant. Descriptor numbers are unsigned indexes
inside that table when represented internally; negative file descriptors first
matter at the later syscall ABI boundary and must map to EBADF there.

The table should reserve descriptor numbers 0, 1, and 2 for inherited stdio
when a process is created:

- fd 0, stdin: readable, attached to the controlling TTY input side only after
  descriptor lifetime, readiness or blocking policy, and scheduler
  sleep/wakeup policy exist.
- fd 1, stdout: writable, attached through a descriptor-owned handle to the
  normal output side of the controlling TTY or runtime-console0.
- fd 2, stderr: writable, a separate descriptor identity that may initially
  share the same runtime-console0 object as stdout.

The first table core may model these as reserved kernel-object handle kinds
without invoking console or TTY code. That keeps the unit-tested table behavior
separate from runtime I/O integration.

## Descriptor Entry State

Each occupied entry should distinguish:

- descriptor flags: per-entry properties such as close-on-exec later;
- access mode: readable, writable, or read-write;
- object kind: reserved stdio, file, directory, pipe endpoint, socket, device,
  or other kernel object handle;
- reference identity: a stable token for the shared open description or kernel
  object in target-independent tests.

The first core does not need to implement file offsets, VFS lookup, device I/O,
pipe buffers, sockets, scheduler blocking, readiness polling, permissions, or
copy-in/copy-out. It should still preserve the split between descriptor entry
state and the referenced object so later open-file-description behavior can be
added without changing close and dup semantics.

## Operations

The first target-independent table core should cover only data-model behavior:

- allocate: place a new descriptor entry in the lowest available table slot
  unless a caller explicitly requests a valid target slot later;
- get: return an occupied entry or EBADF for invalid or closed descriptors;
- close: mark one occupied descriptor number closed and release that table
  entry;
- dup: allocate the lowest available descriptor number that references the
  same object as the source descriptor;
- inherit stdio: create a new table with fd 0, fd 1, and fd 2 populated with
  reserved stdio handles.

Open, read, write, pipe, socket, ioctl, poll/select, fcntl, fork, spawn, exec,
and syscall return-value encoding are vocabulary only until later tasks accept
their contracts and implementation boundaries.

## Deterministic Error Cases

The next implementation must give stable target-independent results for these
edge cases:

- invalid descriptor number: EBADF;
- closed descriptor: EBADF;
- close of an already closed descriptor: EBADF;
- dup from an invalid or closed source descriptor: EBADF;
- table full during allocation or dup: EMFILE;
- invalid requested descriptor target or unsupported flag combination: EINVAL;
- read from a write-only descriptor, or write to a read-only descriptor: EBADF;
- operation known by vocabulary but not implemented by the table core: ENOSYS;
- operation unsupported by a descriptor kind: ENOTSUP;
- descriptor not attached to a TTY for a TTY-only operation: ENOTTY.

The table core should use the accepted POSIX error vocabulary from
`src/posix.rs`; it should not invent console, TTY, or diagnostic command error
strings as table errors.

## Reserved Object Kinds

The first table core should reserve names for object kinds without implementing
their backing subsystems:

- stdio input and output handles backed later by TTY/runtime-console objects;
- regular file and directory handles backed later by VFS/filesystem lookup;
- pipe endpoint handles backed later by local IPC;
- socket handles backed later by networking;
- device handles backed later by explicit device contracts.

Reserved kinds are type tags for tests and future routing. They are not proof
that the underlying subsystem exists.

## Runtime Console And TTY Attachment

runtime-console0 remains the accepted normal kernel console identity. The TTY
line discipline remains the accepted local input and stdio-shape layer above
the runtime console backend. The descriptor layer should attach to those
objects through kernel object handles only after descriptor lifetime exists.

For fd 1 and fd 2, descriptor write integration should later translate a
write request into the same runtime-console output operation used by kernel
diagnostics, then map internal console outcomes to POSIX-facing results at the
descriptor or syscall boundary. For fd 0, descriptor read integration should
later translate TTY input outcomes through descriptor-owned readiness and
blocking policy. This contract accepts neither integration yet.

## Next Implementation Boundary

The next bounded task may implement `phase7-descriptor-table-core-20260528`
as a target-independent data model with unit tests. Its scope should be
limited to descriptor table allocation, lookup, close, dup, stdio inheritance,
reserved object kind tagging, access-mode checks, and deterministic
PosixError results for the edge cases named above.

That task must not add runtime console or TTY I/O integration, syscall ABI,
EL0, VFS lookup, filesystem objects, pipes, sockets, shell behavior,
networking, SSH, RP1/PCIe, UART interrupt ownership, DMA/cache-driver policy,
QEMU boot claims, or Pi 5 hardware claims.

## Validation

- static inspection: git status --short was clean before documentation edits.
- documentation: this document updates the accepted Phase 7.1 descriptor
  contract and the related roadmap, POSIX, console/TTY, SUMMARY, and decision
  log references.
- validation gate: git diff --check.
- validation gate: mdbook build.
- Rust fmt/tests are not required by this documentation-only task.
