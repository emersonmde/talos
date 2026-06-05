# Phase 10 Background Jobs Stale Entry Policy Core

Task: phase10-background-jobs-stale-entry-policy-core-20260605
Status: accepted

## Scope

Implement the smallest shell-visible stale/completed background job retention
policy for the accepted two-record background VFS exec accounting table.

Accepted shell-visible feature path:

- 'exec /bin/status42 &'
- 'exec /bin/zero &'
- 'jobs'
- 'jobs'
- 'jobs'

Non-goals stayed out of scope: arbitrary process tree/procfs inspection,
POSIX fg/bg/kill/disown, signals, process groups, sessions, terminal
ownership, fork, background pipelines/redirections, persistent storage, Pi 5
hardware proof, networking, SSH, and phase transition.

## Findings

- fixed: Added deterministic completed/reaped stale-entry clearing. 'jobs'
  reports retained background records, removes records already completed and
  reaped after that report, then observes one running job completion for the
  next inspection.
- fixed: Preserved the accepted multiple-background accounting surface:
  distinct stable job ids, pids, command labels, running/completed state,
  pending/completed status, observed status, and reaped flags.
- fixed: Preserved foreground lifecycle isolation. Background completions still
  do not create foreground waitable children or replace 'laststatus'; a later
  foreground 'exec /bin/zero' keeps normal consuming 'waitpid' and
  non-consuming 'laststatus' behavior.
- fixed: Added a task-owned QEMU/substitute stale-entry smoke script and
  scenario with three jobs inspections, retained foreground controls, and
  deterministic unsupported async forms.
- fixed: Reran the accepted multiple-background and prior jobs/accounting QEMU
  controls after the policy change; both retained PASS classifications.
- not-an-issue: No ADR is required. The retention rule is a minimal
  shell-owned accounting policy, not POSIX job control, procfs, or a broad
  process-table contract.
- deferred: fg/bg/kill/disown, signals, process groups, sessions, terminal
  ownership, fork, background pipelines/redirections, scheduler fairness proof,
  Pi 5 proof, persistent storage, networking, SSH, and phase transition.

## Evidence

- QEMU/substitute feature smoke:
  'tasks/evidence/2026-06-05-phase10-background-jobs-stale-entry-policy-core/qemu-local-shell-background-jobs-stale-entry-policy-smoke.log'
  reports PASS with
  'classification=qemu-local-shell-background-jobs-stale-entry-policy-complete',
  'final participants=16 expected=16 errors=0'.
- Retained-control inspection:
  'tasks/evidence/2026-06-05-phase10-background-jobs-stale-entry-policy-core/retained-control-inspection.txt'
  maps multiple-background records, prior jobs list, foreground
  waitpid/laststatus isolation, descriptor-backed cat/pipeline/file-redirection
  controls, and deterministic negatives.
- Retained multiple-background control rerun:
  'tasks/evidence/2026-06-05-phase10-multiple-background-vfs-exec-records-core/qemu-local-shell-multiple-background-jobs-smoke.log'
  reports PASS with
  'classification=qemu-local-shell-multiple-background-jobs-complete',
  'final participants=15 expected=15 errors=0'.
- Retained jobs/accounting list control rerun:
  'tasks/evidence/2026-06-05-phase10-jobs-accounting-list-core/qemu-local-shell-jobs-accounting-list-smoke.log'
  reports PASS with
  'classification=qemu-local-shell-jobs-accounting-list-complete',
  'final participants=17 expected=17 errors=0'.

Key task-owned evidence lines:

- 'talos: exec path=/bin/status42 source=vfs-open-read mode=background'
- 'talos: exec path=/bin/zero source=vfs-open-read mode=background'
- 'talos: jobs id=0x0000000000000001 pid=0x0000000000100001 command=/bin/status42 state=completed status=0x000000000000002a observed-status=0x000000000000002a reaped=true source=background-vfs-exec-accounting'
- 'talos: jobs id=0x0000000000000002 pid=0x0000000000100002 command=/bin/zero state=running status=pending reaped=false source=background-vfs-exec-accounting'
- 'talos: jobs id=0x0000000000000002 pid=0x0000000000100002 command=/bin/zero state=completed status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=background-vfs-exec-accounting'
- 'talos: jobs none source=background-vfs-exec-accounting'
- 'talos: waitpid no-child source=lifecycle-record'
- 'talos: last-process none'
- 'talos: waitpid pid=0x0000000000100001 parent=shell owner=0x0000000000000001 path=/bin/zero state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=lifecycle-record'

## Validation

- fmt/lint: 'cargo fmt --all -- --check' passed.
- unit tests: 'cargo -Zjson-target-spec test --quiet' passed with QEMU on PATH.
- QEMU/substitute: 'scripts/qemu-local-shell-background-jobs-stale-entry-policy-smoke.sh'
  passed and retained the feature evidence log.
- QEMU/substitute retained controls:
  'scripts/qemu-local-shell-multiple-background-jobs-smoke.sh' and
  'scripts/qemu-local-shell-jobs-accounting-list-smoke.sh' passed and retained
  updated control logs.
- static inspection: retained-control inspection note created and checked
  multiple-background records, prior jobs list, foreground waitpid/laststatus
  isolation, descriptor-backed cat/pipeline/file-redirection controls, and
  deterministic negatives.
- diff hygiene: 'git diff --check' passed.
- docs: '/home/node/.cargo/bin/mdbook build' passed.
- staged diff hygiene: 'git diff --cached --check' passed before commit.

hardwareTestLock remained unlocked/restored and unused.

## Commit

Commit: recorded in supervisor state after acceptance.
