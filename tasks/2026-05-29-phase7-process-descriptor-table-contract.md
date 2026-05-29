# Phase 7 Process Descriptor Table Contract

Status: accepted and committed.

Task id: phase7-process-descriptor-table-contract-20260529

## Scope

This documentation-only Milestone 7.4 task defines the first process-owned
descriptor-table contract after the accepted file descriptor table source
inventory. It does not change Rust or assembly behavior, run QEMU, run Pi 5
hardware, acquire hardwareTestLock, publish a boot archive, or implement
process loading, close/dup/read syscalls, VFS/filesystem, shell, networking,
SSH, RP1/PCIe, UART interrupt ownership, or DMA/cache-driver policy.

## Evidence

- Contract document:
  `docs/src/project/phase7-process-descriptor-table-contract.md`.
- Accepted source-inventory commit:
  `3f8d14f334486b39b9816991f23b194dced5019b`.
- Static source review:
  `src/posix.rs`, `src/syscall.rs`, `src/runtime_console.rs`, and
  `src/scheduler.rs`.
- Static documentation review:
  accepted descriptor table/source-inventory/descriptor-write docs, roadmap,
  SUMMARY, and decision log.
- Next bounded task:
  `phase7-process-descriptor-table-core-20260529`.

## Validation

- static inspection: `git status --short` before edits was clean.
- whitespace inspection: `git diff --check` passed.
- documentation: `mdbook build` passed.
- Rust fmt/tests, QEMU runs, and Pi 5 hardware runs were not required because
  this task is documentation-only.

## Deferred Surfaces

The contract keeps PID allocation, process table lifetime, fork/spawn/exec,
process loading, process-owned address spaces, close/dup/read syscalls,
stdin/read behavior, TTY blocking/readiness, VFS/filesystem, regular files,
pipes, sockets, devices, shell, networking, SSH, RP1/PCIe, UART interrupt
ownership, DMA/cache-driver policy, and full POSIX descriptor claims blocked.
