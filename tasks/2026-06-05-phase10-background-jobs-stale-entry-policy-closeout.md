# Phase 10 Background Jobs Stale Entry Policy Closeout

Task: phase10-background-jobs-stale-entry-policy-closeout-20260605
Status: accepted
Accepted core commit: a83cdf5d96080d8bf0997158bd0c95db7e1b0c96

## Scope

Close out the accepted background jobs stale-entry policy without adding
runtime behavior.

Accepted shell-visible forms:

- 'exec /bin/status42 &'
- 'exec /bin/zero &'
- 'jobs'
- 'jobs'
- 'jobs'

This closeout reconciles the accepted stale/completed background record
retention rule with retained multiple-background, prior jobs/accounting,
foreground lifecycle isolation, descriptor-backed file, pipeline/redirection,
and deterministic negative controls. It does not add POSIX job-control
commands, signals, process groups, sessions, terminal ownership, fork, Pi 5
hardware proof, networking, SSH, persistent storage, or a phase transition.

## Findings

- fixed: Consolidated the accepted stale-entry evidence. The retained
  transcript records two fixed-/bin background launches through
  'source=vfs-open-read mode=background', then three jobs inspections that
  expose and clear completed/reaped records deterministically.
- fixed: Confirmed the exact accepted retention rule: 'jobs' reports every
  retained record, clears completed/reaped records after the report that
  exposes them, observes one running job completion for the next inspection,
  and eventually reports 'jobs none'.
- fixed: Confirmed the policy preserves distinct multiple-background
  identities and statuses. '/bin/status42' remains job id '0x1', pid
  '0x100001', status '0x2a'; '/bin/zero' remains job id '0x2', pid
  '0x100002', status '0x0'.
- fixed: Confirmed background-only completions do not create foreground
  waitable lifecycle records or replace 'laststatus'. Foreground 'waitpid'
  reports 'no-child' and 'laststatus' reports 'last-process none' until a
  normal foreground 'exec /bin/zero' updates the foreground lifecycle record.
- fixed: Confirmed retained controls still cover prior multiple-background
  records, single-background jobs/accounting, pipeline/file redirection,
  descriptor-backed 'cat /etc/banner.txt', waitpid, laststatus,
  deterministic async/job-control negatives, errors=0, classifications, and
  PASS markers.
- fixed: Confirmed roadmap language names the accepted boundary as a minimal
  shell-owned completed-job retention rule and keeps broader POSIX
  process-control claims deferred.
- not-an-issue: No ADR correction is required for this closeout. The accepted
  contract remains a small shell-owned accounting policy, not a broad process
  table, procfs, or POSIX job-control contract.
- not-an-issue: Pi 5 hardware proof remains outside this QEMU/substitute
  closeout; hardwareTestLock stayed unlocked/restored and unused.
- deferred: fg/bg/kill/disown, signals, process groups, sessions, terminal
  ownership, fork, scheduler fairness proof, true scheduler-concurrent
  userspace execution, background pipelines/redirections, persistent storage,
  Pi 5 proof, networking, SSH, and phase transition.

## Evidence Map

- primary stale-entry policy evidence:
  'tasks/evidence/2026-06-05-phase10-background-jobs-stale-entry-policy-core/qemu-local-shell-background-jobs-stale-entry-policy-smoke.log'
  records 'exec /bin/status42 &' and 'exec /bin/zero &' through
  'source=vfs-open-read mode=background', a first jobs report with both
  retained records, a second jobs report with only the completed '/bin/zero'
  record, a later 'jobs none' report, foreground waitpid/laststatus
  isolation, foreground '/bin/zero' lifecycle controls, deterministic async
  negatives, errors=0,
  'classification=qemu-local-shell-background-jobs-stale-entry-policy-complete',
  and PASS. This evidence was committed with
  a83cdf5d96080d8bf0997158bd0c95db7e1b0c96.
- primary retained-control note:
  'tasks/evidence/2026-06-05-phase10-background-jobs-stale-entry-policy-core/retained-control-inspection.txt'
  maps the accepted stale-entry feature evidence to retained
  multiple-background, prior jobs/accounting, foreground lifecycle,
  descriptor-backed cat, pipeline/file-redirection, and deterministic negative
  controls.
- closeout retained-evidence inspection:
  'tasks/evidence/2026-06-05-phase10-background-jobs-stale-entry-policy-closeout/retained-evidence-inspection.txt'
  records the static closeout inspection for the accepted capability,
  boundaries, and deferred risks.
- retained multiple-background evidence:
  'tasks/evidence/2026-06-05-phase10-multiple-background-vfs-exec-records-core/qemu-local-shell-multiple-background-jobs-smoke.log'.
- retained jobs/accounting list evidence:
  'tasks/evidence/2026-06-05-phase10-jobs-accounting-list-core/qemu-local-shell-jobs-accounting-list-smoke.log'.
- retained multiple-background closeout:
  'tasks/evidence/2026-06-05-phase10-multiple-background-vfs-exec-records-closeout/retained-evidence-inspection.txt'.
- retained pipeline/file-redirection frontier control:
  'tasks/evidence/2026-06-05-phase10-pipeline-file-redirection-frontier-closeout/retained-evidence-inspection.txt'.
- retained waitpid lifecycle evidence:
  'tasks/evidence/2026-06-03-phase10-minimal-waitpid-lifecycle-observation-core/qemu-local-shell-waitpid-lifecycle-smoke.log'.

## Accepted Boundary

Accepted:

- two exact fixed-/bin background VFS exec forms, 'exec /bin/status42 &' and
  'exec /bin/zero &';
- bounded shell-owned background accounting records with stable job id, pid,
  command label, running/completed state, pending/completed status, observed
  status, and reaped flag;
- minimal stale-entry policy where completed/reaped records are visible for
  one jobs report and then cleared deterministically;
- 'jobs none' after all completed background records have been reported and
  cleared;
- foreground waitpid/laststatus isolation after background-only completions;
- normal foreground VFS exec waitpid/laststatus control after the background
  records;
- deterministic unsupported async/job-control negatives;
- retained single-background/jobs, multiple-background, VFS exec/open/read,
  descriptor inheritance/restoration, pipeline/file redirection,
  descriptor-backed cat, waitpid, and laststatus controls.

Deferred:

- fg/bg/kill/disown, process groups, sessions, terminal ownership, signals,
  fork, process-tree/procfs inspection, broad POSIX job control, and
  scheduling fairness proof;
- true scheduler-concurrent userspace execution, background
  pipelines/redirections, pipefail, and multi-stage/concurrent pipeline
  scheduling;
- persistent storage, Pi 5 hardware proof, networking, SSH, and any phase
  transition.

## Next Step

The next mechanically unblocked queued task is
'phase10-process-control-frontier-checkpoint-20260605'. Keep it limited to
static evidence reconciliation for the Milestone 10.2 process-control frontier
and require supervisor planning before any milestone closeout, local storage,
hardware proof, networking, SSH, or phase transition.

## Validation

- static inspection: accepted stale-entry QEMU/substitute evidence was
  inspected for two background VFS exec launches, distinct stable ids/pids,
  first/second/third jobs inspections, deterministic stale-entry clearing,
  foreground waitpid/laststatus isolation, foreground '/bin/zero' controls,
  deterministic negatives, completion marker, errors=0, and PASS.
- static inspection: retained control evidence was inspected for accepted
  multiple-background records, prior jobs/accounting, pipeline/file
  redirection, descriptor-backed cat, waitpid, laststatus, deterministic
  negatives, PASS/classification markers, and accepted commit references.
- diff check: 'git diff --check' passed.
- docs: '/home/node/.cargo/bin/mdbook build' passed.
- staged diff check: 'git diff --cached --check' passed before commit.

hardwareTestLock remained unlocked/restored and unused.

## Commit

Commit: final stale-entry policy closeout commit recorded in supervisor state.
