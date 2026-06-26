# Phase 12 Local Process Table Direct VFS Exec Closeout

Task id: phase12-local-process-table-direct-vfs-exec-closeout-20260626
Status: accepted
Owner: worker
Classification: local-process-table-direct-vfs-exec-closeout-accepted

## Goal

Close out the accepted bounded direct foreground VFS exec process-table
frontier before promoting pipeline/background process-table integration.

## Scope

This is a static closeout over the accepted direct process-table core task,
retained unit/QEMU-substitute evidence, and local POSIX/VFS/userspace docs.

It does not change code, run Pi 5 hardware, publish a boot archive, mutate the
lab, retry live Ethernet/TCP, launch OpenSSH, add a public process enumeration
API, migrate pipeline/background records, or transition Phase 12.

## Accepted Frontier

- Direct foreground VFS exec for the accepted fixed fixtures /bin/init,
  /bin/zero, and /bin/status42 creates an internal kernel-owned bounded
  process-table lifecycle/status record.
- The retained record fields are stable pid 0x100001, parent=shell, owner=0x1,
  path, exited state, status, observed-status, and reaped state.
- Existing shell-visible direct exec, waitpid, laststatus, descriptor-backed
  VFS cat, regular-file redirection, exact pipeline, background jobs, jobs
  accounting, and negative-control regressions remain evidence-backed.
- waitpid remains the consuming observation surface, while laststatus remains a
  non-consuming foreground status surface.
- Exact pipeline and background jobs intentionally remain on their previously
  accepted record sources until the explicitly queued pipeline/background
  process-table integration task.

## Findings And Disposition

- fixed: the accepted direct process-table frontier is reconciled as bounded
  direct foreground VFS exec only.
- fixed: the evidence map now points to the retained direct exec,
  process-table, waitpid, laststatus, descriptor-backed VFS, and regression
  transcripts.
- fixed: selected_next_task is narrowed to the already queued
  phase12-local-process-table-pipeline-background-core-20260626 because the
  direct frontier is accepted and the follow-up has explicit dependencies,
  acceptance criteria, validation gates, docs requirements, and evidence
  requirements.
- not-an-issue: shell-visible direct waitpid source labels remain the accepted
  lifecycle-record compatibility surface; process-table state is inspected by
  focused unit tests for this boundary.
- not-an-issue: retaining pipeline/background records on older accepted sources
  is intentional for this closeout and is the reason the next bounded task is
  selected.
- removed: no accepted direct exec, waitpid, laststatus, VFS cat, redirection,
  exact pipeline, background jobs, jobs accounting, or negative-control
  behavior was removed.
- deferred: pipeline/background process-table migration, public process
  enumeration, procfs/ps, scheduler-concurrent execution, fork/signals, process
  groups/sessions, waitpid options, pid reuse policy beyond bounded
  deterministic controls, multi-stage pipelines, pipefail, persistent storage,
  live networking, SSH, Pi 5 hardware proof, and phase transition.

## Evidence Map

- Direct process-table core task:
  tasks/2026-06-26-phase12-local-process-table-direct-vfs-exec-core.md.
- Direct process-table classification:
  tasks/evidence/2026-06-26-phase12-local-process-table-direct-vfs-exec-core/classification.json.
- Direct process-table evidence map:
  tasks/evidence/2026-06-26-phase12-local-process-table-direct-vfs-exec-core/evidence-map.json.
- Direct VFS exec QEMU/substitute regression:
  tasks/evidence/2026-06-26-phase12-local-process-table-direct-vfs-exec-core/qemu-local-shell-vfs-exec-smoke.log.
- waitpid/laststatus/pipeline/background QEMU/substitute regression:
  tasks/evidence/2026-06-26-phase12-local-process-table-direct-vfs-exec-core/qemu-local-shell-waitpid-lifecycle-smoke.log.
- Prior process-observation closeout:
  tasks/2026-06-26-phase12-local-waitpid-process-observation-closeout.md.

Evidence levels: static inspection of task records, retained unit-test
summaries, retained QEMU/substitute transcripts, roadmap/project
documentation, JSON validation, docs build, and diff checks.

## Selected Next Task

selected_next_task:
phase12-local-process-table-pipeline-background-core-20260626.

The selected follow-up is mechanically objective because the direct
process-table core is accepted and committed, the hardware lock is unlocked and
restored, supervisor intervention is inactive, and the queued
pipeline/background task already defines scope, non-goals, dependencies,
acceptance criteria, validation gates, docs requirements, and evidence
requirements.

## Validation

- static inspection: accepted direct core task, retained evidence paths,
  roadmap/project docs, and git diff inspected.
- JSON validation: jq -e empty passed for task-owned JSON evidence.
- diff checks: git diff --check passed.
- docs validation: /home/node/.cargo/bin/mdbook build passed.
- staged diff checks: git diff --cached --check passed.

## Acceptance

Accepted as local-process-table-direct-vfs-exec-closeout-accepted.

No boot archive was published, no hardwareTestLock was acquired, no lab or Pi 5
hardware action ran, no packet I/O or OpenSSH attempt ran, and no ssh-ready,
remote-receipt, compatibility, live reachability, or phase-transition claim is
accepted.

Implementation commit: recorded in durable supervisor state after commit
creation.
