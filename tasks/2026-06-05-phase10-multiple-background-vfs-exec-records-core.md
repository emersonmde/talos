# Phase 10 Multiple Background VFS Exec Records Core

Task: phase10-multiple-background-vfs-exec-records-core-20260605
Status: accepted

## Scope

Implement the smallest multiple-background VFS exec/accounting record path for
two accepted fixed-/bin background exec commands while preserving foreground
waitpid/laststatus isolation and retained shell controls.

Accepted shell-visible feature path:

- 'exec /bin/status42 &'
- 'exec /bin/zero &'
- 'jobs'

Non-goals stayed out of scope: POSIX fg/bg/kill/disown, signals, process
groups, sessions, terminal ownership, fork, background pipelines or
redirections, scheduler fairness proof, Pi 5 hardware, persistent storage,
networking, SSH, and phase transition.

## Findings

- fixed: Replaced the single retained background job slot with a bounded
  two-record table. Each record keeps a stable job id, pid, command label,
  running/completed state, pending/completed status, observed status, and
  reaped flag.
- fixed: Allowed the second accepted background command, 'exec /bin/zero &',
  through the same descriptor-backed VFS/open/read, loader, launch,
  descriptor-inheritance, startup ABI, and lifecycle/status path as the prior
  '/bin/status42' background command.
- fixed: Preserved foreground lifecycle isolation. Background completions do
  not create foreground waitable children or replace 'laststatus'; a later
  foreground 'exec /bin/zero' still updates consuming 'waitpid' and
  non-consuming 'laststatus' normally.
- fixed: Added a task-owned QEMU/substitute scenario and wrapper script for
  two background jobs, retained jobs controls, foreground isolation, and
  deterministic unsupported async forms.
- fixed: Updated the local command-loop boundary string to include the
  accepted multiple-background VFS exec record surface.
- not-an-issue: No ADR is required. The policy remains a bounded shell-owned
  accounting surface, not POSIX job control or a broad process table contract.
- deferred: stale-entry clearing/retention policy beyond this bounded table,
  fg/bg/kill/disown, signals, process groups, sessions, terminal ownership,
  fork, background pipelines/redirections, scheduler fairness proof, Pi 5
  proof, persistent storage, networking, SSH, and phase transition.

## Evidence

- QEMU/substitute feature smoke:
  'tasks/evidence/2026-06-05-phase10-multiple-background-vfs-exec-records-core/qemu-local-shell-multiple-background-jobs-smoke.log'
  reports PASS with
  'classification=qemu-local-shell-multiple-background-jobs-complete',
  'final participants=15 expected=15 errors=0'.
- Retained-control inspection:
  'tasks/evidence/2026-06-05-phase10-multiple-background-vfs-exec-records-core/retained-control-inspection.txt'
  maps the accepted background/jobs controls, pipeline/file redirection
  controls, descriptor-backed cat, waitpid/laststatus, and deterministic
  negatives.

Key evidence lines in the task-owned smoke:

- 'talos: exec path=/bin/status42 source=vfs-open-read mode=background'
- 'talos: background-job id=0x0000000000000001 pid=0x0000000000100001 command=/bin/status42 state=running reaped=false status=pending shell-responsive=true source=background-vfs-exec-accounting'
- 'talos: background-job id=0x0000000000000001 pid=0x0000000000100001 command=/bin/status42 state=completed status=0x000000000000002a observed-status=0x000000000000002a reaped=true shell-responsive=observed source=background-vfs-exec-accounting'
- 'talos: exec path=/bin/zero source=vfs-open-read mode=background'
- 'talos: background-job id=0x0000000000000002 pid=0x0000000000100002 command=/bin/zero state=running reaped=false status=pending shell-responsive=true source=background-vfs-exec-accounting'
- 'talos: jobs id=0x0000000000000002 pid=0x0000000000100002 command=/bin/zero state=completed status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=background-vfs-exec-accounting'
- 'talos: waitpid no-child source=lifecycle-record'
- 'talos: last-process none'
- 'talos: waitpid pid=0x0000000000100001 parent=shell owner=0x0000000000000001 path=/bin/zero state=exited status=0x0000000000000000 observed-status=0x0000000000000000 reaped=true source=lifecycle-record'

## Validation

- fmt/lint: 'cargo fmt --all -- --check' passed.
- unit tests: 'cargo -Zjson-target-spec test --quiet' passed.
- QEMU/substitute: 'scripts/qemu-local-shell-multiple-background-jobs-smoke.sh'
  passed and retained the feature evidence log.
- static inspection: retained-control inspection note created and checked
  accepted background exec, jobs list, foreground waitpid/laststatus isolation,
  pipeline/file redirection, descriptor-backed cat, and deterministic negative
  controls.
- diff hygiene: 'git diff --check' passed.
- docs: '/home/node/.cargo/bin/mdbook build' passed.
- staged diff hygiene: 'git diff --cached --check' passed before commit.

hardwareTestLock remained unlocked/restored and unused.

## Commit

Commit: recorded in supervisor state after acceptance.
