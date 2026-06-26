# Phase 12 Local Background Explicit Waitpid Observation Core

Task id: phase12-local-background-explicit-waitpid-observation-core-20260626
Status: accepted
Owner: worker
Classification: local-background-explicit-waitpid-observation-core-accepted

## Goal

Wire explicit waitpid-by-pid to the accepted background VFS exec job lifecycle
records so shell-visible background children can be waited by pid exactly once
while Phase 12 live network reachability remains paused.

## Scope

This task is a local POSIX/VFS/userspace continuation only. It starts from the
accepted explicit pipeline pid observation frontier and extends the existing
background VFS exec accounting surface for the already accepted
exec /bin/status42 & and exec /bin/zero & fixture paths.

It does not add async scheduler semantics, concurrent scheduling, broad process
tables, expanded background exec, multi-stage pipelines, pipefail, fork,
signals, job control, process groups, sessions, broad descriptor grammar,
persistent filesystem semantics, networking, OpenSSH, Pi 5 hardware proof, or a
phase transition.

## Implementation

- Restored the previous explicit foreground wait records after launching a
  background VFS exec, so background jobs no longer leak the foreground
  explicit-pid lifecycle record produced by the launch helper.
- Added a background-job lifecycle source to explicit waitpid-by-pid. After the
  accepted shell completion polling path marks a background job completed,
  waitpid <pid> consumes that completed background job slot exactly once.
- Background pid waits report through source=background-job-lifecycle-record.
- Repeated waits for a consumed background pid return deterministic no-child,
  and jobs no longer re-reports the consumed completed job.
- Preserved direct VFS exec explicit waitpid, exact pipeline producer/consumer
  explicit waitpid, no-argument waitpid, laststatus, background jobs
  accounting for unconsumed jobs, descriptor-backed VFS cat, redirection
  controls, and deterministic negative controls.

## Findings And Disposition

- fixed: exec /bin/status42 & followed by waitpid 0x100001 now consumes the
  completed background lifecycle/status record with status 0x2a through
  source=background-job-lifecycle-record.
- fixed: exec /bin/zero & followed by waitpid 0x100002 now consumes the
  completed background lifecycle/status record with status 0 through the same
  background lifecycle source.
- fixed: a consumed completed background job is removed from jobs accounting;
  repeated explicit waitpid on the same pid reports deterministic no-child.
- fixed: background launch no longer pollutes the foreground explicit
  lifecycle record set.
- not-an-issue: completion polling remains the accepted serialized/background
  shell responsiveness model; this task intentionally consumes only completed
  records after that poll path has run.
- not-an-issue: prompt placement differs between the first and second response
  line for a waitpid that also observes background completion; the retained
  transcript records the exact two-line response.
- deferred: no-argument waitpid over arbitrary completed children, broad
  process tables, POSIX waitpid completeness, scheduler concurrency, waitpid
  options, pid reuse/zombie policy, multi-stage pipelines, pipefail,
  fork/signals, process groups/sessions, persistent filesystem semantics, live
  networking, SSH, Pi 5 hardware proof, and phase transition remain outside
  this task.
- removed: no accepted direct VFS exec, exact pipeline, background jobs,
  descriptor-backed VFS cat, redirection, negative-control, waitpid, or
  laststatus behavior was removed.

## Evidence

- Focused QEMU/substitute transcript:
  tasks/evidence/2026-06-26-phase12-local-background-explicit-waitpid-observation-core/qemu-local-shell-waitpid-lifecycle-smoke.log.
- The retained transcript proves:
  - direct VFS exec lifecycle/status, no-argument waitpid, laststatus, negative
    exec controls, descriptor-backed VFS cat, and exact pipeline
    producer/consumer explicit waitpid regressions remain passing;
  - exec /bin/status42 & emits a running background job pid 0x100001 and the
    subsequent waitpid 0x100001 observes the completed status42 lifecycle
    record with status 0x2a through source=background-job-lifecycle-record;
  - repeated waitpid 0x100001 reports deterministic no-child and subsequent
    jobs reports none;
  - exec /bin/zero & emits a running background job pid 0x100002 and the
    subsequent waitpid 0x100002 observes the completed zero lifecycle record
    with status 0 through source=background-job-lifecycle-record;
  - repeated waitpid 0x100002 reports deterministic no-child and subsequent
    jobs reports none;
  - final classification is qemu-local-shell-waitpid-complete with
    participants=33, expected=33, errors=0, and PASS.
- Live network/SSH reachability remains paused. No Pi 5 hardware claim is made.

Evidence levels: fmt/lint/typecheck, unit tests through the QEMU runner,
QEMU/substitute shell smoke, QEMU/substitute background waitpid observation,
QEMU/substitute direct VFS exec/pipeline/VFS cat regressions, docs build, and
diff checks.

## Validation

- fmt/lint/typecheck: cargo fmt --all -- --check passed.
- unit tests/QEMU runner substitute: cargo -Zjson-target-spec test --quiet
  passed with 825 tests.
- focused QEMU/substitute shell smoke:
  ./scripts/qemu-local-shell-waitpid-lifecycle-smoke.sh passed with task-owned
  evidence and explicit background pid waitpid observations.
- retained regression evidence: the focused transcript contains direct VFS exec
  lifecycle/status, no-argument waitpid, laststatus, negative exec controls,
  descriptor-backed VFS cat, exact pipeline producer/consumer explicit pid
  observation, background status42/zero explicit pid observation, stale
  no-child controls, and PASS/final classification.
- task-owned JSON evidence: conditional skip, no task-owned JSON evidence was
  created.
- docs validation: /home/node/.cargo/bin/mdbook build passed.
- diff checks: git diff --check and git diff --cached --check passed.

## Acceptance

Accepted as local-background-explicit-waitpid-observation-core-accepted.

No boot archive was published, no hardwareTestLock was acquired, no lab or Pi 5
hardware action ran, no packet I/O or OpenSSH attempt ran, and no ssh-ready,
remote-receipt, compatibility, live reachability, or phase-transition claim is
accepted.

Implementation commit: recorded in durable supervisor state after commit
creation.
