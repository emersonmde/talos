# Phase 12 Local Bare-Name Pipeline Consumer Stdin Redirection Closeout

Task id: phase12-local-bare-name-pipeline-consumer-stdin-redirection-closeout-20260627

Status: accepted; commit hash recorded in durable supervisor state after
commit.

## Scope

Reconcile the accepted fixed-/bin bare-name pipeline consumer stdin
redirection frontier against the retained task record, task-owned evidence,
and project docs.

The accepted witness remains exactly:

~~~text
stdin | stdin </etc/banner.txt
~~~

Both stage names canonicalize through fixed bounded /bin lookup to
/bin/stdin, then load from descriptor-backed VFS through the accepted loader
and userspace launch/status path. The producer keeps inherited fd0 and fd2
while fd1 is the pipe endpoint. The consumer starts from the accepted
pipeline fd0 handoff, then replaces only the child fd0 with
initramfs:/etc/banner.txt before launch. Shell fd0 restoration, loader
temporary descriptor non-leak, explicit waitpid observations, laststatus,
/proc/talos/processes, zero-argument ps, and pipestatus-compatible
observations remain coherent.

This closeout does not accept redirection on multiple pipeline stages,
multistage pipeline redirection, output redirection, append/truncate, writable
filesystem behavior, environment-backed PATH, current-directory search,
command lookup beyond bounded /bin, broad shell grammar, live networking/SSH,
Pi 5 hardware proof, generated-root retry, or a phase transition.

## Findings

- fixed: The accepted bare-name consumer-stage frontier is reconciled to the
  retained core task record, classification JSON, evidence map,
  QEMU/substitute transcript, regression transcripts, and project docs.
- fixed: The exact accepted witness remains
  'stdin | stdin </etc/banner.txt'; no additional shell grammar or broader
  pipeline redirection shape is accepted by this closeout.
- fixed: The selected next task is
  phase12-local-pipeline-consumer-stdin-redirection-frontier-checkpoint-20260627
  because its dependencies are objective: the bare-name consumer-stage core is
  accepted and committed, this closeout reconciles the accepted evidence,
  supervisor intervention is inactive, and the hardware lock is
  restored/unlocked.
- not-an-issue: No code change is required for this closeout; the core task
  already recorded static inspection, fmt/lint/typecheck, unit test,
  QEMU/substitute, JSON, diff, docs, and staged-diff evidence.
- deferred: Redirection on multiple pipeline stages, multistage pipeline
  redirection, combined input/output redirection, output regular-file
  redirection, append/truncate, writable filesystem behavior,
  environment-backed PATH, current-directory search, command lookup beyond
  bounded /bin, arbitrary shell grammar, unbounded pipelines, pipeline
  concurrency, scheduler concurrency, fork/signals, process groups/sessions,
  persistent storage, live networking/SSH, Pi 5 hardware proof,
  generated-root retry, and phase transition.

## Evidence Map

- Closeout classification:
  tasks/evidence/2026-06-27-phase12-local-bare-name-pipeline-consumer-stdin-redirection-closeout/classification.json.
- Closeout evidence map:
  tasks/evidence/2026-06-27-phase12-local-bare-name-pipeline-consumer-stdin-redirection-closeout/evidence-map.json.
- Retained core task record:
  tasks/2026-06-27-phase12-local-bare-name-pipeline-consumer-stdin-redirection-core.md.
- Retained core evidence:
  tasks/evidence/2026-06-27-phase12-local-bare-name-pipeline-consumer-stdin-redirection-core/.
- Project docs:
  docs/src/roadmap.md, docs/src/project/phase12-networking-ssh.md, and
  docs/src/project/early-posix-shape.md.

## Reconciled Frontier

The local-only, static/unit/QEMU-substitute bare-name consumer-stage frontier
accepts exactly the fixed-/bin two-stage pipeline consumer stdin redirection
shape:

~~~text
stdin | stdin </etc/banner.txt
~~~

The producer resolves to /bin/stdin through fixed bounded /bin lookup and
keeps fd1 connected to the pipe endpoint. The consumer also resolves to
/bin/stdin through the same bounded lookup, replaces only child fd0 with
initramfs:/etc/banner.txt, inherits fd1/fd2, loads through descriptor-backed
VFS, closes the loader temporary descriptor, reads the banner file to EOF, and
exits successfully. The shell restores fd0, records explicit waitpid
observations for both participants, and keeps laststatus,
/proc/talos/processes, zero-argument ps, and pipestatus coherent.

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
phase12-local-pipeline-consumer-stdin-redirection-frontier-checkpoint-20260627.

The consumer-stage pipeline stdin redirection frontier checkpoint is
mechanically unblocked for the next worker wake if the direct and bare-name
consumer-stage closeouts remain accepted and committed, the hardware lock
remains restored/unlocked, supervisor intervention remains inactive, and the
repo has no conflicting uncommitted changes.
