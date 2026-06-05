# Phase 10 Multiple Background VFS Exec Records Closeout

Task: phase10-multiple-background-vfs-exec-records-closeout-20260605
Status: accepted
Accepted core commit: 5e1a092cc3492a87a1c2e97f130a709dee814733

## Scope

Close out the accepted multiple-background VFS exec record core without adding
runtime behavior.

Accepted shell-visible forms:

- 'exec /bin/status42 &'
- 'exec /bin/zero &'
- 'jobs'

This closeout reconciles the accepted two-record background accounting evidence
with retained single-background/jobs, foreground lifecycle isolation,
pipeline/file redirection, descriptor-backed file, and deterministic negative
controls. It does not add a stale-entry retention policy, POSIX job-control
commands, signals, process groups, sessions, terminal ownership, fork, Pi 5
hardware proof, networking, SSH, persistent storage, or a phase transition.

## Findings

- fixed: Consolidated the primary multiple-background evidence. The accepted
  transcript launches 'exec /bin/status42 &' and 'exec /bin/zero &' through the
  accepted fixed-/bin VFS/open/read, loader, descriptor inheritance, startup
  ABI, and background accounting path.
- fixed: Confirmed both records have distinct stable identities. '/bin/status42'
  reports job id '0x1', pid '0x100001', and status '0x2a'; '/bin/zero'
  reports job id '0x2', pid '0x100002', and status '0x0'.
- fixed: Confirmed 'jobs' reports both background accounting records together,
  first with the second job still running/pending and later with both records
  completed, observed, and reaped.
- fixed: Confirmed background-only completions do not create foreground
  waitable lifecycle records or replace 'laststatus'. Foreground 'waitpid'
  reports 'no-child' and 'laststatus' reports 'last-process none' until a
  normal foreground 'exec /bin/zero' updates the foreground lifecycle record.
- fixed: Confirmed deterministic async negatives remain in place for malformed
  'exec /bin/status42&' and unsupported 'exec stdout &'.
- fixed: Confirmed retained controls still cover accepted single-background
  exec, jobs/accounting list, pipeline/file redirection, descriptor-backed
  'cat /etc/banner.txt', waitpid, laststatus, errors=0, classifications, and
  PASS markers.
- fixed: Confirmed roadmap language names the accepted boundary as bounded
  local/QEMU multiple-background accounting only and keeps broader POSIX
  process-control claims deferred.
- not-an-issue: No ADR correction is required for this closeout. The accepted
  contract remains a bounded shell-owned accounting table, not a broad process
  table, procfs, or POSIX job-control policy.
- not-an-issue: Pi 5 hardware proof remains outside this QEMU/substitute
  feature slice; hardwareTestLock stayed unlocked/restored and unused.
- deferred: stale-entry clearing/retention beyond retained records,
  fg/bg/kill/disown, signals, process groups, sessions, terminal ownership,
  fork, scheduler fairness proof, true scheduler-concurrent userspace
  execution, background pipelines/redirections, persistent storage, Pi 5 proof,
  networking, SSH, and phase transition.

## Evidence Map

- primary multiple-background evidence:
  'tasks/evidence/2026-06-05-phase10-multiple-background-vfs-exec-records-core/qemu-local-shell-multiple-background-jobs-smoke.log'
  records 'exec /bin/status42 &' and 'exec /bin/zero &' through
  'source=vfs-open-read mode=background', two distinct background records,
  'jobs' output for both records, foreground waitpid/laststatus isolation,
  foreground '/bin/zero' lifecycle controls, deterministic async negatives,
  errors=0,
  'classification=qemu-local-shell-multiple-background-jobs-complete', and
  PASS. This evidence was committed with
  5e1a092cc3492a87a1c2e97f130a709dee814733.
- primary retained-control note:
  'tasks/evidence/2026-06-05-phase10-multiple-background-vfs-exec-records-core/retained-control-inspection.txt'
  maps the accepted multiple-background feature evidence to retained
  single-background/jobs, pipeline/file redirection, descriptor-backed cat,
  waitpid, laststatus, and deterministic negative controls.
- closeout retained-evidence inspection:
  'tasks/evidence/2026-06-05-phase10-multiple-background-vfs-exec-records-closeout/retained-evidence-inspection.txt'
  records the static closeout inspection for the accepted capability,
  boundaries, and deferred risks.
- retained single-background lifecycle evidence:
  'tasks/evidence/2026-06-05-phase10-background-vfs-exec-lifecycle-core/qemu-local-shell-background-vfs-exec-lifecycle-smoke.log'.
- retained jobs/accounting list evidence:
  'tasks/evidence/2026-06-05-phase10-jobs-accounting-list-core/qemu-local-shell-jobs-accounting-list-smoke.log'.
- retained pipeline consumer-output evidence:
  'tasks/evidence/2026-06-05-phase10-pipeline-consumer-output-redirection-core/qemu-local-shell-pipeline-consumer-output-redirection-smoke.log'.
- retained pipeline producer-redirection-away evidence:
  'tasks/evidence/2026-06-05-phase10-pipeline-producer-file-redirection-away-core/qemu-local-shell-pipeline-producer-file-redirection-away-smoke.log'.
- retained waitpid lifecycle evidence:
  'tasks/evidence/2026-06-03-phase10-minimal-waitpid-lifecycle-observation-core/qemu-local-shell-waitpid-lifecycle-smoke.log'.

## Accepted Boundary

Accepted:

- two exact fixed-/bin background VFS exec forms, 'exec /bin/status42 &' and
  'exec /bin/zero &';
- bounded two-record shell-owned accounting table with stable job id, pid,
  command label, running/completed state, pending/completed status, observed
  status, and reaped flag;
- 'jobs' inspection of both retained records without claiming POSIX job
  control;
- foreground waitpid/laststatus isolation after background-only completions;
- normal foreground VFS exec waitpid/laststatus control after the background
  records;
- deterministic unsupported async negatives;
- retained single-background/jobs, VFS exec/open/read/write, descriptor
  inheritance/restoration, pipeline/file redirection, descriptor-backed cat,
  waitpid, and laststatus controls.

Deferred:

- stale-entry clearing/retention policy beyond the accepted retained records;
- fg/bg/kill/disown, process groups, sessions, terminal ownership, signals,
  fork, process-tree/procfs inspection, broad POSIX job control, and scheduling
  fairness proof;
- true scheduler-concurrent userspace execution, background
  pipelines/redirections, pipefail, and multi-stage/concurrent pipeline
  scheduling;
- persistent storage, Pi 5 hardware proof, networking, SSH, and any phase
  transition.

## Next Step

The next mechanically unblocked task is
'phase10-background-jobs-stale-entry-policy-core-20260605'. Keep it limited to
the smallest documented stale/completed background record retention behavior
for the accepted two-record table. Do not infer POSIX job control, process
groups, signals, terminal ownership, fork, hardware proof, networking, SSH, or
a phase transition.

## Validation

- static inspection: accepted multiple-background QEMU/substitute evidence was
  inspected for two background VFS exec launches, distinct stable ids/pids,
  running/completed jobs output, foreground waitpid/laststatus isolation,
  foreground '/bin/zero' controls, deterministic negatives, completion marker,
  errors=0, and PASS.
- static inspection: retained control evidence was inspected for accepted
  single-background exec, jobs/accounting list, pipeline/file redirection,
  descriptor-backed cat, waitpid, laststatus, deterministic negatives,
  PASS/classification markers, and accepted commit references.
- diff check: 'git diff --check' passed.
- docs: '/home/node/.cargo/bin/mdbook build' passed.
- staged diff check: 'git diff --cached --check' passed before commit.

hardwareTestLock remained unlocked/restored and unused.

## Commit

Commit: final multiple-background records closeout commit recorded in
supervisor state.
