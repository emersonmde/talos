# Phase 12 Local Stderr Regular-File Redirection Frontier Checkpoint

Task id: phase12-local-stderr-regular-file-redirection-frontier-checkpoint-20260627

Status: accepted; commit hash recorded in durable supervisor state after
commit.

## Scope

Reconcile the accepted local-only stderr regular-file output redirection
frontier after the direct path-form and fixed-/bin bare-name closeouts accepted:

~~~text
/bin/stderr 2>/tmp/stderr.txt
stderr 2>/tmp/stderr.txt
~~~

This checkpoint is static reconciliation work. It does not change source
behavior, run Pi 5 hardware, resume live networking/SSH, retry generated-root
command input, accept append/truncate expansion, accept arbitrary output paths,
accept pipeline output redirection, accept combined input/output redirection,
accept persistent writable filesystem behavior, or accept a phase transition.

## Findings

- fixed: The accepted stderr regular-file redirection frontier is reconciled
  against the retained direct path-form core/closeout records, fixed-/bin
  bare-name core/closeout records, task-owned classification/evidence maps,
  QEMU/substitute transcripts, docs, and regression controls.
- fixed: The accepted witnesses remain exactly
  '/bin/stderr 2>/tmp/stderr.txt' and 'stderr 2>/tmp/stderr.txt'. Bare-name
  command lookup is only the accepted fixed bounded /bin lookup to
  '/bin/stderr'.
- fixed: The retained evidence records child-only fd2 rebinding to
  volatile-vfs:/tmp/stderr.txt, userspace stderr fixture writes through the
  regular-file descriptor, descriptor readback through 'cat /tmp/stderr.txt',
  later normal stderr restoration for both '/bin/stderr' and 'stderr', closed
  loader temporary descriptor state, and coherent waitpid/laststatus/process
  observations.
- fixed: No next local POSIX task is selected because no later queued same-lane
  task has complete objective dependencies, acceptance criteria, validation
  gates, docs requirements, and evidence requirements. The durable state must
  set planningNeeded=true for supervisor planning.
- not-an-issue: No implementation change is required for this checkpoint; the
  core tasks already recorded static inspection, fmt/lint/typecheck, unit
  test, QEMU/substitute, JSON, diff, docs, and staged-diff evidence.
- deferred: Append/truncate, arbitrary /tmp policy, pipeline-output
  redirection, combined input/output redirection, persistent writable
  filesystem behavior, environment-backed PATH, current-directory search,
  command lookup beyond bounded /bin, quoting, escaping, globbing, variables,
  shell functions, arbitrary shell grammar, unbounded pipelines, pipeline
  concurrency, scheduler concurrency, fork/signals, process groups/sessions,
  live networking/SSH, Pi 5 hardware proof, generated-root retry, and phase
  transition.

## Evidence Map

- Direct stderr regular-file redirection core:
  tasks/2026-06-27-phase12-local-direct-stderr-regular-file-redirection-core.md.
- Direct stderr regular-file redirection closeout:
  tasks/2026-06-27-phase12-local-direct-stderr-regular-file-redirection-closeout.md.
- Bare-name stderr regular-file redirection core:
  tasks/2026-06-27-phase12-local-bare-name-stderr-regular-file-redirection-core.md.
- Bare-name stderr regular-file redirection closeout:
  tasks/2026-06-27-phase12-local-bare-name-stderr-regular-file-redirection-closeout.md.
- Task-owned checkpoint classification and evidence:
  tasks/evidence/2026-06-27-phase12-local-stderr-regular-file-redirection-frontier-checkpoint/classification.json
  and
  tasks/evidence/2026-06-27-phase12-local-stderr-regular-file-redirection-frontier-checkpoint/evidence-map.json.
- Project notes:
  docs/src/roadmap.md, docs/src/project/phase12-networking-ssh.md, and
  docs/src/project/early-posix-shape.md.

## Accepted Frontier

The accepted stderr regular-file output redirection frontier is local-only and
static/unit/QEMU-substitute backed. The accepted witnesses are exactly:

~~~text
/bin/stderr 2>/tmp/stderr.txt
stderr 2>/tmp/stderr.txt
~~~

The direct witness resolves through the explicit descriptor-backed VFS path.
The bare-name witness resolves only through the bounded fixed /bin lookup to
'/bin/stderr'. Each accepted launch records child-only fd2 rebinding to
volatile-vfs:/tmp/stderr.txt; the userspace stderr fixture writes through fd2;
a later descriptor-backed 'cat /tmp/stderr.txt' reads the same fixture bytes;
and a subsequent normal stderr command records fd2 restored to
runtime-console0/stderr, proving shell fd2 restoration.

Live network/SSH reachability remains paused. No Pi 5 hardware claim is made.

## Deferred Frontier

Deferred surfaces remain append/truncate, arbitrary /tmp policy,
pipeline-output redirection, combined input/output redirection, persistent
writable filesystem behavior, environment-backed PATH, current-directory
search, command lookup beyond bounded /bin, quoting, escaping, globbing,
variables, shell functions, arbitrary shell grammar, unbounded pipelines,
pipeline concurrency, scheduler concurrency, fork/signals, process
groups/sessions, broad procfs/Linux ps, PID policy expansion, waitpid options,
live networking/SSH, Pi 5 hardware proof, generated-root command-input retry,
and phase transition.

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

No later queued same-lane local POSIX/VFS task is mechanically objective after
this checkpoint. Supervisor planning is required before further worker
promotion.
