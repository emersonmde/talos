# Phase 12 Local Bare-Name Combined Pipeline Stdin Stdout Redirection Closeout

Task id: phase12-local-bare-name-combined-pipeline-stdin-stdout-redirection-closeout-20260628

Status: accepted; commit hash recorded in durable supervisor state after
commit.

## Scope

Reconcile the accepted direct path-form and fixed-/bin bare-name combined
pipeline stdin/stdout redirection frontier against retained local
POSIX/VFS/userspace evidence and project docs. No runtime behavior is added by
this closeout.

The accepted witnesses are exactly:

~~~text
/bin/stdin </etc/banner.txt | /bin/stdin >/tmp/pipeline-combined.txt
stdin </etc/banner.txt | stdin >/tmp/pipeline-combined.txt
~~~

The direct form loads both stages by path. The bare-name form resolves both
stage names through the bounded /bin lookup to /bin/stdin before using the same
descriptor-backed VFS open/read, accepted program loader, userspace
launch/status, pipe handoff, final-stage stdout redirection, volatile VFS
readback, and process-status observations.

For both witnesses, the producer gets child-only fd0 from
initramfs:/etc/banner.txt, fd1 as the pipe endpoint, and inherited fd2. The
consumer gets fd0 from that pipe endpoint, child-only fd1 to
volatile-vfs:/tmp/pipeline-combined.txt, and inherited fd2. Descriptor-backed
'cat /tmp/pipeline-combined.txt' reads the nested userspace stdin report back,
and waitpid, laststatus, /proc/talos/processes, zero-argument ps, and
pipestatus-compatible observations remain coherent.

This closeout does not accept append/combined stderr pipeline redirections,
arbitrary input/output paths, separated redirection tokens, explicit fd1
syntax, path-containing bare-name stage names, multistage combined
redirection, persistent writable filesystem behavior, environment-backed PATH,
current-directory search, command lookup beyond bounded /bin, arbitrary shell
grammar, unbounded/concurrent pipelines, scheduler concurrency, fork/signals,
process groups/sessions, generated-root retry, live networking/SSH, Pi 5
hardware action, or phase transition.

## Findings

- fixed: Reconciled the accepted direct and fixed-/bin bare-name combined
  pipeline stdin/stdout boundary against retained core task records,
  classification JSON, evidence maps, focused QEMU/substitute transcripts, and
  project docs.
- fixed: Froze the accepted direct witness as exactly
  '/bin/stdin </etc/banner.txt | /bin/stdin >/tmp/pipeline-combined.txt' and
  the accepted bare-name witness as exactly
  'stdin </etc/banner.txt | stdin >/tmp/pipeline-combined.txt'.
- fixed: Recorded that both accepted forms use descriptor-backed VFS/userspace
  execution: producer fd0 from initramfs:/etc/banner.txt, producer fd1 to the
  pipe endpoint, consumer fd0 from the pipe endpoint, consumer fd1 to
  volatile-vfs:/tmp/pipeline-combined.txt, inherited fd2 for both stages,
  closed loader temporary descriptors, descriptor-backed readback, shell fd0/fd1
  restoration, and coherent process-status observations.
- fixed: Selected
  phase12-local-combined-pipeline-stdin-stdout-redirection-frontier-checkpoint-20260628
  as the next task because the bare-name core is accepted and committed,
  supervisor intervention is inactive, the hardware lock is restored/unlocked,
  and the queued checkpoint has explicit scope, gates, evidence, and non-goals.
- not-an-issue: No runtime code change is required for this closeout; the core
  tasks already recorded static inspection, fmt/lint/typecheck, unit test,
  QEMU/substitute, retained regression/control, JSON, diff, docs, and
  staged-diff evidence.
- deferred: Append/combined stderr pipeline redirections, arbitrary paths,
  persistent writable filesystem behavior, environment-backed PATH,
  current-directory search, command lookup beyond bounded /bin, arbitrary shell
  grammar, unbounded/concurrent pipelines, scheduler concurrency,
  fork/signals, process groups/sessions, live networking/SSH, Pi 5 hardware
  proof, generated-root retry, and phase transition remain deferred.

## Evidence Map

- Closeout classification:
  tasks/evidence/2026-06-28-phase12-local-bare-name-combined-pipeline-stdin-stdout-redirection-closeout/classification.json.
- Closeout evidence map:
  tasks/evidence/2026-06-28-phase12-local-bare-name-combined-pipeline-stdin-stdout-redirection-closeout/evidence-map.json.
- Retained direct combined core task record:
  tasks/2026-06-28-phase12-local-direct-combined-pipeline-stdin-stdout-redirection-core.md.
- Retained direct combined closeout task record:
  tasks/2026-06-28-phase12-local-direct-combined-pipeline-stdin-stdout-redirection-closeout.md.
- Retained direct combined core evidence:
  tasks/evidence/2026-06-28-phase12-local-direct-combined-pipeline-stdin-stdout-redirection-core/.
- Retained bare-name combined core task record:
  tasks/2026-06-28-phase12-local-bare-name-combined-pipeline-stdin-stdout-redirection-core.md.
- Retained bare-name combined core evidence:
  tasks/evidence/2026-06-28-phase12-local-bare-name-combined-pipeline-stdin-stdout-redirection-core/.
- Project docs:
  docs/src/roadmap.md, docs/src/project/phase12-networking-ssh.md, and
  docs/src/project/early-posix-shape.md.

## Reconciled Frontier

The local-only, static/unit/QEMU-substitute combined pipeline stdin/stdout
redirection frontier accepts exactly the direct path-form witness:

~~~text
/bin/stdin </etc/banner.txt | /bin/stdin >/tmp/pipeline-combined.txt
~~~

and exactly the fixed-/bin bare-name witness:

~~~text
stdin </etc/banner.txt | stdin >/tmp/pipeline-combined.txt
~~~

Both forms share the accepted descriptor contract after command resolution:
producer fd0 is a child-only regular file from initramfs:/etc/banner.txt,
producer fd1 is the pipe endpoint, consumer fd0 is that pipe endpoint,
consumer fd1 is a child-only regular file targeting
volatile-vfs:/tmp/pipeline-combined.txt, fd2 is inherited for both stages, and
loader temporary descriptors are closed. Descriptor-backed
'cat /tmp/pipeline-combined.txt' reads the nested userspace stdin report back.
Retained evidence records shell fd0/fd1 restoration, coherent waitpid,
laststatus, /proc/talos/processes entries, zero-argument ps output, and
pipestatus-compatible status accounting.

Unsupported neighboring forms remain fail-closed for append to the combined
path, wrong output path, stdout producer with the combined sink, explicit
'1>', separated redirection tokens, persistent '/var' target,
path-containing bare-name stage names, unsupported stage names, multistage
combined redirection, PATH/current-directory lookup, and persistent storage
claims.

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
phase12-local-combined-pipeline-stdin-stdout-redirection-frontier-checkpoint-20260628.

The combined pipeline stdin/stdout redirection frontier checkpoint is
mechanically unblocked for the next worker wake if this closeout remains
accepted and committed, the hardware lock remains restored/unlocked,
supervisor intervention remains inactive, and the repo has no conflicting
uncommitted changes.
