# Phase 12 Local Bare-Name Pipeline Output Regular-File Redirection Closeout

Task id: phase12-local-bare-name-pipeline-output-regular-file-redirection-closeout-20260627

Status: accepted; commit hash recorded in durable supervisor state after
commit.

## Scope

Reconcile the accepted fixed-/bin bare-name pipeline-output regular-file
redirection core against the retained direct path-form pipeline-output
evidence, task-owned evidence, and project docs.

The accepted bare-name witness remains exactly:

~~~text
stdout | stdin >/tmp/pipeline-report.txt
~~~

The command names resolve only through bounded fixed-/bin lookup to
'/bin/stdout' and '/bin/stdin'. Both stages load through descriptor-backed VFS
open/read and the accepted userspace launch/status path. The producer writes to
the accepted pipe endpoint; the consumer reads from that pipe on fd0 and writes
the userspace stdin report through child-only fd1 to
volatile-vfs:/tmp/pipeline-report.txt. The retained evidence records
descriptor-backed 'cat /tmp/pipeline-report.txt' readback and shell fd1
restoration after the pipeline exits.

This closeout does not implement runtime feature changes, append
pipeline-output redirection, stderr pipeline-output forms, input or combined
pipeline redirections, arbitrary output paths, persistent writable filesystem
behavior, generated-root retry, live networking/SSH, Pi 5 hardware action, or
a phase transition.

## Findings

- fixed: The accepted fixed-/bin bare-name pipeline-output frontier is
  reconciled to the retained core task record, classification JSON, evidence
  map, QEMU/substitute transcript, direct path-form closeout, roadmap entry,
  Phase 12 note, and early POSIX note.
- fixed: The exact accepted witness remains
  'stdout | stdin >/tmp/pipeline-report.txt', canonicalized only to
  '/bin/stdout' and '/bin/stdin' through the bounded /bin lookup before VFS
  open/read, userspace launch/status, pipe handoff, final-stage fd1 file sink,
  descriptor-backed readback, and shell fd1 restoration.
- fixed: The selected next task is
  phase12-local-pipeline-output-regular-file-redirection-frontier-checkpoint-20260627
  because the direct and fixed-/bin bare-name pipeline-output cores and
  closeouts are accepted/committed, supervisor intervention is inactive, and
  the hardware lock is restored/unlocked.
- not-an-issue: No runtime code change is required for this closeout; the core
  task already recorded static inspection, fmt/lint/typecheck, unit test,
  QEMU/substitute, JSON, diff, docs, staged-diff, and commit evidence.
- deferred: Append pipeline-output forms, stderr forms, input/combined pipeline
  redirections, arbitrary paths, persistent storage, environment-backed PATH,
  current-directory search, command lookup beyond bounded /bin, arbitrary shell
  grammar, unbounded/concurrent pipelines, scheduler concurrency,
  fork/signals, process groups/sessions, live networking/SSH, Pi 5 hardware
  proof, generated-root retry, and phase transition remain outside this task.

## Evidence Map

- Closeout classification:
  tasks/evidence/2026-06-27-phase12-local-bare-name-pipeline-output-regular-file-redirection-closeout/classification.json.
- Closeout evidence map:
  tasks/evidence/2026-06-27-phase12-local-bare-name-pipeline-output-regular-file-redirection-closeout/evidence-map.json.
- Retained bare-name core task record:
  tasks/2026-06-27-phase12-local-bare-name-pipeline-output-regular-file-redirection-core.md.
- Retained bare-name core evidence:
  tasks/evidence/2026-06-27-phase12-local-bare-name-pipeline-output-regular-file-redirection-core/.
- Retained direct comparison:
  tasks/2026-06-27-phase12-local-direct-pipeline-output-regular-file-redirection-core.md
  and
  tasks/2026-06-27-phase12-local-direct-pipeline-output-regular-file-redirection-closeout.md.
- Project docs:
  docs/src/roadmap.md, docs/src/project/phase12-networking-ssh.md, and
  docs/src/project/early-posix-shape.md.

## Reconciled Frontier

The local-only static/unit/QEMU-substitute pipeline-output regular-file
redirection frontier now has both accepted spellings:

~~~text
/bin/stdout | /bin/stdin >/tmp/pipeline-report.txt
stdout | stdin >/tmp/pipeline-report.txt
~~~

Both forms load both pipeline stages through descriptor-backed VFS open/read
and the accepted userspace launch/status path. In both forms, the producer fd1
is the pipe endpoint, the consumer fd0 is that pipe endpoint, and the consumer
fd1 is child-only redirected to volatile-vfs:/tmp/pipeline-report.txt.
Descriptor-backed 'cat /tmp/pipeline-report.txt' reads back the userspace
stdin report, loader temporary descriptors close, waitpid/laststatus
observations remain coherent, /proc/talos/processes and zero-argument ps see
the launched programs, pipestatus-compatible status accounting remains
retained, and the shell fd1 is restored afterward.

Unsupported bare-name forms remain fail-closed for alternate output targets,
append syntax, wrong final-stage programs, explicit '1>', spaced output
grammar, and consumer names containing path separators. Unsupported direct
forms remain fail-closed for alternate output targets, append syntax, wrong
final-stage programs, and neighboring malformed pipeline redirections.

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
phase12-local-pipeline-output-regular-file-redirection-frontier-checkpoint-20260627.

The local pipeline-output regular-file redirection frontier checkpoint is
mechanically unblocked for the next worker wake if this closeout remains
accepted and committed, the hardware lock remains restored/unlocked,
supervisor intervention remains inactive, and the repo has no conflicting
uncommitted changes.
