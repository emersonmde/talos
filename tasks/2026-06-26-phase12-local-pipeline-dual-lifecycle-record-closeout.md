# Phase 12 Local Pipeline Dual Lifecycle Record Closeout

Task id: phase12-local-pipeline-dual-lifecycle-record-closeout-20260626
Status: accepted
Owner: worker
Classification: local-pipeline-dual-lifecycle-record-closeout-planning-needed

## Goal

Close out the accepted exact two-stage pipeline producer/consumer
lifecycle/status accounting by mapping retained evidence, documenting the
accepted frontier, and stopping for supervisor planning instead of selecting an
unplanned follow-up.

## Scope

This is a static closeout for
phase12-local-pipeline-dual-lifecycle-record-core-20260626. It does not change
kernel behavior, shell behavior, tests, boot artifacts, lab state, hardware
state, packet I/O, OpenSSH, remote receipt, compatibility, or ssh-ready status.

## Accepted Frontier

The accepted local frontier is exact two-stage pipeline producer/consumer
lifecycle/status accounting only:

- The accepted exact forms exec stdout | exec stdin, exec stderr | exec stdin,
  exec stderr 2>&1 | exec stdin, and exec stdout 1>&2 | exec stdin emit
  phase12-local-pipeline-dual-lifecycle-status-record-v1.
- The pipeline-local record names producer and consumer pid, path, state,
  status, observed-status, and reaped fields while preserving the existing
  descriptor-backed VFS exec summaries.
- Consumer waitpid and laststatus remain the accepted shell status observation
  for the bounded pipeline form. Producer status is visible only in the
  pipeline-local accounting record unless a later explicit process-accounting
  task changes that policy.
- The retained controls preserve stdout-only pipe semantics, stderr
  duplication into the pipe, stdout redirect-away behavior, pipe EOF,
  descriptor restoration, descriptor-backed VFS cat, direct VFS exec lifecycle
  records, and deterministic negative pipeline/redirection behavior.

## Evidence Map

- Core task record:
  tasks/2026-06-26-phase12-local-pipeline-dual-lifecycle-record-core.md.
- Core acceptance commit:
  3a18a68e6201025471cb286640dc47f2936856a8.
- Combined QEMU/substitute transcript:
  tasks/evidence/2026-06-26-phase12-local-pipeline-dual-lifecycle-record-core/qemu-local-shell-pipeline-dual-lifecycle-combined.log.
- Per-scenario QEMU/substitute transcripts:
  - tasks/evidence/2026-06-26-phase12-local-pipeline-dual-lifecycle-record-core/qemu-local-shell-minimal-stdout-to-stdin-pipeline-smoke.log
  - tasks/evidence/2026-06-26-phase12-local-pipeline-dual-lifecycle-record-core/qemu-local-shell-pipeline-stderr-not-piped-smoke.log
  - tasks/evidence/2026-06-26-phase12-local-pipeline-dual-lifecycle-record-core/qemu-local-shell-pipeline-stderr-dup-to-stdout-smoke.log
  - tasks/evidence/2026-06-26-phase12-local-pipeline-dual-lifecycle-record-core/qemu-local-shell-pipeline-stdout-redirect-away-smoke.log
- Direct VFS exec lifecycle/status regression record:
  tasks/2026-06-26-phase12-local-vfs-exec-lifecycle-record-generalization-core.md.
- Direct VFS exec QEMU/substitute transcript:
  tasks/evidence/2026-06-26-phase12-local-vfs-exec-lifecycle-record-generalization-core/qemu-local-shell-vfs-exec-smoke.log.

Evidence levels: static inspection, retained QEMU/substitute transcript
inspection, docs build, and diff checks.

## Findings And Disposition

- fixed: the accepted frontier is documented as exact two-stage pipeline
  producer/consumer lifecycle/status accounting only.
- fixed: the consumer waitpid/laststatus policy is explicitly retained as the
  bounded pipeline shell status observation.
- fixed: the evidence map points to retained combined and per-scenario
  QEMU/substitute transcripts, plus the direct VFS exec, redirection, VFS cat,
  and negative-control regressions retained by the core task.
- fixed: no mechanically objective same-slice local follow-up is selected;
  supervisor planning is required for the next task.
- not-an-issue: no code or test change is needed for this closeout because the
  core task already retained the behavioral evidence.
- deferred: multi-stage or concurrent pipelines, pipefail, separate
  process-table identities, broad process accounting, async jobs,
  fork/signals, job control, broader descriptor grammar, file/device semantics,
  persistent filesystem semantics, live networking, SSH, Pi 5 hardware proof,
  and phase transition remain outside this closeout.
- removed: no code, docs, or evidence was removed.

## Deferred Surface

Multi-stage or concurrent pipelines, pipefail, separate process-table
identities, broad process accounting, async jobs, fork/signals, job control,
broader descriptor grammar, file/device semantics, persistent filesystem
semantics, live networking, SSH, Pi 5 hardware proof, and phase transition
remain deferred.

Live Ethernet/TCP reachability remains paused. No Pi 5 hardware claim is made.

## Validation

- static inspection: task record, retained evidence paths, roadmap/project
  docs, and git diff inspected.
- diff check: git diff --check passed.
- docs build: /home/node/.cargo/bin/mdbook build passed.
- staged diff check: git diff --cached --check passed.

## Acceptance

Accepted as local-pipeline-dual-lifecycle-record-closeout-planning-needed.

selected_next_task: null.
planningNeeded: true.

No boot archive was published, no hardwareTestLock was acquired, no lab or Pi 5
hardware action ran, no packet I/O or OpenSSH attempt ran, and no ssh-ready,
remote-receipt, compatibility, live reachability, or phase-transition claim is
accepted.

Acceptance commit: recorded in durable supervisor state after commit creation.
