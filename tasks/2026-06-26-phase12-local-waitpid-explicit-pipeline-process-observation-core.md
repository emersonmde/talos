# Phase 12 Local Waitpid Explicit Pipeline Process Observation Core

Task id: phase12-local-waitpid-explicit-pipeline-process-observation-core-20260626
Status: accepted
Owner: worker
Classification: local-waitpid-explicit-pipeline-process-observation-core-accepted

## Goal

Expose bounded explicit pid-based wait/status observation for the accepted
serialized local pipeline producer and consumer identities while Phase 12 live
network reachability remains paused.

## Scope

This task is a local POSIX/VFS/userspace continuation only. It starts from the
accepted exact two-stage pipeline identity frontier and adds a bounded explicit
waitpid-by-pid observation path over the latest retained local lifecycle
records. It preserves the existing no-argument consumer waitpid and laststatus
policy for accepted exact pipeline forms.

It does not add concurrent scheduling, multi-stage pipelines, pipefail,
background pipeline behavior, fork, signals, job control, process groups,
sessions, broad process tables, broad descriptor grammar, persistent filesystem
semantics, networking, OpenSSH, Pi 5 hardware proof, or a phase transition.

## Implementation

- Added a two-entry explicit lifecycle observation set to the descriptor-backed
  local command loop.
- Direct VFS exec refreshes the explicit observation set with its single
  lifecycle record; exact pipeline exec refreshes it with the distinct
  producer and consumer records.
- Added waitpid-by-pid parsing for bounded hexadecimal pid observation.
- waitpid 0x100001 and waitpid 0x100002 can consume and report the latest
  accepted pipeline producer and consumer records through
  source=explicit-pid-lifecycle-record.
- Unknown or stale pids return deterministic no-child; malformed pids return
  deterministic invalid-pid; pid zero returns deterministic unsupported-pid.
- Preserved no-argument waitpid as the existing consumer/latest waitable
  lifecycle observation path.

## Findings And Disposition

- fixed: accepted exact two-stage pipeline evidence can now explicitly observe
  producer pid 0x100001 and consumer pid 0x100002 by pid.
- fixed: stale explicit pid observation fails closed with no-child after the
  pid record has been consumed.
- fixed: malformed and unsupported explicit pid controls fail deterministically
  without mutating descriptor-backed VFS exec, pipeline, redirection, or cat
  behavior.
- not-an-issue: no-argument waitpid remains a separate consumer/latest waitable
  observation surface; existing consumer waitpid and laststatus regressions
  remain accepted.
- deferred: broad process tables, POSIX waitpid completeness, concurrent
  scheduling, multi-stage pipelines, pipefail, async jobs, fork/signals,
  broader descriptor grammar, persistent filesystem semantics, live networking,
  SSH, Pi 5 hardware proof, and phase transition remain outside this task.
- removed: no accepted direct VFS exec, pipeline, redirection, cat, no-argument
  waitpid, or laststatus behavior was removed.

## Evidence

- Focused QEMU/substitute transcript:
  tasks/evidence/2026-06-26-phase12-local-waitpid-explicit-pipeline-process-observation-core/qemu-local-shell-waitpid-lifecycle-smoke.log.
- The retained transcript proves:
  - direct VFS exec lifecycle/status and existing no-argument waitpid
    regressions remain passing;
  - exact exec stdout | exec stdin emits
    phase12-local-pipeline-distinct-process-lifecycle-status-record-v1 with
    producer pid 0x100001 and consumer pid 0x100002;
  - waitpid 0x100001 observes /bin/stdout through
    source=explicit-pid-lifecycle-record;
  - waitpid 0x100002 observes /bin/stdin through
    source=explicit-pid-lifecycle-record;
  - a repeated waitpid 0x100001 reports no-child, waitpid bogus reports
    invalid-pid, and waitpid 0x0 reports unsupported-pid;
  - descriptor-backed VFS cat remains passing;
  - final classification is qemu-local-shell-waitpid-complete with
    participants=25, expected=25, errors=0, and PASS.
- Live network/SSH reachability remains paused. No Pi 5 hardware claim is made.

Evidence levels: fmt/lint/typecheck, unit tests through the QEMU runner,
QEMU/substitute shell smoke, QEMU/substitute VFS cat regression, docs build,
and diff checks.

## Validation

- fmt/lint/typecheck: cargo fmt --all -- --check passed.
- unit tests/QEMU runner substitute: cargo -Zjson-target-spec test --quiet
  passed.
- focused QEMU/substitute shell smoke:
  ./scripts/qemu-local-shell-waitpid-lifecycle-smoke.sh passed with task-owned
  evidence and explicit producer/consumer pid observations.
- retained regression evidence: the focused transcript contains direct VFS exec
  lifecycle/status, no-argument waitpid, laststatus, negative exec controls,
  descriptor-backed VFS cat, exact pipeline lifecycle/status, explicit pid
  observation controls, and PASS/final classification.
- task-owned JSON evidence: conditional skip, no task-owned JSON evidence was
  created.
- docs validation: /home/node/.cargo/bin/mdbook build passed.
- diff checks: git diff --check and git diff --cached --check passed.

## Acceptance

Accepted as
local-waitpid-explicit-pipeline-process-observation-core-accepted.

No boot archive was published, no hardwareTestLock was acquired, no lab or Pi 5
hardware action ran, no packet I/O or OpenSSH attempt ran, and no ssh-ready,
remote-receipt, compatibility, live reachability, or phase-transition claim is
accepted.

Implementation commit: recorded in durable supervisor state after commit
creation.
