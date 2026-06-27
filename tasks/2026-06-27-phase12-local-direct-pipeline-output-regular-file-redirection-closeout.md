# Phase 12 Local Direct Pipeline Output Regular-File Redirection Closeout

Task id: phase12-local-direct-pipeline-output-regular-file-redirection-closeout-20260627

Status: accepted; commit hash recorded in durable supervisor state after
commit.

## Scope

Reconcile the accepted direct path-form pipeline-output regular-file
redirection frontier against the retained task record, task-owned evidence, and
project docs.

The accepted witness remains exactly:

~~~text
/bin/stdout | /bin/stdin >/tmp/pipeline-report.txt
~~~

Both stages load through descriptor-backed VFS open/read and the accepted
userspace launch/status path. The producer writes to the accepted pipe
endpoint. The consumer reads from that pipe through fd0 and writes the
userspace stdin report through child-only fd1 to
volatile-vfs:/tmp/pipeline-report.txt. Shell descriptors are restored after the
pipeline exits.

This closeout does not accept fixed-/bin bare-name pipeline-output
redirection, append pipeline-output forms, pipeline input redirection
expansion, stderr or combined pipeline redirection forms, arbitrary output
paths, persistent writable filesystem behavior, generated-root retry, live
networking/SSH, Pi 5 hardware action, or phase transition.

## Findings

- fixed: The accepted direct pipeline-output regular-file redirection frontier
  is reconciled to the retained core task record, classification JSON, evidence
  map, QEMU/substitute regression, and project docs.
- fixed: The exact accepted witness remains
  '/bin/stdout | /bin/stdin >/tmp/pipeline-report.txt'; no fixed-/bin
  bare-name grammar, append behavior, stderr routing, arbitrary path policy, or
  persistent storage behavior is accepted by this closeout.
- fixed: The selected next task is
  phase12-local-bare-name-pipeline-output-regular-file-redirection-core-20260627
  because its dependencies are objective: the direct core is accepted and
  committed, this closeout reconciles the accepted evidence, supervisor
  intervention is inactive, and the hardware lock is restored/unlocked.
- not-an-issue: No runtime code change is required for this closeout; the core
  task already recorded static inspection, fmt/lint/typecheck, unit test,
  QEMU/substitute, JSON, diff, docs, and staged-diff evidence.
- deferred: Fixed-/bin bare-name pipeline-output redirection remains separate
  queued implementation work. Append pipeline-output forms, input/stderr/
  combined pipeline redirections, arbitrary paths, persistent writable
  filesystem behavior, environment-backed PATH, current-directory search,
  command lookup beyond bounded /bin, arbitrary shell grammar, unbounded or
  concurrent pipelines, scheduler concurrency, fork/signals, process
  groups/sessions, live networking/SSH, Pi 5 hardware proof, generated-root
  retry, and phase transition remain outside this task.

## Evidence Map

- Closeout classification:
  tasks/evidence/2026-06-27-phase12-local-direct-pipeline-output-regular-file-redirection-closeout/classification.json.
- Closeout evidence map:
  tasks/evidence/2026-06-27-phase12-local-direct-pipeline-output-regular-file-redirection-closeout/evidence-map.json.
- Retained core task record:
  tasks/2026-06-27-phase12-local-direct-pipeline-output-regular-file-redirection-core.md.
- Retained core evidence:
  tasks/evidence/2026-06-27-phase12-local-direct-pipeline-output-regular-file-redirection-core/.
- Project docs:
  docs/src/roadmap.md, docs/src/project/phase12-networking-ssh.md, and
  docs/src/project/early-posix-shape.md.

## Reconciled Frontier

The local-only, static/unit/QEMU-substitute direct pipeline-output
regular-file redirection frontier accepts exactly the absolute path-form
two-stage witness:

~~~text
/bin/stdout | /bin/stdin >/tmp/pipeline-report.txt
~~~

The producer is /bin/stdout, loaded through descriptor-backed VFS, with fd1 as
the pipe endpoint. The consumer is /bin/stdin, loaded through the same accepted
VFS/userspace path, with fd0 inherited as the pipe endpoint and child-only fd1
targeting volatile-vfs:/tmp/pipeline-report.txt. The retained evidence records
exec-redirection op=sink, target-path=/tmp/pipeline-report.txt, descriptor-
backed 'cat /tmp/pipeline-report.txt' readback of the userspace stdin report,
closed loader temporary descriptors, shell fd1 restoration, coherent
waitpid/laststatus observations, /proc/talos/processes entries, zero-argument
ps output, and pipestatus-compatible status accounting.

Unsupported direct forms remain fail-closed for alternate output targets,
append syntax, wrong final-stage programs, and neighboring malformed pipeline
redirections. Fixed-/bin bare-name pipeline-output redirection is the next
separate task, not accepted by this closeout.

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
phase12-local-bare-name-pipeline-output-regular-file-redirection-core-20260627.

The fixed-/bin bare-name pipeline-output regular-file redirection core is
mechanically unblocked for the next worker wake if this closeout remains
accepted and committed, the hardware lock remains restored/unlocked,
supervisor intervention remains inactive, and the repo has no conflicting
uncommitted changes.
