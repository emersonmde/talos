# Phase 7 Descriptor Lifetime And Close Contract

Task: phase7-descriptor-lifetime-close-contract-20260529
Owner: worker
Status: accepted
Date: 2026-05-29
Milestone: Phase 7.4, File Descriptor Table

## Goal

Define the descriptor lifetime and close-semantics contract for process-owned
descriptor tables before any syscall implementation.

## Scope

- Added the documentation contract at
  docs/src/project/phase7-descriptor-lifetime-close-contract.md.
- Defined exact supported table-local close behavior, process-owned mutable
  lookup through ProcessDescriptorStore, EBADF error cases, dup/reuse
  interaction, and open-file-description reference-count vocabulary.
- Identified target-independent code and focused unit-test evidence required
  for the next implementation task.
- Updated roadmap, SUMMARY, and the decision log.
- Recommended exactly one next bounded task:
  phase7-descriptor-close-core-20260529.

## Non-Goals

- No Rust or assembly implementation changes.
- No QEMU run, Pi 5 hardware run, archive publication, or hardwareTestLock
  acquisition.
- No close/dup/read syscall numbers, lower-EL close/dup/read ABI, process
  loading, VFS/filesystem, shell, networking, SSH, RP1/PCIe, UART interrupt
  ownership, DMA/cache-driver policy, physical descriptor close claim, or full
  POSIX descriptor readiness claim.

## Evidence

- static inspection: git status --short before documentation edits was clean.
- static source review: inspected the accepted descriptor lifetime source
  inventory, process descriptor table contract/closeout, descriptor syscall
  contract, src/posix.rs close/dup/process-store surfaces, src/syscall.rs, and
  retained descriptor evidence references.
- static documentation diff: added
  docs/src/project/phase7-descriptor-lifetime-close-contract.md and this task
  record; updated docs/src/SUMMARY.md, docs/src/roadmap.md, and
  docs/src/decisions/README.md.
- whitespace inspection: git diff --check passed.
- documentation: mdbook build passed.
- Rust fmt/tests, QEMU runs, and Pi 5 hardware runs were not required because
  this task changes only Markdown documentation and durable worker state.

## Result

Accepted as the Milestone 7.4 descriptor lifetime and close-semantics contract.
The next bounded task should be phase7-descriptor-close-core-20260529, a
target-independent implementation task for process-owned close helper behavior
and focused unit tests.

Close/dup/read syscalls, process loading, VFS/filesystem, path copying, shell,
networking, SSH, RP1/PCIe, UART interrupt ownership, DMA/cache-driver policy,
Pi 5 physical close/dup/read claims, open-file-description finalization, and
full POSIX descriptor readiness remain blocked.
