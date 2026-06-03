# Phase 8 Open/Read Syscall Surface

Task ID: phase8-open-read-syscall-surface-20260603

## Scope

- Add the narrow POSIX-shaped open syscall-substitute surface for read-only
  initramfs regular files.
- Connect read on filesystem-backed descriptors to the accepted
  descriptor-backed initramfs file-object path.
- Prove /etc/banner.txt and /bin/init through the syscall-substitute
  boundary with deterministic error behavior.

## Implementation

- Added TALOS_OPEN_SYSCALL (5) and SyscallNumber::TalosOpen.
- Added dispatch_process_descriptor_with_initramfs, which dispatches the
  existing write/close/dup/read vocabulary plus the new initramfs-backed open
  boundary.
- open copies an explicit-length user path through copy_from_user, rejects
  nonzero flags/reserved registers, and allocates a read-only regular-file
  descriptor through ReadOnlyInitramfs::open_regular_descriptor.
- read now distinguishes stdio input from regular-file descriptors. Stdio
  reads keep the existing fixed-stdin substitute behavior; regular-file reads
  use ReadOnlyInitramfs::read_descriptor with shared offsets and all-or-nothing
  user-copy semantics.
- Added the qemu_open_read_syscall_surface_smoke QEMU/substitute scenario and
  scripts/qemu-open-read-syscall-surface-smoke.sh.

## Findings

- fixed: scalar dispatch knew about descriptor read/write/close/dup but had no
  path-taking open number, so filesystem-backed descriptors could not be
  obtained through a syscall-shaped boundary.
- fixed: read was limited to fixed stdin in the process-descriptor syscall
  dispatch path; regular-file descriptors now route to the initramfs file-object
  path when the initramfs context is explicitly supplied.
- fixed: error tests now cover invalid flags, bad user path pointer, directory
  open, bad read descriptor, and read copy fault without offset or destination
  mutation.
- not-an-issue: full POSIX openat, writable flags, mode creation, cwd storage,
  and userspace trap entry remain deliberately deferred by task scope.

## Evidence

- Source/test evidence: src/syscall.rs, src/target/qemu_virt.rs, build.rs,
  src/main.rs, and scripts/qemu-open-read-syscall-surface-smoke.sh.
- Unit tests: cargo -Zjson-target-spec test --quiet passed with 373 Talos
  no_std tests.
- QEMU/substitute: scripts/qemu-open-read-syscall-surface-smoke.sh passed and
  retained tasks/evidence/2026-06-03-qemu-open-read-syscall-surface/qemu-open-read-syscall-surface-smoke.log.
- QEMU/substitute evidence proves:
  - open rejects invalid flags, bad user path pointers, and directories without
    descriptor leaks.
  - /etc/banner.txt opens as fd 3, read returns fixture contents, copy faults
    preserve offset, and EOF returns 0.
  - /bin/init opens as fd 4 and read returns ELF magic 7f454c46.
  - bad read descriptors return -EBADF.
- Hardware: not run; this task required QEMU/substitute evidence only and did
  not make a Pi 5 physical claim.

## Validation

- cargo fmt --all: passed.
- cargo -Zjson-target-spec test --quiet: passed with 373 no_std tests.
- scripts/qemu-open-read-syscall-surface-smoke.sh: passed.
- git diff --check: passed.
- mdbook build: passed; warning only: large search index.
- git diff --cached --check: passed.

## Deferred

- Full POSIX openat, flags/mode handling, cwd/process path context, filesystem
  writes, directory file descriptors, userspace trap execution, program loading
  from VFS, process launch, shell behavior backed by userspace, networking, SSH,
  RP1/PCIe, UART interrupt ownership, and DMA/cache policy.

## Acceptance

- Status: accepted.
- Commit: this acceptance commit; exact hash is recorded in supervisor state.
