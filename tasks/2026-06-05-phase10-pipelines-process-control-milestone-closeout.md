# Phase 10 Pipelines and Process-Control Milestone Closeout

Task: phase10-pipelines-process-control-milestone-closeout-20260605
Status: accepted

## Scope

Close out Milestone 10.2 at the accepted local/QEMU frontier without adding
runtime behavior.

This closeout reconciles accepted shell-visible evidence for:

- exact two-stage pipelines and pipeline plus volatile-file redirection
  composition;
- VFS-backed exec/open/read/write lineage, descriptor inheritance/restoration,
  loader temporary descriptor non-leak, waitpid, laststatus, and
  descriptor-backed cat controls;
- exact fixed-/bin background VFS exec records for '/bin/status42' and
  '/bin/zero';
- shell-owned 'jobs' accounting with distinct job ids, pids, command labels,
  lifecycle state, status, observed-status, and reaped flags;
- deterministic completed-job stale-entry exposure and clearing.

This closeout does not claim true scheduler-concurrent userspace execution,
full POSIX job control, process groups, sessions, terminal ownership, signals,
fork, background pipelines/redirections, multi-stage/concurrent pipelines,
pipefail, persistent or larger local storage, Pi 5 hardware proof, networking,
SSH, Milestone 10.3 implementation, Phase 11, or any phase transition.

## Findings

- fixed: Closed Milestone 10.2 explicitly as a local/QEMU frontier. The
  accepted evidence proves simple pipelines, descriptor/file redirection
  composition, exit-status observation, waitpid/laststatus controls, descriptor
  inheritance/restoration, background VFS exec accounting, multiple background
  records, and stale-entry clearing at the current command-loop boundary.
- fixed: Mapped simple pipeline evidence to retained QEMU/substitute logs for
  stdout-to-stdin transfer, stdout-only stderr behavior, descriptor-mixing
  forms, consumer-output redirection, and producer-output-away redirection.
- fixed: Mapped exit-status and lifecycle evidence to retained waitpid,
  laststatus, foreground VFS exec, and background accounting records with
  deterministic zero and nonzero statuses.
- fixed: Mapped descriptor evidence to retained standard fd0/fd1/fd2
  inheritance, child-only redirection/restoration, pipeline endpoint routing,
  volatile VFS file readbacks, and loader temporary descriptor non-leak
  records.
- fixed: Mapped the accepted multiple-program progress claim narrowly:
  multiple fixed-/bin VFS-backed user program records can be launched through
  the background path, completed, inspected through jobs, and cleared while the
  command loop remains responsive for later inspections and commands.
- fixed: Roadmap language now records Milestone 10.2 as accepted at this
  local/QEMU frontier and keeps Milestone 10.3 as a subsequent explicit plan.
- not-an-issue: No ADR update is needed for this closeout; it summarizes
  accepted behavior and deferred boundaries rather than codifying a new
  expensive-to-reverse process-control policy.
- not-an-issue: Pi 5 proof is outside this docs/evidence-only closeout;
  hardwareTestLock stayed unlocked/restored and unused.
- deferred: true scheduler-concurrent userspace execution, POSIX job control
  beyond the bounded jobs inspection surface, close-on-exec expansion beyond
  accepted loader-temp non-leak/restoration controls, multi-stage/concurrent
  pipelines, pipefail, background pipelines/redirections, fg/bg/kill/disown,
  process groups, sessions, terminal ownership, signals, fork,
  process-tree/procfs inspection, scheduling fairness proof, persistent/larger
  local storage, Pi 5 proof, networking, SSH, Milestone 10.3 implementation,
  Phase 11, and phase transition.

## Evidence Map

- milestone checkpoint:
  'tasks/2026-06-05-phase10-process-control-frontier-checkpoint.md' and
  'tasks/evidence/2026-06-05-phase10-process-control-frontier-checkpoint/retained-evidence-inspection.txt'
  reconcile the accepted Milestone 10.2 frontier and the narrow
  multiple-program progress claim.
- simple pipeline controls:
  'tasks/evidence/2026-06-04-phase10-minimal-stdout-to-stdin-pipe-core/qemu-local-shell-minimal-stdout-to-stdin-pipeline-smoke.log'
  records exact stdout-to-stdin transfer with errors=0,
  'classification=qemu-local-shell-minimal-stdout-to-stdin-pipeline-complete',
  and PASS.
- pipeline stderr and descriptor-mixing controls:
  'tasks/evidence/2026-06-04-phase10-pipeline-stderr-not-piped-core/qemu-local-shell-pipeline-stderr-not-piped-smoke.log',
  'tasks/evidence/2026-06-04-phase10-pipeline-stderr-dup-to-stdout-core/qemu-local-shell-pipeline-stderr-dup-to-stdout-smoke.log',
  and
  'tasks/evidence/2026-06-04-phase10-pipeline-stdout-redirect-away-core/qemu-local-shell-pipeline-stdout-redirect-away-smoke.log'
  retain stdout-only stderr behavior and exact descriptor-mixing pipeline
  directions.
- pipeline and volatile-file redirection frontier:
  'tasks/2026-06-05-phase10-pipeline-file-redirection-frontier-closeout.md'
  maps
  'tasks/evidence/2026-06-05-phase10-pipeline-consumer-output-redirection-core/qemu-local-shell-pipeline-consumer-output-redirection-smoke.log'
  and
  'tasks/evidence/2026-06-05-phase10-pipeline-producer-file-redirection-away-core/qemu-local-shell-pipeline-producer-file-redirection-away-smoke.log'
  to accepted consumer-output and producer-output-away behavior.
- descriptor inheritance and loader-temp non-leak:
  'tasks/evidence/2026-06-03-phase10-standard-descriptor-inheritance-exec-core/qemu-local-shell-standard-descriptor-inheritance-smoke.log'
  retains standard fd inheritance and 'loader-temp-open=false'.
- lifecycle/status controls:
  'tasks/evidence/2026-06-03-phase10-minimal-waitpid-lifecycle-observation-core/qemu-local-shell-waitpid-lifecycle-smoke.log'
  and
  'tasks/evidence/2026-06-03-phase10-shell-last-process-status-observation/qemu-local-shell-last-process-status-smoke.log'
  retain consuming waitpid and non-consuming laststatus behavior.
- descriptor-backed file control:
  'tasks/evidence/2026-06-02-qemu-local-cat-banner-core/qemu-local-cat-banner-smoke.log'
  retains descriptor-backed 'cat /etc/banner.txt' behavior.
- background VFS exec lifecycle:
  'tasks/evidence/2026-06-05-phase10-background-vfs-exec-lifecycle-core/qemu-local-shell-background-vfs-exec-lifecycle-smoke.log'
  records fixed-/bin background exec through 'source=vfs-open-read
  mode=background', shell responsiveness, status 0x2a, foreground isolation,
  errors=0, classification, and PASS.
- jobs accounting:
  'tasks/evidence/2026-06-05-phase10-jobs-accounting-list-core/qemu-local-shell-jobs-accounting-list-smoke.log'
  records 'jobs none', running/completed '/bin/status42' accounting,
  foreground isolation, deterministic fg/bg/kill negatives, errors=0,
  classification, and PASS.
- multiple background records:
  'tasks/evidence/2026-06-05-phase10-multiple-background-vfs-exec-records-core/qemu-local-shell-multiple-background-jobs-smoke.log'
  records distinct '/bin/status42' and '/bin/zero' job ids/pids/statuses,
  foreground lifecycle isolation, deterministic async negatives, errors=0,
  classification, and PASS.
- stale-entry policy:
  'tasks/evidence/2026-06-05-phase10-background-jobs-stale-entry-policy-core/qemu-local-shell-background-jobs-stale-entry-policy-smoke.log'
  records one-report completed-entry exposure, deterministic clearing,
  foreground lifecycle isolation, deterministic negatives, errors=0,
  classification, and PASS.
- closeout retained-evidence inspection:
  'tasks/evidence/2026-06-05-phase10-pipelines-process-control-milestone-closeout/retained-evidence-inspection.txt'.

## Accepted Milestone Boundary

Accepted at the local/QEMU frontier:

- simple exact two-stage pipelines with exit status observation through the
  accepted lifecycle/status surfaces;
- pipeline composition with child-only descriptor redirection and volatile VFS
  file sinks for the accepted exact forms;
- descriptor inheritance/restoration controls, including loader temporary
  descriptor non-leak for accepted VFS exec;
- bounded background VFS exec accounting for fixed-/bin jobs;
- bounded shell-owned jobs inspection and deterministic stale-entry clearing;
- multiple fixed-/bin VFS-backed user program records making progress through
  the accepted background accounting path while the command loop remains
  responsive for later inspections and commands.

Deferred before broader POSIX claims:

- preemptive or otherwise proven scheduler-concurrent userspace execution;
- POSIX job control, signals, process groups, sessions, terminal ownership,
  fork, and process-tree/procfs inspection;
- multi-stage/concurrent pipelines, pipefail, background pipelines and
  redirections, and scheduling fairness proof;
- broader close-on-exec and descriptor policy beyond accepted evidence;
- persistent/larger local storage, Pi 5 hardware proof, networking, SSH,
  Milestone 10.3 implementation, Phase 11, and phase transition.

## Next Step

The next explicit queued task is
'phase10-local-storage-path-evaluation-checkpoint-20260605'. It remains a
separate Milestone 10.3 evaluation checkpoint and must not be treated as
implemented or selected by this Milestone 10.2 closeout.

## Validation

- static inspection: accepted Milestone 10.2 task records and retained
  QEMU/substitute logs were inspected for PASS/classification markers, errors=0,
  pipeline transfer/redirection composition, waitpid/laststatus controls,
  descriptor inheritance/restoration, loader-temp descriptor non-leak,
  background jobs accounting, multiple records, stale-entry clearing, and
  deterministic negatives.
- static inspection: roadmap language was updated and checked to keep Milestone
  10.2 local/QEMU-only and Milestone 10.3 separate.
- diff check: 'git diff --check' passed.
- docs: '/home/node/.cargo/bin/mdbook build' passed.
- staged diff check: 'git diff --cached --check' passed before commit.

hardwareTestLock remained unlocked/restored and unused.

## Commit

Commit: final Milestone 10.2 closeout commit recorded in supervisor state.
