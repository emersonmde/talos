# Phase 12 Local Waitpid Any Completed Child Observation Core

Task id: phase12-local-waitpid-any-completed-child-observation-core-20260626
Status: accepted
Owner: worker
Classification: local-waitpid-any-completed-child-observation-core-accepted

## Goal

Wire no-argument waitpid to the accepted local completed-child lifecycle/status
records so direct VFS exec, exact two-stage pipeline consumer, and completed
background jobs can be consumed through one bounded shell-visible wait surface
while Phase 12 live network reachability remains paused.

## Scope

This task is a local POSIX/VFS/userspace continuation only. It starts from the
accepted background explicit waitpid frontier and extends the existing
no-argument waitpid path over already accepted lifecycle/status records.

It does not add broad process tables, scheduler concurrency, waitpid options,
pid reuse/zombie policy, multi-stage pipelines, pipefail, fork, signals,
process groups/sessions, persistent filesystem semantics, networking, OpenSSH,
Pi 5 hardware proof, or a phase transition.

## Implementation

- Added a source-aware no-argument wait hook for local command sinks while
  preserving the existing source=lifecycle-record output for direct foreground
  waits.
- No-argument waitpid now consumes the foreground waitable lifecycle record and
  clears the matching explicit-pid record for that same child, preventing a
  later explicit wait from re-observing an already consumed direct exec or
  pipeline consumer record.
- When no foreground waitable exists, no-argument waitpid consumes exactly one
  completed/reaped background job slot and reports it through
  source=background-job-lifecycle-record.
- Explicit waitpid-by-pid remains available for still-unconsumed retained
  records, including the exact two-stage pipeline producer after no-argument
  waitpid consumes the consumer record.
- Added a focused QEMU/substitute smoke and boot scenario for the completed
  child waitpid sequence.
- Removed the now-dead waitpid status wrapper after all no-argument wait output
  was routed through the source-aware writer.

## Findings And Disposition

- fixed: no-argument waitpid after direct exec /bin/status42 consumes the
  lifecycle/status record with status 0x2a and repeated no-argument waitpid
  reports deterministic no-child.
- fixed: no-argument waitpid after exec stdout | exec stdin consumes the
  accepted exact pipeline consumer lifecycle/status record while waitpid
  0x100001 still observes the producer record.
- fixed: waitpid 0x100002 after no-argument waitpid consumes the pipeline
  consumer now reports deterministic no-child instead of re-reporting the same
  consumed lifecycle.
- fixed: no-argument waitpid after exec /bin/status42 & consumes the completed
  background lifecycle/status record through
  source=background-job-lifecycle-record, and jobs reports none afterward.
- fixed: unconsumed completed background jobs remain visible through jobs
  accounting; the retained /bin/zero control lists the completed job and then
  jobs reports none after cleanup.
- removed: the obsolete write_waitpid_status_line wrapper was removed after the
  source-aware writer became the only no-argument wait output path.
- not-an-issue: laststatus remains non-consuming and continues to report the
  latest foreground status after no-argument waitpid consumes the waitable
  child.
- deferred: broad process tables, scheduler concurrency, waitpid options, pid
  reuse/zombie policy, multi-stage pipelines, pipefail, async jobs,
  fork/signals, process groups/sessions, persistent filesystem semantics, live
  networking, SSH, Pi 5 hardware proof, and phase transition remain outside
  this task.

## Evidence

- Focused QEMU/substitute transcript:
  tasks/evidence/2026-06-26-phase12-local-waitpid-any-completed-child-observation-core/qemu-local-shell-waitpid-any-completed-child-smoke.log.
- The retained transcript proves:
  - no-argument waitpid starts with deterministic no-child;
  - exec /bin/status42 followed by no-argument waitpid reports status 0x2a
    through source=lifecycle-record, repeated no-argument waitpid reports
    no-child, and laststatus remains non-consuming;
  - exec stdout | exec stdin followed by no-argument waitpid consumes the
    consumer pid 0x100002 through source=lifecycle-record;
  - waitpid 0x100001 still observes the producer pid through
    source=explicit-pid-lifecycle-record, while waitpid 0x100002 reports
    deterministic no-child after the consumer was consumed;
  - exec /bin/status42 & followed by no-argument waitpid reports the completed
    background job lifecycle/status record with status 0x2a through
    source=background-job-lifecycle-record, and jobs reports none afterward;
  - an unconsumed exec /bin/zero & completion remains visible through jobs
    accounting and is cleared by the existing jobs cleanup policy;
  - final classification is
    qemu-local-shell-waitpid-any-completed-child-complete with participants=19,
    expected=19, errors=0, and PASS.
- Live network/SSH reachability remains paused. No Pi 5 hardware claim is made.

Evidence levels: fmt/lint/typecheck, unit tests, QEMU/substitute shell smoke,
QEMU/substitute direct VFS exec/pipeline/background waitpid observation,
QEMU/substitute retained explicit-pid waitpid regression, docs build, and diff
checks.

## Validation

- fmt/lint/typecheck: cargo fmt --all -- --check passed.
- unit tests: cargo -Zjson-target-spec test --quiet passed with 825 tests.
- focused QEMU/substitute shell smoke:
  ./scripts/qemu-local-shell-waitpid-any-completed-child-smoke.sh passed with
  task-owned evidence and no-argument waitpid observations over direct exec,
  exact pipeline consumer, and background job completion.
- retained regression evidence: ./scripts/qemu-local-shell-waitpid-lifecycle-smoke.sh
  passed, preserving direct VFS exec explicit pid waitpid, exact pipeline
  producer/consumer explicit pid waitpid, background explicit pid waitpid,
  descriptor-backed VFS cat, redirection controls, negative exec/waitpid
  controls, and laststatus behavior.
- task-owned JSON evidence: conditional skip, no task-owned JSON evidence was
  created.
- docs validation: /home/node/.cargo/bin/mdbook build passed.
- diff checks: git diff --check and git diff --cached --check passed.

## Acceptance

Accepted as local-waitpid-any-completed-child-observation-core-accepted.

No boot archive was published, no hardwareTestLock was acquired, no lab or Pi 5
hardware action ran, no packet I/O or OpenSSH attempt ran, and no ssh-ready,
remote-receipt, compatibility, live reachability, or phase-transition claim is
accepted.

Implementation commit: recorded in durable supervisor state after commit
creation.
