# Phase 7 Descriptor Syscall Source Inventory

Task: phase7-descriptor-syscall-source-inventory-20260529
Status: accepted

## Scope

This documentation-only task inventoried descriptor syscall source boundaries
after the accepted Pi 5 pointer-copy proof closeout. It mapped descriptor table
operations, syscall argument extraction, copy helper use, runtime console/TTY
boundaries, return/error encoding, task/process ownership gaps, and evidence
ownership before any descriptor syscall contract or implementation.

It did not change Rust or assembly behavior, contract descriptor syscalls, run
QEMU, run Pi 5 hardware, publish a boot archive, acquire hardwareTestLock, add
process loading, VFS/filesystem behavior, shell behavior, networking, SSH,
RP1/PCIe, UART interrupt ownership, or DMA/cache-driver policy.

## Source Files Reviewed

- src/posix.rs: descriptor table, descriptor entries, descriptor object kinds,
  access checks, copy_from_user/copy_to_user, user-memory validation, and POSIX
  error vocabulary.
- src/syscall.rs: stable svc #0 vocabulary, x8 syscall numbers, x0-through-x5
  arguments, negative errno returns, scalar dispatch, and proof-only
  talos_copy_probe quarantine.
- src/arch/aarch64/exceptions.rs: lower-AArch64 svc #0 routing, saved-frame
  argument capture, x0 return mutation, and frame preservation rules.
- src/runtime_console.rs: runtime-console0 write and input-poll facade
  outcomes.
- src/tty.rs: TTY line discipline, input outcomes, control events, and polling
  receive vocabulary.
- src/scheduler.rs: TaskId, Task, ProcessOwnerId, process-owner placeholder,
  and production scheduler ownership boundaries.
- Existing accepted docs and task records for descriptor tables, syscall ABI,
  copy helpers, pointer-copy smoke/proof, and Pi 5 pointer-copy closeout.

## Recommendation

The next bounded task should be phase7-descriptor-syscall-contract-20260529,
scoped to a stdout/stderr descriptor write contract. The contract should define
one stable write-style syscall boundary for fd 1 and fd 2, user pointer and
length arguments, copy_from_user use, descriptor-table lookup and write-access
checks, runtime-console0 as the first backing object, exact x0 byte-count and
negative errno returns, proof evidence level, and blocked deferred work.

stdin/read, close, dup, pipes, regular files, VFS/filesystem paths, process
loading, shell behavior, networking, SSH, live process-owned address spaces,
blocking/readiness, signals, and restart semantics remain blocked until later
explicit contracts.

## Validation

- static inspection: git status --short before documentation edits showed a
  pre-existing docs/src/roadmap.md working-tree edit, which was preserved.
- static documentation diff: added
  docs/src/project/phase7-descriptor-syscall-source-inventory.md, linked it
  from docs/src/SUMMARY.md, updated roadmap current status and Phase 7.3
  progress, updated the decision log, and added this task record.
- whitespace inspection: git diff --check passed.
- documentation: mdbook build passed.
- Rust fmt/tests, QEMU runs, and Pi 5 hardware runs were not required because
  this inventory changes only Markdown documentation and durable worker state.
