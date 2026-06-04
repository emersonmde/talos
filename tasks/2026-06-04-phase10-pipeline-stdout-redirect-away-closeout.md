# Phase 10 Pipeline Stdout Redirect Away Closeout

Task: phase10-pipeline-stdout-redirect-away-closeout-20260604

Status: accepted

## Summary

This closeout checkpoints the accepted descriptor-mixing pipeline frontier after
both exact two-stage mixed forms are covered:

- `exec stderr 2>&1 | exec stdin`: producer fd1 is installed as the pipe
  writer, then child-only `2>&1` duplicates that pipe-backed fd1 endpoint
  onto producer fd2. The VFS-backed `/bin/stderr` fixture writes 31 bytes
  through fd2 into the pipe, and the VFS-backed `/bin/stdin` consumer reads
  those bytes from inherited fd0 and reports them through inherited fd1.
- `exec stdout 1>&2 | exec stdin`: producer fd1 is installed as the pipe
  writer, then child-only `1>&2` rebinds producer fd1 to inherited
  fd2/stderr. The VFS-backed `/bin/stdout` fixture writes 31 bytes through
  `stream=stderr route=runtime-console0/stderr`, no fixture bytes enter the
  pipe, and the consumer reports deterministic `pipe-eof/no-data`.

Plain `exec stdout | exec stdin` remains the stdout transfer control. Plain
`exec stderr | exec stdin` remains the stdout-only stderr-not-piped control.
No arbitrary `N>&M`, explicit stderr pipe syntax, file/device redirection,
multi-stage pipeline, concurrent pipeline scheduling, or phase-transition claim
is accepted by this checkpoint.

## Findings And Disposition

- fixed: Reconciled the two accepted mixed pipeline directions as exact
  descriptor-ordering rules: install the producer pipe on fd1 first, then apply
  the child-only descriptor operation for the producer command only.
- fixed: Confirmed `exec stderr 2>&1 | exec stdin` sends the stderr fixture
  through the pipe and into consumer fd0 while preserving shell descriptor
  restoration, writer-close EOF, consumer `waitpid`, consumer `laststatus`,
  final classification, errors=0, and PASS.
- fixed: Confirmed `exec stdout 1>&2 | exec stdin` redirects the stdout
  fixture away from the pipe to the inherited stderr route, transfers zero pipe
  bytes, reports `pipe-eof/no-data`, restores shell fd0/fd1/fd2, records
  consumer `waitpid` and `laststatus`, and passes.
- fixed: Preserved plain stdout pipeline, stdout-only stderr-not-piped,
  descriptor-dup, descriptor-close, normal stdio/stderr routing, stdin
  readiness/EOF, VFS exec, lifecycle/status, waitpid, laststatus, negative
  controls, and descriptor-backed cat evidence as retained regression
  coverage.
- not-an-issue: This closeout does not add implementation. The accepted core
  task owns the code changes, task-owned QEMU/substitute smoke, refreshed
  controls, and direct validation gates; this task records the accepted
  boundary and evidence map.
- deferred: arbitrary descriptor syntax and arbitrary `N>&M`, explicit
  stderr pipe syntax, file/device redirection, multi-stage/concurrent
  pipelines, pipefail, jobs, async execution, fork, signals, writable
  filesystem behavior, Pi 5 proof, networking, SSH, and any phase transition
  remain out of scope.

## Evidence Map

- inverse stdout-redirection-away positive evidence:
  `tasks/evidence/2026-06-04-phase10-pipeline-stdout-redirect-away-core/qemu-local-shell-pipeline-stdout-redirect-away-smoke.log`.
  Static inspection shows `exec stdout 1>&2 | exec stdin`,
  `producer-path=/bin/stdout`, `consumer-path=/bin/stdin`,
  child-only `exec-redirection op=dup source-fd=1 target-fd=2`,
  `target-stream=stderr target-route=runtime-console0/stderr`,
  `exec-stdout fd=1` writing 31 bytes through
  `stream=stderr route=runtime-console0/stderr`, consumer fd0 reading zero
  bytes from `pipe:stdout-to-stdin`, `read-result=pipe-eof/no-data`,
  shell restoration, consumer `waitpid`, consumer `laststatus`, final
  classification `qemu-local-shell-pipeline-stdout-redirect-away-complete`,
  errors=0, and PASS.
- stderr-to-pipe mixed positive evidence:
  `tasks/evidence/2026-06-04-phase10-pipeline-stderr-dup-to-stdout-core/qemu-local-shell-pipeline-stderr-dup-to-stdout-smoke.log`.
  Static inspection shows `exec stderr 2>&1 | exec stdin`, producer fd1 as
  the pipe writer, producer fd2 as the child-only duplicate of that pipe
  endpoint, `/bin/stderr` writing 31 bytes through
  `stream=pipe-writer route=pipe:stdout-to-stdin`, consumer fd0 reading the
  stderr fixture bytes, writer-close EOF, shell restoration, consumer
  `waitpid`, consumer `laststatus`, final classification
  `qemu-local-shell-pipeline-stderr-dup-to-stdout-complete`, errors=0, and
  PASS.
- plain stdout pipeline control:
  `tasks/evidence/2026-06-04-phase10-minimal-stdout-to-stdin-pipe-core/qemu-local-shell-minimal-stdout-to-stdin-pipeline-smoke.log`.
  Retained evidence covers `exec stdout | exec stdin` as a 31-byte fd1/stdout
  pipe transfer with writer-close EOF, shell restoration, consumer
  lifecycle/status, waitpid, laststatus, final classification, and PASS.
- stdout-only stderr-not-piped control:
  `tasks/evidence/2026-06-04-phase10-pipeline-stderr-not-piped-core/qemu-local-shell-pipeline-stderr-not-piped-smoke.log`.
  Retained evidence covers `exec stderr | exec stdin` as stdout-only:
  producer fd1 is the pipe endpoint, producer fd2 remains
  `stream=stderr route=runtime-console0/stderr`, zero pipe bytes are
  written/read, the consumer reports `read-result=pipe-eof/no-data`, shell
  restoration remains true, final classification is recorded, and PASS.
- descriptor-dup controls:
  `tasks/evidence/2026-06-04-phase10-stdout-to-stderr-fd-dup-redirection-core/qemu-local-shell-stdout-to-stderr-redirection-smoke.log`
  and
  `tasks/evidence/2026-06-04-phase10-stderr-to-stdout-fd-dup-redirection-core/qemu-local-shell-stderr-to-stdout-redirection-smoke.log`.
- descriptor-close controls:
  `tasks/evidence/2026-06-04-phase10-stdout-close-redirection-core/qemu-local-shell-stdout-close-redirection-smoke.log`
  and
  `tasks/evidence/2026-06-04-phase10-stderr-close-redirection-core/qemu-local-shell-stderr-close-redirection-smoke.log`.
- normal stdio and stderr routing controls:
  `tasks/evidence/2026-06-03-phase10-userspace-stdout-inherited-fd-core/qemu-local-shell-userspace-stdout-smoke.log`,
  `tasks/evidence/2026-06-03-phase10-userspace-stdin-inherited-fd-core/qemu-local-shell-userspace-stdin-smoke.log`,
  `tasks/evidence/2026-06-03-phase10-userspace-stderr-inherited-fd-core/qemu-local-shell-userspace-stderr-smoke.log`, and
  `tasks/evidence/2026-06-04-phase10-distinct-stderr-routing-core/qemu-local-shell-distinct-stderr-routing-smoke.log`.
- stdin readiness and EOF controls:
  `tasks/evidence/2026-06-04-phase10-scheduler-backed-stdin-wait-core/qemu-local-shell-scheduler-backed-stdin-wait-smoke.log`
  and
  `tasks/evidence/2026-06-04-phase10-terminal-ctrl-d-eof-core/qemu-local-shell-terminal-ctrl-d-eof-smoke.log`.
- VFS exec, lifecycle/status, waitpid, laststatus, negative controls, and
  descriptor-backed file I/O:
  `tasks/evidence/2026-06-03-phase10-absolute-vfs-exec-dispatch-core/qemu-local-shell-absolute-vfs-exec-dispatch-smoke.log`,
  `tasks/evidence/2026-06-03-phase10-vfs-exec-nonzero-status-core/qemu-local-shell-nonzero-vfs-exec-status-smoke.log`,
  `tasks/evidence/2026-06-03-phase10-minimal-waitpid-lifecycle-observation-core/qemu-local-shell-waitpid-lifecycle-smoke.log`,
  `tasks/evidence/2026-06-03-phase10-shell-last-process-status-observation/qemu-local-shell-last-process-status-smoke.log`, and
  `tasks/evidence/2026-06-02-qemu-local-cat-banner-core/qemu-local-cat-banner-smoke.log`.

## Accepted Frontier

Accepted:

- exactly `exec stderr 2>&1 | exec stdin` for stderr-to-pipe descriptor
  mixing;
- exactly `exec stdout 1>&2 | exec stdin` for stdout-redirection-away
  descriptor mixing;
- pipe fd1 is installed before the producer child-only descriptor operation;
- child-only `2>&1` duplicates the pipe-backed fd1 endpoint onto producer
  fd2, so stderr bytes enter the pipe;
- child-only `1>&2` rebinds producer fd1 to inherited stderr, so stdout
  bytes do not enter the pipe and the consumer sees `pipe-eof/no-data`;
- shell fd0/fd1/fd2 restoration, writer-close EOF, consumer waitpid,
  consumer laststatus, no leaked pipe endpoints, and descriptor-backed VFS
  exec are covered by QEMU/substitute evidence;
- plain `exec stdout | exec stdin` and plain `exec stderr | exec stdin`
  controls remain passing.

Deferred:

- arbitrary descriptor syntax and arbitrary `N>&M`;
- explicit stderr pipe syntax, file/device redirection, and writable
  filesystem behavior;
- multi-stage/concurrent pipelines, pipefail, jobs, async execution, fork,
  signals, Pi 5 proof, networking, SSH, and any phase transition.

## Next Step Requirement

The next explicit queued task is
`phase10-pipeline-descriptor-mixing-frontier-closeout-20260604`. It is a
frontier closeout only: reconcile the minimal pipeline, stdout-only
stderr-not-piped semantics, both accepted mixed forms, descriptor redirection
controls, stdio/stdin controls, VFS/lifecycle evidence, and deferred work. It
must not implement file/device redirection, arbitrary descriptor syntax,
multi-stage pipelines, writable filesystem behavior, networking, SSH, or a
phase transition.

## Validation Summary

- static inspection: accepted inverse mixed core task docs and retained mixed
  pipeline, plain stdout pipeline, stdout-only stderr-not-piped, descriptor
  redirection, descriptor close, normal stdio, stdin readiness/EOF, VFS exec,
  lifecycle/status, waitpid, laststatus, negative control, and
  descriptor-backed cat evidence logs were inspected.
- diff check: `git diff --check` passed.
- docs: `/home/node/.cargo/bin/mdbook build` passed.
- staged diff check: `git diff --cached --check` passed before commit.

hardwareTestLock remained unlocked/restored and unused.

Acceptance commit: recorded in durable supervisor state after commit creation.
