# Phase 8 Read-Only Initramfs/VFS Core

Status: accepted as the target-independent Milestone 8.1 read-only
initramfs/VFS object core after the accepted
[Phase 8 Read-Only Initramfs/VFS Contract](../docs/src/project/phase8-readonly-initramfs-vfs-contract.md)
and
[Phase 8 Read-Only Initramfs/VFS Smoke Plan](../docs/src/project/phase8-readonly-initramfs-vfs-smoke-plan.md).

## Scope

- Added src/initramfs.rs as the small target-independent owner for immutable
  initramfs/VFS nodes, deterministic fixture metadata, lookup, regular-file
  open-file descriptions, file offsets, and byte reads.
- Registered the module from src/main.rs only; no production boot scenario,
  lower-EL syscall routing, descriptor syscall behavior, QEMU smoke, Pi 5
  hardware run, boot archive publication, or hardwareTestLock action was
  added.
- Preserved the accepted path and user-copy boundaries by using
  normalize_path() and copy_to_user() from src/posix.rs instead of duplicating
  those rules.

## Changed Source And Test Files

- src/initramfs.rs
- src/main.rs

## Accepted Behavior

- The stable fixture name is phase8-readonly-initramfs-vfs-v1.
- The root directory has deterministic entries for etc, bin, empty, and dir.
- The fixture provides:
  - /etc/banner.txt with "Talos initramfs fixture\n"
  - /bin/init with "not-executable-yet\n" as regular-file data only
  - /empty as a zero-length regular file
  - /dir/nested.txt with "nested fixture\n"
- Lookup supports normalized absolute paths and first-process
  current-directory-relative paths against the initramfs root.
- Regular-file reads copy through copy_to_user(), advance offsets only after a
  successful nonzero copy, return 0 at EOF, and keep zero-length reads
  non-mutating.
- Negative tests cover ENOENT, ENOTDIR, EISDIR, ENAMETOOLONG, EFAULT, EINVAL,
  EBADF, and ENOTSUP at this target-independent boundary through lookup,
  open-file-description, and descriptor fixture helpers.

## Deferred Surfaces

QEMU runtime evidence, Pi 5 hardware proof, boot archive publication,
firmware/TFTP initramfs delivery, descriptor-backed production syscall wiring,
open syscall ABI, directory iteration, seek syscalls, object final release,
ELF/program loading, executable /bin/init behavior, argv/envp setup, process
creation, exec/spawn/wait, shell behavior, writable filesystems, persistent
storage, networking, SSH, RP1/PCIe, UART interrupt ownership, and
DMA/cache-driver policy remain blocked.

## Evidence

- static inspection: git status --short before edits was clean.
- source/test diff: src/initramfs.rs added the target-independent object core
  and focused no_std tests; src/main.rs registered the module.
- formatting: cargo fmt --all -- --check passed.
- unit tests: cargo -Zjson-target-spec test passed with 261 no_std tests.
- whitespace inspection: git diff --check passed.
- documentation: mdbook build passed.
- staged whitespace inspection: git diff --cached --check passed before
  commit.

## Next Action

The next mechanically unblocked task is
phase8-qemu-readonly-initramfs-vfs-smoke-core-20260530, which should add only
the planned QEMU/substitute smoke evidence for this accepted core.
