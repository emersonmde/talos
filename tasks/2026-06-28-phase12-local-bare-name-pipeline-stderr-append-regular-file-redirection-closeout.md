# Phase 12 Local Bare-Name Pipeline Stderr Append Regular-File Redirection Closeout

Task id: phase12-local-bare-name-pipeline-stderr-append-regular-file-redirection-closeout-20260628

Status: accepted; commit hash recorded in durable supervisor state after
commit.

## Scope

Reconcile the accepted direct path-form and fixed-/bin bare-name pipeline
final-stage stderr append regular-file redirection cores against retained local
POSIX/VFS/userspace evidence and project docs. No runtime behavior is added by
this closeout.

The accepted witness sequences remain exactly:

~~~text
/bin/stdout | /bin/stderr 2>/tmp/pipeline-stderr.txt
/bin/stdout | /bin/stderr 2>>/tmp/pipeline-stderr.txt
stdout | stderr 2>/tmp/pipeline-stderr.txt
stdout | stderr 2>>/tmp/pipeline-stderr.txt
~~~

Direct forms keep explicit program paths. Bare-name forms resolve only through
the accepted fixed bounded /bin lookup to /bin/stdout and /bin/stderr. All
forms load both stages through descriptor-backed VFS open/read and the accepted
userspace launch/status path before pipe handoff. The producer receives fd1 as
the pipe endpoint. The final-stage consumer receives fd0 from that pipe
endpoint and only a child-owned fd2 redirection to
volatile-vfs:/tmp/pipeline-stderr.txt. The first pipeline truncates/sinks the
stderr fixture into the volatile file. The second pipeline appends the same
fixture at EOF.

This closeout does not accept input or combined pipeline redirections, stdout
final-stage redirection for this pipeline shape, arbitrary output paths,
persistent writable filesystem behavior, generated-root retry, live
networking/SSH, Pi 5 hardware action, or phase transition.

## Findings

- fixed: Reconciled the accepted direct path-form and fixed-/bin bare-name
  pipeline stderr append boundaries against retained core task records,
  classification JSON, evidence maps, focused QEMU/substitute transcripts,
  retained regression summaries, and project docs.
- fixed: Froze the accepted direct sequence as exactly
  '/bin/stdout | /bin/stderr 2>/tmp/pipeline-stderr.txt' followed by
  '/bin/stdout | /bin/stderr 2>>/tmp/pipeline-stderr.txt'.
- fixed: Froze the accepted fixed-/bin bare-name sequence as exactly
  'stdout | stderr 2>/tmp/pipeline-stderr.txt' followed by
  'stdout | stderr 2>>/tmp/pipeline-stderr.txt', with stage names resolving
  only through bounded /bin lookup to /bin/stdout and /bin/stderr.
- fixed: Selected
  phase12-local-pipeline-stderr-append-regular-file-redirection-frontier-checkpoint-20260628
  as the next task because the bare-name core is accepted and committed,
  supervisor intervention is inactive, the hardware lock is restored/unlocked,
  and the queued checkpoint has explicit scope, gates, evidence, and non-goals.
- not-an-issue: No runtime code change is required for this closeout; the core
  tasks already recorded static inspection, fmt/lint/typecheck, unit test,
  QEMU/substitute, retained regression, JSON, diff, docs, and staged-diff
  evidence.
- not-an-issue: The stderr fixture intentionally writes to stderr and does not
  read stdin; retained evidence therefore keeps producer bytes-written=0x1f,
  consumer pipe bytes-read=0, and reader-eof=false for both truncate and append
  runs while still proving the final stage inherited fd0 from the pipe
  endpoint.
- deferred: Input/combined pipeline redirections, stdout final-stage
  redirection for this pipeline shape, arbitrary output paths, persistent
  writable filesystem behavior, environment-backed PATH, current-directory
  search, command lookup beyond bounded /bin, arbitrary shell grammar,
  unbounded or concurrent pipelines, scheduler concurrency, fork/signals,
  process groups/sessions, live networking/SSH, Pi 5 hardware proof,
  generated-root retry, and phase transition remain outside this task.

## Evidence Map

- Closeout classification:
  tasks/evidence/2026-06-28-phase12-local-bare-name-pipeline-stderr-append-regular-file-redirection-closeout/classification.json.
- Closeout evidence map:
  tasks/evidence/2026-06-28-phase12-local-bare-name-pipeline-stderr-append-regular-file-redirection-closeout/evidence-map.json.
- Retained direct core task record:
  tasks/2026-06-28-phase12-local-direct-pipeline-stderr-append-regular-file-redirection-core.md.
- Retained direct closeout task record:
  tasks/2026-06-28-phase12-local-direct-pipeline-stderr-append-regular-file-redirection-closeout.md.
- Retained bare-name core task record:
  tasks/2026-06-28-phase12-local-bare-name-pipeline-stderr-append-regular-file-redirection-core.md.
- Retained bare-name core evidence:
  tasks/evidence/2026-06-28-phase12-local-bare-name-pipeline-stderr-append-regular-file-redirection-core/.
- Project docs:
  docs/src/roadmap.md, docs/src/project/phase12-networking-ssh.md, and
  docs/src/project/early-posix-shape.md.

## Reconciled Frontier

The local-only, static/unit/QEMU-substitute pipeline stderr append frontier
accepts exactly the direct path-form command sequence:

~~~text
/bin/stdout | /bin/stderr 2>/tmp/pipeline-stderr.txt
/bin/stdout | /bin/stderr 2>>/tmp/pipeline-stderr.txt
~~~

and exactly the fixed-/bin bare-name command sequence:

~~~text
stdout | stderr 2>/tmp/pipeline-stderr.txt
stdout | stderr 2>>/tmp/pipeline-stderr.txt
~~~

Direct forms keep explicit program paths. Bare-name forms resolve only through
bounded /bin lookup to /bin/stdout and /bin/stderr. The producer is stdout with
fd1 as the pipe endpoint. The consumer is stderr with fd0 inherited as the
pipe endpoint and child-only fd2 targeting
volatile-vfs:/tmp/pipeline-stderr.txt. Retained evidence records
exec-redirection op=sink for fd2 on the first run, exec-redirection op=append
at EOF for fd2 on the second run, two userspace stderr fixture writes,
descriptor-backed 'cat /tmp/pipeline-stderr.txt' readback bytes=0x3e, closed
loader temporary descriptors, shell fd2 restoration, coherent
waitpid/laststatus observations, /proc/talos/processes entries, zero-argument
ps output, and pipestatus-compatible status accounting.

Unsupported direct and bare-name forms remain fail-closed for unsupported
command names, path-containing stage names, stdout final-stage redirection,
input redirection, unsupported append targets, malformed spacing, alternate or
arbitrary paths, and persistent storage.

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
phase12-local-pipeline-stderr-append-regular-file-redirection-frontier-checkpoint-20260628.

The pipeline stderr append regular-file redirection frontier checkpoint is
mechanically unblocked for the next worker wake if this closeout remains
accepted and committed, the hardware lock remains restored/unlocked,
supervisor intervention remains inactive, and the repo has no conflicting
uncommitted changes.
