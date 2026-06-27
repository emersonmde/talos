# Phase 12 Local Dual-Stage Pipeline Stdin Redirection Closeout

Task id: phase12-local-dual-stage-pipeline-stdin-redirection-closeout-20260627

Status: accepted; commit hash recorded in durable supervisor state after
commit.

## Scope

Reconcile the accepted dual-stage pipeline stdin redirection frontier against
the retained task record, task-owned evidence, and project docs.

The accepted witnesses remain exactly:

~~~text
/bin/stdin </etc/banner.txt | /bin/stdin </etc/banner.txt
stdin </etc/banner.txt | stdin </etc/banner.txt
~~~

Both stages load through descriptor-backed VFS open/read and the accepted
loader/userspace launch/status path. Each child fd0 is independently replaced
with initramfs:/etc/banner.txt. The producer keeps fd1 as the pipe endpoint
and writes the redirected banner bytes to the pipe surface; the consumer reads
its own redirected regular-file fd0 to EOF. Loader temporary descriptors,
shell fd restoration, explicit waitpid observations, laststatus,
/proc/talos/processes, zero-argument ps, and pipestatus-compatible
observations remain coherent.

This closeout does not accept output redirection, append/truncate, writable
filesystem behavior, multistage pipeline redirection, arbitrary redirection
placement, combined input/output redirection, environment-backed PATH,
current-directory search, command lookup beyond bounded /bin, broad shell
grammar, live networking/SSH, Pi 5 hardware proof, generated-root retry, or a
phase transition.

## Findings

- fixed: The accepted dual-stage pipeline stdin redirection frontier is
  reconciled to the retained core task record, classification JSON, evidence
  map, QEMU/substitute transcripts, regression transcripts, and project docs.
- fixed: The exact accepted witnesses remain
  '/bin/stdin </etc/banner.txt | /bin/stdin </etc/banner.txt' and
  'stdin </etc/banner.txt | stdin </etc/banner.txt'; no additional shell
  grammar, output redirection, writable filesystem behavior, or broader
  pipeline shape is accepted by this closeout.
- fixed: The selected next task is
  phase12-local-dual-stage-pipeline-stdin-redirection-frontier-checkpoint-20260627
  because its dependencies are objective: the dual-stage core is accepted and
  committed, this closeout reconciles the accepted evidence, supervisor
  intervention is inactive, and the hardware lock is restored/unlocked.
- not-an-issue: No code change is required for this closeout; the core task
  already recorded static inspection, fmt/lint/typecheck, unit test,
  QEMU/substitute, JSON, diff, docs, and staged-diff evidence.
- deferred: Multistage pipeline redirection, output regular-file redirection,
  append/truncate, writable filesystem behavior, combined redirections beyond
  the accepted exact forms, environment-backed PATH, current-directory search,
  command lookup beyond bounded /bin, quoting, escaping, globbing, variables,
  shell functions, arbitrary shell grammar, unbounded pipelines, pipeline
  concurrency, scheduler concurrency, fork/signals, process groups/sessions,
  persistent storage, live networking/SSH, Pi 5 hardware proof,
  generated-root retry, and phase transition.

## Evidence Map

- Closeout classification:
  tasks/evidence/2026-06-27-phase12-local-dual-stage-pipeline-stdin-redirection-closeout/classification.json.
- Closeout evidence map:
  tasks/evidence/2026-06-27-phase12-local-dual-stage-pipeline-stdin-redirection-closeout/evidence-map.json.
- Retained core task record:
  tasks/2026-06-27-phase12-local-dual-stage-pipeline-stdin-redirection-core.md.
- Retained core evidence:
  tasks/evidence/2026-06-27-phase12-local-dual-stage-pipeline-stdin-redirection-core/.
- Project docs:
  docs/src/roadmap.md, docs/src/project/phase12-networking-ssh.md, and
  docs/src/project/early-posix-shape.md.

## Reconciled Frontier

The local-only, static/unit/QEMU-substitute dual-stage pipeline stdin
redirection frontier accepts exactly the direct path-form and fixed-/bin
bare-name two-stage witnesses where both stages independently receive the same
read-only initramfs regular file as fd0:

~~~text
/bin/stdin </etc/banner.txt | /bin/stdin </etc/banner.txt
stdin </etc/banner.txt | stdin </etc/banner.txt
~~~

The direct path-form surface uses /bin/stdin for both stages. The bare-name
surface canonicalizes both stage names through fixed bounded /bin lookup to
/bin/stdin before launch. In both forms, each child fd0 is an independent
initramfs:/etc/banner.txt regular-file descriptor, the producer fd1 is the
pipe endpoint, the consumer fd1/fd2 remain inherited, loader temporary
descriptors are closed, the shell restores fd0 after the pipeline, userspace
stdin reads through the accepted descriptor-backed VFS/open/read and loader
path, and waitpid, laststatus, /proc/talos/processes, zero-argument ps, and
pipestatus-compatible observations remain intact.

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
phase12-local-dual-stage-pipeline-stdin-redirection-frontier-checkpoint-20260627.

The dual-stage pipeline stdin redirection frontier checkpoint is mechanically
unblocked for the next worker wake if this closeout remains accepted and
committed, the hardware lock remains restored/unlocked, supervisor
intervention remains inactive, and the repo has no conflicting uncommitted
changes.
