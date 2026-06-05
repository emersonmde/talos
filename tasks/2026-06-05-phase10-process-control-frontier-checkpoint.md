# Phase 10 Process-Control Frontier Checkpoint

Task: phase10-process-control-frontier-checkpoint-20260605
Status: accepted

## Scope

Checkpoint the accepted Phase 10 Milestone 10.2 pipelines and process-control
frontier without adding runtime behavior.

Accepted shell-visible surfaces covered by this checkpoint:

- exact two-stage stdout-to-stdin pipelines and accepted pipeline/redirection
  compositions;
- descriptor-backed VFS exec, waitpid, laststatus, and descriptor inheritance
  controls;
- exact fixed-/bin background VFS exec forms, 'exec /bin/status42 &' and
  'exec /bin/zero &';
- 'jobs' as a bounded shell-owned background accounting inspection surface;
- completed/reaped background record retention where completed jobs are visible
  for one jobs report and then cleared deterministically.

This checkpoint does not add code, run Pi 5 hardware, acquire hardwareTestLock,
close Milestone 10.2 by implication, transition to Milestone 10.3 or Phase 11,
add persistent storage, networking, SSH, signals, process groups, sessions,
terminal ownership, fork, or fake/kernel-backed command expansion beyond
accepted regression/control surfaces.

## Findings

- fixed: Consolidated the accepted pipeline frontier. The retained evidence
  covers exact two-stage stdout-to-stdin transfer, stdout-only stderr behavior,
  descriptor-mixing pipeline forms, consumer stdout file redirection, producer
  stdout redirection away from the pipe, descriptor-backed readbacks, endpoint
  closure/restoration, waitpid, laststatus, errors=0, classifications, and
  PASS markers.
- fixed: Consolidated accepted file-redirection composition boundaries.
  Volatile '/tmp/<basename>' stdout/stderr sinks, append/create behavior,
  explicit fd1 redirection, combined stdin/stdout redirection, and pipeline
  plus file-redirection forms are documented as local volatile VFS behavior, not
  persistent filesystem semantics.
- fixed: Consolidated the accepted background VFS exec lifecycle. The evidence
  records fixed-/bin background launches through 'source=vfs-open-read
  mode=background', stable shell-owned job ids and pids, command labels,
  running/completed lifecycle states, pending/completed status values,
  observed-status fields, and reaped flags.
- fixed: Consolidated foreground lifecycle isolation. Background-only
  completions leave foreground 'waitpid' at 'no-child' and 'laststatus' at
  'last-process none' until a normal foreground exec updates the foreground
  lifecycle record.
- fixed: Consolidated the accepted stale-entry rule. 'jobs' reports retained
  completed/reaped records once, clears them after the report that exposes them,
  observes later background completion on a subsequent inspection, and then
  reports 'jobs none' after all completed records have been exposed and
  removed.
- fixed: Explicitly scoped the milestone acceptance statement: at the accepted
  local/QEMU frontier, multiple VFS-backed user program records can be launched
  through the shell-owned background path and complete while the command loop
  remains responsive enough to inspect jobs and run later commands. This does
  not prove preemptive scheduler-concurrent userspace execution or full POSIX
  job control.
- not-an-issue: No ADR update is needed for this checkpoint. It summarizes
  accepted evidence and defers expensive process-control policy rather than
  creating a new policy.
- not-an-issue: Pi 5 hardware proof remains outside this local/QEMU
  checkpoint; hardwareTestLock stayed unlocked/restored and unused.
- deferred: true scheduler-concurrent userspace execution, multi-stage or
  concurrent pipelines, pipefail, background pipelines/redirections,
  fg/bg/kill/disown, process groups, sessions, terminal ownership, signals,
  fork, process-tree/procfs inspection, scheduling fairness proof, close-on-exec
  expansion beyond accepted descriptor restoration controls, persistent/larger
  local storage, Pi 5 proof, networking, SSH, Milestone 10.3, Phase 11, and any
  phase transition.

## Evidence Map

- pipeline file-redirection frontier:
  'tasks/2026-06-05-phase10-pipeline-file-redirection-frontier-closeout.md'
  reconciles accepted consumer-output and producer-output-away pipeline/file
  redirection evidence. The primary logs are
  'tasks/evidence/2026-06-05-phase10-pipeline-consumer-output-redirection-core/qemu-local-shell-pipeline-consumer-output-redirection-smoke.log'
  and
  'tasks/evidence/2026-06-05-phase10-pipeline-producer-file-redirection-away-core/qemu-local-shell-pipeline-producer-file-redirection-away-smoke.log'.
- retained baseline pipeline controls:
  'tasks/evidence/2026-06-04-phase10-minimal-stdout-to-stdin-pipe-core/qemu-local-shell-minimal-stdout-to-stdin-pipeline-smoke.log',
  'tasks/evidence/2026-06-04-phase10-pipeline-stderr-not-piped-core/qemu-local-shell-pipeline-stderr-not-piped-smoke.log',
  'tasks/evidence/2026-06-04-phase10-pipeline-stderr-dup-to-stdout-core/qemu-local-shell-pipeline-stderr-dup-to-stdout-smoke.log',
  and
  'tasks/evidence/2026-06-04-phase10-pipeline-stdout-redirect-away-core/qemu-local-shell-pipeline-stdout-redirect-away-smoke.log'.
- retained file-redirection controls:
  'tasks/evidence/2026-06-04-phase10-stdout-arbitrary-tmp-output-redirection-core/qemu-local-shell-stdout-arbitrary-tmp-output-redirection-smoke.log',
  'tasks/evidence/2026-06-04-phase10-stderr-arbitrary-tmp-output-redirection-core/qemu-local-shell-stderr-arbitrary-tmp-output-redirection-smoke.log',
  'tasks/evidence/2026-06-05-phase10-combined-stdin-stdout-redirection-core/qemu-local-shell-combined-stdin-stdout-redirection-smoke.log',
  and the earlier descriptor dup/close, /dev/null, regular-file, append/create,
  explicit-fd, stdin-redirection, descriptor-backed cat, waitpid, and
  laststatus controls referenced by the closeout records.
- background VFS exec lifecycle:
  'tasks/evidence/2026-06-05-phase10-background-vfs-exec-lifecycle-core/qemu-local-shell-background-vfs-exec-lifecycle-smoke.log'
  records 'exec /bin/status42 &' through the accepted VFS exec path, shell
  responsiveness, completed status 0x2a, foreground waitpid/laststatus
  isolation, foreground '/bin/zero' controls, deterministic negatives,
  errors=0, classification, and PASS.
- jobs/accounting list:
  'tasks/evidence/2026-06-05-phase10-jobs-accounting-list-core/qemu-local-shell-jobs-accounting-list-smoke.log'
  records 'jobs none', running and completed '/bin/status42' records,
  foreground lifecycle isolation, foreground '/bin/zero' controls, deterministic
  fg/bg/kill negatives, errors=0, classification, and PASS.
- multiple background records:
  'tasks/evidence/2026-06-05-phase10-multiple-background-vfs-exec-records-core/qemu-local-shell-multiple-background-jobs-smoke.log'
  records 'exec /bin/status42 &' and 'exec /bin/zero &' with distinct stable
  job ids and pids, status values 0x2a and 0x0, foreground lifecycle isolation,
  deterministic async negatives, errors=0, classification, and PASS.
- stale-entry policy:
  'tasks/evidence/2026-06-05-phase10-background-jobs-stale-entry-policy-core/qemu-local-shell-background-jobs-stale-entry-policy-smoke.log'
  records first/second/third jobs inspections proving completed-entry exposure
  and deterministic clearing, foreground lifecycle isolation, deterministic
  negatives, errors=0, classification, and PASS.
- checkpoint retained-evidence inspection:
  'tasks/evidence/2026-06-05-phase10-process-control-frontier-checkpoint/retained-evidence-inspection.txt'
  records the static evidence review for this checkpoint.

## Accepted Frontier

Accepted:

- exact two-stage local/QEMU pipelines and documented pipeline/redirection
  compositions;
- descriptor-backed VFS exec/open/read/write controls, descriptor inheritance
  and restoration, waitpid, laststatus, and descriptor-backed cat controls;
- exact fixed-/bin background VFS exec records for '/bin/status42' and
  '/bin/zero';
- bounded shell-owned jobs accounting with stable job ids, pids, command
  labels, running/completed state, pending/completed status, observed-status,
  and reaped flags;
- minimal completed-job stale-entry policy with deterministic one-report
  exposure and clearing;
- command-loop responsiveness at the accepted local/QEMU boundary after
  background launches and accounting observations;
- deterministic unsupported async/job-control negatives.

Deferred before broader POSIX claims:

- preemptive or otherwise proven scheduler-concurrent userspace execution;
- POSIX job control: fg/bg/kill/disown, process groups, sessions, terminal
  ownership, signal delivery, fork, and process-tree/procfs inspection;
- multi-stage or concurrent pipelines, pipefail, background
  pipelines/redirections, and scheduling fairness proof;
- persistent/larger local storage, Pi 5 hardware proof, networking, SSH, and
  any phase transition.

## Recommendation

Supervisor planning should treat Milestone 10.2 closeout as the next plausible
step only if it explicitly names the accepted local/QEMU frontier and keeps
true scheduler-concurrent userspace execution and POSIX job control deferred.
If the supervisor wants the milestone acceptance text to require a stronger
multiple-program progress claim, the next task should be one more bounded local
process-control proof before closeout. No Matthew decision is required by this
checkpoint.

## Validation

- static inspection: accepted pipeline and pipeline/file-redirection evidence
  was inspected through closeout records and task-owned QEMU/substitute logs for
  PASS/classification markers, endpoint routing, descriptor-backed readbacks,
  waitpid, laststatus, deterministic negatives, and errors=0.
- static inspection: accepted background lifecycle, jobs/accounting,
  multiple-background, and stale-entry QEMU/substitute evidence was inspected
  for VFS exec provenance, stable ids/pids, command labels, lifecycle/status
  fields, foreground waitpid/laststatus isolation, deterministic negatives,
  errors=0, classifications, and PASS.
- static inspection: roadmap language was inspected for accepted/deferred
  boundary consistency and updated for this checkpoint.
- diff check: 'git diff --check' passed.
- docs: '/home/node/.cargo/bin/mdbook build' passed.
- staged diff check: 'git diff --cached --check' passed before commit.

hardwareTestLock remained unlocked/restored and unused.

## Commit

Commit: final process-control frontier checkpoint commit recorded in supervisor
state.
