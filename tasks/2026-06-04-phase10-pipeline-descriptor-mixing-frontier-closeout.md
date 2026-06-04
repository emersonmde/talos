# Phase 10 Pipeline Descriptor Mixing Frontier Closeout

Task: phase10-pipeline-descriptor-mixing-frontier-closeout-20260604

Status: accepted

## Summary

This closeout checkpoints the accepted descriptor-mixing pipeline frontier
after the minimal stdout pipeline, stdout-only stderr-not-piped control, and
both exact mixed descriptor forms are covered.

Accepted pipeline behavior is limited to these exact two-stage forms:

- 'exec stdout | exec stdin': producer fd1/stdout is the pipe writer,
  consumer fd0/stdin is the pipe reader, and 31 fixture bytes move through
  the pipe.
- 'exec stderr | exec stdin': producer fd1/stdout is still the only pipe
  writer, producer fd2/stderr stays routed to 'runtime-console0/stderr', no
  fixture bytes enter the pipe, and the consumer reports deterministic
  'pipe-eof/no-data'.
- 'exec stderr 2>&1 | exec stdin': the shell installs the producer pipe
  writer on fd1 first, then child-only '2>&1' duplicates that pipe endpoint
  onto producer fd2, so stderr fixture bytes enter consumer fd0.
- 'exec stdout 1>&2 | exec stdin': the shell installs the producer pipe
  writer on fd1 first, then child-only '1>&2' rebinds producer fd1 to
  inherited fd2/stderr, so stdout fixture bytes do not enter the pipe and the
  consumer reports 'pipe-eof/no-data'.

All accepted forms remain backed by descriptor-backed VFS exec, inherited
descriptor tables, shell fd0/fd1/fd2 restoration, writer-close EOF or
deterministic pipe no-data, consumer 'waitpid', consumer 'laststatus', and
QEMU/substitute evidence. This frontier does not accept arbitrary descriptor
syntax, arbitrary 'N>&M', explicit stderr pipe syntax, file/device
redirection, writable filesystem behavior, multi-stage or concurrent
pipelines, pipefail, jobs, networking, SSH, or a phase transition.

## Findings And Disposition

- fixed: Reconciled the accepted pipeline grammar as exact two-stage
  stdout-to-stdin forms with producer fd1 as the default pipe source.
- fixed: Reconciled the descriptor ordering rule for mixed forms: install
  the producer pipe on fd1 first, then apply the producer child-only
  descriptor operation.
- fixed: Confirmed 'exec stderr 2>&1 | exec stdin' sends the stderr fixture
  through the pipe by duplicating the pipe-backed fd1 endpoint onto fd2.
- fixed: Confirmed 'exec stdout 1>&2 | exec stdin' redirects stdout away
  from the pipe by rebinding fd1 to inherited stderr before the stdout fixture
  writes.
- fixed: Preserved descriptor-dup, descriptor-close, normal stdio/stderr
  routing, stdin wait/readiness/EOF, VFS exec, lifecycle/status, waitpid,
  laststatus, negative controls, and descriptor-backed cat evidence as
  retained regression coverage.
- not-an-issue: This closeout does not implement code. The accepted core
  tasks own implementation changes, QEMU/substitute smokes, and direct
  feature validation; this task records the accepted boundary and evidence
  map.
- deferred: arbitrary descriptor syntax, arbitrary 'N>&M', explicit stderr
  pipe syntax, file/device redirection, writable filesystem behavior,
  multi-stage/concurrent pipelines, pipefail, jobs, async execution, fork,
  signals, Pi 5 proof, networking, SSH, and any phase transition.

## Evidence Map

- stdout-redirection-away mixed evidence:
  'tasks/evidence/2026-06-04-phase10-pipeline-stdout-redirect-away-core/qemu-local-shell-pipeline-stdout-redirect-away-smoke.log'.
  Static inspection shows 'exec stdout 1>&2 | exec stdin', producer
  '/bin/stdout', consumer '/bin/stdin', child-only
  'exec-redirection op=dup source-fd=1 target-fd=2',
  'target-stream=stderr target-route=runtime-console0/stderr',
  'exec-stdout fd=1' writing 31 bytes through stderr, zero pipe bytes
  written/read, 'read-result=pipe-eof/no-data', shell restoration, consumer
  'waitpid', consumer 'laststatus', final classification
  'qemu-local-shell-pipeline-stdout-redirect-away-complete', errors=0, and
  PASS.
- stderr-to-pipe mixed evidence:
  'tasks/evidence/2026-06-04-phase10-pipeline-stderr-dup-to-stdout-core/qemu-local-shell-pipeline-stderr-dup-to-stdout-smoke.log'.
  Static inspection shows 'exec stderr 2>&1 | exec stdin', producer
  '/bin/stderr', consumer '/bin/stdin', producer fd1 as the pipe writer,
  producer fd2 as the child-only duplicate of that pipe endpoint,
  'exec-stderr fd=2' writing 31 bytes through
  'stream=pipe-writer route=pipe:stdout-to-stdin', consumer fd0 reading
  those bytes, writer-close EOF, shell restoration, consumer 'waitpid',
  consumer 'laststatus', final classification
  'qemu-local-shell-pipeline-stderr-dup-to-stdout-complete', errors=0, and
  PASS.
- plain stdout pipeline control:
  'tasks/evidence/2026-06-04-phase10-minimal-stdout-to-stdin-pipe-core/qemu-local-shell-minimal-stdout-to-stdin-pipeline-smoke.log'.
  Static inspection confirms 'exec stdout | exec stdin', 31 bytes written
  and read through 'pipe:stdout-to-stdin', writer-close EOF, shell
  restoration, consumer lifecycle/status, 'waitpid', 'laststatus',
  descriptor-backed 'cat /etc/banner.txt', final classification, and PASS.
- stdout-only stderr-not-piped control:
  'tasks/evidence/2026-06-04-phase10-pipeline-stderr-not-piped-core/qemu-local-shell-pipeline-stderr-not-piped-smoke.log'.
  Static inspection confirms 'exec stderr | exec stdin', producer fd2
  remaining on 'stream=stderr route=runtime-console0/stderr', zero pipe bytes
  written/read, 'read-result=pipe-eof/no-data', shell restoration,
  consumer 'waitpid', consumer 'laststatus', final classification, and PASS.
- descriptor-dup controls:
  'tasks/evidence/2026-06-04-phase10-stdout-to-stderr-fd-dup-redirection-core/qemu-local-shell-stdout-to-stderr-redirection-smoke.log'
  and
  'tasks/evidence/2026-06-04-phase10-stderr-to-stdout-fd-dup-redirection-core/qemu-local-shell-stderr-to-stdout-redirection-smoke.log'.
- descriptor-close controls:
  'tasks/evidence/2026-06-04-phase10-stdout-close-redirection-core/qemu-local-shell-stdout-close-redirection-smoke.log'
  and
  'tasks/evidence/2026-06-04-phase10-stderr-close-redirection-core/qemu-local-shell-stderr-close-redirection-smoke.log'.
- normal stdio and stderr routing controls:
  'tasks/evidence/2026-06-03-phase10-userspace-stdout-inherited-fd-core/qemu-local-shell-userspace-stdout-smoke.log',
  'tasks/evidence/2026-06-03-phase10-userspace-stdin-inherited-fd-core/qemu-local-shell-userspace-stdin-smoke.log',
  'tasks/evidence/2026-06-03-phase10-userspace-stderr-inherited-fd-core/qemu-local-shell-userspace-stderr-smoke.log', and
  'tasks/evidence/2026-06-04-phase10-distinct-stderr-routing-core/qemu-local-shell-distinct-stderr-routing-smoke.log'.
- stdin readiness and EOF controls:
  'tasks/evidence/2026-06-04-phase10-scheduler-backed-stdin-wait-core/qemu-local-shell-scheduler-backed-stdin-wait-smoke.log'
  and
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

- exactly 'exec stdout | exec stdin';
- exactly 'exec stderr | exec stdin' as the stdout-only stderr-not-piped
  control;
- exactly 'exec stderr 2>&1 | exec stdin';
- exactly 'exec stdout 1>&2 | exec stdin';
- producer fd1 is the default pipe writer for accepted two-stage forms;
- mixed forms install the producer pipe endpoint before applying producer
  child-only descriptor redirection;
- shell fd0/fd1/fd2 restoration, writer-close EOF or deterministic pipe
  no-data, consumer waitpid, consumer laststatus, no leaked pipe endpoints,
  and descriptor-backed VFS exec are covered by QEMU/substitute evidence.

Deferred:

- arbitrary descriptor syntax and arbitrary 'N>&M';
- explicit stderr pipe syntax and file/device redirection;
- writable filesystem behavior;
- multi-stage/concurrent pipelines, pipefail, jobs, async execution, fork,
  signals, and job control;
- Pi 5 proof, networking, SSH, and any phase transition.

## Next Step Requirement

Supervisor planning is required before the next feature-led shell I/O task.
The queue has no further explicit task after this closeout, and the worker
must not infer a broader feature or phase transition.

The bounded recommendation is to plan a file/device redirection slice next
only if the supervisor records an explicit target/sink contract. Multi-stage
pipeline status or scheduling needs a separate process-accounting plan first.
Descriptor syntax cleanup is lower value unless it directly supports an
explicit feature slice.

## Validation Summary

- static inspection: accepted mixed pipeline, minimal pipeline,
  stdout-only stderr-not-piped, descriptor redirection, descriptor close,
  normal stdio, stdin readiness/EOF, VFS exec, lifecycle/status, waitpid,
  laststatus, negative control, and descriptor-backed cat task records and
  evidence logs were inspected.
- diff check: 'git diff --check' passed.
- docs: '/home/node/.cargo/bin/mdbook build' passed.
- staged diff check: 'git diff --cached --check' passed before commit.

hardwareTestLock remained unlocked/restored and unused.

Acceptance commit: recorded in durable supervisor state after commit creation.
