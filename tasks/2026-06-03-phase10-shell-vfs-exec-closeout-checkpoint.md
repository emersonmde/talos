# Phase 10 Shell VFS Exec Closeout Checkpoint

Task: phase10-shell-vfs-exec-closeout-checkpoint-20260603

Status: accepted

## Scope

Documentation-only closeout for the accepted shell-visible exec /bin/init
boundary backed by VFS descriptor reads, program loading, process-install,
address-space, materialization, initial-stack, and launch-boundary records.

No runtime code, boot archive, hardware state, or hardwareTestLock state
changed.

## Records

- Project closeout:
  docs/src/project/phase10-shell-vfs-exec-closeout-checkpoint.md.
- Accepted implementation task:
  tasks/2026-06-03-phase10-shell-vfs-exec-boundary.md.
- Accepted implementation commit:
  0e418a08c5ab3a5e2b9c67c9d48a007ca70d85cb.
- Shell exec evidence:
  tasks/evidence/2026-06-03-phase10-shell-vfs-exec-boundary/qemu-local-shell-vfs-exec-smoke.log.
- VFS cat regression evidence:
  tasks/evidence/2026-06-03-phase10-shell-vfs-exec-boundary/qemu-local-cat-banner-regression.log.

## Findings And Dispositions

- fixed: The accepted implementation task provides the shell-visible VFS exec
  boundary through the accepted VFS/open/read, loader, process-install,
  address-space, materialization, stack, and launch-boundary records.
- fixed: Negative exec paths are deterministic: exec /missing returns
  exec-not-found, and exec /etc/banner.txt returns exec-not-executable.
- fixed: Retained task-local QEMU/substitute evidence proves the positive exec
  path, the negative cases, ready-prompt behavior, and VFS cat regression.
- not-an-issue: Existing built-ins remain in help/status as regression/control
  surfaces and are not counted as new OS progress.
- not-an-issue: Hardware was unused because no new physical Pi 5 claim was
  accepted by this checkpoint.
- deferred: userspace completion/status handoff, process lifecycle, argv/envp,
  PATH, pipes, redirection, writable filesystem, hardware proof, networking,
  and SSH remain outside the accepted frontier.

## Accepted Frontier

Talos now has a QEMU/substitute-proven shell-visible exec /bin/init boundary.
The program bytes come from the descriptor-backed initramfs /bin/init path,
then flow through the accepted loader and userspace launch planning records,
ending in the retained lower-aarch64-svc-launch-boundary-equivalent signal.

This is still not a full process lifecycle or shell command execution model.
The shell observes the launch boundary, not userspace completion or exit
status.

## Next Task

After this closeout is accepted and committed, the queued
phase10-shell-userspace-exit-status-20260603 task is the next dependency-based
feature task. It should add only the narrow userspace status/completion
observation described by its explicit acceptance criteria.

## Validation

- static inspection: accepted task record, implementation commit, shell exec
  evidence, and VFS cat regression evidence inspected.
- git diff --check: passed.
- /home/node/.cargo/bin/mdbook build: passed.
- git diff --cached --check: passed.

Commit: recorded in durable supervisor state after commit creation.
