# Phase 12 Local Bare-Name Pipeline-Output Append Regular-File Redirection Closeout

Task id: phase12-local-bare-name-pipeline-output-append-regular-file-redirection-closeout-20260627

Status: accepted; commit hash recorded in durable supervisor state after
commit.

## Scope

Reconcile the accepted direct path-form and fixed-/bin bare-name
pipeline-output append regular-file redirection cores against retained local
POSIX/VFS/userspace evidence and project docs. No runtime behavior is added by
this closeout.

The accepted witnesses remain exactly:

~~~text
/bin/stdout | /bin/stdin >/tmp/pipeline-report.txt
/bin/stdout | /bin/stdin >>/tmp/pipeline-report.txt
stdout | stdin >/tmp/pipeline-report.txt
stdout | stdin >>/tmp/pipeline-report.txt
~~~

Both direct and bare-name forms load both pipeline stages through
descriptor-backed VFS open/read and the accepted userspace launch/status path
before pipe handoff. The bare-name form resolves only through the accepted
bounded fixed-/bin lookup to '/bin/stdout' and '/bin/stdin'. Only the final
stage receives child-only fd1 redirection to
'volatile-vfs:/tmp/pipeline-report.txt'. The first command in each pair
truncates/sinks the consumer report; the second appends the same report at
regular-file EOF.

This closeout does not accept stderr pipeline append, input or combined
pipeline redirections, arbitrary output paths, persistent writable filesystem
behavior, environment-backed PATH, current-directory search, command lookup
beyond bounded /bin, arbitrary shell grammar, generated-root retry, live
networking/SSH, Pi 5 hardware action, or phase transition.

## Findings

- fixed: Reconciled the accepted direct path-form and fixed-/bin bare-name
  pipeline-output append boundaries against retained core task records,
  classification JSON, evidence maps, QEMU/substitute transcripts, retained
  regression summary, and project docs.
- fixed: Froze the accepted direct witnesses as exactly
  '/bin/stdout | /bin/stdin >/tmp/pipeline-report.txt' followed by
  '/bin/stdout | /bin/stdin >>/tmp/pipeline-report.txt'.
- fixed: Froze the accepted bare-name witnesses as exactly
  'stdout | stdin >/tmp/pipeline-report.txt' followed by
  'stdout | stdin >>/tmp/pipeline-report.txt', resolved only through the
  bounded fixed-/bin lookup policy.
- fixed: Selected
  phase12-local-pipeline-output-append-regular-file-redirection-frontier-checkpoint-20260627
  as the next task because both direct and bare-name append cores are accepted
  and committed, supervisor intervention is inactive, the hardware lock is
  restored/unlocked, and the queued checkpoint has explicit scope, gates,
  evidence, and non-goals.
- not-an-issue: No runtime code change is required for this closeout; retained
  core evidence already records static inspection, fmt/lint/typecheck, unit
  test, QEMU/substitute, retained regression, JSON, diff, docs, and staged-diff
  gates.
- deferred: Stderr pipeline append, input/combined pipeline redirections,
  arbitrary output paths, persistent writable filesystem behavior,
  environment-backed PATH, current-directory search, command lookup beyond
  bounded /bin, arbitrary shell grammar, unbounded or concurrent pipelines,
  scheduler concurrency, fork/signals, process groups/sessions, live
  networking/SSH, Pi 5 hardware proof, generated-root retry, and phase
  transition remain outside this task.

## Evidence Map

- Closeout classification:
  tasks/evidence/2026-06-27-phase12-local-bare-name-pipeline-output-append-regular-file-redirection-closeout/classification.json.
- Closeout evidence map:
  tasks/evidence/2026-06-27-phase12-local-bare-name-pipeline-output-append-regular-file-redirection-closeout/evidence-map.json.
- Retained direct core task record:
  tasks/2026-06-27-phase12-local-direct-pipeline-output-append-regular-file-redirection-core.md.
- Retained direct closeout task record:
  tasks/2026-06-27-phase12-local-direct-pipeline-output-append-regular-file-redirection-closeout.md.
- Retained bare-name core task record:
  tasks/2026-06-27-phase12-local-bare-name-pipeline-output-append-regular-file-redirection-core.md.
- Retained direct and bare-name core evidence:
  tasks/evidence/2026-06-27-phase12-local-direct-pipeline-output-append-regular-file-redirection-core/.
  tasks/evidence/2026-06-27-phase12-local-bare-name-pipeline-output-append-regular-file-redirection-core/.
- Project docs:
  docs/src/roadmap.md, docs/src/project/phase12-networking-ssh.md, and
  docs/src/project/early-posix-shape.md.

## Reconciled Frontier

The local-only, static/unit/QEMU-substitute pipeline-output append frontier
accepts exactly the two direct path-form commands and the two fixed-/bin
bare-name commands listed in Scope. For both forms, the producer writes to the
pipe endpoint, the consumer reads from that pipe on fd0, and only the consumer
receives child-only fd1 redirection to
'volatile-vfs:/tmp/pipeline-report.txt'. The first command in each pair records
exec-redirection op=sink/truncate, and the second records op=append at EOF.
Descriptor-backed 'cat /tmp/pipeline-report.txt' reads two userspace stdin
reports in order, shell fd1 is restored afterward, and waitpid, laststatus,
/proc/talos/processes, zero-argument ps, and pipestatus-compatible
observations remain coherent.

Unsupported direct and bare-name variants remain fail-closed for alternate
output targets, unsupported command names, path-containing consumer names,
wrong final-stage programs, explicit '1>', spaced output grammar, malformed
append grammar, stderr forms, input redirection on pipelines, arbitrary output
paths, and persistent-storage claims.

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
phase12-local-pipeline-output-append-regular-file-redirection-frontier-checkpoint-20260627.

The pipeline-output append regular-file redirection frontier checkpoint is
mechanically unblocked for the next worker wake if this closeout remains
accepted and committed, the hardware lock remains restored/unlocked,
supervisor intervention remains inactive, and the repo has no conflicting
uncommitted changes.
