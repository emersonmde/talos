# Phase 7 User Memory Permission Core

## Task

- Title: Phase 7 user memory permission core
- Owner: worker
- Date: 2026-05-28
- Milestone: Phase 7.2, EL0 Trap Path and User Address Spaces
- Scope: target-independent user address-range and permission validation

## Goal

Implement the first no_std-compatible user virtual range, mapping permission,
access-kind, and copy-boundary validation primitives under the accepted
EL0/address-space contract.

## Acceptance Criteria

- Implementation is limited to target-independent address-range and permission
  validation matching the accepted contract.
- Tests cover null, wraparound, kernel-range, guard-page, unmapped,
  read/write/execute mismatch, and length-limit edge cases.
- Invalid user ranges map to the accepted PosixError vocabulary, including
  EFAULT for invalid userspace pointers.
- No runtime console, diagnostic command, scheduler, boot image, QEMU proof,
  or hardware behavior is changed.

## Work Performed

- Extended `src/posix.rs` with the accepted canonical user-address ceiling,
  null-guard boundary, default copy limit, `UserAccessKind`,
  `UserMappingPermissions`, `UserRange`, `UserMapping`, and
  `validate_user_memory_access`.
- Kept validation independent of TTBR/TCR/SCTLR state, exception vectors, EL0
  entry, process tables, syscall ABI, VFS/filesystems, descriptors, and live
  user-memory copying.
- Added target-independent no_std unit tests for null guard rejection,
  kernel-range rejection, wraparound rejection, copy length limit rejection,
  zero-length/guard-overlapping mappings, contiguous readable ranges,
  read/write/execute permission mismatches, unmapped gaps, and no-access guard
  gaps.

## Evidence

- static inspection: touched files are `src/posix.rs` and this task record.
- static inspection: user-memory validation is target-independent and does not
  call runtime console, diagnostic command, scheduler, boot image, QEMU, EL0
  entry, translation-register, VFS, filesystem, descriptor, syscall, shell,
  networking, SSH, RP1/PCIe, UART interrupt, DMA/cache, or hardware paths.
- unit tests: `cargo -Zjson-target-spec test` passed with 189 no_std tests,
  including 6 user-memory permission tests.
- fmt/lint: `cargo fmt --all -- --check` passed.
- whitespace inspection: `git diff --check` passed.
- documentation: mdBook was not required because docs under `docs/src` were
  not touched.
- QEMU/hardware: no QEMU proof, boot image, or Pi 5 hardware run was claimed
  for this target-independent task.

## Result

Accepted as the first target-independent Phase 7.2 user memory permission
core. EL0 entry, trap-return assembly, TTBR/TCR/SCTLR changes, live
copy-in/copy-out, syscall ABI, process table, VFS/filesystems, descriptor I/O,
shell behavior, networking, SSH, RP1/PCIe, UART interrupt ownership, and
DMA/cache-driver policy remain blocked for later explicit tasks.
