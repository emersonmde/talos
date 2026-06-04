# Phase 10 Minimal Stdout-To-Stdin Pipe Closeout

Task: phase10-minimal-stdout-to-stdin-pipe-closeout-20260604

Status: accepted

## Summary

This closeout checkpoints the accepted minimal pipeline frontier before any
broader pipe syntax, scheduling, file redirection, or process-control work.
The accepted feature is one exact shell-visible form:

- 'exec stdout | exec stdin': '/bin/stdout' writes through child fd1 into a
  pipe writer, and '/bin/stdin' reads the same bytes through child fd0 from the
  matching pipe reader.

Both participants launch through the accepted fixed '/bin' descriptor-backed
VFS exec path. The shell descriptor table is restored after the pipeline
command completes, and the accepted lifecycle observation for this bounded
form is the consumer record reported by 'waitpid' and 'laststatus'.

This frontier accepts descriptor ownership, byte transfer, close/EOF behavior,
and status observation for one exact two-stage producer/consumer form. It does
not accept concurrent POSIX pipeline scheduling, multi-stage pipelines,
pipefail, stderr piping policy beyond the already-deferred next slice, async
jobs, fork, signals, job control, file redirection, writable filesystem
behavior, networking, or SSH.

## Findings And Disposition

- fixed: Reconciled the accepted minimal pipeline core as a narrow
  shell-visible pipeline boundary, not a general pipe syntax or scheduler
  boundary.
- fixed: Preserved producer fd1 pipe-writer ownership, consumer fd0 pipe-reader
  ownership, matching byte counts, writer-close EOF, and shell descriptor
  restoration as the defining acceptance rules.
- fixed: Preserved the consumer lifecycle/status record as the accepted
  'waitpid' and 'laststatus' observation for this bounded two-stage form.
- fixed: Preserved descriptor-dup and descriptor-close redirection controls,
  normal userspace stdout/stdin controls, scheduler-backed stdin wait/readiness,
  VFS exec/status/wait controls, deterministic negative controls, and
  descriptor-backed 'cat /etc/banner.txt' evidence as retained regression
  coverage.
- not-an-issue: The accepted core executes the bounded producer before the
  consumer. That is consistent with this slice because the accepted evidence is
  descriptor ownership, deterministic byte transfer, writer-close EOF, shell
  restoration, and lifecycle observation for one exact form.
- deferred: stdout-only stderr-not-piped proof, multi-stage pipelines,
  concurrent pipe scheduling, pipefail, background jobs, async execution, fork,
  signals, job control, file redirection, arbitrary descriptor syntax, writable
  filesystem behavior, Pi 5 proof, networking, and SSH.

## Evidence Map

- minimal stdout-to-stdin pipeline:
  'tasks/evidence/2026-06-04-phase10-minimal-stdout-to-stdin-pipe-core/qemu-local-shell-minimal-stdout-to-stdin-pipeline-smoke.log'.
  Static inspection of QEMU/substitute evidence shows
  'exec stdout | exec stdin', 'producer-path=/bin/stdout',
  'consumer-path=/bin/stdin', 'producer-fd=1', 'consumer-fd=0',
  'bytes-written=0x1f', 'bytes-read=0x1f', 'writer-closed=true',
  'reader-eof=true', 'shell-restored=true',
  'fd1=pipe-endpoint' for the producer, 'fd0=pipe-endpoint' for the consumer,
  'stream=pipe-writer route=pipe:stdout-to-stdin',
  'read-source=pipe:stdout-to-stdin',
  'read-result=pipe-eof-after-writer-close', producer and consumer
  lifecycle/status records, consumer 'waitpid', consumer 'laststatus',
  descriptor-backed 'cat /etc/banner.txt', final classification
  'qemu-local-shell-minimal-stdout-to-stdin-pipeline-complete', and PASS.
- descriptor redirection controls:
  'tasks/evidence/2026-06-04-phase10-stdout-to-stderr-fd-dup-redirection-core/qemu-local-shell-stdout-to-stderr-redirection-smoke.log',
  'tasks/evidence/2026-06-04-phase10-stderr-to-stdout-fd-dup-redirection-core/qemu-local-shell-stderr-to-stdout-redirection-smoke.log',
  'tasks/evidence/2026-06-04-phase10-stdout-close-redirection-core/qemu-local-shell-stdout-close-redirection-smoke.log', and
  'tasks/evidence/2026-06-04-phase10-stderr-close-redirection-core/qemu-local-shell-stderr-close-redirection-smoke.log'.
  Retained evidence covers child-only descriptor rebinding or close behavior,
  shell restoration, lifecycle/status, 'waitpid', 'laststatus', negative
  redirection controls, descriptor-backed 'cat /etc/banner.txt', and PASS.
- normal userspace stdout/stdin controls:
  'tasks/evidence/2026-06-03-phase10-userspace-stdout-inherited-fd-core/qemu-local-shell-userspace-stdout-smoke.log' and
  'tasks/evidence/2026-06-03-phase10-userspace-stdin-inherited-fd-core/qemu-local-shell-userspace-stdin-smoke.log'.
- stdin readiness and EOF controls:
  'tasks/evidence/2026-06-04-phase10-scheduler-backed-stdin-wait-core/qemu-local-shell-scheduler-backed-stdin-wait-smoke.log' and
  'tasks/evidence/2026-06-04-phase10-terminal-ctrl-d-eof-core/qemu-local-shell-terminal-ctrl-d-eof-smoke.log'.
- descriptor-backed VFS file control:
  'tasks/evidence/2026-06-02-qemu-local-cat-banner-core/qemu-local-cat-banner-smoke.log'.

## Accepted Frontier

Accepted:

- 'exec stdout | exec stdin' is the only accepted pipeline form.
- Both commands launch as VFS-backed '/bin' executables through the accepted
  loader/process/lifecycle path.
- Producer child fd1 is a pipe writer; consumer child fd0 is the matching pipe
  reader.
- Producer bytes written through fd1 are read by the consumer through fd0.
- The pipe reports deterministic EOF after the producer writer closes.
- Shell fd0/fd1/fd2 are restored after the pipeline command completes.
- 'waitpid' and 'laststatus' report the consumer lifecycle for this bounded
  two-stage form.
- Leading pipe, trailing pipe, multi-stage pipe, redirection-mixed pipeline,
  and bad command forms fail deterministically without shrinking accepted exec
  or descriptor-redirection behavior.

Deferred:

- stdout-only stderr-not-piped proof;
- multi-stage pipelines and concurrent scheduling;
- pipefail, background jobs, async execution, fork, signals, and job control;
- file redirection, arbitrary descriptor syntax, writable filesystem behavior,
  Pi 5 proof, networking, and SSH.

## Next Step Requirement

The next mechanically queued feature-led task is the stdout-only pipeline
semantic proof: producer stderr must remain fd2/stderr and must not enter the
consumer fd0 pipe. Promote that task only after this closeout is committed, and
keep it bounded to stdout-only pipe behavior with the accepted minimal
pipeline as the positive control.

## Validation Summary

- static inspection: accepted minimal pipeline task docs and retained evidence
  logs were inspected.
- diff check: 'git diff --check' passed.
- docs: '/home/node/.cargo/bin/mdbook build' passed.
- staged diff check: 'git diff --cached --check' passed before commit.

hardwareTestLock remained unlocked/restored and unused.
