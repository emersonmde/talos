# Phase 12 Local Pipeline Distinct Process Identity Closeout

Task id: phase12-local-pipeline-distinct-process-identity-closeout-20260626
Status: accepted
Owner: worker
Classification: local-pipeline-distinct-process-identity-closeout-accepted

## Goal

Close out the distinct serialized pipeline process identity frontier and decide
whether explicit waitpid/process-accounting observation is mechanically
unblocked while Phase 12 live network reachability remains paused.

## Scope

This is a static closeout for the local POSIX/VFS/userspace pipeline identity
slice accepted by
phase12-local-pipeline-distinct-process-identity-core-20260626. It reconciles
the retained exact two-stage pipeline evidence, documents the accepted boundary
as serialized local pipeline identity/accounting only, confirms the consumer
waitpid/laststatus policy, and records whether the queued explicit pid
observation task is mechanically objective.

It does not add implementation behavior, publish a boot archive, acquire the
hardwareTestLock, touch the Pi 5/lab path, retry OpenSSH, broaden descriptor
grammar, accept concurrent pipelines, or transition Phase 12 live networking.

## Findings And Disposition

- fixed: the accepted frontier is documented as exact two-stage local pipeline
  lifecycle/status accounting with distinct serialized producer and consumer
  pids only.
- fixed: evidence mapping now points to the retained combined and per-scenario
  QEMU/substitute transcripts for producer pid 0x100001 and consumer pid
  0x100002 across the accepted exact pipeline forms.
- fixed: consumer waitpid and laststatus remain documented as the shell status
  observation after the bounded pipeline form; producer status remains visible
  only through pipeline-local accounting unless a later selected task changes
  the observation surface.
- not-an-issue: exact pipeline execution remains serialized; this closeout does
  not require or imply scheduler concurrency, POSIX fork, process groups,
  sessions, broad process tables, pid reuse, or zombie policy.
- deferred: explicit producer/consumer pid-based wait/status observation,
  multi-stage or concurrent pipelines, pipefail, async jobs, fork/signals,
  broader descriptor grammar, persistent filesystem semantics, live networking,
  SSH, Pi 5 hardware proof, and phase transition remain outside this accepted
  closeout.
- removed: no accepted direct VFS exec, pipeline, redirection, cat, waitpid, or
  laststatus behavior was removed.

## Evidence Map

- Core task record:
  tasks/2026-06-26-phase12-local-pipeline-distinct-process-identity-core.md.
- Core implementation commit: c1821e91fdff1a79149e8112d79b85555326b3e5.
- Combined QEMU/substitute transcript:
  tasks/evidence/2026-06-26-phase12-local-pipeline-distinct-process-identity-core/qemu-local-shell-pipeline-distinct-process-identity-combined.log.
- Per-scenario QEMU/substitute transcripts:
  - tasks/evidence/2026-06-26-phase12-local-pipeline-distinct-process-identity-core/qemu-local-shell-minimal-stdout-to-stdin-pipeline-smoke.log
  - tasks/evidence/2026-06-26-phase12-local-pipeline-distinct-process-identity-core/qemu-local-shell-pipeline-stderr-not-piped-smoke.log
  - tasks/evidence/2026-06-26-phase12-local-pipeline-distinct-process-identity-core/qemu-local-shell-pipeline-stderr-dup-to-stdout-smoke.log
  - tasks/evidence/2026-06-26-phase12-local-pipeline-distinct-process-identity-core/qemu-local-shell-pipeline-stdout-redirect-away-smoke.log

The retained combined transcript shows:

- phase12-local-pipeline-distinct-process-lifecycle-status-record-v1 records
  with producer pid 0x100001 and consumer pid 0x100002 for the accepted exact
  pipeline forms.
- producer paths /bin/stdout or /bin/stderr, consumer path /bin/stdin, exited
  states, zero status/observed-status, and reaped flags.
- consumer waitpid and laststatus observing pid 0x100002 after pipeline
  completion.
- descriptor-backed VFS/open/read, loader, startup ABI, pipe EOF, fd
  restoration, stdout-only pipe semantics, descriptor dup/redirect controls,
  and descriptor-backed cat /etc/banner.txt regressions.
- final classifications with errors=0 and PASS for all retained focused
  pipeline smokes.

Evidence levels: static inspection, QEMU/substitute transcript inspection,
docs build, and diff checks. Live network/SSH reachability remains paused. No
Pi 5 hardware claim is made.

## Deferred Surfaces

Explicit producer/consumer pid-based wait/status observation is not accepted by
this closeout, but it is now mechanically objective as a follow-up because the
accepted core records stable distinct producer and consumer pids and the queued
follow-up has explicit scope, dependencies, acceptance criteria, validation
gates, docs requirements, and evidence requirements.

Still deferred: multi-stage or concurrent pipelines, pipefail, background jobs,
async scheduling, fork, signals, job control, process groups, sessions, broad
process tables, pid reuse policy, zombie policy, PATH/environment expansion,
arbitrary descriptor syntax, persistent filesystem semantics, packet I/O, live
networking, SSH, RP1/PCIe, DMA/cache policy, Pi 5 hardware proof, and phase
transition.

## Validation

- static inspection: task record, retained evidence paths, roadmap/project docs,
  and git diff inspected.
- diff checks: git diff --check passed.
- docs validation: /home/node/.cargo/bin/mdbook build passed.
- staged diff check: git diff --cached --check passed.

## Acceptance

Accepted as local-pipeline-distinct-process-identity-closeout-accepted.

selected_next_task:
phase12-local-waitpid-explicit-pipeline-process-observation-core-20260626.
planningNeeded: false.

No boot archive was published, no hardwareTestLock was acquired, no lab or Pi 5
hardware action ran, no packet I/O or OpenSSH attempt ran, and no ssh-ready,
remote-receipt, compatibility, live reachability, or phase-transition claim is
accepted.

Acceptance commit: recorded in durable supervisor state after commit creation.
