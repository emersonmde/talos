# Phase 12 Local Direct Combined Pipeline Stdin Stdout Redirection Closeout

Task id: phase12-local-direct-combined-pipeline-stdin-stdout-redirection-closeout-20260628

Status: accepted; commit hash recorded in durable supervisor state after
commit.

## Scope

Reconcile the accepted direct path-form combined pipeline stdin/stdout
redirection core against retained local POSIX/VFS/userspace evidence and
project docs. No runtime behavior is added by this closeout.

The accepted witness remains exactly:

~~~text
/bin/stdin </etc/banner.txt | /bin/stdin >/tmp/pipeline-combined.txt
~~~

Both stages load through descriptor-backed VFS and the accepted userspace
launch/status path. The producer receives child-only fd0 from
initramfs:/etc/banner.txt, fd1 as the pipe endpoint, and inherited fd2. The
consumer receives fd0 from that pipe endpoint, child-only fd1 to
volatile-vfs:/tmp/pipeline-combined.txt, and inherited fd2. The shell fd0/fd1
surface is restored after the pipeline.

This closeout does not accept fixed-/bin bare-name combined pipeline
redirection, append or stderr combined pipeline forms, arbitrary input/output
paths, separated redirection tokens, explicit fd1 syntax, multistage combined
redirection, unsupported stage names, persistent writable filesystem behavior,
generated-root retry, live networking/SSH, Pi 5 hardware action, or phase
transition.

## Findings

- fixed: Reconciled the accepted direct path-form combined pipeline
  stdin/stdout boundary against the retained core task record, classification
  JSON, evidence map, focused QEMU/substitute transcript, retained local
  POSIX/VFS regression summaries, and project docs.
- fixed: Froze the accepted command as exactly
  '/bin/stdin </etc/banner.txt | /bin/stdin >/tmp/pipeline-combined.txt',
  preserving descriptor-backed VFS loading for both stages, producer fd0 from
  initramfs:/etc/banner.txt, producer fd1 to the pipe endpoint, consumer fd0
  from that pipe endpoint, consumer fd1 to
  volatile-vfs:/tmp/pipeline-combined.txt, inherited fd2, closed loader
  temporary descriptors, descriptor-backed readback, and coherent process
  status observations.
- fixed: Selected
  phase12-local-bare-name-combined-pipeline-stdin-stdout-redirection-core-20260628
  as the next task because the direct core is accepted and committed,
  supervisor intervention is inactive, the hardware lock is restored/unlocked,
  and the queued bare-name core has explicit scope, gates, evidence, and
  non-goals.
- not-an-issue: No runtime code change is required for this closeout; the core
  task already recorded static inspection, fmt/lint/typecheck, unit test,
  QEMU/substitute, retained regression, JSON, diff, docs, and staged-diff
  evidence.
- deferred: Fixed-/bin bare-name combined pipeline redirection remains
  deferred to the selected bare-name core.
- deferred: Append or stderr combined pipeline forms, arbitrary paths,
  separated redirection tokens, explicit fd1 syntax, multistage combined
  redirection, unsupported stage names, persistent storage, environment-backed
  PATH, current-directory search, command lookup beyond bounded /bin,
  arbitrary shell grammar, unbounded or concurrent pipelines, scheduler
  concurrency, fork/signals, process groups/sessions, live networking/SSH,
  Pi 5 hardware proof, generated-root retry, and phase transition remain
  outside this task.

## Evidence Map

- Closeout classification:
  tasks/evidence/2026-06-28-phase12-local-direct-combined-pipeline-stdin-stdout-redirection-closeout/classification.json.
- Closeout evidence map:
  tasks/evidence/2026-06-28-phase12-local-direct-combined-pipeline-stdin-stdout-redirection-closeout/evidence-map.json.
- Retained direct combined core task record:
  tasks/2026-06-28-phase12-local-direct-combined-pipeline-stdin-stdout-redirection-core.md.
- Retained direct combined core evidence:
  tasks/evidence/2026-06-28-phase12-local-direct-combined-pipeline-stdin-stdout-redirection-core/.
- Project docs:
  docs/src/roadmap.md, docs/src/project/phase12-networking-ssh.md, and
  docs/src/project/early-posix-shape.md.

## Reconciled Frontier

The local-only, static/unit/QEMU-substitute direct combined pipeline
stdin/stdout redirection frontier accepts exactly:

~~~text
/bin/stdin </etc/banner.txt | /bin/stdin >/tmp/pipeline-combined.txt
~~~

The producer is '/bin/stdin' with fd0 sourced from
initramfs:/etc/banner.txt, fd1 as the pipe endpoint, inherited fd2, and closed
loader temporary descriptors. The consumer is '/bin/stdin' with fd0 inherited
as the pipe endpoint, child-only fd1 targeting
volatile-vfs:/tmp/pipeline-combined.txt, inherited fd2, and closed loader
temporary descriptors. Retained evidence records descriptor-backed
'cat /tmp/pipeline-combined.txt' reading the nested userspace stdin report,
shell fd0/fd1 restoration, coherent waitpid/laststatus observations,
/proc/talos/processes entries, zero-argument ps output, and
pipestatus-compatible status accounting.

Unsupported direct forms remain fail-closed for bare-name combined pipeline
redirection, append, wrong output path, stdout producer with the combined
path, explicit '1>', separated redirection tokens, persistent '/var' target,
unsupported stage names, PATH/current-directory lookup, and persistent storage.

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
phase12-local-bare-name-combined-pipeline-stdin-stdout-redirection-core-20260628.

The fixed-/bin bare-name combined pipeline stdin/stdout core is mechanically
unblocked for the next worker wake if this closeout remains accepted and
committed, the hardware lock remains restored/unlocked, supervisor
intervention remains inactive, and the repo has no conflicting uncommitted
changes.
