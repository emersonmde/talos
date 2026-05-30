# Phase 7 Read And Stdin Contract

Task: phase7-read-stdin-contract-20260529
Owner: worker
Status: accepted
Date: 2026-05-29
Milestone: Phase 7.4, File Descriptor Table

## Goal

Define the first bounded user-visible talos_read/stdin contract after the
accepted read/stdin source inventory identified source owners and gaps.

## Scope

- Added `docs/src/project/phase7-read-stdin-contract.md`.
- Fixed talos_read as syscall number 4 with x0 fd, x1 destination pointer,
  x2 requested count, x3 through x5 reserved zero, and x0 bytes-read/0 EOF or
  negative errno return encoding.
- Chose fixed proof input as the first bounded stdin source, with immediate
  readiness, proof-buffer short reads, 0 EOF, and no runtime-console0/TTY/
  filesystem/hardware input claim.
- Defined ProcessDescriptorStore lookup, StdioInput object matching,
  copy_to_user failure ordering, reserved-register behavior, errno cases, and
  blocked surfaces.
- Recommended exactly one next bounded task:
  `phase7-read-stdin-core-20260529`.

## Changed Files

- `docs/src/SUMMARY.md`
- `docs/src/decisions/README.md`
- `docs/src/project/phase7-read-stdin-contract.md`
- `docs/src/roadmap.md`
- `tasks/2026-05-29-phase7-read-stdin-contract.md`

## Evidence

- Accepted dependency:
  `phase7-read-stdin-source-inventory-20260529` at
  `c00267891b928e53b25c8ebdbe6a6a0dc549e0ae`.
- Contract document:
  `docs/src/project/phase7-read-stdin-contract.md`.
- Reserved-register, errno, EOF/readiness, and copy-out summary:
  talos_read uses x8 = 4, x0 fd, x1 destination, x2 count, x3 through x5
  reserved zero; returns copied byte count, 0 at bounded EOF, -EBADF for
  invalid/non-readable/missing descriptor owners, -ENOTSUP for readable
  non-StdioInput or unavailable proof source, -EFAULT for copy-out/over-limit
  failures, -EINVAL for reserved-register or malformed kernel-side state, and
  -ENOSYS for unaccepted syscall numbers.
- Blocked-surface summary:
  read implementation, QEMU/Pi 5 read proof, runtime-console0/TTY/hardware
  stdin, process loading, VFS/filesystem, shell, networking, SSH, object
  finalization, RP1/PCIe, UART interrupt ownership, DMA/cache-driver policy,
  and full POSIX descriptor readiness remain blocked.
- Static inspection:
  `git status --short` before edits was clean.
- Whitespace inspection:
  `git diff --check` passed.
- Documentation:
  `mdbook build` passed.
- Staged whitespace inspection:
  `git diff --cached --check` passed before commit.

## Deferred Work

Target-independent read/stdin implementation, QEMU read/stdin smoke plan/core,
Pi 5 read/stdin proof, runtime-console0-backed stdin, TTY stdin, process
loading, VFS/filesystem behavior, local shell, networking, SSH, object
finalization, descriptor lifetime beyond accepted write/close/dup/read proof
source behavior, blocking/readiness, signals, restart semantics, RP1/PCIe,
UART interrupt ownership, DMA/cache-driver policy, and full POSIX descriptor
readiness remain blocked.

## Result

Accepted as the documentation-only read/stdin contract. The next bounded task
should be `phase7-read-stdin-core-20260529`, scoped to target-independent
talos_read/stdin implementation and focused tests before any QEMU or Pi 5
runtime proof work.
