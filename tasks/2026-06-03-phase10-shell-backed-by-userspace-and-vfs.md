# Phase 10 Shell Backed By Userspace And VFS

Task ID: phase10-shell-backed-by-userspace-and-vfs-20260603

Status: accepted

## Scope

- Demonstrate one shell-visible file operation through the accepted VFS,
  descriptor, syscall-substitute, and userspace-memory copy boundaries.
- Keep existing Phase 10 serial command-loop behavior as regression/control
  evidence only.
- Do not add fake/kernel-backed command expansion as accepted OS progress.
- Do not run Pi 5 hardware, publish boot archives, or acquire
  hardwareTestLock.

## Implementation

- `cat /etc/banner.txt` and `cat banner.txt` from `/etc` now read the banner
  bytes through `TalosOpen` and `TalosRead` via
  `dispatch_process_descriptor_with_initramfs`.
- `DescriptorBackedLocalCommandIo` now owns a bounded read-only initramfs file
  description table for shell file reads. The shell read path opens a regular
  file descriptor, reads into a userspace-memory buffer through the syscall
  surface, copies the result back to the command output buffer, and closes the
  descriptor while removing the file-description slot.
- The command-loop status marker remains compatible with existing smoke
  harnesses, but now records `+vfs-syscall-cat` so retained kernel-backed
  fixtures are visibly regression controls.

## Findings

- fixed: `cat /etc/banner.txt` previously read directly from the initramfs
  fixture through `regular_file_bytes`, bypassing the accepted descriptor and
  syscall read surface.
- fixed: A repeated shell-visible file read could not be accepted until the
  command-loop owned a regular-file description table and cleaned up the opened
  descriptor/file-description pair after each `cat`.
- not-an-issue: Existing `help`, `status`, `stdio`, `pwd`, `echo`, `ls`, and
  `cd` command behavior remains a regression/control surface, not new OS
  progress.
- deferred: Broader shell command execution, external command lookup, argv/envp
  process ABI, pipes, redirection, writable filesystem, and Pi 5 hardware proof
  remain out of scope for this bounded task.

## Evidence

- Static inspection:
  - `src/local_command_loop.rs` routes banner `cat` through the syscall-backed
    initramfs read helper and no longer uses `regular_file_bytes` for shell
    output.
- Unit/source evidence:
  - Added
    `local_command_loop_cats_banner_through_reusable_vfs_syscall_descriptor`,
    which runs `cat /etc/banner.txt` twice through the same descriptor-backed
    local command I/O and observes both banner outputs.
- QEMU/substitute:
  - `tasks/evidence/2026-06-03-phase10-shell-backed-by-userspace-and-vfs/qemu-local-shell-vfs-cat-smoke.log`
    shows `cat /etc/banner.txt`, visible `Talos initramfs fixture`, descriptor
    backed stdio markers, next-prompt readiness, final
    `qemu-local-cat-banner-complete` classification, and PASS.
- Hardware:
  - Not run. This task made no physical claim and did not acquire
    hardwareTestLock.

## Validation

- `cargo fmt --all -- --check`: passed.
- `cargo -Zjson-target-spec check --quiet`: passed for the default
  `aarch64-talos-virt` target.
- `TALOS_BOOT_SCENARIO=rpi5_local_cat_banner cargo -Zjson-target-spec check --target targets/aarch64-talos-rpi5-bcm2712.json --quiet`:
  passed.
- QEMU/substitute shell/VFS integration smoke:
  `qemu-local-cat-banner` PASS with retained task-specific transcript.
- `git diff --check`: passed.
- `/home/node/.cargo/bin/mdbook build`: passed; warning only: large search
  index.
- `git diff --cached --check`: passed before commit.

## Validation Gap

- A repo-wide `cargo -Zjson-target-spec test --quiet` run still reaches the
  custom no_std test binary and fails later at
  `process_install::tests::derives_metadata_only_install_plan_from_fixture`
  with source digest `0xf4a6cc15f4d94461` instead of
  `0x3892eed223900c65`. The touched files are limited to the local command
  shell path and do not alter `/bin/init`, the program loader, or process
  install code. The listed acceptance gate for this task is the shell/VFS QEMU
  integration smoke plus source/build checks; the digest anomaly remains a
  residual test-harness/system issue for supervisor follow-up if it persists.

## Acceptance

Accepted at: 2026-06-03T10:32Z

Commit: recorded in durable supervisor state after commit.
