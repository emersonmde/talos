# Phase 12 Local Pipeline Dual Lifecycle Record Core

Task id: phase12-local-pipeline-dual-lifecycle-record-core-20260626
Status: accepted
Owner: worker
Classification: local-pipeline-dual-lifecycle-record-core-accepted

## Goal

Add the smallest shell-visible dual lifecycle/status accounting for the accepted
exact two-stage pipeline forms while Phase 12 live network reachability remains
paused.

## Scope

This task is a local POSIX/VFS/userspace continuation only. It preserves the
accepted exact two-stage pipeline forms and adds pipeline-local lifecycle/status
accounting for the producer and consumer. It does not add concurrent scheduling,
multi-stage pipelines, pipefail, background pipeline behavior, fork, signals,
job control, arbitrary descriptor syntax, live networking, OpenSSH, Pi 5
hardware proof, or a phase transition.

## Implementation

- Added LocalCommandPipelineLifecycleStatusRecord with identity
  phase12-local-pipeline-dual-lifecycle-status-record-v1.
- write_pipeline_summary now emits one pipeline-lifecycle-status line after
  the existing pipeline line and before the producer/consumer exec summaries.
- The record reports the pipeline id plus producer and consumer pid, path,
  state, status, observed-status, and reaped fields.
- The existing producer and consumer still launch through descriptor-backed
  VFS/open/read, loader, startup ABI, and lifecycle paths.
- waitpid and laststatus still observe the consumer /bin/stdin lifecycle;
  producer status is visible only through the pipeline-local accounting record.
- Updated local unit expectations, QEMU scenario response counts, and focused
  smoke assertions for the new line.
- Updated exact pipeline smoke wrappers so task-owned evidence paths can
  override the older Phase 10 evidence directories.

## Findings And Disposition

- fixed: exact exec stdout | exec stdin now emits a named/versioned
  pipeline-local lifecycle/status record for producer /bin/stdout and consumer
  /bin/stdin.
- fixed: exact exec stderr | exec stdin now emits the record while preserving
  stdout-only pipe semantics; stderr output remains outside the pipe and the
  consumer observes pipe EOF/no-data.
- fixed: exact exec stderr 2>&1 | exec stdin now emits the record while
  preserving child-only fd2-to-fd1 duplication into the pipe.
- fixed: exact exec stdout 1>&2 | exec stdin now emits the record while
  preserving stdout redirect-away behavior and consumer pipe EOF/no-data.
- fixed: QEMU/substitute smoke evidence can now be retained under the
  task-owned evidence directory for the four accepted exact pipeline forms.
- fixed: a stale generated-root unit response-count expectation was corrected
  from 10 to 9 to match the already accepted QEMU scenario and implementation;
  no generated-root behavior changed.
- not-an-issue: producer and consumer currently share the existing local
  substitute process id in these serialized pipeline fixtures; this task
  records per-role lifecycle/status without accepting process-table identity
  expansion.
- deferred: concurrent scheduling, separate process-table identities,
  multi-stage pipelines, pipefail, background jobs, fork/signals, job control,
  arbitrary descriptor syntax, persistent filesystem semantics, live networking,
  SSH, RP1/PCIe, DMA/cache policy, Pi 5 hardware proof, and phase transition
  remain outside this task.
- removed: no pipeline behavior or accepted command surface was removed.

## Evidence

- Combined QEMU/substitute transcript:
  tasks/evidence/2026-06-26-phase12-local-pipeline-dual-lifecycle-record-core/qemu-local-shell-pipeline-dual-lifecycle-combined.log.
- Per-scenario QEMU/substitute transcripts:
  - tasks/evidence/2026-06-26-phase12-local-pipeline-dual-lifecycle-record-core/qemu-local-shell-minimal-stdout-to-stdin-pipeline-smoke.log
  - tasks/evidence/2026-06-26-phase12-local-pipeline-dual-lifecycle-record-core/qemu-local-shell-pipeline-stderr-not-piped-smoke.log
  - tasks/evidence/2026-06-26-phase12-local-pipeline-dual-lifecycle-record-core/qemu-local-shell-pipeline-stderr-dup-to-stdout-smoke.log
  - tasks/evidence/2026-06-26-phase12-local-pipeline-dual-lifecycle-record-core/qemu-local-shell-pipeline-stdout-redirect-away-smoke.log
- The retained transcripts prove:
  - each accepted exact two-stage form emits pipeline-lifecycle-status
    record=phase12-local-pipeline-dual-lifecycle-status-record-v1;
  - producer paths /bin/stdout and /bin/stderr and consumer path /bin/stdin are
    recorded with zero status and observed-status;
  - consumer waitpid and laststatus remain the accepted status observation;
  - descriptor-backed VFS/open/read, loader, startup ABI, pipe EOF, fd
    restoration, descriptor mixing, and cat /etc/banner.txt regressions remain
    covered;
  - each focused smoke ends with errors=0 and PASS.
- Live network/SSH reachability remains paused. No Pi 5 hardware claim is made.

Evidence levels: fmt/lint/typecheck, unit tests through the QEMU runner,
QEMU/substitute shell smokes, QEMU/substitute VFS cat regression, docs build,
and diff checks.

## Validation

- fmt/lint/typecheck: cargo fmt --all -- --check passed.
- unit tests/QEMU runner substitute: cargo -Zjson-target-spec test --quiet
  passed with 823 tests.
- focused QEMU/substitute shell smokes:
  - ./scripts/qemu-local-shell-minimal-stdout-to-stdin-pipeline-smoke.sh
    passed with task-owned evidence.
  - ./scripts/qemu-local-shell-pipeline-stderr-not-piped-smoke.sh passed with
    task-owned evidence.
  - ./scripts/qemu-local-shell-pipeline-stderr-dup-to-stdout-smoke.sh passed
    with task-owned evidence.
  - ./scripts/qemu-local-shell-pipeline-stdout-redirect-away-smoke.sh passed
    with task-owned evidence.
- retained regression evidence: combined transcript contains waitpid,
  laststatus, descriptor-backed VFS cat, exact descriptor-mixing controls, and
  PASS/final classifications for the accepted exact forms.
- task-owned JSON evidence: conditional skip, no task-owned JSON evidence was
  created.
- docs validation, whitespace checks, and commit hash are recorded after final
  validation/commit.

## Acceptance

Accepted as local-pipeline-dual-lifecycle-record-core-accepted.

No boot archive was published, no hardwareTestLock was acquired, no lab or Pi 5
hardware action ran, no packet I/O or OpenSSH attempt ran, and no ssh-ready,
remote-receipt, compatibility, live reachability, or phase-transition claim is
accepted.

Acceptance commit: recorded in durable supervisor state after commit creation.
