# Phase 12 Local Direct Command Argv Closeout

Task id: phase12-local-direct-command-argv-closeout-20260626

Status: accepted; commit hash recorded in durable supervisor state after
commit.

## Scope

Reconcile the accepted direct absolute-path command argv frontier after the
core task accepted:

~~~text
/bin/status42 alpha beta
~~~

This closeout is static reconciliation work. It does not change source
behavior, run Pi 5 hardware, resume live networking/SSH, retry generated-root
command input, accept broader shell grammar, or accept a phase transition.

## Findings

- fixed: The accepted direct command argv frontier is reconciled against the
  retained task record, classification, evidence map, QEMU/substitute
  transcript, docs, and regression evidence.
- fixed: The accepted evidence records argc=3, argv0=/bin/status42,
  argv1=alpha, argv2=beta, deterministic empty envp, inherited fd0/fd1/fd2,
  a closed loader temporary descriptor, status 0x2a, waitpid, laststatus,
  /proc/talos/processes, zero-argument ps, and pipestatus compatibility.
- fixed: Existing no-argument direct path command, bare-name command,
  bare-name pipeline, exec-prefixed literal argv, process-status VFS, ps, and
  pipestatus regression surfaces remain cited as retained controls.
- not-an-issue: No implementation change is required for this checkpoint; the
  direct argv core task already accepted the source behavior and regression
  evidence.
- deferred: Pipeline stage argv, redirections, environment-backed PATH,
  current-directory search, command lookup beyond the bounded /bin surface,
  arbitrary shell grammar, unbounded pipelines, pipeline concurrency,
  scheduler concurrency, fork/signals, process groups/sessions, persistent
  storage, live networking/SSH, Pi 5 hardware proof, generated-root retry, and
  phase transition.

## Evidence Map

- Direct command argv core:
  tasks/2026-06-26-phase12-local-direct-command-argv-core.md.
- Direct command argv classification and evidence:
  tasks/evidence/2026-06-26-phase12-local-direct-command-argv-core/classification.json
  and
  tasks/evidence/2026-06-26-phase12-local-direct-command-argv-core/evidence-map.json.
- Direct command argv QEMU/substitute transcript:
  tasks/evidence/2026-06-26-phase12-local-direct-command-argv-core/qemu-local-shell-direct-command-argv-smoke.log.
- Retained regression records:
  tasks/2026-06-26-phase12-local-bare-name-vfs-command-core.md,
  tasks/2026-06-26-phase12-local-bare-name-vfs-pipeline-core.md,
  tasks/2026-06-26-phase12-local-bare-name-path-frontier-checkpoint.md,
  tasks/2026-06-26-phase12-local-absolute-path-vfs-command-core.md,
  tasks/2026-06-26-phase12-local-absolute-path-vfs-pipeline-core.md,
  tasks/2026-06-26-phase12-local-path-command-frontier-checkpoint.md,
  tasks/2026-06-26-phase12-local-process-table-frontier-checkpoint.md,
  tasks/2026-06-26-phase12-local-posix-frontier-checkpoint.md, and
  tasks/2026-06-26-phase12-local-pipeline-frontier-checkpoint.md.
- Closeout classification and evidence map:
  tasks/evidence/2026-06-26-phase12-local-direct-command-argv-closeout/classification.json
  and
  tasks/evidence/2026-06-26-phase12-local-direct-command-argv-closeout/evidence-map.json.
- Project notes:
  docs/src/roadmap.md, docs/src/project/phase12-networking-ssh.md, and
  docs/src/project/early-posix-shape.md.

## Accepted Frontier

The accepted direct command argv frontier is local-only and
static/unit/QEMU-substitute backed. A direct absolute-path command can carry a
small literal argv vector through the accepted descriptor-backed path:

~~~text
/bin/status42 alpha beta
~~~

The executable still comes from VFS open/read and the accepted loader. The
userspace startup ABI records argc=3, canonical argv0=/bin/status42,
argv1=alpha, argv2=beta, empty envp, inherited standard descriptors, and a
closed loader temporary descriptor. The process exits with status 0x2a and
retains bounded process-table, waitpid, laststatus, /proc/talos/processes,
zero-argument ps, and pipestatus observations.

Direct command argv now gives a mechanically objective follow-up for bounded
bare-name argv through the already accepted fixed /bin lookup policy.

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

selected_next_task: phase12-local-bare-name-command-argv-core-20260626.

The bare-name command argv core task is mechanically unblocked after this
accepted closeout is committed, provided the hardware lock remains
restored/unlocked and supervisor intervention remains inactive.
