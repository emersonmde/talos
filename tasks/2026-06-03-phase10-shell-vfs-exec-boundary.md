# Phase 10 Shell VFS Exec Boundary

Task: phase10-shell-vfs-exec-boundary-20260603

Status: accepted

## Goal

Make one shell-visible command attempt execution of a VFS-backed userspace
program through the accepted VFS/open/read, loader, process-install,
address-space, materialization, initial-stack, and launch-boundary layers.

## Changed Files

- src/local_command_loop.rs
- src/target/qemu_virt.rs
- build.rs
- scripts/qemu-local-serial-command-loop-smoke.sh
- scripts/qemu-local-shell-vfs-exec-smoke.sh
- docs/src/roadmap.md
- tasks/evidence/2026-06-03-phase10-shell-vfs-exec-boundary/

## Findings And Dispositions

- fixed: The shell had VFS-backed file reads, but no command path exercised the
  accepted userspace launch chain from a shell-visible request. Added
  exec /bin/init, which reads /bin/init through the descriptor-backed
  TalosOpen/TalosRead path, parses those bytes with the program loader, and
  builds the accepted process-install, address-space, descriptor-image
  materialization, initial process launch, and initial user stack records.
- fixed: Negative exec behavior was previously indistinguishable from generic
  unknown-command handling. Added deterministic exec-not-found and
  exec-not-executable responses for missing and regular non-executable paths.
- fixed: The QEMU local command-loop harness had no task-specific shell exec
  scenario. Added qemu_local_shell_vfs_exec and
  scripts/qemu-local-shell-vfs-exec-smoke.sh with retained transcript checks.
- deferred: exec /bin/init is not a process lifecycle implementation and does
  not publish a runnable task, wait for exit, provide argv/envp/PATH, or enter
  lower EL directly from the interactive shell. It records the accepted
  launch-boundary-equivalent observation for the shell command path; broader
  lifecycle/status handoff remains the next explicit task.
- not-an-issue: Existing help/status output changed to include exec and the
  expanded builtin boundary string. Older builtins remain regression/control
  surfaces rather than new fake OS progress.

## Accepted Boundary

exec /bin/init now demonstrates:

- shell-visible request: exec /bin/init;
- source bytes from the read-only initramfs/VFS descriptor syscall path, not an
  embedded shell fixture;
- /bin/init loader identity, source length/digest, entry, and segment count;
- process install, address-space model, descriptor-image materialization,
  initial launch plan, and initial user stack lineage;
- explicit launch-boundary-equivalent marker:
  talos: exec-signal lower-aarch64-svc-launch-boundary-equivalent;
- deterministic negative behavior for exec /missing and exec /etc/banner.txt;
- retained VFS cat regression after the exec command.

## Evidence

- Source/unit evidence: cargo -Zjson-target-spec test --quiet passed
  repo-wide with the new
  local_command_loop_execs_init_through_vfs_launch_boundary and
  local_command_loop_rejects_missing_and_non_executable_exec_targets tests.
- QEMU/substitute shell exec transcript:
  tasks/evidence/2026-06-03-phase10-shell-vfs-exec-boundary/qemu-local-shell-vfs-exec-smoke.log
  with qemu-local-shell-vfs-exec-complete and qemu-local-shell-vfs-exec: PASS.
- QEMU/substitute VFS cat regression:
  tasks/evidence/2026-06-03-phase10-shell-vfs-exec-boundary/qemu-local-cat-banner-regression.log
  with qemu-local-cat-banner-complete and qemu-local-cat-banner: PASS.

## Validation

- cargo fmt --all -- --check: passed.
- cargo -Zjson-target-spec check --quiet: passed.
- cargo -Zjson-target-spec test --quiet: passed.
- scripts/qemu-local-shell-vfs-exec-smoke.sh --quiet: passed.
- scripts/qemu-local-cat-banner-smoke.sh --quiet with task-local evidence
  overrides: passed.
- git diff --check: passed.
- mdbook build: passed.
- git diff --cached --check: passed.

## Non-Goals Preserved

No fake/kernel-backed command expansion was accepted as OS progress. This task
does not implement general argv/envp/auxv/TLS, PATH lookup, pipes, redirection,
process table, wait/exit lifecycle, writable filesystem, networking, SSH,
RP1/PCIe, DMA/cache policy, job control, Pi 5 hardware proof, or hardware lock
usage.
