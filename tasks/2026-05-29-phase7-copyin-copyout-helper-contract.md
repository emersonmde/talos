# Phase 7 Copy-In/Copy-Out Helper Contract

Task: phase7-copyin-copyout-helper-contract-20260529
Status: accepted

## Scope

This documentation-only task defined the first target-independent
copy-in/copy-out helper contract after accepted QEMU and Pi 5 production
syscall routing proof. It did not change Rust, assembly, boot scenarios, QEMU
scripts, Pi 5 hardware state, boot archives, descriptor I/O, process loading,
VFS/filesystem behavior, shell behavior, networking, SSH, RP1/PCIe, UART
interrupt ownership, or DMA/cache-driver policy.

## Accepted Contract

- Helper inputs: accepted user mappings, user virtual start address, bounded
  byte length, kernel buffer, and direction-specific read/write access kind.
- Helper outputs: full-length success count, PosixError::Fault for
  user-memory boundary failures, and optional PosixError::InvalidArgument only
  for malformed kernel-side helper use.
- Validation order: kernel length/limit check, UserRange construction, whole
  range permission validation, byte copy, then success count.
- EFAULT failures: null guard, kernel range, wraparound, length limit,
  unmapped gap, no-access mapping, permission mismatch, guard/kernel/MMIO/DTB
  or bootstrap-table overlap.
- Partial-copy policy: all-or-nothing; no short successful copy and no
  destination mutation after validation failure.
- Fault policy: helper validation failures are recoverable EFAULT before side
  effects; direct lower-EL data or instruction aborts remain future
  process-fatal classifications until a later policy exists.

## Evidence

- static documentation diff summary: added
  docs/src/project/phase7-copyin-copyout-helper-contract.md; linked it from
  docs/src/SUMMARY.md; updated docs/src/roadmap.md; updated
  docs/src/decisions/README.md; and added this task record.
- static inspection: git diff --check passed.
- documentation: mdbook build passed.

## Next Action

The recommended next bounded implementation task is
phase7-copyin-copyout-helper-core-20260529. Supervisor planning is required
before promotion because the durable queue currently names only this contract
task. Pointer-taking syscalls, descriptor I/O, process loading,
VFS/filesystem, shell, networking, and SSH remain blocked.
