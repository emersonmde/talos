# Phase 8 Program Loader From VFS File

Task ID: phase8-program-loader-from-vfs-file-20260603

Status: accepted

## Scope

- Route `/bin/init` loading through the accepted read-only initramfs VFS
  regular-file boundary.
- Preserve the existing ELF64/AArch64 loader validation, image-plan-only
  output, segment planning, permissions, and deterministic rejection matrix.
- Do not claim process creation, scheduler publication, user-stack setup, or
  userspace execution.

## Implementation

- Added `ReadOnlyInitramfs::read_regular_file_to_kernel`, a kernel-buffer
  reader over the accepted `ReadOnlyFileDescription` file-object offset.
- Updated `plan_phase8_init_image` to open `/bin/init` with
  `ReadOnlyInitramfs::open_regular_file` and copy the file through
  `read_regular_file_to_kernel` before invoking the ELF planner.
- Added `qemu_program_loader_from_vfs_smoke` and
  `scripts/qemu-program-loader-from-vfs-smoke.sh` to retain a transcript that
  names the VFS/file-object source boundary.
- Left `plan_elf64_aarch64_image` available for direct byte-slice mutation
  tests so the loader rejection matrix remains focused and deterministic.

## Findings

- fixed: `plan_phase8_init_image` previously obtained `/bin/init` through
  `regular_file_bytes`, which bypassed the accepted open-file-description
  boundary.
- fixed: the initramfs file-object model had user-copy reads but no
  kernel-buffer read helper for kernel consumers such as the program loader.
- fixed: QEMU/substitute evidence now distinguishes loader validation from the
  loader input source by requiring a `vfs-source` transcript line.
- not-an-issue: direct byte-slice planning remains useful for malformed ELF
  unit tests and QEMU negative cases; it is not the accepted `/bin/init`
  loader entry point.
- deferred: process installation, lower-EL launch, argv/envp stack setup,
  scheduler publication, and userspace execution remain later tasks.

## Evidence

- Source/test evidence: `src/initramfs.rs` adds kernel file-object reads and
  tests ELF magic/offset/EOF behavior; `src/program_loader.rs` routes
  `plan_phase8_init_image` through the VFS file object and tests source-error
  propagation.
- QEMU/substitute evidence:
  `tasks/evidence/2026-06-03-qemu-program-loader-from-vfs-file/qemu-program-loader-from-vfs-smoke.log`
  records `/bin/init` as a VFS regular file, `bytes=516`, `eof=true`, and
  `qemu-program-loader-from-vfs-smoke: PASS`.
- Hardware: not run; this task makes no Pi 5 physical claim.

## Validation

- `cargo fmt --all`: passed.
- `cargo fmt --all -- --check`: passed.
- `cargo -Zjson-target-spec check --quiet`: passed.
- `cargo -Zjson-target-spec test --quiet`: passed with 375 Talos no_std
  tests.
- `scripts/qemu-program-loader-from-vfs-smoke.sh`: passed and retained the
  VFS-source transcript.
- `scripts/qemu-program-loader-smoke.sh`: passed as retained loader-regression
  coverage.
- `git diff --check`: passed.
- `/home/node/.cargo/bin/mdbook build`: passed; warning only: large search
  index.
- `git diff --cached --check`: recorded before commit.

## Deferred

Process installation, scheduler publication, lower-EL execution, argv/envp
stack construction, shell behavior backed by userspace, Pi 5 hardware proof,
writable filesystem support, networking, SSH, RP1/PCIe, UART interrupt
ownership, and DMA/cache policy.

## Acceptance

- Status: accepted.
- Commit: recorded in durable supervisor state.
