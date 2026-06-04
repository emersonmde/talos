# Phase 10 /dev/null Stderr Redirection Closeout

Task: phase10-dev-null-stderr-redirection-closeout-20260604
Status: accepted

## Scope

Close out the accepted stderr-to-/dev/null redirection behavior before the
broader /dev/null redirection frontier checkpoint.

The accepted file/device redirection behavior is exactly:

- `exec stdout >/dev/null` launches the VFS-backed `/bin/stdout` fixture
  with child fd1 rebound to the explicit `/dev/null` sink device.
- `exec stderr 2>/dev/null` launches the VFS-backed `/bin/stderr` fixture
  with child fd2 rebound to the same explicit `/dev/null` sink device.
- Both sink forms are child-only descriptor-table mutations. The shell
  restores the relevant standard descriptor after the child exits.
- The redirection record reports `op=sink`, `target-path=/dev/null`,
  `target-stream=null-sink`, and `target-route=device:/dev/null`.
- `TalosWrite` validates/copies the userspace buffer, discards 31 bytes, and
  returns the accepted byte count for the sink route.

This closeout does not add code and does not expand into regular-file
redirection, append/truncate, input redirection, writable filesystem behavior,
arbitrary descriptor syntax, broader file/device semantics, multi-stage
pipelines, Pi 5 proof, networking, SSH, or a phase transition.

## Findings

- fixed: The accepted stderr core records `/dev/null` as the same explicit
  sink device accepted for stdout, not writable filesystem support.
- fixed: The accepted evidence maps child-only fd2 rebinding, null-sink route
  identity, byte discard accounting, stderr payload absence for the redirected
  command, and shell fd2 restoration.
- fixed: The evidence map retains stdout-to-/dev/null as the sibling sink
  control and keeps normal stdout/stderr restoration controls available.
- fixed: The roadmap now has a closeout entry that prevents acceptance drift
  from the two exact `/dev/null` sink forms into regular-file or broader
  file/device redirection.
- not-an-issue: The stderr smoke log includes one stderr fixture payload line
  because the later normal `exec stderr` control intentionally proves shell
  fd2 restoration after the redirected child exits.
- not-an-issue: The stdout-to-/dev/null control remains visible in the stderr
  core smoke because this closeout needs both accepted sink directions mapped
  before the frontier checkpoint.
- deferred: `1>/dev/null`, `exec stderr 2>file`,
  `exec stderr 2>>/dev/null`, `exec stderr </dev/null`, regular-file
  redirection, append/truncate, input redirection, arbitrary descriptor syntax,
  writable filesystem behavior, multi-stage/concurrent pipelines, Pi 5 proof,
  networking, SSH, and a phase transition remain deferred. The queued
  /dev/null redirection frontier closeout is the only mechanically unblocked
  follow-up.

## Evidence Map

- stderr-to-/dev/null task smoke:
  `tasks/evidence/2026-06-04-phase10-dev-null-stderr-redirection-core/qemu-local-shell-dev-null-stderr-redirection-smoke.log`
  records command 3 `exec stderr 2>/dev/null`, `fd2=device`,
  `exec-redirection op=sink ... target-path=/dev/null ...
  target-stream=null-sink target-route=device:/dev/null`,
  `exec-stderr ... stream=null-sink route=device:/dev/null`, and
  `bytes=0x1f return=0x1f`.
- normal stderr restoration control: the same log records the following
  command `exec stderr`, visible `Talos userspace stderr fixture`,
  `fd2=stdio-output`, and
  `stream=stderr route=runtime-console0/stderr`.
- stdout-to-/dev/null sibling control: the same log records
  `exec stdout >/dev/null`, `fd1=device`,
  `exec-redirection op=sink ... source=shell-redirection-stdout-dev-null`,
  and `exec-stdout ... stream=null-sink route=device:/dev/null`.
- lifecycle controls: the same log records `waitpid`, `laststatus`,
  `exec-lifecycle ... state=exited status=0`, and
  `exec-status ... complete=true source=lifecycle-record`.
- deterministic negatives: the same log records unsupported
  `exec stderr 2>file`, `exec stderr 2>>/dev/null`, and
  `exec stderr </dev/null` as `exec-invalid-path`/unexpected-argument
  negatives.
- descriptor-backed VFS control: the same log records
  `cat /etc/banner.txt` printing `Talos initramfs fixture` after the
  redirection and restoration checks.
- retained QEMU/substitute controls from the accepted stderr core iteration:
  stdout-to-/dev/null, normal userspace stderr, stdout-to-stderr descriptor
  dup, descriptor-mixing pipeline with stderr dup-to-stdout, runtime stdin
  readiness, terminal EOF/no-data, and descriptor-backed cat all passed.

## Validation

- static inspection: accepted stdout closeout, accepted stderr core task
  record, roadmap entry, and task-owned evidence logs inspected.
- diff check: `git diff --check` passed.
- docs: `/home/node/.cargo/bin/mdbook build` passed.
- staged diff check: `git diff --cached --check` passed before commit.

## Commit

Commit: final closeout commit recorded in supervisor state.
