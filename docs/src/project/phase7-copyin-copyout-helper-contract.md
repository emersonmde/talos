# Phase 7 Copy-In/Copy-Out Helper Contract

Status: accepted as the documentation-only Phase 7.3 copy-in/copy-out helper
contract. This document follows the accepted
[Phase 7 Pi 5 Syscall Proof Closeout Checkpoint](phase7-pi5-syscall-proof-closeout-checkpoint.md)
and the accepted
[Phase 7 EL0 Trap and Address-Space Contract](phase7-el0-trap-address-space-contract.md).
It does not add Rust behavior, assembly behavior, boot scenarios, QEMU runs,
Pi 5 hardware runs, archive publishing, hardware-lock use, descriptor I/O,
pointer-taking syscalls, process loading, VFS, filesystem, shell behavior,
networking, SSH, RP1/PCIe, UART interrupt ownership, or DMA/cache-driver
policy.

This contract defines the target-independent helper boundary that later
implementation may use before any syscall copies bytes across the user/kernel
boundary. It deliberately stays below descriptor I/O: the helpers validate and
copy byte ranges, but they do not interpret file descriptors, paths, pipes,
TTY streams, process loaders, or filesystem objects.

## Helper Boundary

The first implementation should add target-independent helpers with these
inputs:

- the accepted user mapping slice used by validate_user_memory_access;
- a user virtual start address transported as a 64-bit syscall argument;
- a byte length bounded by DEFAULT_USER_COPY_LIMIT;
- a kernel buffer represented by an explicit destination or source slice;
- an access direction: copy-in reads user memory into a kernel buffer;
  copy-out writes kernel bytes into user memory.

The helpers should produce one of these outputs:

- success with the exact number of bytes copied, which must equal the
  requested length for the first implementation;
- PosixError::Fault when address, range, permission, mapping, or kernel buffer
  validation fails;
- PosixError::InvalidArgument only for malformed kernel-side helper use that
  is independent of user memory, such as a mismatched kernel buffer length if
  the implementation chooses to expose that as recoverable.

The first helper names should be direction-specific, for example
copy_from_user and copy_to_user, or a single copy_user_bytes wrapper around
direction-specific internals. The implementation task may choose the exact
shape, but it must keep the direction and access-kind relationship visible in
tests.

## Validation Order

The helper must reject invalid requests before committing any byte movement.
The deterministic order is:

1. Check the requested length against the helper's kernel buffer length and the
   accepted DEFAULT_USER_COPY_LIMIT.
2. Construct the accepted UserRange for start and length.
3. Validate the whole user range against the provided mappings with
   UserAccessKind::Read for copy-in or UserAccessKind::Write for copy-out.
4. Perform the byte copy only after the entire range is known to be valid.
5. Return the copied byte count on success.

Execute permission is not a read or write substitute. A USER_TEXT mapping is
readable for copy-in only because it contains read permission; executable-only
future mappings would not be readable. A USER_DATA mapping is usable for
copy-in and copy-out because it contains read and write permission.

## EFAULT Mapping

The first helpers must map all user-memory boundary failures to
PosixError::Fault / EFAULT:

- null guard addresses below USER_NULL_GUARD_END;
- start addresses at or beyond USER_ADDRESS_SPACE_END;
- start + length wraparound;
- ranges that cross the accepted user/kernel split;
- ranges longer than the selected copy limit;
- unmapped gaps inside an otherwise valid user range;
- mappings with UserMappingPermissions::NONE;
- permission mismatch, including writing to text or read-only memory;
- ranges that touch UserGuard, kernel mappings, MMIO, bootstrap translation
  tables, kernel stacks, DTB data, or future device windows.

Zero-length copies are side-effect-free. They may return success with a copied
length of 0 only after the start address passes the accepted non-guard user
address check. This keeps zero-length behavior aligned with the current
UserRange vocabulary and prevents unchecked kernel pointers from becoming
valid just because no bytes move.

## Partial-Copy Policy

The first implementation must be all-or-nothing:

- validate the complete range before copying;
- do not return a short successful copy;
- do not report how many bytes would have been copied on EFAULT;
- do not modify a user destination or kernel destination after validation
  fails.

This avoids committing Linux-style short-copy or restart semantics before Talos
has signals, interrupted syscalls, per-thread errno storage, demand paging, or
resumable user faults. Later work may add partial-copy policy, but it requires
a new contract because it changes observable syscall behavior.

## Recoverable Versus Fatal Faults

A fault discovered while a syscall validates a user pointer is recoverable at
the syscall boundary when no side effect has been committed. The syscall should
return -EFAULT in x0 through the accepted syscall return convention.

A lower-EL data abort caused by direct user execution remains process-fatal
until a later process-fault policy exists. It is not automatically equivalent
to a recoverable copy helper failure. The distinction is:

- helper validation failure before byte movement: recoverable EFAULT;
- synchronous user data or instruction abort outside a recoverable helper
  boundary: future process-fatal fault classification;
- kernel fault while copying after a helper accepted the range: kernel
  invariant failure unless a later fault-table mechanism explicitly makes that
  instruction recoverable.

The first helper implementation must not install fault-table recovery, signal
delivery, demand paging, copy-on-write resolution, or syscall restart behavior.

## Unit-Testable Contract

The next implementation can be accepted with target-independent unit tests
only if it changes pure helper logic under the existing mapping vocabulary.
Tests must cover:

- successful copy-in from readable user data;
- successful copy-out to writable user data;
- zero-length success for a valid non-guard user address;
- null guard rejection;
- kernel-range rejection;
- wraparound rejection;
- length-limit rejection;
- unmapped gap rejection;
- no-access mapping rejection;
- read/write permission mismatch;
- all-or-nothing behavior when validation fails.

QEMU or Pi 5 evidence is not required for pure helper logic. QEMU becomes
required once a pointer-taking syscall uses the helpers through lower-EL trap
routing. Pi 5 hardware becomes required only for a later explicit physical
proof task.

## Blocked Surfaces

This contract keeps these surfaces blocked until later explicit tasks:

- descriptor read/write/close/dup through syscall entry;
- TTY-backed stdio reads or writes through descriptors;
- path copying, string termination policy, argv/envp loading, ELF loading, and
  filesystem pathname traversal;
- process tables, PID allocation, exit/wait, credentials, sessions,
  controlling TTY, signals, syscall restart, per-thread errno storage,
  demand paging, copy-on-write, and resumable user faults;
- VFS/filesystem behavior, shell behavior, networking, SSH, RP1/PCIe, UART
  interrupt ownership, and DMA/cache-driver policy.

## Next Implementation Boundary

The next bounded task should be phase7-copyin-copyout-helper-core-20260529. It
may implement only target-independent byte copy helpers, error mapping, and
unit tests matching this contract. It must not add a pointer-taking syscall,
mutate descriptor tables, read or write descriptor-backed streams, enter new
QEMU or Pi 5 proof scenarios, load programs, expose VFS/filesystem behavior,
create a shell, or touch networking/SSH.

Supervisor planning is required before that implementation task is promoted,
because the current durable queue names only this contract task.

## Validation

- static inspection: git status --short before edits was clean.
- static documentation diff: this contract names helper inputs, outputs,
  validation order, recoverable error behavior, EFAULT mapping, partial-copy
  policy, process-fatal fault boundaries, unit-testable cases, blocked
  surfaces, and the next implementation boundary.
- whitespace inspection: git diff --check passed.
- documentation: mdbook build passed.
- Rust fmt/tests, QEMU reruns, and Pi 5 hardware runs were not required
  because this task changes only Markdown documentation and durable worker
  state.
