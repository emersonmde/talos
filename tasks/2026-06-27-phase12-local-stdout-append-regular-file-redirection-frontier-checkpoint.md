# Phase 12 Local Stdout Append Regular-File Redirection Frontier Checkpoint

Task id: phase12-local-stdout-append-regular-file-redirection-frontier-checkpoint-20260627

Status: accepted; commit hash recorded in durable supervisor state after
commit.

## Scope

Reconcile the accepted local-only stdout append regular-file redirection
frontier after the direct path-form and fixed-/bin bare-name closeouts accepted:

~~~text
/bin/stdout >/tmp/stdout.txt
/bin/stdout >>/tmp/stdout.txt
stdout >/tmp/stdout.txt
stdout >>/tmp/stdout.txt
~~~

This checkpoint is static reconciliation work. It does not change source
behavior, run Pi 5 hardware, resume live networking/SSH, retry generated-root
command input, accept stderr append, accept arbitrary output paths, accept
pipeline-output redirection, accept combined input/output redirection, accept
persistent writable filesystem behavior, or accept a phase transition.

## Findings

- fixed: The accepted stdout append frontier is reconciled against the retained
  direct path-form core/closeout records, fixed-/bin bare-name core/closeout
  records, task-owned classification/evidence maps, QEMU/substitute
  transcripts, docs, and regression controls.
- fixed: The accepted witnesses remain exactly the direct path-form sequence
  '/bin/stdout >/tmp/stdout.txt' then '/bin/stdout >>/tmp/stdout.txt' and the
  fixed-/bin bare-name sequence 'stdout >/tmp/stdout.txt' then
  'stdout >>/tmp/stdout.txt'. Bare-name command lookup is only the accepted
  fixed bounded /bin lookup to '/bin/stdout'.
- fixed: The retained evidence records child-only fd1 rebinding to
  volatile-vfs:/tmp/stdout.txt, initial truncate/sink output, second append at
  regular-file EOF, descriptor-backed 'cat /tmp/stdout.txt' readback of both
  userspace stdout fixture writes in order, later normal stdout restoration for
  both '/bin/stdout' and 'stdout', closed loader temporary descriptor state,
  and coherent waitpid/laststatus/process observations.
- fixed: No next local POSIX/VFS task is selected because no later queued
  same-lane task has complete objective dependencies, acceptance criteria,
  validation gates, docs requirements, and evidence requirements. Durable state
  must set planningNeeded=true for supervisor planning.
- not-an-issue: No implementation change is required for this checkpoint; the
  core tasks already recorded static inspection, fmt/lint/typecheck, unit
  test, QEMU/substitute, JSON, diff, docs, and staged-diff evidence.
- deferred: Stderr append, arbitrary output path policy, pipeline-output
  redirection, combined input/output redirection, persistent writable
  filesystem behavior, environment-backed PATH, current-directory search,
  command lookup beyond bounded /bin, quoting, escaping, globbing, variables,
  shell functions, arbitrary shell grammar, unbounded pipelines, pipeline
  concurrency, scheduler concurrency, fork/signals, process groups/sessions,
  live networking/SSH, Pi 5 hardware proof, generated-root retry, and phase
  transition.

## Evidence Map

- Direct stdout append regular-file redirection core:
  tasks/2026-06-27-phase12-local-direct-stdout-append-regular-file-redirection-core.md.
- Direct stdout append regular-file redirection closeout:
  tasks/2026-06-27-phase12-local-direct-stdout-append-regular-file-redirection-closeout.md.
- Bare-name stdout append regular-file redirection core:
  tasks/2026-06-27-phase12-local-bare-name-stdout-append-regular-file-redirection-core.md.
- Bare-name stdout append regular-file redirection closeout:
  tasks/2026-06-27-phase12-local-bare-name-stdout-append-regular-file-redirection-closeout.md.
- Task-owned checkpoint classification and evidence:
  tasks/evidence/2026-06-27-phase12-local-stdout-append-regular-file-redirection-frontier-checkpoint/classification.json
  and
  tasks/evidence/2026-06-27-phase12-local-stdout-append-regular-file-redirection-frontier-checkpoint/evidence-map.json.
- Project notes:
  docs/src/roadmap.md, docs/src/project/phase12-networking-ssh.md, and
  docs/src/project/early-posix-shape.md.

## Accepted Frontier

The accepted stdout append regular-file output redirection frontier is
local-only and static/unit/QEMU-substitute backed. The accepted witnesses are
exactly:

~~~text
/bin/stdout >/tmp/stdout.txt
/bin/stdout >>/tmp/stdout.txt
stdout >/tmp/stdout.txt
stdout >>/tmp/stdout.txt
~~~

The direct witnesses resolve through the explicit descriptor-backed VFS path.
The bare-name witnesses resolve only through the bounded fixed /bin lookup to
'/bin/stdout'. Each accepted launch records child-only fd1 rebinding to
volatile-vfs:/tmp/stdout.txt; the first write truncates/sinks the volatile
regular file; the second write appends at regular-file EOF; a later
descriptor-backed 'cat /tmp/stdout.txt' reads both stdout fixture writes in
order; and a subsequent normal stdout command records fd1 restored to
runtime-console0/stdout, proving shell fd1 restoration.

Live network/SSH reachability remains paused. No Pi 5 hardware claim is made.

## Deferred Frontier

Deferred surfaces remain stderr append, arbitrary output path policy,
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
