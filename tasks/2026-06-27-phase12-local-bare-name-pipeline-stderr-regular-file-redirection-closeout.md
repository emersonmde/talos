# Phase 12 Local Bare-Name Pipeline Stderr Regular-File Redirection Closeout

Task id: phase12-local-bare-name-pipeline-stderr-regular-file-redirection-closeout-20260627

Status: accepted; commit hash recorded in durable supervisor state after
commit.

## Scope

Reconcile the accepted direct path-form and fixed-/bin bare-name pipeline
final-stage stderr regular-file redirection frontier against retained local
POSIX/VFS/userspace evidence and project docs. No runtime behavior is added by
this closeout.

The accepted witnesses remain exactly:

~~~text
/bin/stdout | /bin/stderr 2>/tmp/pipeline-stderr.txt
stdout | stderr 2>/tmp/pipeline-stderr.txt
~~~

Both forms load both stages through descriptor-backed VFS open/read and the
accepted userspace launch/status path before pipe handoff. The direct form uses
explicit program paths. The bare-name form resolves only through the accepted
fixed bounded /bin lookup to /bin/stdout and /bin/stderr. In both forms the
producer receives fd1 as the pipe endpoint, and the final-stage consumer
receives fd0 from that pipe endpoint plus only a child-owned fd2 redirection to
volatile-vfs:/tmp/pipeline-stderr.txt.

This closeout does not accept stderr append for pipelines, stdout final-stage
redirection for this pipeline shape, input or combined pipeline redirections,
arbitrary output paths, persistent writable filesystem behavior,
environment-backed PATH, current-directory search, command lookup beyond bounded
/bin, generated-root retry, live networking/SSH, Pi 5 hardware action, or phase
transition.

## Findings

- fixed: Reconciled the accepted bare-name pipeline stderr redirection core
  against the retained direct closeout, bare-name core task record,
  classification JSON, evidence map, QEMU/substitute transcripts, regression
  summary, and project docs.
- fixed: Froze the accepted bare-name witness as exactly
  'stdout | stderr 2>/tmp/pipeline-stderr.txt'; it resolves only through the
  accepted fixed bounded /bin lookup and does not imply PATH, current-directory
  search, or broader command lookup.
- fixed: Preserved the direct path-form witness
  '/bin/stdout | /bin/stderr 2>/tmp/pipeline-stderr.txt' as an accepted
  regression control for the same final-stage child-only stderr sink/truncate
  semantics.
- fixed: Selected
  phase12-local-pipeline-stderr-regular-file-redirection-frontier-checkpoint-20260627
  as the next task because the direct and bare-name pipeline stderr boundaries
  are accepted and committed, supervisor intervention is inactive, the hardware
  lock is restored/unlocked, and the queued checkpoint has explicit scope,
  gates, evidence, and non-goals.
- not-an-issue: No runtime code change is required for this closeout; the core
  task already recorded static inspection, fmt/lint/typecheck, full test,
  QEMU/substitute smoke, retained regression, JSON, diff, docs, and staged-diff
  evidence.
- not-an-issue: The stderr fixture intentionally writes to stderr and does not
  read stdin; retained evidence therefore keeps producer bytes-written=0x1f,
  consumer pipe bytes-read=0, and reader-eof=false while still proving the
  final stage inherited fd0 from the pipe endpoint.
- deferred: Stderr append pipeline redirection, stdout final-stage redirection
  for this pipeline shape, input/combined pipeline redirections, arbitrary
  output paths, persistent writable filesystem behavior, environment-backed
  PATH, current-directory search, command lookup beyond bounded /bin, arbitrary
  shell grammar, unbounded or concurrent pipelines, scheduler concurrency,
  fork/signals, process groups/sessions, live networking/SSH, Pi 5 hardware
  proof, generated-root retry, and phase transition remain outside this task.

## Evidence Map

- Closeout classification:
  tasks/evidence/2026-06-27-phase12-local-bare-name-pipeline-stderr-regular-file-redirection-closeout/classification.json.
- Closeout evidence map:
  tasks/evidence/2026-06-27-phase12-local-bare-name-pipeline-stderr-regular-file-redirection-closeout/evidence-map.json.
- Retained bare-name core task record:
  tasks/2026-06-27-phase12-local-bare-name-pipeline-stderr-regular-file-redirection-core.md.
- Retained bare-name core evidence:
  tasks/evidence/2026-06-27-phase12-local-bare-name-pipeline-stderr-regular-file-redirection-core/.
- Retained direct closeout:
  tasks/2026-06-27-phase12-local-direct-pipeline-stderr-regular-file-redirection-closeout.md.
- Project docs:
  docs/src/roadmap.md, docs/src/project/phase12-networking-ssh.md, and
  docs/src/project/early-posix-shape.md.

## Reconciled Frontier

The local-only, static/unit/QEMU-substitute pipeline stderr redirection frontier
now accepts exactly the direct path-form command:

~~~text
/bin/stdout | /bin/stderr 2>/tmp/pipeline-stderr.txt
~~~

and the fixed-/bin bare-name command:

~~~text
stdout | stderr 2>/tmp/pipeline-stderr.txt
~~~

For the bare-name form, stdout and stderr resolve only through bounded /bin
lookup to /bin/stdout and /bin/stderr. For both forms, the producer and consumer
load through descriptor-backed VFS and the accepted userspace launch/status
path. The producer fd1 is the pipe endpoint. The consumer fd0 is the same pipe
endpoint, and consumer fd2 is a child-only regular-file route to
volatile-vfs:/tmp/pipeline-stderr.txt using sink/truncate semantics. Retained
evidence records descriptor-backed 'cat /tmp/pipeline-stderr.txt' readback of
the 0x1f-byte stderr fixture, shell fd2 restoration through a later normal
stderr command, closed loader temporary descriptors, coherent
waitpid/laststatus observations, /proc/talos/processes entries, zero-argument
ps output, and pipestatus-compatible status accounting.

Unsupported neighboring forms remain fail-closed for append 2>>, unsupported
command names, path-containing stage names in the bare-name witness, stdout
final-stage redirection, input redirection on pipelines, combined pipeline
redirections, malformed spacing/grammar, alternate or arbitrary paths,
persistent storage claims, PATH/current-directory lookup, and command lookup
beyond bounded /bin.

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
phase12-local-pipeline-stderr-regular-file-redirection-frontier-checkpoint-20260627.

The pipeline stderr regular-file redirection frontier checkpoint is mechanically
unblocked for the next worker wake if this closeout remains accepted and
committed, the hardware lock remains restored/unlocked, supervisor intervention
remains inactive, and the repo has no conflicting uncommitted changes.
