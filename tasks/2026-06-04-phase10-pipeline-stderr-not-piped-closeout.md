# Phase 10 Pipeline Stderr Not Piped Closeout

Task: phase10-pipeline-stderr-not-piped-closeout-20260604

Status: accepted

## Summary

This closeout checkpoints the accepted stdout-only pipeline semantic before
broader pipe syntax, redirection mixing, scheduling, file redirection, or
process-control work.

The accepted feature boundary now includes two exact shell-visible pipeline
forms:

- 'exec stdout | exec stdin': producer fd1 is a pipe writer, consumer fd0 is
  the matching pipe reader, and 31 bytes move through the pipe.
- 'exec stderr | exec stdin': producer fd1 is still the only pipe writer, but
  the stderr fixture writes through fd2 to the inherited stderr route, so the
  consumer reads zero pipe bytes and reports deterministic pipe no-data.

Both participants launch through the accepted fixed '/bin' descriptor-backed
VFS exec path. Shell descriptors are restored after the pipeline command, and
the accepted lifecycle observation remains the consumer record reported by
'waitpid' and 'laststatus' for this bounded two-stage form.

The stdout-only rule is a durable architectural boundary for this phase slice:
the pipeline operator connects producer stdout only. Stderr enters the pipe
only after a later explicit task accepts descriptor mixing such as '2>&1'
inside a pipeline.

## Findings And Disposition

- fixed: Reconciled the accepted stderr-not-piped core as a stdout-only
  pipeline semantic boundary, not a general pipeline or descriptor-mixing
  boundary.
- fixed: Preserved producer fd1 as the only pipe writer endpoint and producer
  fd2 as the inherited stderr route for 'exec stderr | exec stdin'.
- fixed: Preserved deterministic empty-pipe behavior as
  'read-result=pipe-eof/no-data', distinct from terminal Ctrl-D EOF and
  runtime-console0 readiness/no-data.
- fixed: Preserved the positive 'exec stdout | exec stdin' control with
  matching 31-byte write/read counts and writer-close EOF.
- fixed: Preserved normal stderr routing, descriptor dup/close redirection,
  shell descriptor restoration, stdin wait/readiness and EOF, lifecycle/status,
  negative command, and descriptor-backed cat controls as retained regression
  coverage.
- fixed: Added an ADR index entry because the stdout-only pipeline rule
  constrains later POSIX-compatible pipeline/redirection ordering.
- deferred: '2>&1' inside pipelines, explicit stderr piping, pipefail,
  multi-stage pipelines, concurrent POSIX pipeline scheduling, background jobs,
  async execution, fork, signals, job control, file redirection, arbitrary
  descriptor syntax, writable filesystem behavior, Pi 5 proof, networking, and
  SSH remain out of scope.

## Evidence Map

- stderr-not-piped pipeline:
  'tasks/evidence/2026-06-04-phase10-pipeline-stderr-not-piped-core/qemu-local-shell-pipeline-stderr-not-piped-smoke.log'.
  Static inspection of QEMU/substitute evidence shows
  'exec stderr | exec stdin', producer path '/bin/stderr', consumer path
  '/bin/stdin', producer fd1 as the pipeline stdout endpoint, producer fd2
  writing 'stream=stderr route=runtime-console0/stderr', zero pipe bytes
  written/read, 'writer-closed=true', 'reader-eof=true',
  'shell-restored=true', 'read-result=pipe-eof/no-data', consumer 'waitpid',
  consumer 'laststatus', final classification
  'qemu-local-shell-pipeline-stderr-not-piped-complete', and PASS.
- positive stdout-to-stdin pipeline control:
  'tasks/evidence/2026-06-04-phase10-pipeline-stderr-not-piped-core/qemu-local-shell-pipeline-stderr-not-piped-smoke.log'
  also records 'exec stdout | exec stdin' with producer path '/bin/stdout',
  consumer path '/bin/stdin', 31 bytes written/read through
  'pipe:stdout-to-stdin', 'stream=pipe-writer', writer-close EOF,
  shell descriptor restoration, and descriptor-backed 'cat /etc/banner.txt'
  after both pipeline forms.
- original minimal pipeline positive-control evidence:
  'tasks/evidence/2026-06-04-phase10-minimal-stdout-to-stdin-pipe-core/qemu-local-shell-minimal-stdout-to-stdin-pipeline-smoke.log'.
  Static inspection confirms the accepted minimal pipe frontier with matching
  write/read counts, producer and consumer lifecycle/status records, consumer
  'waitpid' and 'laststatus', descriptor-backed 'cat /etc/banner.txt', and
  PASS.
- normal stderr routing control:
  'tasks/evidence/2026-06-04-phase10-distinct-stderr-routing-core/qemu-local-shell-distinct-stderr-routing-smoke.log'
  records normal 'exec stderr' writes through
  'stream=stderr route=runtime-console0/stderr' and PASS.
- descriptor redirection controls:
  'tasks/evidence/2026-06-04-phase10-stdout-to-stderr-fd-dup-redirection-core/qemu-local-shell-stdout-to-stderr-redirection-smoke.log',
  'tasks/evidence/2026-06-04-phase10-stderr-to-stdout-fd-dup-redirection-core/qemu-local-shell-stderr-to-stdout-redirection-smoke.log',
  'tasks/evidence/2026-06-04-phase10-stdout-close-redirection-core/qemu-local-shell-stdout-close-redirection-smoke.log', and
  'tasks/evidence/2026-06-04-phase10-stderr-close-redirection-core/qemu-local-shell-stderr-close-redirection-smoke.log'.
- stdin readiness and EOF controls:
  'tasks/evidence/2026-06-04-phase10-scheduler-backed-stdin-wait-core/qemu-local-shell-scheduler-backed-stdin-wait-smoke.log' and
  'tasks/evidence/2026-06-04-phase10-terminal-ctrl-d-eof-core/qemu-local-shell-terminal-ctrl-d-eof-smoke.log'.
- descriptor-backed VFS file control:
  'tasks/evidence/2026-06-02-qemu-local-cat-banner-core/qemu-local-cat-banner-smoke.log'.

## Accepted Frontier

Accepted:

- The exact two-stage pipeline grammar connects producer stdout only.
- 'exec stdout | exec stdin' transfers producer fd1 bytes through the pipe to
  consumer fd0.
- 'exec stderr | exec stdin' leaves producer stderr on fd2 and out of the
  consumer fd0 pipe.
- Empty stdout from the producer side reports deterministic
  'pipe-eof/no-data' to the consumer, distinct from terminal EOF and runtime
  readiness/no-data.
- Both commands launch through the accepted descriptor-backed VFS/userspace
  lifecycle path.
- Shell fd0/fd1/fd2 are restored after the pipeline command completes.
- 'waitpid' and 'laststatus' report the consumer lifecycle for this bounded
  two-stage form.
- Unsupported descriptor-mixing pipeline forms fail deterministically.

Deferred:

- '2>&1' inside pipelines and explicit stderr piping;
- multi-stage pipelines and concurrent scheduling;
- pipefail, background jobs, async execution, fork, signals, and job control;
- file redirection, arbitrary descriptor syntax, writable filesystem behavior,
  Pi 5 proof, networking, and SSH.

## Next Step Requirement

The next mechanically queued task is the minimal pipeline frontier closeout.
Promote it only after this closeout is committed, keep it docs/evidence only,
and do not infer broader pipeline syntax, redirection mixing, file redirection,
writable filesystem behavior, networking, SSH, or a phase transition.

## Validation Summary

- static inspection: accepted stderr-not-piped task docs and retained evidence
  logs were inspected.
- diff check: 'git diff --check' passed.
- docs: '/home/node/.cargo/bin/mdbook build' passed.
- staged diff check: 'git diff --cached --check' passed before commit.

hardwareTestLock remained unlocked/restored and unused.
