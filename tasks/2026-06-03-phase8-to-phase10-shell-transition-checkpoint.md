# Phase 8 To Phase 10 Shell Transition Checkpoint

Task ID: phase8-to-phase10-shell-transition-checkpoint-20260603

Status: accepted

## Scope

- Record the accepted POSIX-backed chain from descriptor-backed initramfs file
  I/O through the first real `/bin/init` userspace launch signal.
- Confirm that the next Phase 10 shell task may proceed only as a consumer of
  VFS, descriptors, syscalls, and userspace behavior.
- Do not change runtime code, boot archives, Pi 5 hardware state, or the
  hardware test lock.

## Accepted Chain

1. `phase8-open-read-initramfs-descriptor-integration-20260603`
   - Commit: `5bdfcb21d410fe6fd4fbc2e6eb277d5f62a4e568`
   - Evidence:
     `tasks/evidence/2026-05-30-qemu-readonly-initramfs-vfs-smoke-core/qemu-readonly-initramfs-vfs-smoke.log`
   - Accepted frontier: immutable initramfs regular files are attached to
     descriptor-backed open-file descriptions and read through the file-object
     path.
2. `phase8-open-read-syscall-surface-20260603`
   - Commit: `2f9e00d05c1b0f946306fe018ad2caa2f744d81b`
   - Evidence:
     `tasks/evidence/2026-06-03-qemu-open-read-syscall-surface/qemu-open-read-syscall-surface-smoke.log`
   - Accepted frontier: a path-taking read-only open syscall substitute creates
     filesystem-backed descriptors, and read returns file contents, EOF, and
     deterministic POSIX-shaped errors.
3. `phase8-program-loader-from-vfs-file-20260603`
   - Commit: `5556415874b712f11df9a22f32fdedab189f4b1f`
   - Evidence:
     `tasks/evidence/2026-06-03-qemu-program-loader-from-vfs-file/qemu-program-loader-from-vfs-smoke.log`
   - Accepted frontier: the program loader sources `/bin/init` through the
     read-only initramfs VFS/file-object boundary before planning ELF segments.
4. `phase8-initial-userspace-process-launch-20260603`
   - Commit: `b879d1af0018c640f5d8f2451f2cf61c8fd8da3d`
   - Evidence:
     `tasks/evidence/2026-06-03-qemu-initial-userspace-process-launch/qemu-initial-userspace-process-launch-smoke.log`
   - Accepted frontier: QEMU/substitute execution enters the VFS-backed
     `/bin/init` ELF text at EL0 and observes the lower-AArch64 SVC marker from
     the init entry path.

## Findings

- fixed: The supervisor queue dependency text for the descriptor-backed
  initramfs task used `5bdfcb25820789196322af140dfc60493976d6067`, but the
  repository commit is `5bdfcb21d410fe6fd4fbc2e6eb277d5f62a4e568`.
- fixed: The supervisor queue dependency text for the open/read syscall
  surface used `2f9e00d35872976e52b7eec2837bd47f6cb81813`, but the repository
  commit is `2f9e00d05c1b0f946306fe018ad2caa2f744d81b`.
- not-an-issue: The program-loader and initial-userspace-launch dependency
  commits matched the repository.
- not-an-issue: The retained QEMU/substitute evidence files exist and end with
  the expected PASS classifications.
- deferred: Phase 10 still needs a bounded implementation task to make a
  shell-visible file operation consume the accepted layers. This checkpoint
  only authorizes that direction; it does not implement shell behavior.

## Phase 10 Rule

`phase10-shell-backed-by-userspace-and-vfs-20260603` is allowed to proceed
only because the accepted Phase 8 chain now provides real VFS, descriptor,
syscall-substitute, loader, and userspace-launch boundaries. Any shell-visible
file operation accepted after this checkpoint must consume those layers.
Expanding fake or kernel-backed command fixtures is not acceptable progress.
Existing command-loop fixtures may remain as regression or control surfaces.

## Validation

- Static review: accepted task records, commit IDs, and retained evidence paths
  inspected.
- Evidence inspection: the retained QEMU/substitute logs for descriptor-backed
  initramfs reads, open/read syscall surface, program-loader-from-VFS, and
  initial userspace launch all end in PASS.
- Runtime code, boot archives, Pi 5 hardware, and hardwareTestLock: untouched.
- `git diff --check`: passed.
- `/home/node/.cargo/bin/mdbook build`: passed; warning only: large search
  index.
- `git diff --cached --check`: passed.

## Acceptance

Accepted at: 2026-06-03T10:02Z

Commit: recorded in durable supervisor state after commit.
