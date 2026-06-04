# Phase 10 /dev/null Stdout Redirection Closeout

Task: phase10-dev-null-stdout-redirection-closeout-20260604
Status: accepted

## Scope

Close out the accepted stdout-to-/dev/null redirection contract before
extending the same explicit sink device to stderr.

The accepted behavior remains exactly:

- `exec stdout >/dev/null` launches the VFS-backed `/bin/stdout` fixture
  with child fd1 rebound to the explicit `/dev/null` sink device.
- The child descriptor table reports `fd1=device`; the redirection record
  reports `op=sink`, `target-path=/dev/null`,
  `target-stream=null-sink`, and `target-route=device:/dev/null`.
- `TalosWrite` validates/copies the userspace buffer, discards the bytes,
  and returns the accepted byte count.
- The redirected command does not print the stdout fixture payload on
  runtime-console0/stdout. A following normal `exec stdout` prints
  `Talos userspace stdout fixture`, proving shell fd1 restoration.

This closeout does not add code and does not expand into writable filesystem
behavior, regular-file redirection, append/truncate, input redirection,
stderr-to-/dev/null, arbitrary descriptor syntax, broader file/device
semantics, multi-stage pipelines, Pi 5 proof, networking, SSH, or a phase
transition.

## Findings

- fixed: The accepted core task records `/dev/null` as an explicit sink
  device, not writable filesystem support.
- fixed: The accepted evidence maps child-only fd1 rebinding, null-sink route
  identity, byte discard accounting, stdout payload absence for the redirected
  command, and shell fd1 restoration.
- fixed: The roadmap now has a closeout entry that prevents acceptance drift
  from stdout-to-/dev/null into regular-file or broader file/device
  redirection.
- not-an-issue: The task smoke log includes one stdout fixture payload line
  because the later normal `exec stdout` control intentionally proves
  restoration after the redirected child exits.
- deferred: `1>/dev/null`, regular-file redirection, append/truncate, input
  redirection, stderr-to-/dev/null, arbitrary descriptor syntax, writable
  filesystem behavior, multi-stage/concurrent pipelines, Pi 5 proof,
  networking, SSH, and a phase transition remain deferred. The queued
  stderr-to-/dev/null task is the only mechanically unblocked follow-up.

## Evidence Map

- stdout-to-/dev/null task smoke:
  `tasks/evidence/2026-06-04-phase10-dev-null-stdout-redirection-contract-core/qemu-local-shell-dev-null-stdout-redirection-smoke.log`
  records command 3 `exec stdout >/dev/null`, `fd1=device`,
  `exec-redirection op=sink ... target-path=/dev/null ...
  target-stream=null-sink target-route=device:/dev/null`,
  `exec-stdout ... stream=null-sink route=device:/dev/null`, and
  `bytes=0x1f return=0x1f`.
- normal stdout restoration control: the same log records the following
  command `exec stdout`, visible `Talos userspace stdout fixture`,
  `fd1=stdio-output`, and `stream=stdout route=runtime-console0/stdout`.
- lifecycle controls: the same log records `waitpid`, `laststatus`,
  `exec-lifecycle ... state=exited status=0`, and
  `exec-status ... complete=true source=lifecycle-record`.
- deterministic negatives: the same log records unsupported
  `exec stdout 1>/dev/null`, `exec stdout 1>file`, and
  `exec stdout | stderr` as `exec-invalid-path`/unexpected-argument
  negatives.
- descriptor-backed VFS control: the same log records
  `cat /etc/banner.txt` printing `Talos initramfs fixture` after the
  redirection and restoration checks.
- retained QEMU/substitute controls from the accepted core iteration:
  normal userspace stdout, stdout-to-stderr descriptor dup, stdout descriptor
  close, descriptor-mixing pipeline, runtime stdin readiness, terminal Ctrl-D
  EOF, and descriptor-backed cat all passed.

## Validation

- static inspection: accepted core task record, ADR entry, roadmap entry, and
  task-owned evidence log inspected.
- diff check: `git diff --check` passed.
- docs: `/home/node/.cargo/bin/mdbook build` passed.
- staged diff check: `git diff --cached --check` passed before commit.

## Commit

Commit: final closeout commit recorded in supervisor state.
