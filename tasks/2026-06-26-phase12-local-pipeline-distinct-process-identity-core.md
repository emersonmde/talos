# Phase 12 Local Pipeline Distinct Process Identity Core

Task id: phase12-local-pipeline-distinct-process-identity-core-20260626
Status: accepted
Owner: worker
Classification: local-pipeline-distinct-process-identity-core-accepted

## Goal

Give the accepted exact two-stage local pipeline forms distinct serialized
producer and consumer process identities in the lifecycle/status record while
Phase 12 live network reachability remains paused.

## Scope

This task is a local POSIX/VFS/userspace continuation only. It preserves the
accepted exact two-stage pipeline forms and refines the pipeline-local
lifecycle/status record so producer and consumer records carry distinct stable
serialized process ids. It does not add concurrent scheduling, broad process
tables, pid reuse policy, multi-stage pipelines, pipefail, background pipeline
behavior, fork, signals, job control, arbitrary descriptor syntax, live
networking, OpenSSH, Pi 5 hardware proof, or a phase transition.

## Implementation

- Added distinct local constants for serialized pipeline producer and consumer
  identities: producer 0x100001 and consumer 0x100002.
- Updated exec_vfs_pipeline to rewrite the producer and consumer exec
  lifecycle records after each serialized descriptor-backed VFS exec completes.
- Updated the shell waitable and last-process lifecycle state to the consumer
  lifecycle record after the pipeline completes, preserving the accepted
  consumer waitpid/laststatus policy.
- Versioned the pipeline lifecycle/status record identity to
  phase12-local-pipeline-distinct-process-lifecycle-status-record-v1.
- Updated unit expectations and QEMU/substitute smoke checks to require
  distinct producer and consumer pids in pipeline lifecycle/status evidence.

## Findings And Disposition

- fixed: exact exec stdout | exec stdin now reports producer pid 0x100001 and
  consumer pid 0x100002 in the pipeline-local lifecycle/status record.
- fixed: exact exec stderr | exec stdin now reports distinct pids while
  preserving stdout-only pipe semantics and consumer pipe EOF/no-data behavior.
- fixed: exact exec stderr 2>&1 | exec stdin now reports distinct pids while
  preserving child-only fd2-to-fd1 duplication into the pipe.
- fixed: exact exec stdout 1>&2 | exec stdin now reports distinct pids while
  preserving stdout redirect-away behavior and consumer pipe EOF/no-data.
- fixed: consumer waitpid and laststatus now report the consumer pid 0x100002
  after an accepted exact pipeline, matching the pipeline-local consumer record.
- not-an-issue: producer and consumer execution remains serialized in the local
  substitute path; distinct process-table entries, scheduler concurrency, pid
  reuse, process groups, sessions, and zombies remain outside this feature.
- deferred: explicit waitpid/process-accounting observation beyond the consumer
  status, broad process tables, multi-stage or concurrent pipelines, pipefail,
  background jobs, fork/signals, job control, arbitrary descriptor syntax,
  persistent filesystem semantics, live networking, SSH, RP1/PCIe, DMA/cache
  policy, Pi 5 hardware proof, and phase transition remain outside this task.
- removed: no accepted pipeline, direct VFS exec, descriptor, redirection, or
  cat behavior was removed.

## Evidence

- Combined QEMU/substitute transcript:
  tasks/evidence/2026-06-26-phase12-local-pipeline-distinct-process-identity-core/qemu-local-shell-pipeline-distinct-process-identity-combined.log.
- Per-scenario QEMU/substitute transcripts:
  - tasks/evidence/2026-06-26-phase12-local-pipeline-distinct-process-identity-core/qemu-local-shell-minimal-stdout-to-stdin-pipeline-smoke.log
  - tasks/evidence/2026-06-26-phase12-local-pipeline-distinct-process-identity-core/qemu-local-shell-pipeline-stderr-not-piped-smoke.log
  - tasks/evidence/2026-06-26-phase12-local-pipeline-distinct-process-identity-core/qemu-local-shell-pipeline-stderr-dup-to-stdout-smoke.log
  - tasks/evidence/2026-06-26-phase12-local-pipeline-distinct-process-identity-core/qemu-local-shell-pipeline-stdout-redirect-away-smoke.log
- The retained transcripts prove:
  - each accepted exact two-stage form emits
    phase12-local-pipeline-distinct-process-lifecycle-status-record-v1;
  - producer pid 0x100001 and consumer pid 0x100002 are distinct in the
    pipeline-local lifecycle/status record;
  - producer paths /bin/stdout or /bin/stderr and consumer path /bin/stdin are
    recorded with zero status and observed-status;
  - consumer waitpid and laststatus report pid 0x100002 after the pipeline;
  - descriptor-backed VFS/open/read, loader, startup ABI, pipe EOF, fd
    restoration, descriptor mixing, and cat /etc/banner.txt regressions remain
    covered;
  - each focused smoke ends with errors=0 and PASS/final classification.
- Live network/SSH reachability remains paused. No Pi 5 hardware claim is made.

Evidence levels: fmt/lint/typecheck, unit tests through the QEMU runner,
QEMU/substitute shell smokes, QEMU/substitute VFS cat regression, docs build,
and diff checks.

## Validation

- fmt/lint/typecheck: cargo fmt --all -- --check passed.
- unit tests/QEMU runner substitute: cargo -Zjson-target-spec test --quiet
  passed with 823 tests.
- focused QEMU/substitute shell smokes:
  - ./scripts/qemu-local-shell-minimal-stdout-to-stdin-pipeline-smoke.sh passed
    with task-owned evidence.
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
- docs validation: /home/node/.cargo/bin/mdbook build passed.
- diff checks: git diff --check and git diff --cached --check passed.

## Acceptance

Accepted as local-pipeline-distinct-process-identity-core-accepted.

No boot archive was published, no hardwareTestLock was acquired, no lab or Pi 5
hardware action ran, no packet I/O or OpenSSH attempt ran, and no ssh-ready,
remote-receipt, compatibility, live reachability, or phase-transition claim is
accepted.

Implementation commit: fdb636a5ae2f460320ded924050d7a8e7d854610.
