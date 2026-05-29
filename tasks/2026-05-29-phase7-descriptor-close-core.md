# Phase 7 Descriptor Close Core

Task: phase7-descriptor-close-core-20260529
Owner: worker
Status: accepted
Date: 2026-05-29
Milestone: Phase 7.4, File Descriptor Table

## Goal

Implement the target-independent descriptor lifetime and close core selected by
the accepted descriptor lifetime/close contract, without adding a syscall
surface.

## Scope

- Added `ProcessDescriptorStore::close_current_descriptor()` as the
  process-owned close helper that resolves an optional current
  `ProcessOwnerId`, borrows the owning mutable `DescriptorTable`, and applies
  table-local `DescriptorTable::close()`.
- Added focused no_std unit tests for process-owned stdout/stderr close,
  missing/unknown owner, invalid descriptor, double-close, descriptor reuse,
  and dup interaction.
- Kept close/dup/read syscall routing, QEMU execution, Pi 5 hardware,
  hardwareTestLock acquisition, boot archive publication, process loading,
  VFS/filesystem, shell, networking, SSH, RP1/PCIe, UART interrupt ownership,
  DMA/cache-driver policy, and full POSIX descriptor readiness blocked.

## Changed Files

- `src/posix.rs`
- `tasks/2026-05-29-phase7-descriptor-close-core.md`

## Evidence

- Accepted contract dependency:
  `4ff46a6f68bf8349ba0b974d610a8ceb3d92ccd1`.
- Focused unit-test coverage:
  `process_descriptor_close_stdout_blocks_descriptor_write_lookup`,
  `process_descriptor_close_stderr_follows_table_local_rule`,
  `process_descriptor_close_failures_map_to_ebadf`, and
  `process_descriptor_close_reuses_lowest_slot_and_preserves_duplicates`.
- Formatting:
  `cargo fmt --all -- --check` passed.
- Unit tests:
  `cargo -Zjson-target-spec test` passed with 226 no_std tests.
- Static inspection:
  `git diff --check` passed.
- Documentation:
  `mdbook build` passed because this task record was touched; no docs/src
  contract wording changed.

## Deferred Work

close/dup/read syscalls, syscall numbers and lower-EL ABI for close/dup/read,
QEMU close smoke, Pi 5 physical close/dup/read claims, process loading,
fork/spawn/exec, close-on-exec application, process exit teardown,
open-file-description reference counting, object finalizers, VFS/filesystem,
regular files, directories, pipes, sockets, stdin/read behavior, TTY
blocking/readiness, shell behavior, networking, SSH, RP1/PCIe, UART interrupt
ownership, DMA/cache-driver policy, and full POSIX descriptor readiness remain
blocked.

## Result

Accepted as the target-independent descriptor lifetime and close core. The next
bounded task should be
`phase7-descriptor-close-core-closeout-checkpoint-20260529`, scoped to
reconciling the accepted inventory, contract, implementation, and unit-test
evidence before any close/dup/read syscall contract or QEMU/Pi 5 proof task.
