# Phase 7 Read And Stdin Core

Task: phase7-read-stdin-core-20260529
Owner: worker
Status: accepted
Date: 2026-05-29
Milestone: Phase 7.4, File Descriptor Table

## Goal

Wire the accepted talos_read/stdin contract through target-independent syscall
dispatch and the current process-owned descriptor table without adding QEMU
read proof, Pi 5 hardware proof, runtime-console0/TTY stdin, or broader POSIX
descriptor readiness.

## Scope

- Added `TALOS_READ_SYSCALL` as stable syscall number 4 and
  `SyscallNumber::TalosRead`.
- Added `FixedStdin` proof-buffer state and
  `read_descriptor_from_fixed_stdin()` for bounded fd 0/stdin reads through
  `copy_to_user()`.
- Added `dispatch_process_descriptor_with_fixed_stdin()` so the
  target-independent process descriptor path can read from fd 0 or duplicates
  of fd 0 after resolving the current `ProcessOwnerId` through
  `ProcessDescriptorStore`.
- Preserved the accepted scalar behavior: context-requiring descriptor
  syscalls, including talos_read, return `-ENOTSUP` without descriptor-store
  context.
- Kept existing descriptor-write, close, dup, unknown-syscall, diagnostic
  marker, and copy-probe quarantine behavior intact.

## Non-Goals

- No QEMU read/stdin smoke acceptance, Pi 5 hardware proof, boot archive
  publication, hardwareTestLock acquisition, runtime-console0/TTY/hardware
  stdin, process loading, VFS/filesystem, shell, networking, SSH, object
  finalization, pipes, sockets, signals, wait queues, nonblocking I/O,
  RP1/PCIe, UART interrupt ownership, DMA/cache-driver policy, or full POSIX
  descriptor readiness claim.
- No broad descriptor-table refactor or cache-maintenance policy change.

## Changed Files

- `src/posix.rs`
- `src/syscall.rs`
- `src/target/qemu_virt.rs`
- `src/target/rpi5.rs`
- `docs/src/decisions/README.md`
- `docs/src/roadmap.md`
- `tasks/2026-05-29-phase7-read-stdin-core.md`

## Evidence

- Accepted contract dependency:
  `49d292935b4bff2220946e9eb7fe6b60de209a26`.
- Focused unit-test coverage:
  `descriptor_read_number_requires_context_in_scalar_dispatch`,
  `talos_read_stdin_copies_fixed_input_and_advances_after_copy`,
  `talos_read_short_count_and_eof_are_bounded_to_fixed_input`,
  `talos_read_duplicate_of_stdin_shares_fixed_input_cursor`,
  `talos_read_zero_length_does_not_consume_or_use_destination`,
  `talos_read_fd_errors_do_not_copy_or_consume_input`,
  `talos_read_reserved_registers_reject_without_mutation`,
  `talos_read_copy_faults_do_not_consume_fixed_input`, and
  `talos_read_reports_enotsup_for_non_stdin_readable_objects_or_missing_source`.
- Formatting:
  `cargo fmt --all -- --check` passed.
- Unit tests:
  `cargo -Zjson-target-spec test` passed with 248 no_std tests after adding
  the Talos QEMU install directory to `PATH`.
- QEMU/substitute scalar regression:
  `scripts/qemu-syscall-smoke.sh` passed with
  `classification=qemu-syscall-smoke-complete` and `PASS`.
- QEMU/substitute descriptor-write regression:
  `scripts/qemu-descriptor-write-smoke.sh` passed with
  `classification=qemu-descriptor-write-smoke-complete` and `PASS`.
- QEMU/substitute close regression:
  `scripts/qemu-close-syscall-smoke.sh` passed with
  `classification=qemu-close-syscall-smoke-complete` and `PASS`.
- QEMU/substitute dup regression:
  `scripts/qemu-dup-syscall-smoke.sh` passed with
  `classification=qemu-dup-syscall-smoke-complete` and `PASS`.
- Static inspection:
  `git diff --check` passed.
- Documentation:
  `mdbook build` passed.

## Deferred Work

QEMU read/stdin smoke, Pi 5 physical read proof, runtime-console0/TTY/hardware
stdin, process loading, fork/spawn/exec, descriptor inheritance across exec,
close-on-exec application, process exit teardown, open-file-description
reference counting, object finalizers, file offsets, VFS/filesystem lookup,
regular files, directories, pipes, sockets, terminal sessions, foreground
process groups, blocking/readiness, nonblocking flags, poll/select, wait
queues, signals, restart semantics, path copying, argv/envp loading,
per-thread errno storage, libc/Rust std stdio, shell behavior, networking, SSH,
RP1/PCIe, UART interrupt ownership, DMA/cache-driver policy, and full POSIX
descriptor readiness remain blocked.

## Result

Accepted as the target-independent read/stdin core. The next bounded task is
`phase7-qemu-read-stdin-smoke-plan-20260529`, scoped to a documentation-only
QEMU read/stdin smoke plan before any QEMU read implementation/evidence task or
Pi 5 physical proof work.
