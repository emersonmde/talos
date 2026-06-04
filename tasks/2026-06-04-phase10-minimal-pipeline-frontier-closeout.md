# Phase 10 Minimal Pipeline Frontier Closeout

Task: phase10-minimal-pipeline-frontier-closeout-20260604

Status: accepted

## Summary

This closeout checkpoints the accepted minimal pipeline frontier before any
broader pipe syntax, file/device redirection, descriptor-mixing syntax,
process-control expansion, networking, or SSH work.

The accepted feature boundary is still exactly two shell-visible two-stage
pipeline forms:

- 'exec stdout | exec stdin': producer fd1/stdout is connected to the pipe
  writer, consumer fd0/stdin is the matching pipe reader, and 31 bytes move
  through the pipe.
- 'exec stderr | exec stdin': producer fd1/stdout is still the only pipe
  writer, while producer fd2/stderr remains routed to
  'runtime-console0/stderr'; the consumer reads zero pipe bytes and reports
  deterministic pipe no-data.

Both participants launch through the accepted fixed '/bin' descriptor-backed
VFS exec path. The shell descriptor table is restored after each accepted
pipeline command, and the accepted lifecycle observation for the bounded
pipeline form is the consumer record reported through 'waitpid' and
'laststatus'.

This frontier accepts descriptor ownership, deterministic byte transfer,
writer-close EOF, empty-pipe no-data, stdout-only pipe semantics, shell
descriptor restoration, and consumer lifecycle observation for the exact
two-stage forms above. It does not accept multi-stage pipelines, concurrent
POSIX pipeline scheduling, pipefail, background jobs, async execution, fork,
signals, job control, file/device redirection, arbitrary descriptor syntax,
writable filesystem behavior, Pi 5 proof, networking, or SSH.

## Findings And Disposition

- fixed: Reconciled the minimal stdout-to-stdin pipeline evidence as one exact
  two-stage producer/consumer shell form, not a general pipe scheduler or
  arbitrary pipeline grammar.
- fixed: Reconciled the stderr-not-piped evidence as the durable stdout-only
  pipeline semantic boundary for the accepted exact two-stage forms.
- fixed: Confirmed producer fd1 pipe-writer ownership, consumer fd0 pipe-reader
  ownership, matching 31-byte write/read counts, writer-close EOF, and shell
  fd0/fd1/fd2 restoration for 'exec stdout | exec stdin'.
- fixed: Confirmed 'exec stderr | exec stdin' leaves producer fd2 on the
  inherited stderr route, writes zero pipe bytes, and reports
  'read-result=pipe-eof/no-data' for the consumer.
- fixed: Preserved descriptor-dup and descriptor-close redirection controls,
  normal stdout/stderr route controls, stdin wait/readiness and Ctrl-D EOF
  controls, VFS exec, lifecycle/status, waitpid, laststatus, deterministic
  negative controls, and descriptor-backed 'cat /etc/banner.txt' as retained
  regression coverage.
- fixed: Recorded that the next shell I/O step is not mechanically executable
  by the worker because no explicit queued task remains after this closeout.
- not-an-issue: The accepted pipeline evidence observes the consumer lifecycle
  as the waitable/latest status record for this bounded pipeline form; producer
  and consumer exec summaries remain visible in the transcript, but broad
  multi-child process accounting is still deferred.
- deferred: '2>&1' inside pipelines, explicit stderr piping, multi-stage
  pipelines, concurrent POSIX pipeline scheduling, pipefail, background jobs,
  async execution, fork, signals, job control, file/device redirection,
  arbitrary descriptor syntax, writable filesystem behavior, Pi 5 proof,
  networking, and SSH.

## Evidence Map

- stdout-only stderr-not-piped pipeline:
  'tasks/evidence/2026-06-04-phase10-pipeline-stderr-not-piped-core/qemu-local-shell-pipeline-stderr-not-piped-smoke.log'.
  Static inspection shows 'exec stderr | exec stdin',
  'producer-path=/bin/stderr', 'consumer-path=/bin/stdin',
  'producer-fd=1', fd2 writes with
  'stream=stderr route=runtime-console0/stderr', zero pipe bytes
  written/read, 'writer-closed=true', 'reader-eof=true',
  'shell-restored=true', 'read-result=pipe-eof/no-data', consumer
  'waitpid', consumer 'laststatus', final classification
  'qemu-local-shell-pipeline-stderr-not-piped-complete', and PASS.
- positive stdout-to-stdin pipeline control:
  'tasks/evidence/2026-06-04-phase10-pipeline-stderr-not-piped-core/qemu-local-shell-pipeline-stderr-not-piped-smoke.log'
  also records 'exec stdout | exec stdin', 'producer-path=/bin/stdout',
  'consumer-path=/bin/stdin', 31 bytes written/read through
  'pipe:stdout-to-stdin', 'stream=pipe-writer',
  'read-result=pipe-eof-after-writer-close', shell descriptor restoration,
  descriptor-backed 'cat /etc/banner.txt', errors=0, and PASS.
- original minimal pipeline positive-control evidence:
  'tasks/evidence/2026-06-04-phase10-minimal-stdout-to-stdin-pipe-core/qemu-local-shell-minimal-stdout-to-stdin-pipeline-smoke.log'.
  Static inspection confirms producer fd1 and consumer fd0 pipe endpoints,
  matching 31-byte transfer, writer-close EOF, producer and consumer
  lifecycle/status records, consumer 'waitpid', consumer 'laststatus',
  descriptor-backed 'cat /etc/banner.txt', final classification
  'qemu-local-shell-minimal-stdout-to-stdin-pipeline-complete', and PASS.
- descriptor redirection controls:
  'tasks/evidence/2026-06-04-phase10-stdout-to-stderr-fd-dup-redirection-core/qemu-local-shell-stdout-to-stderr-redirection-smoke.log',
  'tasks/evidence/2026-06-04-phase10-stderr-to-stdout-fd-dup-redirection-core/qemu-local-shell-stderr-to-stdout-redirection-smoke.log',
  'tasks/evidence/2026-06-04-phase10-stdout-close-redirection-core/qemu-local-shell-stdout-close-redirection-smoke.log', and
  'tasks/evidence/2026-06-04-phase10-stderr-close-redirection-core/qemu-local-shell-stderr-close-redirection-smoke.log'.
  Retained evidence covers child-only descriptor rebinding or close behavior,
  shell restoration, lifecycle/status, 'waitpid', 'laststatus',
  deterministic negative redirection controls, descriptor-backed
  'cat /etc/banner.txt', errors=0, and PASS.
- normal stdio and stderr routing controls:
  'tasks/evidence/2026-06-03-phase10-userspace-stdout-inherited-fd-core/qemu-local-shell-userspace-stdout-smoke.log',
  'tasks/evidence/2026-06-03-phase10-userspace-stdin-inherited-fd-core/qemu-local-shell-userspace-stdin-smoke.log',
  'tasks/evidence/2026-06-03-phase10-userspace-stderr-inherited-fd-core/qemu-local-shell-userspace-stderr-smoke.log', and
  'tasks/evidence/2026-06-04-phase10-distinct-stderr-routing-core/qemu-local-shell-distinct-stderr-routing-smoke.log'.
- stdin readiness and EOF controls:
  'tasks/evidence/2026-06-04-phase10-scheduler-backed-stdin-wait-core/qemu-local-shell-scheduler-backed-stdin-wait-smoke.log' and
  'tasks/evidence/2026-06-04-phase10-terminal-ctrl-d-eof-core/qemu-local-shell-terminal-ctrl-d-eof-smoke.log'.
- descriptor-backed VFS file control:
  'tasks/evidence/2026-06-02-qemu-local-cat-banner-core/qemu-local-cat-banner-smoke.log'.

## Accepted Frontier

Accepted:

- the exact two-stage pipeline grammar connects producer stdout only;
- 'exec stdout | exec stdin' transfers producer fd1 bytes through the pipe to
  consumer fd0;
- 'exec stderr | exec stdin' leaves producer stderr on fd2 and out of the
  consumer fd0 pipe;
- empty stdout from the producer side reports deterministic
  'pipe-eof/no-data', distinct from terminal Ctrl-D EOF and runtime-console0
  readiness/no-data;
- both commands launch through the accepted descriptor-backed VFS/userspace
  lifecycle path;
- shell fd0/fd1/fd2 are restored after the pipeline command completes;
- 'waitpid' and 'laststatus' report the consumer lifecycle for the bounded
  two-stage form;
- unsupported descriptor-mixing pipeline forms fail deterministically.

Deferred:

- '2>&1' inside pipelines and explicit stderr piping;
- multi-stage pipelines and concurrent POSIX scheduling;
- pipefail, background jobs, async execution, fork, signals, and job control;
- file/device redirection, arbitrary descriptor syntax, writable filesystem
  behavior, Pi 5 proof, networking, and SSH.

## Next Step Requirement

Supervisor planning is required before the next feature-led shell I/O task.
No explicit queued task remains after this closeout, and the worker must not
invent one.

The strongest local recommendation is an explicit descriptor-mixing pipeline
slice such as 'exec stderr 2>&1 | exec stdin', because it directly follows the
accepted stdout-only pipeline boundary and reuses the accepted descriptor-dup
redirection frontier as a control. A file/device redirection slice needs an
explicit target/sink contract, and broader multi-stage pipelines need a
separate scheduling/status plan.

## Validation Summary

- static inspection: accepted minimal pipeline, stderr-not-piped, descriptor
  redirection, normal stdio, stdin readiness/EOF, lifecycle/status, waitpid,
  laststatus, and descriptor-backed cat task records and evidence logs were
  inspected.
- diff check: 'git diff --check' passed.
- docs: '/home/node/.cargo/bin/mdbook build' passed.
- staged diff check: 'git diff --cached --check' passed before commit.

hardwareTestLock remained unlocked/restored and unused.

Acceptance commit: recorded in durable supervisor state after commit creation.
