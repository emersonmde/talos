# Phase 12 Local Direct Pipeline-Output Append Regular-File Redirection Closeout

Task id: phase12-local-direct-pipeline-output-append-regular-file-redirection-closeout-20260627

Status: accepted; commit hash recorded in durable supervisor state after
commit.

## Scope

Reconcile the accepted direct path-form pipeline-output append regular-file
redirection core against retained local POSIX/VFS/userspace evidence and
project docs. No runtime behavior is added by this closeout.

The accepted witness remains exactly:

~~~text
/bin/stdout | /bin/stdin >/tmp/pipeline-report.txt
/bin/stdout | /bin/stdin >>/tmp/pipeline-report.txt
~~~

Both commands load both stages through descriptor-backed VFS open/read and the
accepted userspace launch/status path before pipe handoff. The final stage only
receives child-only fd1 redirection to
volatile-vfs:/tmp/pipeline-report.txt. The first command truncates/sinks the
consumer report; the second appends the same report at regular-file EOF.

This closeout does not accept fixed-/bin bare-name append, stderr pipeline
append, input or combined pipeline redirections, arbitrary output paths,
persistent writable filesystem behavior, generated-root retry, live
networking/SSH, Pi 5 hardware action, or phase transition.

## Findings

- fixed: Reconciled the accepted direct path-form pipeline-output append
  boundary against the retained core task record, classification JSON,
  evidence map, QEMU/substitute transcript, retained regression summary, and
  project docs.
- fixed: Froze the accepted witness as the exact direct sequence
  '/bin/stdout | /bin/stdin >/tmp/pipeline-report.txt' followed by
  '/bin/stdout | /bin/stdin >>/tmp/pipeline-report.txt'; no bare-name append,
  stderr append, input/combined pipeline redirection, arbitrary path policy, or
  persistent storage behavior is accepted by this closeout.
- fixed: Selected
  phase12-local-bare-name-pipeline-output-append-regular-file-redirection-core-20260627
  as the next task because the direct append core is accepted and committed,
  supervisor intervention is inactive, the hardware lock is restored/unlocked,
  and the queued bare-name task has explicit scope, gates, evidence, and
  non-goals.
- not-an-issue: No runtime code change is required for this closeout; the core
  task already recorded static inspection, fmt/lint/typecheck, unit test,
  QEMU/substitute, retained regression, JSON, diff, docs, and staged-diff
  evidence.
- deferred: Fixed-/bin bare-name pipeline-output append remains separate queued
  implementation work. Stderr pipeline append, input/combined pipeline
  redirections, arbitrary output paths, persistent writable filesystem
  behavior, environment-backed PATH, current-directory search, command lookup
  beyond bounded /bin, arbitrary shell grammar, unbounded or concurrent
  pipelines, scheduler concurrency, fork/signals, process groups/sessions,
  live networking/SSH, Pi 5 hardware proof, generated-root retry, and phase
  transition remain outside this task.

## Evidence Map

- Closeout classification:
  tasks/evidence/2026-06-27-phase12-local-direct-pipeline-output-append-regular-file-redirection-closeout/classification.json.
- Closeout evidence map:
  tasks/evidence/2026-06-27-phase12-local-direct-pipeline-output-append-regular-file-redirection-closeout/evidence-map.json.
- Retained core task record:
  tasks/2026-06-27-phase12-local-direct-pipeline-output-append-regular-file-redirection-core.md.
- Retained core evidence:
  tasks/evidence/2026-06-27-phase12-local-direct-pipeline-output-append-regular-file-redirection-core/.
- Project docs:
  docs/src/roadmap.md, docs/src/project/phase12-networking-ssh.md, and
  docs/src/project/early-posix-shape.md.

## Reconciled Frontier

The local-only, static/unit/QEMU-substitute direct pipeline-output append
frontier accepts exactly the absolute path-form two-command sequence:

~~~text
/bin/stdout | /bin/stdin >/tmp/pipeline-report.txt
/bin/stdout | /bin/stdin >>/tmp/pipeline-report.txt
~~~

For both commands, the producer is /bin/stdout, loaded through
descriptor-backed VFS, with fd1 as the pipe endpoint. The consumer is
/bin/stdin, loaded through the same accepted VFS/userspace path, with fd0
inherited as the pipe endpoint and child-only fd1 targeting
volatile-vfs:/tmp/pipeline-report.txt. The retained evidence records
exec-redirection op=sink for the first command, op=append for the second,
descriptor-backed 'cat /tmp/pipeline-report.txt' readback of two userspace
stdin reports in order, closed loader temporary descriptors, shell fd1
restoration, coherent waitpid/laststatus observations, /proc/talos/processes
entries, zero-argument ps output, and pipestatus-compatible status accounting.

Unsupported direct forms remain fail-closed for alternate output targets,
fixed-/bin bare-name append, wrong final-stage programs, stderr forms, input
redirection on pipelines, malformed append grammar, and arbitrary/persistent
paths. Fixed-/bin bare-name pipeline-output append is the next separate task,
not accepted by this closeout.

Live network/SSH remains paused. No Pi 5 hardware claim is made.

## Validation

- Static inspection of task records, retained evidence paths, docs, and git
  diff: passed.
- jq empty on task-owned JSON evidence: passed.
- git diff --check: passed.
- /home/node/.cargo/bin/mdbook build: passed.
- git diff --cached --check: passed before commit.

## Result

selected_next_task:
phase12-local-bare-name-pipeline-output-append-regular-file-redirection-core-20260627.

The fixed-/bin bare-name pipeline-output append regular-file redirection core
is mechanically unblocked for the next worker wake if this closeout remains
accepted and committed, the hardware lock remains restored/unlocked,
supervisor intervention remains inactive, and the repo has no conflicting
uncommitted changes.
