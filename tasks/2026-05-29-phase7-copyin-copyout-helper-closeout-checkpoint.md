# Phase 7 Copy-In/Copy-Out Helper Closeout Checkpoint

Task: phase7-copyin-copyout-helper-closeout-checkpoint-20260529
Status: accepted

## Scope

This checkpoint reconciles the accepted copy-in/copy-out helper contract and
the accepted target-independent helper-core implementation. It updates the
roadmap and decision log to distinguish the accepted pure byte-copy helper
boundary from later lower-EL pointer-taking syscall, descriptor I/O, process
loading, filesystem, shell, networking, and SSH work.

No Rust or assembly implementation changed for this checkpoint. No QEMU run,
Pi 5 hardware run, hardwareTestLock acquisition, boot archive publishing,
pointer-taking syscall, descriptor I/O, process loading, filesystem, shell,
networking, SSH, demand paging, signal/restart semantics, or fault-table
recovery was added.

## Accepted Inputs

- helper contract:
  docs/src/project/phase7-copyin-copyout-helper-contract.md
- helper-core task record:
  tasks/2026-05-29-phase7-copyin-copyout-helper-core.md
- helper-core commit:
  b675a6f10fbb3e91781f98bd0ae63290ee4e967c

## Closeout Summary

The accepted helper core adds copy_from_user and copy_to_user in src/posix.rs.
The helpers validate the whole user range before copying, use
UserAccessKind::Read for copy-in and UserAccessKind::Write for copy-out,
return exact requested lengths on success, map user-boundary failures to
PosixError::Fault, reserve PosixError::InvalidArgument for malformed
kernel-side helper use, and preserve all-or-nothing failure behavior.

The accepted helper-core unit coverage includes successful copy-in,
successful copy-out, valid zero-length copy, null guard, kernel range,
wraparound, DEFAULT_USER_COPY_LIMIT, unmapped gaps, no-access mappings,
read/write permission mismatches, backing-storage gaps, short kernel
destination/source EINVAL, and all-or-nothing destination preservation.

## Deferred Work

Pointer-taking syscall routing, descriptor read/write/close/dup, runtime
console or TTY-backed stdio, path copying, argv/envp loading, process loading,
VFS/filesystem behavior, shell behavior, networking, SSH, RP1/PCIe, UART
interrupt ownership, DMA/cache-driver policy, demand paging, copy-on-write,
signal/restart semantics, and lower-EL fault-table recovery remain blocked.

## Recommendation

The next bounded task should be
phase7-pointer-taking-syscall-source-inventory-20260529. It should inventory
source owners and gaps for the first lower-EL pointer-taking syscall smoke
boundary before any implementation or QEMU smoke plan is promoted.

## Evidence

- static inspection: reviewed helper-core commit
  b675a6f10fbb3e91781f98bd0ae63290ee4e967c and helper-core task record.
- documentation diff: added
  docs/src/project/phase7-copyin-copyout-helper-closeout-checkpoint.md, linked
  it from docs/src/SUMMARY.md, updated docs/src/roadmap.md and
  docs/src/decisions/README.md, and added this task record.
- static inspection: git diff --check passed.
- documentation: mdbook build passed.

## Acceptance

Accepted; final commit hash is recorded in durable supervisor state for this
task after commit creation.
