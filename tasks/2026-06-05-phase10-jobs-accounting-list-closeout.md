# Phase 10 Jobs/Accounting List Closeout

Task: phase10-jobs-accounting-list-closeout-20260605
Status: accepted
Accepted core commit: c06926cf8bc9533ff7ec76a5579fbb6e64c480cd

## Scope

Close out the accepted jobs/accounting list core without adding runtime
behavior.

Accepted shell-visible forms:

- 'jobs'
- retained background launch control: 'exec /bin/status42 &'

This closeout reconciles the accepted evidence and documentation for the
minimal one-record shell-owned accounting inspection surface. It does not add
new job-control commands, add signals, process groups, sessions, terminal
ownership, multiple jobs, true scheduler-concurrent userspace execution, Pi 5
hardware proof, networking, SSH, or a phase transition.

## Findings

- fixed: Consolidated the primary jobs/accounting list evidence. The accepted
  transcript records 'jobs none' before launch, then stable id/pid/command
  reporting for '/bin/status42' after 'exec /bin/status42 &'.
- fixed: Confirmed the running accounting observation reports
  'state=running', 'status=pending', and 'reaped=false' from the accepted
  background VFS exec accounting record.
- fixed: Confirmed the completed accounting observation reports the same
  stable id/pid/command with 'state=completed', status '0x2a',
  'observed-status=0x2a', and 'reaped=true'.
- fixed: Confirmed background jobs/accounting inspection does not create or
  consume foreground lifecycle records: foreground 'waitpid' reports
  'no-child' and 'laststatus' reports 'last-process none' after the background
  accounting observations.
- fixed: Confirmed foreground lifecycle controls remain coherent after the
  background accounting slice: foreground 'exec /bin/zero' still produces the
  accepted consuming 'waitpid' and non-consuming 'laststatus' records.
- fixed: Confirmed unsupported job-control commands 'fg', 'bg', and 'kill %1'
  remain deterministic unknown-command negatives.
- fixed: Confirmed retained controls still cover accepted background exec,
  waitpid, laststatus, pipeline/file redirection, descriptor inheritance, and
  descriptor-backed 'cat /etc/banner.txt' behavior with PASS/classification
  markers.
- fixed: Confirmed roadmap language names the accepted boundary precisely and
  keeps full POSIX job control, scheduling fairness, terminal ownership,
  process groups, signals, multiple jobs, Pi 5 proof, networking, SSH, and
  phase transition deferred.
- not-an-issue: No ADR correction is required for this closeout. The accepted
  contract remains a narrow shell-owned accounting inspection surface, not a
  broad process table, procfs, or POSIX job-control policy.
- not-an-issue: Pi 5 hardware proof remains outside this QEMU/substitute
  feature slice; hardwareTestLock stayed unlocked/restored and unused.
- deferred: multiple jobs, kill/fg/bg/disown semantics, signals, process
  groups, sessions, terminal ownership, process-tree/procfs inspection, true
  scheduler-concurrent userspace execution, background pipelines/redirections,
  scheduling fairness proof, Pi 5 proof, networking, SSH, and phase transition.

## Evidence Map

- primary jobs/accounting list evidence:
  'tasks/evidence/2026-06-05-phase10-jobs-accounting-list-core/qemu-local-shell-jobs-accounting-list-smoke.log'
  records 'jobs none', 'exec /bin/status42 &' through the accepted VFS exec
  provenance, a running jobs record, a completed/reaped jobs record, foreground
  waitpid/laststatus isolation, foreground '/bin/zero' lifecycle controls,
  plain pipeline transfer, descriptor-backed 'cat /etc/banner.txt',
  deterministic unsupported job-control negatives, errors=0,
  'classification=qemu-local-shell-jobs-accounting-list-complete', and PASS.
  This evidence was committed with
  c06926cf8bc9533ff7ec76a5579fbb6e64c480cd.
- retained control inspection:
  'tasks/evidence/2026-06-05-phase10-jobs-accounting-list-core/retained-control-inspection.txt'
  records PASS/classification markers for the task-owned jobs evidence,
  accepted background exec, pipeline consumer-output redirection, pipeline
  producer-output-away redirection, and waitpid lifecycle controls.
- retained background exec lifecycle evidence:
  'tasks/evidence/2026-06-05-phase10-background-vfs-exec-lifecycle-core/qemu-local-shell-background-vfs-exec-lifecycle-smoke.log'.
- retained pipeline consumer-output evidence:
  'tasks/evidence/2026-06-05-phase10-pipeline-consumer-output-redirection-core/qemu-local-shell-pipeline-consumer-output-redirection-smoke.log'.
- retained pipeline producer-redirection-away evidence:
  'tasks/evidence/2026-06-05-phase10-pipeline-producer-file-redirection-away-core/qemu-local-shell-pipeline-producer-file-redirection-away-smoke.log'.
- retained waitpid lifecycle evidence:
  'tasks/evidence/2026-06-03-phase10-minimal-waitpid-lifecycle-observation-core/qemu-local-shell-waitpid-lifecycle-smoke.log'.

## Accepted Boundary

Accepted:

- 'jobs' as a minimal shell-owned accounting inspection command;
- 'jobs none' before a background launch;
- one stable background accounting record for 'exec /bin/status42 &' with job
  id, pid, command label, running/completed state, pending/completed status,
  observed status, and reaped flag;
- foreground lifecycle isolation for 'waitpid' and 'laststatus' after
  background accounting observation;
- normal foreground VFS exec waitpid/laststatus control after the jobs slice;
- deterministic unsupported 'fg', 'bg', and 'kill %1' negatives;
- retained background exec, pipeline/file redirection, descriptor inheritance,
  waitpid, laststatus, and descriptor-backed cat controls.

Deferred:

- multiple jobs and stale-entry policy beyond the single accepted record;
- kill/fg/bg/disown, process groups, sessions, terminal ownership, signals,
  fork, process-tree/procfs inspection, and broad POSIX job control;
- true scheduler-concurrent userspace execution and scheduling fairness;
- background pipelines/redirections, pipefail, and multi-stage/concurrent
  pipeline scheduling;
- Pi 5 hardware proof, networking, SSH, and any phase transition.

## Next Step

The next mechanically unblocked task is
'phase10-async-process-control-frontier-closeout-20260605'. Keep it limited to
frontier evidence reconciliation for the accepted background launch and
jobs/accounting inspection slices, then require supervisor planning before any
further process-control expansion.

## Validation

- static inspection: accepted jobs/accounting list QEMU/substitute evidence
  was inspected, including no-job, running, completed/reaped, foreground
  waitpid/laststatus isolation, foreground '/bin/zero' controls, pipeline/cat
  controls, deterministic negatives, completion marker, errors=0, and PASS.
- static inspection: retained control evidence was inspected for accepted
  background exec, pipeline/file redirection, waitpid, laststatus, descriptor
  inheritance, descriptor-backed cat, PASS/classification markers, and accepted
  commit references.
- diff check: 'git diff --check' passed.
- docs: '/home/node/.cargo/bin/mdbook build' passed.
- staged diff check: 'git diff --cached --check' passed before commit.

hardwareTestLock remained unlocked/restored and unused.

## Commit

Commit: final jobs/accounting list closeout commit recorded in supervisor
state.
