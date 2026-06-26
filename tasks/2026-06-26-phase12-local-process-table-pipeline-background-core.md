# Phase 12 Local Process Table Pipeline/Background Core

Task id: phase12-local-process-table-pipeline-background-core-20260626

Status: accepted; commit hash recorded in durable supervisor state after commit.

## Scope

Integrate the accepted exact two-stage pipeline and background VFS exec records
with the bounded internal process-table lifecycle/status substrate introduced
for direct foreground VFS exec.

This task does not add public process enumeration, procfs/ps, scheduler
concurrency, fork/signals, process groups/sessions, waitpid options, PID reuse
policy beyond bounded deterministic controls, multi-stage pipelines, pipefail,
persistent storage, live networking, SSH, Pi 5 hardware proof, or a phase
transition.

## Findings

- fixed: The process-table record identity is generalized from direct-only to
  the accepted bounded local lifecycle substrate.
- fixed: Exact two-stage pipeline producer and consumer records now install
  bounded process-table entries in slots 0 and 1 after their stable pids are
  assigned.
- fixed: Background /bin/status42 and /bin/zero launches preserve the prior
  foreground state while installing bounded process-table records in the
  corresponding background job slots.
- fixed: Focused unit tests inspect the pipeline producer/consumer table
  records and multiple background job table records directly.
- not-an-issue: Shell-visible waitpid source labels remain lifecycle-record,
  explicit-pid-lifecycle-record, and background-job-lifecycle-record for
  compatibility with retained transcripts; the process-table substrate is
  verified by focused unit inspection.
- not-an-issue: Existing pipeline-local accounting and jobs accounting output
  remain the compatibility surface while the internal process-table substrate
  backs the lifecycle/status records.
- deferred: Public process enumeration, procfs/ps command, true scheduler
  concurrency, fork/signals, process groups/sessions, waitpid options, PID
  reuse policy beyond bounded deterministic controls, multi-stage pipelines,
  pipefail, persistent storage, live networking, SSH, Pi 5 hardware proof, and
  phase transition.

## Evidence

- Static inspection: src/local_command_loop.rs generalizes the process-table
  record identity and extends table population to exact pipeline and accepted
  background job records without changing shell-visible compatibility output.
- Unit tests: cargo -Zjson-target-spec test --quiet local_command_loop passed.
- QEMU/substitute: scripts/qemu-local-shell-waitpid-any-completed-child-smoke.sh
  passed with task-owned evidence at
  tasks/evidence/2026-06-26-phase12-local-process-table-pipeline-background-core/qemu-local-shell-waitpid-any-completed-child-smoke.log.
- Retained QEMU/substitute transcript classification:
  qemu-local-shell-waitpid-any-completed-child-complete with final
  participants=19 expected=19 errors=0.

## Validation

- cargo fmt --all: passed before focused tests.
- cargo -Zjson-target-spec test --quiet local_command_loop: passed.
- QEMU/substitute waitpid-any-completed-child smoke: passed.
- cargo fmt --all -- --check: passed.
- cargo -Zjson-target-spec test --quiet: passed.
- jq -e empty on task-owned JSON evidence: passed.
- git diff --check: passed.
- /home/node/.cargo/bin/mdbook build: passed.
- git diff --cached --check: passed.

## Result

Accepted frontier: bounded internal process-table lifecycle/status records now
back direct foreground VFS exec, exact two-stage pipelines, and accepted
background VFS exec job records.

Selected next task after commit:
phase12-local-process-table-pipeline-background-closeout-20260626.

Live network/SSH reachability remains paused. No Pi 5 hardware claim is made.
