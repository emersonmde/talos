# Phase 10 jobs/accounting list core

Task: 'phase10-jobs-accounting-list-core-20260605'

## Goal

Accept the thinnest real jobs/accounting inspection surface for the existing
background VFS exec lifecycle record: 'jobs' reports the single accepted
background job before launch, while running, and after completion without
claiming POSIX job control.

Accepted shell-visible forms:

- 'jobs'
- retained background launch control: 'exec /bin/status42 &'
- unsupported job-control negatives: 'fg', 'bg', 'kill %1'

## Scope

- Add a bounded 'jobs' command to the local command loop.
- Report the existing shell-owned background accounting record with stable job
  id, pid, command label, state, status, observed status when completed, and
  reaped flag.
- Preserve foreground 'waitpid' and 'laststatus' semantics: background
  accounting must not create or consume a foreground lifecycle record.
- Retain accepted background exec, foreground waitpid/laststatus, pipeline,
  descriptor inheritance, and descriptor-backed cat behavior as controls.

## Non-goals

- No 'kill', 'fg', 'bg', 'disown', signals, process groups, sessions, terminal
  ownership, arbitrary process tree inspection, multiple background jobs, Pi 5
  hardware proof, networking, SSH, or phase transition.
- No broad 'ps' or procfs contract; this remains a minimal shell-owned
  accounting inspection surface.

## Findings and Disposition

- Fixed: 'jobs' now reports 'talos: jobs none' before any background launch.
- Fixed: after 'exec /bin/status42 &', the first 'jobs' reports the stable job
  id, pid, command '/bin/status42', 'state=running', 'status=pending', and
  'reaped=false' from the accepted background accounting record.
- Fixed: a later 'jobs' reports the same stable id/pid/command with
  'state=completed', status '0x2a', matching observed status, and 'reaped=true'.
- Fixed: background accounting inspection leaves foreground 'waitpid' as
  'no-child' and foreground 'laststatus' as 'last-process none' until a normal
  foreground exec occurs.
- Fixed: the command inventory and builtins frontier now include
  'jobs-accounting-list', and the task has a dedicated QEMU/substitute wrapper,
  scenario, dispatch table, classification, and evidence path.
- Fixed: the new QEMU scenario is registered in 'build.rs' check-cfg metadata,
  avoiding noisy unexpected-cfg warnings.
- Deferred: multiple jobs, POSIX job-control commands, signal delivery, process
  groups, sessions, terminal ownership, process-tree/procfs inspection, true
  scheduler-concurrent userspace execution, Pi 5 proof, networking, SSH, and
  phase transition.
- Not-an-issue: the first 'jobs' inspection advances the accepted model from
  running to completed for the next inspection. This mirrors the previous
  command-boundary completion model while letting the inspection surface prove
  both supported states in one bounded transcript.

## Evidence

Primary QEMU/substitute evidence:

- 'tasks/evidence/2026-06-05-phase10-jobs-accounting-list-core/qemu-local-shell-jobs-accounting-list-smoke.log'

The retained log records:

- 'jobs' before launch reporting no job;
- 'exec /bin/status42 &' through VFS/open/read, loader, launch, descriptor
  inheritance, startup ABI, and background accounting;
- 'jobs' reporting the running record with stable id/pid/command and pending
  status;
- a second 'jobs' reporting the completed/reaped record with status '0x2a';
- foreground 'waitpid no-child' and 'last-process none' after background
  accounting observation;
- foreground '/bin/zero' 'waitpid' and 'laststatus' controls;
- retained plain pipeline transfer and descriptor-backed 'cat /etc/banner.txt';
- deterministic unsupported job-control negatives for 'fg', 'bg', and
  'kill %1';
- 'errors=0', 'classification=qemu-local-shell-jobs-accounting-list-complete',
  and 'PASS'.

Retained control evidence inspected:

- 'tasks/evidence/2026-06-05-phase10-jobs-accounting-list-core/retained-control-inspection.txt'
- 'tasks/evidence/2026-06-05-phase10-background-vfs-exec-lifecycle-core/qemu-local-shell-background-vfs-exec-lifecycle-smoke.log'
- 'tasks/evidence/2026-06-05-phase10-pipeline-consumer-output-redirection-core/qemu-local-shell-pipeline-consumer-output-redirection-smoke.log'
- 'tasks/evidence/2026-06-05-phase10-pipeline-producer-file-redirection-away-core/qemu-local-shell-pipeline-producer-file-redirection-away-smoke.log'
- 'tasks/evidence/2026-06-03-phase10-minimal-waitpid-lifecycle-observation-core/qemu-local-shell-waitpid-lifecycle-smoke.log'

## Validation

- fmt/lint: 'cargo fmt --all -- --check' passed.
- unit tests: 'cargo -Zjson-target-spec test --quiet' passed with 419 no_std
  tests.
- QEMU/substitute: 'scripts/qemu-local-shell-jobs-accounting-list-smoke.sh
  --quiet' passed and retained the primary evidence log.
- static inspection: retained-control grep check passed for task-owned and
  historical control logs.
- docs validation: '/home/node/.cargo/bin/mdbook build' passed, with the
  existing large search-index warning.
- diff hygiene: 'git diff --check' passed.
- staged diff hygiene: 'git diff --cached --check' passed before commit.

Implementation/evidence commit: recorded in durable supervisor state after
commit.
