# Phase 10 Pipeline Stderr Dup To Stdout Closeout

Task: phase10-pipeline-stderr-dup-to-stdout-closeout-20260604

Status: accepted

## Summary

This closeout checkpoints the first accepted descriptor-mixing pipeline form
before inverse redirection-away behavior or broader pipe syntax.

The accepted feature boundary is exactly:

- 'exec stderr 2>&1 | exec stdin': the shell installs the producer pipe writer
  on fd1, then applies child-only '2>&1' so producer fd2 duplicates that
  pipe-backed fd1 endpoint. The VFS-backed '/bin/stderr' fixture writes 31
  bytes through fd2 into the pipe, the VFS-backed '/bin/stdin' consumer reads
  those bytes from inherited fd0, reports the stderr fixture line through
  inherited fd1, observes writer-close EOF, and leaves shell fd0/fd1/fd2
  restored afterward.

Plain 'exec stdout | exec stdin' remains the stdout positive control. Plain
'exec stderr | exec stdin' remains the stdout-only stderr-not-piped control.
No inverse 'exec stdout 1>&2 | exec stdin', arbitrary descriptor syntax,
explicit stderr pipe syntax, file/device redirection, multi-stage pipeline,
or phase-transition claim is accepted by this checkpoint.

## Findings And Disposition

- fixed: Reconciled the accepted mixed pipeline evidence as one exact
  descriptor-ordering rule: pipe fd1 first, then child-only fd2 duplicates the
  pipe-backed fd1 endpoint.
- fixed: Confirmed the positive mixed transcript records
  'exec-redirection op=dup source-fd=2 target-fd=1',
  'target-stream=pipe-writer target-route=pipe:stdout-to-stdin',
  '/bin/stderr' writing through 'stream=pipe-writer', 31 bytes written/read,
  writer-close EOF, shell restoration, consumer 'waitpid', consumer
  'laststatus', final classification, errors=0, and PASS.
- fixed: Preserved plain stdout pipeline, stdout-only stderr-not-piped,
  descriptor-dup, descriptor-close, normal stdio/stderr routing, stdin
  readiness/EOF, VFS exec, lifecycle/status, waitpid, laststatus, negative
  controls, and descriptor-backed cat evidence as retained regression
  coverage.
- not-an-issue: This closeout does not add implementation. The accepted core
  already owns the code, task-owned QEMU/substitute smoke, and refreshed
  controls; this task only records the boundary and evidence map.
- deferred: Inverse 'exec stdout 1>&2 | exec stdin', arbitrary descriptor
  syntax, explicit stderr pipe syntax, multi-stage pipelines, concurrent POSIX
  pipeline scheduling, pipefail, async execution, fork, signals, job control,
  file/device redirection, writable filesystem behavior, Pi 5 proof,
  networking, SSH, and any phase transition remain out of scope.

## Evidence Map

- mixed stderr-to-pipe positive evidence:
  'tasks/evidence/2026-06-04-phase10-pipeline-stderr-dup-to-stdout-core/qemu-local-shell-pipeline-stderr-dup-to-stdout-smoke.log'.
  Static inspection shows 'exec stderr 2>&1 | exec stdin',
  'producer-path=/bin/stderr', 'consumer-path=/bin/stdin',
  producer fd1 as the pipe writer, producer fd2 as the child-only duplicate of
  that pipe endpoint, 'target-stream=pipe-writer',
  'target-route=pipe:stdout-to-stdin', 'exec-stderr fd=2' writing 31 bytes
  through 'stream=pipe-writer route=pipe:stdout-to-stdin', consumer
  '/bin/stdin' reading 31 stderr fixture bytes from fd0, stdout reporting the
  stderr fixture through fd1, 'read-result=pipe-eof-after-writer-close',
  'writer-closed=true', 'reader-eof=true', 'shell-restored=true', consumer
  'waitpid', consumer 'laststatus', final classification
  'qemu-local-shell-pipeline-stderr-dup-to-stdout-complete', errors=0, and
  PASS.
- plain stdout pipeline control:
  'tasks/evidence/2026-06-04-phase10-minimal-stdout-to-stdin-pipe-core/qemu-local-shell-minimal-stdout-to-stdin-pipeline-smoke.log'
  and the refreshed mixed-pipeline smoke both preserve 'exec stdout | exec
  stdin' as the 31-byte fd1/stdout pipe transfer with writer-close EOF,
  shell restoration, consumer lifecycle/status, waitpid, laststatus, final
  classification, and PASS.
- stdout-only stderr-not-piped control:
  'tasks/evidence/2026-06-04-phase10-pipeline-stderr-not-piped-core/qemu-local-shell-pipeline-stderr-not-piped-smoke.log'
  and the refreshed mixed-pipeline smoke both preserve 'exec stderr | exec
  stdin' as stdout-only: producer fd1 is the pipe endpoint, producer fd2 stays
  on 'stream=stderr route=runtime-console0/stderr', zero pipe bytes are
  written/read, the consumer reports 'read-result=pipe-eof/no-data',
  shell restoration remains true, final classification is recorded, and PASS.
- descriptor-dup controls:
  'tasks/evidence/2026-06-04-phase10-stderr-to-stdout-fd-dup-redirection-core/qemu-local-shell-stderr-to-stdout-redirection-smoke.log' and
  'tasks/evidence/2026-06-04-phase10-stdout-to-stderr-fd-dup-redirection-core/qemu-local-shell-stdout-to-stderr-redirection-smoke.log'.
  Retained evidence covers child-only descriptor rebinding, shell restoration,
  lifecycle/status, waitpid, laststatus, deterministic negative redirection
  controls, descriptor-backed cat, errors=0, and PASS.
- descriptor-close controls:
  'tasks/evidence/2026-06-04-phase10-stdout-close-redirection-core/qemu-local-shell-stdout-close-redirection-smoke.log' and
  'tasks/evidence/2026-06-04-phase10-stderr-close-redirection-core/qemu-local-shell-stderr-close-redirection-smoke.log'.
  Retained evidence covers child-only close behavior, shell restoration,
  lifecycle/status, waitpid, laststatus, deterministic negative controls,
  descriptor-backed cat, errors=0, and PASS.
- normal stdio and stderr routing controls:
  'tasks/evidence/2026-06-03-phase10-userspace-stdout-inherited-fd-core/qemu-local-shell-userspace-stdout-smoke.log',
  'tasks/evidence/2026-06-03-phase10-userspace-stdin-inherited-fd-core/qemu-local-shell-userspace-stdin-smoke.log',
  'tasks/evidence/2026-06-03-phase10-userspace-stderr-inherited-fd-core/qemu-local-shell-userspace-stderr-smoke.log', and
  'tasks/evidence/2026-06-04-phase10-distinct-stderr-routing-core/qemu-local-shell-distinct-stderr-routing-smoke.log'.
- stdin readiness and EOF controls:
  'tasks/evidence/2026-06-04-phase10-scheduler-backed-stdin-wait-core/qemu-local-shell-scheduler-backed-stdin-wait-smoke.log' and
  'tasks/evidence/2026-06-04-phase10-terminal-ctrl-d-eof-core/qemu-local-shell-terminal-ctrl-d-eof-smoke.log'.
- VFS exec, lifecycle/status, waitpid, laststatus, negative controls, and
  descriptor-backed file I/O:
  'tasks/evidence/2026-06-03-phase10-absolute-vfs-exec-dispatch-core/qemu-local-shell-absolute-vfs-exec-dispatch-smoke.log',
  'tasks/evidence/2026-06-03-phase10-vfs-exec-nonzero-status-core/qemu-local-shell-nonzero-vfs-exec-status-smoke.log',
  'tasks/evidence/2026-06-03-phase10-minimal-waitpid-lifecycle-observation-core/qemu-local-shell-waitpid-lifecycle-smoke.log',
  'tasks/evidence/2026-06-03-phase10-shell-last-process-status-observation/qemu-local-shell-last-process-status-smoke.log', and
  'tasks/evidence/2026-06-02-qemu-local-cat-banner-core/qemu-local-cat-banner-smoke.log'.

## Accepted Frontier

Accepted:

- only the exact 'exec stderr 2>&1 | exec stdin' descriptor-mixing pipeline;
- producer fd1 is installed as the pipe writer before child-only '2>&1';
- producer fd2 duplicates that pipe-backed fd1 endpoint for the '/bin/stderr'
  write;
- consumer fd0 reads the stderr fixture bytes from the pipe and reports them
  through inherited fd1;
- writer-close EOF, no leaked pipe endpoints, shell descriptor restoration,
  consumer waitpid, consumer laststatus, and descriptor-backed VFS exec are
  covered by QEMU/substitute evidence;
- plain 'exec stdout | exec stdin' and plain 'exec stderr | exec stdin'
  controls remain passing.

Deferred:

- inverse 'exec stdout 1>&2 | exec stdin';
- arbitrary descriptor syntax and explicit stderr pipe syntax;
- multi-stage pipelines, concurrent POSIX pipeline scheduling, pipefail, async
  execution, fork, signals, and job control;
- file/device redirection, writable filesystem behavior, Pi 5 proof,
  networking, SSH, and any phase transition.

## Next Step Requirement

The next explicit queued task is
'phase10-pipeline-stdout-redirect-away-core-20260604', which is bounded to
the inverse redirection-away form 'exec stdout 1>&2 | exec stdin'. It must not
be broadened into arbitrary descriptor syntax, explicit stderr pipe syntax,
file/device redirection, multi-stage pipelines, writable filesystem behavior,
networking, SSH, or a phase transition.

## Validation Summary

- static inspection: accepted mixed pipeline core task docs and retained
  mixed pipeline, plain stdout pipeline, stdout-only stderr-not-piped,
  descriptor redirection, descriptor close, normal stdio, stdin readiness/EOF,
  VFS exec, lifecycle/status, waitpid, laststatus, negative control, and
  descriptor-backed cat evidence logs were inspected.
- diff check: 'git diff --check' passed.
- docs: '/home/node/.cargo/bin/mdbook build' passed.
- staged diff check: 'git diff --cached --check' passed before commit.

hardwareTestLock remained unlocked/restored and unused.

Acceptance commit: recorded in durable supervisor state after commit creation.
