# Phase 7 Copy-In/Copy-Out Helper Core

Task: phase7-copyin-copyout-helper-core-20260529
Status: accepted

## Scope

This implementation task added only the target-independent Phase 7.3
copy-in/copy-out helper core required by the accepted helper contract. It did
not add syscall table entries, pointer-taking syscalls, descriptor I/O,
runtime console or TTY integration, process loading, VFS/filesystem behavior,
path or argv/envp copying, shell behavior, networking, SSH, RP1/PCIe work,
UART interrupt ownership, DMA/cache-driver policy, demand paging,
copy-on-write, signal/restart semantics, lower-EL fault-table recovery, QEMU
scenarios, Pi 5 hardware runs, boot archive publication, hardwareTestLock
acquisition, scheduler/runtime refactors, allocator/MMU policy changes, or
broad cleanup.

## Accepted Core

- Added copy_from_user for direction-specific user read into a kernel buffer.
- Added copy_to_user for direction-specific user write from a kernel buffer.
- Kept the access-kind relationship explicit: copy-in validates
  UserAccessKind::Read and copy-out validates UserAccessKind::Write.
- Validates the complete user range before byte movement.
- Maps user boundary failures to PosixError::Fault, including null guard,
  kernel range, wraparound, copy limit, unmapped gaps, no-access mappings,
  permission mismatches, and backing-storage gaps.
- Maps malformed kernel-side buffer length to PosixError::InvalidArgument
  before side effects.
- Returns the exact requested byte count on success, including zero for valid
  zero-length copies.
- Preserves the accepted all-or-nothing policy: failures do not mutate the
  destination kernel buffer or user backing storage.

## Evidence

- static inspection: git status --short before edits was clean.
- implementation diff summary: changed src/posix.rs only for the helper API,
  shared validation helper, and target-independent unit tests.
- unit tests: cargo -Zjson-target-spec test passed with 205 no_std tests.
- covered helper unit cases: successful copy-in, successful copy-out,
  zero-length valid user start, null guard, kernel range, wraparound, copy
  limit, unmapped gap, no-access mapping, read/write permission mismatch,
  backing-storage gap, short kernel buffer/source EINVAL, and all-or-nothing
  destination preservation.
- formatting: cargo fmt --all -- --check passed.
- static inspection: git diff --check passed.
- documentation: mdbook build was not required because docs/src was not
  touched.

## Deferred Work

Pointer-taking syscalls, descriptor read/write/close/dup routing, runtime
console or TTY integration, process loading, VFS/filesystem behavior, path
copying, argv/envp loading, shell behavior, networking, SSH, RP1/PCIe, UART
interrupt ownership, DMA/cache-driver policy, demand paging, copy-on-write,
signal/restart semantics, lower-EL fault-table recovery, QEMU scenarios, and
Pi 5 hardware proof remain blocked until later explicit tasks.

## Acceptance

Accepted; final commit hash is recorded in durable supervisor state for this
task after commit creation.
