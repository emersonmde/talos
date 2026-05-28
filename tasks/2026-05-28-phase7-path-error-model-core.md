# Phase 7 Path/Error Model Core

## Task

- Title: Phase 7 path/error model core
- Owner: worker
- Date: 2026-05-28
- Milestone: Phase 7.1, POSIX Contract Baseline
- Scope: target-independent POSIX errno vocabulary and lexical path
  normalization core

## Goal

Implement the first no_std-compatible path normalization and structured
errno-style error mapping primitives under the accepted POSIX baseline.

## Acceptance Criteria

- Path/error model implementation is limited to the accepted
  target-independent contract surface.
- Tests cover normalization and error edge cases named by the contract.
- No runtime console, diagnostic command, scheduler, boot image, QEMU proof, or
  hardware behavior is changed.
- Accepted work is committed before descriptor-table implementation depends on
  it.

## Work Performed

- Added src/posix.rs with the Phase 7.1 PosixError vocabulary and stable
  errno-style names.
- Added a bounded, allocation-free normalize_path implementation that records
  root versus current-working-directory start, normalized components, and
  trailing-slash directory requirements.
- Covered empty input, root paths, repeated separators, dot and dot-dot,
  absolute parent clamping, relative leading parents, trailing slash,
  embedded NUL rejection, full-path limits, component limits, and
  component-count/storage limits with target-independent unit tests.

## Evidence

- static inspection: touched files are src/main.rs, src/posix.rs, and this
  task record.
- static inspection: path/error implementation is target-independent,
  allocation-free, and does not call runtime console, diagnostic command,
  scheduler, boot image, QEMU, VFS, filesystem, EL0, syscall, descriptor, or
  hardware paths.
- unit tests: cargo -Zjson-target-spec test passed with 172 no_std tests,
  including 16 posix path/error tests.
- fmt/lint: cargo fmt --all -- --check passed.
- whitespace inspection: git diff --check passed.
- documentation: mdbook build passed.
- QEMU/hardware: no QEMU proof, boot image, or Pi 5 hardware run was claimed
  for this target-independent task.

## Result

Accepted as the first target-independent Phase 7.1 path/error model core.
Descriptor-table implementation remains blocked until its own accepted
contract and dependency gates are satisfied.
