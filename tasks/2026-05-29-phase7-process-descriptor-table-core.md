# Phase 7 Process Descriptor Table Core

Task: phase7-process-descriptor-table-core-20260529
Status: accepted

## Scope

This implementation task added only the target-independent Milestone 7.4
process-owned descriptor-table owner/attachment/lookup boundary required by
the accepted process descriptor table contract. It did not add live syscall
routing through the process-owned table, QEMU execution, Pi 5 hardware
execution, boot archive publication, hardwareTestLock acquisition, close,
dup, read syscalls, PID allocation, fork/spawn/exec, process loading,
VFS/filesystem behavior, shell behavior, networking, SSH, RP1/PCIe, UART
interrupt ownership, DMA/cache-driver policy, or a full POSIX descriptor
claim.

## Implementation Notes

- \`src/posix.rs\` adds \`ProcessDescriptorOwner\`, a ProcessOwnerId-backed owner
  record holding exactly one \`DescriptorTable\`.
- \`src/posix.rs\` adds \`ProcessDescriptorStore\`, a bounded owner store for
  creating inherited-stdio owners and resolving immutable or mutable current
  descriptor-table borrows.
- Missing current owner, unknown owner, or missing table lookup maps to
  \`PosixError::BadDescriptor\` for the first descriptor-syscall slice.
- Duplicate owner creation maps to \`PosixError::InvalidArgument\`; full owner
  stores and inherited-stdio tables that cannot reserve fd 0, fd 1, and fd 2
  preserve \`PosixError::TooManyOpenFiles\`.

## Evidence

- Accepted contract commit:
  \`adc0ed9ea37fe35b0c45dd19666ba68fe8546187\`.
- Implementation diff summary:
  \`src/posix.rs\` adds the process descriptor owner/store API and focused
  no_std unit tests.
- Unit tests:
  \`cargo -Zjson-target-spec test\` passed with 222 no_std tests.
- Formatting:
  \`cargo fmt --all -- --check\` passed after formatting.
- Static inspection:
  \`git diff --check\` passed.
- Documentation:
  \`mdbook build\` passed.

## Deferred Work

The QEMU/substitute proof that talos_write fd 1/fd 2 routes through the
process-owned table remains blocked behind an explicit smoke plan/core task.
Pi 5 physical proof, live syscall dispatch integration, close/dup/read
syscalls, stdin/read behavior, PID allocation, fork/spawn/exec, process
loading, VFS/filesystem, shell, networking, SSH, RP1/PCIe, UART interrupt
ownership, DMA/cache-driver policy, and full POSIX descriptor claims remain
blocked.

## Next Task

The next bounded task is
\`phase7-qemu-process-descriptor-stdio-smoke-plan-20260529\`, scoped to a
documentation-only QEMU/substitute process-owned stdio smoke plan before any
QEMU run or physical claim.
