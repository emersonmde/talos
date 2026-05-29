# Phase 7 Dup Syscall Core

Task: phase7-dup-syscall-core-20260529
Owner: worker
Status: accepted
Date: 2026-05-29
Milestone: Phase 7.4, File Descriptor Table

## Goal

Wire the accepted talos_dup contract through target-independent syscall
dispatch and the current process-owned descriptor table without adding read,
QEMU dup proof, Pi 5 hardware proof, or broader descriptor readiness behavior.

## Scope

- Added `TALOS_DUP_SYSCALL` as stable syscall number 3 and
  `SyscallNumber::TalosDup`.
- Added `ProcessDescriptorStore::dup_current_descriptor()` so descriptor
  duplication resolves the current `ProcessOwnerId` before mutating the
  table-local `DescriptorTable`.
- Added `dispatch_talos_dup()` through `dispatch_process_descriptor()` with
  reserved-zero x1 through x5 validation, source descriptor conversion,
  lowest-free-slot allocation, and `-EBADF`, `-EMFILE`, and `-EINVAL`
  returns.
- Added `EMFILE` syscall return encoding for `PosixError::TooManyOpenFiles`.
- Allowed descriptor-write dispatch to write through duplicated stdout/stderr
  descriptors by checking the copied `DescriptorEntry` access and
  `DescriptorObjectKind::StdioOutput` instead of hard-coding only fd 1 and fd
  2.
- Preserved scalar dispatch behavior for context-requiring syscalls:
  `talos_write`, `talos_close`, and `talos_dup` return `-ENOTSUP` without
  descriptor-store context.

## Non-Goals

- No read syscall implementation, stdin/read object model, process loading,
  VFS/filesystem, shell, networking, SSH, Pi 5 hardware run, boot archive
  publication, hardwareTestLock acquisition, dup2/fcntl, object finalization,
  or full POSIX descriptor readiness claim.
- No QEMU dup syscall smoke or Pi 5 physical dup proof. Those remain separate
  evidence tasks.

## Changed Files

- `src/posix.rs`
- `src/syscall.rs`
- `src/target/qemu_virt.rs`
- `src/target/rpi5.rs`
- `docs/src/decisions/README.md`
- `docs/src/roadmap.md`
- `tasks/2026-05-29-phase7-dup-syscall-core.md`

## Evidence

- Accepted contract dependency:
  `041ca2f449afc9bd7889497720702b4f4f849bc3`.
- Focused unit-test coverage:
  `process_descriptor_store_dups_current_owner_descriptors`,
  `process_descriptor_dup_failures_map_to_ebadf_or_emfile`,
  `descriptor_dup_number_requires_context_in_scalar_dispatch`,
  `talos_dup_stdout_returns_lowest_free_descriptor_and_preserves_source`,
  `talos_dup_duplicate_remains_writable_after_source_close`,
  `talos_dup_stderr_and_stdin_follow_table_local_descriptor_rules`,
  `talos_dup_failures_are_deterministic_and_do_not_mutate_on_einval`, and
  `talos_dup_full_table_returns_emfile_without_mutation`.
- Unit tests:
  `cargo -Zjson-target-spec test` passed with 239 no_std tests.
- Formatting:
  `cargo fmt --all -- --check` passed.
- QEMU/substitute syscall regression:
  `scripts/qemu-syscall-smoke.sh` passed.
- QEMU/substitute descriptor-write regression:
  `scripts/qemu-descriptor-write-smoke.sh` passed.
- QEMU/substitute close regression:
  `scripts/qemu-close-syscall-smoke.sh` passed.
- Static inspection:
  `git diff --check` passed.
- Documentation:
  `mdbook build` passed.

## Deferred Work

QEMU dup syscall smoke, Pi 5 physical dup proof, read syscall behavior,
stdin/read object model, process loading, fork/spawn/exec, descriptor
inheritance across exec, close-on-exec application, dup2/dup3/fcntl, process
exit teardown, open-file-description reference counting, object finalizers,
file-offset sharing, VFS/filesystem lookup, regular files, directories, pipes,
sockets, device registries, TTY blocking/readiness, EOF, nonblocking flags,
wait queues, signals, restart semantics, path copying, argv/envp loading,
per-thread errno storage, libc/Rust std stdio, shell behavior, networking, SSH,
RP1/PCIe, UART interrupt ownership, DMA/cache-driver policy, and full POSIX
descriptor readiness remain blocked.

## Result

Accepted as the target-independent dup syscall core. The next bounded task is
`phase7-qemu-dup-syscall-smoke-plan-20260529`, scoped to a documentation-only
QEMU dup smoke plan before any QEMU dup implementation or Pi 5 physical proof
work.
