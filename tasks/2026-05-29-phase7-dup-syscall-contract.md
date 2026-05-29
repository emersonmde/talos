# Phase 7 Dup Syscall Contract Task

Status: accepted

Task: phase7-dup-syscall-contract-20260529

Phase: Phase 7: POSIX Contract, EL0, Syscalls, and File Descriptors

Milestone: Milestone 7.4: File Descriptor Table

## Goal

Define a bounded talos_dup syscall policy for the current ProcessOwnerId-backed
descriptor table before any dup implementation, QEMU smoke, Pi 5 proof, or
read/stdin syscall work.

## Scope

- Added docs/src/project/phase7-dup-syscall-contract.md.
- Updated docs/src/SUMMARY.md, docs/src/roadmap.md, and
  docs/src/decisions/README.md.
- Recorded the target-independent policy for duplicating an occupied source
  descriptor into the lowest free descriptor slot in the current process-owned
  descriptor table.
- Preserved read, stdin object policy, process loading, VFS/filesystem, shell,
  networking, SSH, dup2/fcntl, object finalization, and full POSIX descriptor
  readiness as blocked.

## Non-Goals

- No Rust or assembly implementation changes.
- No QEMU run, Pi 5 hardware run, boot archive publication, or
  hardwareTestLock acquisition.
- No read syscall contract, stdin object model, process loading,
  VFS/filesystem behavior, shell behavior, networking, SSH, RP1/PCIe, UART
  interrupt ownership, object finalization, or DMA/cache-driver policy.

## Contract Summary

- talos_dup uses stable svc #0 with syscall number x8 = 3.
- x0 is the source descriptor; x1 through x5 are reserved and must be zero.
- Success returns the new descriptor number in x0 after copying the source
  DescriptorEntry into the lowest free slot in the current
  ProcessDescriptorStore owner table.
- Invalid, empty, closed, out-of-range, missing-owner, or unknown-owner source
  descriptors return -EBADF.
- Full descriptor tables return -EMFILE without mutation.
- Nonzero reserved arguments return -EINVAL without mutation.
- The duplicated entry preserves access mode, flags, DescriptorObject kind, and
  DescriptorObject reference. No open-file-description reference count, object
  finalizer, file offset sharing, dup2/fcntl, or close-on-exec application is
  accepted.

## Evidence

- static inspection: git status --short before edits was clean.
- static source review: inspected src/posix.rs, src/syscall.rs, the accepted
  close/dup/read syscall source inventory, descriptor lifetime and close
  contract, close syscall contract, and Pi 5 close syscall proof closeout.
- static documentation diff: added the dup syscall contract, linked it from
  docs/src/SUMMARY.md, updated roadmap/decision-log frontier text, and added
  this task record.
- whitespace inspection: git diff --check passed.
- documentation: mdbook build passed.

## Accepted Deferred Surfaces

Read syscall behavior, stdin/read object policy, QEMU dup/read smoke, Pi 5
physical dup/read proof, process loading, fork/spawn/exec, descriptor
inheritance across exec, close-on-exec application, dup2/dup3/fcntl, process
exit teardown, open-file-description reference counting, object finalizers,
file-offset sharing, VFS/filesystem lookup, regular files, directories, pipes,
sockets, device registries, TTY blocking/readiness, EOF, nonblocking flags,
wait queues, signals, restart semantics, path copying, argv/envp loading,
per-thread errno storage, libc/Rust std stdio, shell behavior, networking, SSH,
RP1/PCIe, UART interrupt ownership, DMA/cache-driver policy, and full POSIX
descriptor readiness remain blocked.

## Recommended Next Task

The next bounded Milestone 7.4 task should be
phase7-dup-syscall-core-20260529.

That task should implement only target-independent dup syscall dispatch and
focused unit tests for the accepted contract. It should not run QEMU, acquire
hardwareTestLock, publish a boot archive, run Pi 5 hardware, implement read, or
advance process loading, VFS/filesystem, shell, networking, SSH, RP1/PCIe, UART
interrupt ownership, DMA/cache-driver policy, or a phase transition.

## Acceptance

Accepted on 2026-05-29 after git diff --check and mdbook build passed.
