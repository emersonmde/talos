# Phase 8 Open/Read Initramfs Descriptor Integration

Status: planned

Task: phase8-open-read-initramfs-descriptor-integration-20260603

## Reason

The Phase 10 local command-loop work proved useful serial input/output and
command-context behavior on Raspberry Pi 5, but further fake/kernel-backed
`ls`, `cd`, or `cat` expansion does not materially advance Talos toward a
working POSIX operating system. Future shell-visible behavior must exercise a
real kernel feature through the intended boundary.

## Goal

Connect the accepted read-only initramfs/VFS core to descriptor-backed
POSIX-shaped file I/O so immutable files such as `/etc/banner.txt` and
`/bin/init` can be opened/read through kernel file objects rather than
command-specific string fixtures.

## Scope

- Implement the smallest descriptor-backed regular-file path for the accepted
  immutable initramfs fixture.
- Define or implement the internal open-file-description allocation path needed
  for read-only regular files, including offset behavior and descriptor
  lifetime interactions.
- Expose the path through the narrowest syscall/substitute boundary that moves
  Talos toward POSIX open/read semantics.
- Use existing command-loop behavior only as a consumer/regression after the
  VFS-backed file-object path exists.
- Retain QEMU/substitute evidence for successful file reads, EOF/offset
  behavior, deterministic POSIX-shaped errors, and no-partial-copy/no-partial
  offset mutation.

## Non-Goals

- No new fake command fixtures or cosmetic shell expansion.
- No writable filesystem, persistent storage, block devices, mount namespaces,
  symlinks, or directory mutation.
- No userspace shell execution, exec/spawn/wait, process lifecycle, networking,
  SSH, RP1/PCIe, UART interrupt ownership, or DMA/cache-driver policy.
- No Pi 5 hardware proof until QEMU/substitute evidence proves the feature and
  a smallest physical claim is explicitly needed.

## Acceptance Criteria

- A real VFS regular-file object from the accepted initramfs fixture is attached
  to an open-file description and read through descriptor-backed file I/O.
- Offsets, EOF, zero-length reads, duplicate/shared-offset behavior where
  applicable, and all-or-nothing user-copy behavior match accepted contracts.
- Negative cases preserve deterministic POSIX-shaped errors without descriptor
  leaks or offset mutation.
- QEMU/substitute evidence proves `/etc/banner.txt` and `/bin/init` are
  reachable as regular files through the new file-object path.
- Any touched `cat`/`ls` command behavior is documented as a consumer/regression
  of the VFS-backed path, not as the accepted capability itself.

## Validation Gates

- Repo-standard Rust/no_std tests for initramfs/VFS and descriptor file-object
  behavior.
- QEMU/substitute smoke for descriptor-backed read-only initramfs file I/O.
- `git diff --check`.
- `mdbook build` if docs or mdbook-linked records are touched.
- `git diff --cached --check` before commit.

## Deferred Surfaces

Writable filesystems, persistent storage, block devices, general path
mutation, userspace shell execution, process lifecycle, networking, SSH,
RP1/PCIe, UART interrupt ownership, and DMA/cache-driver policy remain blocked
until explicit feature tasks accept their contracts and validation gates.

## Scheduled Follow-Up Chain

The supervisor state now schedules the drift correction as an explicit
POSIX-backed task chain:

1. `phase8-open-read-initramfs-descriptor-integration-20260603`: connect the
   accepted initramfs/VFS fixture to descriptor-backed file objects and read
   behavior.
2. `phase8-open-read-syscall-surface-20260603`: expose the file-object path
   through the narrowest POSIX-shaped open/read syscall or syscall-substitute
   surface.
3. `phase8-program-loader-from-vfs-file-20260603`: make the accepted program
   loader consume `/bin/init` through the real VFS/file-object path.
4. `phase8-initial-userspace-process-launch-20260603`: advance from
   metadata-only process plans to the smallest real initial userspace
   `/bin/init` launch path, or record the one missing launch prerequisite as a
   bounded blocker task.
5. `phase10-shell-backed-by-userspace-and-vfs-20260603`: return to shell UX
   only after VFS/file I/O and userspace execution exist, so shell-visible file
   behavior consumes real OS layers.

The Talos async worker cron and supervisor audit cron have both been updated to
use this priority. Existing Phase 10 commands are regression/control surfaces,
not the next planned feature path.
