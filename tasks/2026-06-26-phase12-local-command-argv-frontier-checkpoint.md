# Phase 12 Local Command Argv Frontier Checkpoint

Task id: phase12-local-command-argv-frontier-checkpoint-20260626

Status: accepted; commit hash recorded in durable supervisor state after
commit.

## Scope

Reconcile the accepted command argv frontier after the direct absolute-path
and bare-name command argv slices:

~~~text
/bin/status42 alpha beta
status42 alpha beta
~~~

This checkpoint is static reconciliation work. It does not change source
behavior, run Pi 5 hardware, resume live networking/SSH, retry generated-root
command input, accept broader shell grammar, or accept a phase transition.

## Findings

- fixed: The accepted command argv frontier is reconciled across direct
  absolute-path argv and bounded bare-name argv.
- fixed: The evidence map cites retained direct command argv, bare-name
  command argv, no-argument direct and bare-name commands, bare-name pipeline,
  exec-prefixed literal argv, process-status VFS, zero-argument ps, and
  pipestatus task records and transcripts.
- fixed: Roadmap, Phase 12 project notes, and early POSIX notes now record the
  reconciled command argv frontier and the planning-needed result.
- not-an-issue: No implementation change is required; the direct and bare-name
  argv core tasks already accepted the source behavior and regression
  evidence.
- deferred: Pipeline stage argv, redirections, environment-backed PATH,
  command lookup beyond the bounded /bin surface, arbitrary shell grammar,
  unbounded pipelines, pipeline concurrency, scheduler concurrency,
  fork/signals, process groups/sessions, persistent storage, live
  networking/SSH, Pi 5 hardware proof, generated-root retry, and phase
  transition.

## Evidence Map

- Direct command argv core and closeout:
  tasks/2026-06-26-phase12-local-direct-command-argv-core.md,
  tasks/2026-06-26-phase12-local-direct-command-argv-closeout.md,
  tasks/evidence/2026-06-26-phase12-local-direct-command-argv-core/qemu-local-shell-direct-command-argv-smoke.log,
  and
  tasks/evidence/2026-06-26-phase12-local-direct-command-argv-closeout/evidence-map.json.
- Bare-name command argv core:
  tasks/2026-06-26-phase12-local-bare-name-command-argv-core.md,
  tasks/evidence/2026-06-26-phase12-local-bare-name-command-argv-core/qemu-local-shell-bare-name-command-argv-smoke.log,
  tasks/evidence/2026-06-26-phase12-local-bare-name-command-argv-core/classification.json,
  and
  tasks/evidence/2026-06-26-phase12-local-bare-name-command-argv-core/evidence-map.json.
- Retained command/process regression frontier:
  tasks/2026-06-26-phase12-local-absolute-path-vfs-command-core.md,
  tasks/2026-06-26-phase12-local-bare-name-vfs-command-core.md,
  tasks/2026-06-26-phase12-local-bare-name-vfs-pipeline-core.md,
  tasks/2026-06-26-phase12-local-bare-name-path-frontier-checkpoint.md,
  tasks/2026-06-26-phase12-local-process-status-vfs-core.md,
  tasks/2026-06-26-phase12-local-ps-command-vfs-backed-core.md,
  tasks/2026-06-26-phase12-local-pipefail-status-core.md, and
  tasks/2026-06-26-phase12-local-pipeline-frontier-checkpoint.md.
- Descriptor/VFS/POSIX docs:
  docs/src/project/early-posix-shape.md,
  docs/src/project/phase12-networking-ssh.md, and docs/src/roadmap.md.
- Checkpoint classification and evidence map:
  tasks/evidence/2026-06-26-phase12-local-command-argv-frontier-checkpoint/classification.json
  and
  tasks/evidence/2026-06-26-phase12-local-command-argv-frontier-checkpoint/evidence-map.json.

## Accepted Frontier

The accepted command argv frontier is local-only and static/unit/QEMU
substitute backed. Direct absolute-path commands and direct bare-name commands
can carry the small literal argv vector accepted by the two core slices:

~~~text
/bin/status42 alpha beta
status42 alpha beta
~~~

The direct path form opens /bin/status42 directly. The bare-name form resolves
only through the fixed /bin lookup to /bin/status42. Both then use
descriptor-backed VFS open/read, the accepted loader, userspace
startup/status, inherited standard descriptors, a closed loader temporary
descriptor, bounded process-table observations, waitpid, laststatus,
/proc/talos/processes, zero-argument ps, and pipestatus compatibility. Both
record argc=3, canonical argv0=/bin/status42, argv1=alpha, argv2=beta,
deterministic empty envp, and status 0x2a.

Live network/SSH reachability remains paused. No Pi 5 hardware claim is made.

## Deferred Frontier

Deferred surfaces remain pipeline stage argv, redirections, environment-backed
PATH, current-directory search, command lookup beyond the bounded /bin
surface, quoting, escaping, globbing, variables, arbitrary shell grammar,
unbounded pipelines, pipeline concurrency, scheduler concurrency,
fork/signals, process groups/sessions, broad procfs/Linux ps, PID policy
expansion, waitpid options, persistent storage, live networking/SSH, Pi 5
hardware proof, generated-root command-input retry, and phase transition.

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

Planning reason: no later queued same-lane local POSIX/shell task exists with
complete objective dependencies, acceptance criteria, validation gates, docs,
and evidence requirements. Supervisor planning is required before any next
local POSIX feature, any return to live network/SSH reachability work, any
hardware action, generated-root retry, pipeline stage argv, redirection,
environment-backed PATH expansion, broad shell expansion, or phase transition.
