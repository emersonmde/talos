# Phase 7 Copy-In/Copy-Out Helper Closeout Checkpoint

Status: accepted as the Phase 7.3 checkpoint after the target-independent
copy-in/copy-out helper core. This checkpoint follows the accepted
[Phase 7 Copy-In/Copy-Out Helper Contract](phase7-copyin-copyout-helper-contract.md)
and the accepted helper-core task record. It does not add Rust or assembly
behavior, QEMU runs, Pi 5 hardware runs, boot archive publishing,
hardware-lock use, pointer-taking syscalls, descriptor I/O, runtime console or
TTY integration, process loading, VFS, filesystem, shell behavior, networking,
SSH, RP1/PCIe, UART interrupt ownership, or DMA/cache-driver policy.

## Accepted Byte-Copy Boundary

The accepted helper core at commit
b675a6f10fbb3e91781f98bd0ae63290ee4e967c adds target-independent helpers in
src/posix.rs:

- copy_from_user reads bytes from a validated user range into a kernel buffer.
- copy_to_user writes bytes from a kernel buffer into a validated user range.
- copy-in validates with UserAccessKind::Read, and copy-out validates with
  UserAccessKind::Write.
- both helpers validate the complete user range before byte movement.
- successful calls return the exact requested length, including zero for a
  valid zero-length range.
- user-boundary failures return PosixError::Fault.
- malformed kernel-side helper use, such as a short kernel buffer or source
  slice, returns PosixError::InvalidArgument before side effects.
- the first implementation is all-or-nothing; validation failure does not
  mutate the destination kernel buffer or user backing storage used by tests.

This accepts only pure byte-copy helper behavior under the existing
POSIX/user-memory vocabulary. It does not prove a lower-EL syscall can pass a
pointer to the helpers, does not allocate a pointer-taking syscall number, and
does not expose descriptor-backed I/O.

## Unit-Test Coverage

The accepted helper-core task record reports cargo -Zjson-target-spec test
passing with 205 no_std tests. The helper-specific coverage includes:

- successful copy-in from readable user data;
- successful copy-out to writable user data;
- valid zero-length copy;
- null guard rejection;
- kernel-range rejection;
- wraparound rejection;
- DEFAULT_USER_COPY_LIMIT rejection;
- unmapped gap rejection;
- no-access mapping rejection;
- read/write permission mismatch;
- backing-storage gap rejection;
- short kernel destination/source EINVAL;
- all-or-nothing preservation for failed copy-in and copy-out.

These tests satisfy the contract cases for a target-independent helper. QEMU is
not required until a later pointer-taking syscall exercises the helpers through
lower-EL trap routing. Pi 5 hardware is not required until a later explicit
physical proof task.

## Deferred Surfaces

The following surfaces remain blocked:

- lower-EL pointer-taking syscall argument capture and routing;
- descriptor read, write, close, dup, and descriptor-backed stream behavior;
- runtime console or TTY-backed stdio through file descriptors;
- path copying, string termination policy, argv/envp loading, ELF loading, and
  filesystem pathname traversal;
- process loading, PID lifecycle, exit/wait, credentials, sessions,
  controlling TTY, signals, syscall restart, per-thread errno storage, demand
  paging, copy-on-write, and resumable user faults;
- VFS/filesystem behavior, shell behavior, networking, SSH, RP1/PCIe, UART
  interrupt ownership, and DMA/cache-driver policy.

## Next Direction

The next bounded task should be
phase7-pointer-taking-syscall-source-inventory-20260529. That inventory should
map source owners and gaps for a later proof-only or stable lower-EL
pointer-taking syscall boundary, including frame argument extraction,
syscall-number allocation, helper invocation, return/error encoding, user
mapping provenance, retained QEMU evidence, and diagnostic-surface quarantine.

Descriptor I/O and filesystem behavior should remain out of scope until the
pointer-taking syscall boundary has an accepted source inventory and contract.

## Validation

- static inspection: helper-core commit
  b675a6f10fbb3e91781f98bd0ae63290ee4e967c and task record
  tasks/2026-05-29-phase7-copyin-copyout-helper-core.md reviewed.
- unit tests: helper-core acceptance recorded cargo -Zjson-target-spec test
  passing with 205 no_std tests.
- formatting: helper-core acceptance recorded cargo fmt --all -- --check
  passing.
- whitespace inspection: git diff --check passed for this checkpoint.
- documentation: mdbook build passed for this checkpoint.
