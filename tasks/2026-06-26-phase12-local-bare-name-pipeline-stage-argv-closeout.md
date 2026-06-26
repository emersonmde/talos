# Phase 12 Local Bare-Name Pipeline Stage Argv Closeout

Task id: phase12-local-bare-name-pipeline-stage-argv-closeout-20260626

Status: accepted; commit hash recorded in durable supervisor state after
commit.

## Scope

Reconcile the accepted bare-name pipeline stage argv frontier after the core
task accepted:

~~~text
stdout alpha | stdin beta
~~~

This closeout is static reconciliation work. It does not change source
behavior, run Pi 5 hardware, resume live networking/SSH, retry generated-root
command input, accept broader shell grammar, or accept a phase transition.

## Findings

- fixed: The accepted bare-name pipeline stage argv frontier is reconciled
  against the retained task record, classification, evidence map,
  QEMU/substitute transcript, docs, and regression evidence.
- fixed: The accepted evidence records producer argc=2, canonical
  argv0=/bin/stdout, argv1=alpha, deterministic empty envp, inherited fd0/fd2,
  fd1 as the pipe endpoint, a closed loader temporary descriptor, and a
  bounded process-table entry.
- fixed: The accepted evidence records consumer argc=2, canonical
  argv0=/bin/stdin, argv1=beta, deterministic empty envp, fd0 as the pipe
  endpoint, inherited fd1/fd2, a closed loader temporary descriptor, and a
  bounded process-table entry.
- fixed: Pipeline lifecycle/status, waitpid, laststatus,
  /proc/talos/processes, zero-argument ps, and pipestatus observations remain
  attached to the producer and consumer participants.
- fixed: Existing direct path-form pipeline stage argv, direct and bare-name
  command argv, no-argument direct and bare-name pipelines, multistage
  pipeline, process-status VFS, zero-argument ps, and pipestatus regression
  surfaces remain cited as retained controls.
- fixed: Unsupported bare-name pipeline argument shapes, unsupported literal
  characters, and unsupported bare commands remain fail-closed without
  successful process records.
- not-an-issue: No implementation change is required for this checkpoint; the
  bare-name pipeline stage argv core task already accepted the source behavior
  and regression evidence.
- deferred: Multistage pipeline argv, redirections, environment-backed PATH,
  current-directory search, command lookup beyond the bounded /bin surface,
  quoting, escaping, globbing, variables, shell functions, arbitrary shell
  grammar, unbounded pipelines, pipeline concurrency, scheduler concurrency,
  fork/signals, process groups/sessions, persistent storage, live
  networking/SSH, Pi 5 hardware proof, generated-root retry, and phase
  transition.

## Evidence Map

- Bare-name pipeline stage argv core:
  tasks/2026-06-26-phase12-local-bare-name-pipeline-stage-argv-core.md.
- Bare-name pipeline stage argv classification and evidence:
  tasks/evidence/2026-06-26-phase12-local-bare-name-pipeline-stage-argv-core/classification.json
  and
  tasks/evidence/2026-06-26-phase12-local-bare-name-pipeline-stage-argv-core/evidence-map.json.
- Bare-name pipeline stage argv QEMU/substitute transcript:
  tasks/evidence/2026-06-26-phase12-local-bare-name-pipeline-stage-argv-core/qemu-local-shell-bare-name-pipeline-stage-argv-smoke.log.
- Retained regression records:
  tasks/2026-06-26-phase12-local-direct-pipeline-stage-argv-core.md,
  tasks/2026-06-26-phase12-local-direct-pipeline-stage-argv-closeout.md,
  tasks/2026-06-26-phase12-local-direct-command-argv-core.md,
  tasks/2026-06-26-phase12-local-bare-name-command-argv-core.md,
  tasks/2026-06-26-phase12-local-command-argv-frontier-checkpoint.md,
  tasks/2026-06-26-phase12-local-absolute-path-vfs-pipeline-core.md,
  tasks/2026-06-26-phase12-local-bare-name-vfs-pipeline-core.md,
  tasks/2026-06-26-phase12-local-multistage-pipeline-core.md,
  tasks/2026-06-26-phase12-local-process-status-vfs-core.md,
  tasks/2026-06-26-phase12-local-ps-command-vfs-backed-core.md, and
  tasks/2026-06-26-phase12-local-pipeline-frontier-checkpoint.md.
- Closeout classification and evidence map:
  tasks/evidence/2026-06-26-phase12-local-bare-name-pipeline-stage-argv-closeout/classification.json
  and
  tasks/evidence/2026-06-26-phase12-local-bare-name-pipeline-stage-argv-closeout/evidence-map.json.
- Project notes:
  docs/src/roadmap.md, docs/src/project/phase12-networking-ssh.md, and
  docs/src/project/early-posix-shape.md.

## Accepted Frontier

The accepted bare-name pipeline stage argv frontier is local-only and
static/unit/QEMU-substitute backed. A bare-name two-stage pipeline can carry
one bounded literal argument per stage:

~~~text
stdout alpha | stdin beta
~~~

Each stage resolves through the fixed bounded /bin lookup and then executes
through descriptor-backed VFS open/read, the accepted loader, userspace
launch/status, and the serialized pipe descriptor handoff. The producer
records argc=2, canonical argv0=/bin/stdout, argv1=alpha, empty envp,
inherited standard descriptors with fd1 as the pipe endpoint, a closed loader
temporary descriptor, bounded process-table observation, and status 0. The
consumer records argc=2, canonical argv0=/bin/stdin, argv1=beta, empty envp,
inherited standard descriptors with fd0 as the pipe endpoint, a closed loader
temporary descriptor, bounded process-table observation, and status 0. The
bounded process table, waitpid, laststatus, /proc/talos/processes,
zero-argument ps, and pipestatus surfaces remain intact.

This closes the currently selected pipeline stage argv slice. The mechanically
objective follow-up is a frontier checkpoint that reconciles direct path-form
and bare-name pipeline argv before any later local POSIX/shell expansion.

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

selected_next_task: phase12-local-pipeline-stage-argv-frontier-checkpoint-20260626.

The pipeline stage argv frontier checkpoint is mechanically unblocked after
this accepted closeout is committed, provided the hardware lock remains
restored/unlocked and supervisor intervention remains inactive.
