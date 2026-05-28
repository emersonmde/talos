# Phase 7 Descriptor Table Core

## Task

- Title: Phase 7 descriptor table core
- Owner: worker
- Date: 2026-05-28
- Milestone: Phase 7.1, POSIX Contract Baseline
- Scope: target-independent process-local descriptor table data model

## Goal

Implement the first target-independent descriptor table core under the
accepted descriptor-table contract.

## Acceptance Criteria

- Descriptor table core is limited to target-independent data model behavior
  and deterministic error handling.
- Tests cover the descriptor edge cases named by the contract.
- No runtime console, TTY, scheduler, boot image, QEMU proof, or hardware
  behavior is changed.
- Accepted work is committed before any syscall, EL0, VFS, or console
  descriptor integration task starts.

## Work Performed

- Extended `src/posix.rs` with fixed-capacity process-local descriptor tables,
  descriptor entries, descriptor flags, access modes, reserved object kinds,
  and stable object-reference identities.
- Added inherited stdio table construction for fd 0, fd 1, and fd 2 using
  reserved stdio object kinds without invoking runtime console or TTY code.
- Added allocation, exact-slot allocation, lookup, close, dup, access checks,
  TTY-only checks, unimplemented-operation, and unsupported-kind error helpers.
- Covered invalid fd, closed fd, double close, dup from closed source, table
  full, inherited stdio, reserved future object kinds, unsupported flags,
  invalid requested descriptor target, access-mode mismatch, unimplemented
  operation, unsupported descriptor kind, and non-TTY TTY-only operation with
  target-independent unit tests.

## Evidence

- static inspection: touched files are `src/posix.rs` and this task record.
- static inspection: descriptor implementation is target-independent and does
  not call runtime console, TTY, scheduler, boot image, QEMU, VFS, filesystem,
  EL0, syscall, pipe, socket, shell, networking, SSH, RP1/PCIe, UART interrupt,
  DMA/cache, or hardware paths.
- unit tests: `cargo -Zjson-target-spec test` passed with 183 no_std tests,
  including 11 descriptor-table tests.
- fmt/lint: `cargo fmt --all -- --check` passed.
- whitespace inspection: `git diff --check` passed.
- documentation: mdBook was not required because docs under `docs/src` were
  not touched.
- QEMU/hardware: no QEMU proof, boot image, or Pi 5 hardware run was claimed
  for this target-independent task.

## Result

Accepted as the first target-independent Phase 7.1 descriptor table core.
Runtime console/TTY descriptor I/O integration, syscall ABI, EL0, VFS,
filesystem, pipes, sockets, shell behavior, networking, SSH, RP1/PCIe, UART
interrupt ownership, and DMA/cache-driver policy remain blocked for later
explicit tasks.
