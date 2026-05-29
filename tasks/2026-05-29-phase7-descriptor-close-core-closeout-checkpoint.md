# Phase 7 Descriptor Close Core Closeout Checkpoint

Task: phase7-descriptor-close-core-closeout-checkpoint-20260529
Status: accepted

## Scope

This documentation-only checkpoint reconciles the accepted descriptor lifetime
and close source inventory, descriptor lifetime and close contract,
target-independent descriptor close core, changed files, unit-test evidence,
validation gates, residual risks, and deferred surfaces. It adds no Rust or
assembly behavior, QEMU rerun, Pi 5 hardware action, boot archive publication,
hardwareTestLock acquisition, close/dup/read syscalls, process loading,
VFS/filesystem, shell, networking, SSH, RP1/PCIe, UART interrupt ownership,
DMA/cache-driver policy, or full POSIX descriptor claim.

## Accepted Matrix

- Source inventory:
  phase7-descriptor-lifetime-close-source-inventory-20260529,
  commit 0de2bf2be47986da3220d9fb3edea534448822b8.
- Contract:
  phase7-descriptor-lifetime-close-contract-20260529,
  commit 4ff46a6f68bf8349ba0b974d610a8ceb3d92ccd1.
- Core:
  phase7-descriptor-close-core-20260529,
  commit 1e8cdd6fcb4bd16cbb04febd56529b66b0579182.

## Evidence

- Closeout document:
  docs/src/project/phase7-descriptor-close-core-closeout-checkpoint.md.
- Accepted core task record:
  tasks/2026-05-29-phase7-descriptor-close-core.md.
- Accepted changed files from core:
  src/posix.rs; tasks/2026-05-29-phase7-descriptor-close-core.md.
- Accepted core behavior:
  ProcessDescriptorStore::close_current_descriptor() resolves an optional
  current ProcessOwnerId to a mutable owner table and applies table-local
  DescriptorTable::close().
- Accepted unit-test evidence:
  process_descriptor_close_stdout_blocks_descriptor_write_lookup;
  process_descriptor_close_stderr_follows_table_local_rule;
  process_descriptor_close_failures_map_to_ebadf;
  process_descriptor_close_reuses_lowest_slot_and_preserves_duplicates.
- Accepted core validation:
  cargo fmt --all -- --check passed; cargo -Zjson-target-spec test passed
  with 226 no_std tests; git diff --check passed; mdbook build passed.
- This closeout validation:
  git status --short before edits was clean; git diff --check passed;
  mdbook build passed.

## Deferred Work

close syscall, dup syscall, read syscall, syscall numbers and lower-EL ABI,
QEMU close/dup/read smoke, Pi 5 physical close/dup/read proof, boot archive
publication, process loading, fork/spawn/exec, close-on-exec application,
process exit teardown, open-file-description reference counting, object
finalizers, VFS/filesystem lookup, regular files, directories, pipes, sockets,
stdin/read behavior, TTY blocking/readiness, EOF, nonblocking flags, wait
queues, signals, restart semantics, per-thread errno storage, shell behavior,
libc/Rust std stdio, networking, SSH, RP1/PCIe, UART interrupt ownership,
DMA/cache-driver policy, and full POSIX descriptor readiness remain blocked.

## Next Task

The next bounded Milestone 7.4 task should be supervisor-planned as a
documentation-only close/dup/read syscall source inventory, for example
phase7-close-dup-read-syscall-source-inventory-20260529. No queued task is
created by this worker closeout.
