# Phase 12 Local Pipeline Stage Argv Frontier Checkpoint

Task id: phase12-local-pipeline-stage-argv-frontier-checkpoint-20260626

Status: accepted; commit hash recorded in durable supervisor state after
commit.

## Scope

Reconcile the accepted direct path-form and bare-name two-stage pipeline argv
frontier after these accepted surfaces:

~~~text
/bin/stdout alpha | /bin/stdin beta
stdout alpha | stdin beta
~~~

This checkpoint is static reconciliation work. It does not change source
behavior, run Pi 5 hardware, resume live networking/SSH, retry generated-root
command input, accept broader shell grammar, or accept a phase transition.

## Findings

- fixed: The accepted pipeline stage argv frontier is reconciled against the
  retained direct path-form and bare-name core/closeout records, task-owned
  classification/evidence JSON, QEMU/substitute transcripts, docs, and
  regression evidence.
- fixed: The direct path-form surface remains exactly
  /bin/stdout alpha | /bin/stdin beta through descriptor-backed VFS open/read,
  the accepted loader, userspace startup/status, serialized pipe descriptor
  handoff, bounded process-table observations, waitpid, laststatus,
  /proc/talos/processes, zero-argument ps, and pipestatus.
- fixed: The bare-name surface remains exactly stdout alpha | stdin beta; each
  stage resolves only through fixed bounded /bin lookup before using the same
  VFS/open/read, loader, userspace startup/status, pipe, process-table,
  waitpid, procfs, ps, and pipestatus layers.
- fixed: Existing command argv, no-argument direct and bare-name pipelines,
  multistage pipeline, process-status VFS, zero-argument ps, and pipestatus
  regression surfaces remain cited as retained controls.
- fixed: Unsupported extra pipeline stage arguments, unsupported literal
  characters, unsupported bare-name pipeline argument shapes, and unsupported
  bare commands remain fail-closed without accepted process records.
- not-an-issue: No implementation change is required for this checkpoint; the
  direct and bare-name pipeline stage argv core tasks already accepted the
  source behavior and regression evidence.
- deferred: Multistage pipeline argv, redirections, environment-backed PATH,
  current-directory search, command lookup beyond the bounded /bin surface,
  quoting, escaping, globbing, variables, shell functions, arbitrary shell
  grammar, unbounded pipelines, pipeline concurrency, scheduler concurrency,
  fork/signals, process groups/sessions, persistent storage, live
  networking/SSH, Pi 5 hardware proof, generated-root retry, and phase
  transition.

## Evidence Map

- Direct path-form pipeline stage argv core and closeout:
  tasks/2026-06-26-phase12-local-direct-pipeline-stage-argv-core.md and
  tasks/2026-06-26-phase12-local-direct-pipeline-stage-argv-closeout.md.
- Bare-name pipeline stage argv core and closeout:
  tasks/2026-06-26-phase12-local-bare-name-pipeline-stage-argv-core.md and
  tasks/2026-06-26-phase12-local-bare-name-pipeline-stage-argv-closeout.md.
- Direct path-form pipeline stage argv classification/evidence/transcript:
  tasks/evidence/2026-06-26-phase12-local-direct-pipeline-stage-argv-core/classification.json,
  tasks/evidence/2026-06-26-phase12-local-direct-pipeline-stage-argv-core/evidence-map.json,
  tasks/evidence/2026-06-26-phase12-local-direct-pipeline-stage-argv-core/qemu-local-shell-direct-pipeline-stage-argv-smoke.log,
  tasks/evidence/2026-06-26-phase12-local-direct-pipeline-stage-argv-closeout/classification.json,
  and
  tasks/evidence/2026-06-26-phase12-local-direct-pipeline-stage-argv-closeout/evidence-map.json.
- Bare-name pipeline stage argv classification/evidence/transcript:
  tasks/evidence/2026-06-26-phase12-local-bare-name-pipeline-stage-argv-core/classification.json,
  tasks/evidence/2026-06-26-phase12-local-bare-name-pipeline-stage-argv-core/evidence-map.json,
  tasks/evidence/2026-06-26-phase12-local-bare-name-pipeline-stage-argv-core/qemu-local-shell-bare-name-pipeline-stage-argv-smoke.log,
  tasks/evidence/2026-06-26-phase12-local-bare-name-pipeline-stage-argv-closeout/classification.json,
  and
  tasks/evidence/2026-06-26-phase12-local-bare-name-pipeline-stage-argv-closeout/evidence-map.json.
- Retained regression records:
  tasks/2026-06-26-phase12-local-direct-command-argv-core.md,
  tasks/2026-06-26-phase12-local-bare-name-command-argv-core.md,
  tasks/2026-06-26-phase12-local-command-argv-frontier-checkpoint.md,
  tasks/2026-06-26-phase12-local-absolute-path-vfs-pipeline-core.md,
  tasks/2026-06-26-phase12-local-bare-name-vfs-pipeline-core.md,
  tasks/2026-06-26-phase12-local-multistage-pipeline-core.md,
  tasks/2026-06-26-phase12-local-process-status-vfs-core.md,
  tasks/2026-06-26-phase12-local-ps-command-vfs-backed-core.md, and
  tasks/2026-06-26-phase12-local-pipeline-frontier-checkpoint.md.
- Checkpoint classification and evidence map:
  tasks/evidence/2026-06-26-phase12-local-pipeline-stage-argv-frontier-checkpoint/classification.json
  and
  tasks/evidence/2026-06-26-phase12-local-pipeline-stage-argv-frontier-checkpoint/evidence-map.json.
- Project notes:
  docs/src/roadmap.md, docs/src/project/phase12-networking-ssh.md, and
  docs/src/project/early-posix-shape.md.

## Accepted Frontier

The accepted pipeline stage argv frontier is local-only and
static/unit/QEMU-substitute backed. Direct path-form and bare-name two-stage
pipelines can carry one bounded literal argument per stage:

~~~text
/bin/stdout alpha | /bin/stdin beta
stdout alpha | stdin beta
~~~

Both forms execute through descriptor-backed VFS open/read, the accepted
loader, userspace launch/status, and the serialized pipe descriptor handoff.
The direct path-form surface uses the supplied absolute paths as argv0. The
bare-name surface canonicalizes through fixed bounded /bin lookup before
launch. In both forms, the producer records argc=2, argv1=alpha, empty envp,
inherited fd0/fd2, fd1 as the pipe endpoint, a closed loader temporary
descriptor, bounded process-table observation, and status 0. The consumer
records argc=2, argv1=beta, empty envp, fd0 as the pipe endpoint, inherited
fd1/fd2, a closed loader temporary descriptor, bounded process-table
observation, and status 0. The bounded process table, waitpid, laststatus,
/proc/talos/processes, zero-argument ps, and pipestatus surfaces remain
intact.

No later queued same-lane local POSIX/shell task exists with complete objective
dependencies, acceptance criteria, validation gates, docs, and evidence
requirements, so this checkpoint records planningNeeded=true rather than
selecting a new worker task.

Live network/SSH reachability remains paused. No Pi 5 hardware claim is made.

## Deferred Frontier

Deferred surfaces remain multistage pipeline argv, redirections,
environment-backed PATH, current-directory search, command lookup beyond the
bounded /bin surface, quoting, escaping, globbing, variables, shell functions,
arbitrary shell grammar, unbounded pipelines, pipeline concurrency, scheduler
concurrency, fork/signals, process groups/sessions, broad procfs/Linux ps, PID
policy expansion, waitpid options, persistent storage, live networking/SSH,
Pi 5 hardware proof, generated-root command-input retry, and phase transition.

## Validation

- Static inspection of task records, retained evidence paths, docs, and git
  diff: passed.
- jq empty on task-owned JSON evidence: passed.
- git diff --check: passed.
- /home/node/.cargo/bin/mdbook build: passed.
- git diff --cached --check: passed.

## Result

selected_next_task: null.

planningNeeded: true.

planningReason: No later queued same-lane local POSIX/shell task exists with
complete objective dependencies, acceptance criteria, validation gates, docs,
and evidence requirements after the accepted pipeline stage argv frontier.
