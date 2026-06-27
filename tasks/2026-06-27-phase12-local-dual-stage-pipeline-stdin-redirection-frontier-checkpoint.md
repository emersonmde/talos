# Phase 12 Local Dual-Stage Pipeline Stdin Redirection Frontier Checkpoint

Task id: phase12-local-dual-stage-pipeline-stdin-redirection-frontier-checkpoint-20260627

Status: accepted; commit hash recorded in durable supervisor state after
commit.

## Scope

Checkpoint the accepted or blocked dual-stage two-stage pipeline stdin
redirection frontier after the core and closeout tasks.

The reconciled accepted witnesses remain exactly:

~~~text
/bin/stdin </etc/banner.txt | /bin/stdin </etc/banner.txt
stdin </etc/banner.txt | stdin </etc/banner.txt
~~~

Both stages load through descriptor-backed VFS open/read and the accepted
loader/userspace launch/status path. Each child fd0 is independently replaced
with initramfs:/etc/banner.txt. The producer keeps fd1 as the pipe endpoint
and writes the redirected banner bytes to the pipe surface; the consumer reads
its own redirected regular-file fd0 to EOF. Loader temporary descriptors,
shell descriptor restoration, explicit waitpid observations, laststatus,
/proc/talos/processes, zero-argument ps, and pipestatus-compatible
observations remain coherent.

This checkpoint does not accept output regular-file redirection,
append/truncate, writable filesystem behavior, combined redirections beyond
the accepted exact forms, environment-backed PATH, current-directory search,
command lookup beyond bounded /bin, arbitrary shell grammar, unbounded
pipelines, pipeline concurrency, scheduler concurrency, fork/signals, process
groups/sessions, persistent storage, live networking/SSH, Pi 5 hardware proof,
generated-root retry, or a phase transition.

## Findings

- fixed: The dual-stage two-stage pipeline stdin redirection frontier is
  reconciled against the retained core task, closeout task, classification
  JSON, evidence maps, QEMU/substitute transcripts, regression transcripts,
  roadmap, Phase 12 note, and early POSIX shape note.
- fixed: The accepted frontier remains exactly the direct path-form witness
  '/bin/stdin </etc/banner.txt | /bin/stdin </etc/banner.txt' and the
  fixed-/bin bare-name witness
  'stdin </etc/banner.txt | stdin </etc/banner.txt'.
- fixed: The retained evidence still bounds both children to independent
  initramfs:/etc/banner.txt fd0 replacement, producer fd1 pipe-endpoint
  setup, closed loader temporary descriptors, restored shell descriptors, and
  coherent waitpid, laststatus, process-table, procfs, ps, and
  pipestatus-compatible observations.
- fixed: The deferred frontier is explicit: multistage pipeline redirection,
  output regular-file redirection, append/truncate, writable filesystem
  behavior, combined redirections beyond the accepted exact forms,
  environment-backed PATH, current-directory search, command lookup beyond
  bounded /bin, arbitrary shell grammar, unbounded pipelines, pipeline
  concurrency, scheduler concurrency, fork/signals, process groups/sessions,
  persistent storage, live networking/SSH, Pi 5 hardware proof,
  generated-root retry, and phase transition remain outside this checkpoint.
- not-an-issue: No code change is required for this checkpoint; the accepted
  core already recorded fmt/lint/typecheck, unit, QEMU/substitute, regression,
  JSON, diff, docs, and staged-diff evidence.
- deferred: No next local POSIX/shell task is selected because no later queued
  same-lane task exists with explicit dependencies, acceptance criteria,
  validation gates, docs requirements, and evidence requirements. Supervisor
  planning is required before the worker can promote more work.

## Evidence Map

- Checkpoint classification:
  tasks/evidence/2026-06-27-phase12-local-dual-stage-pipeline-stdin-redirection-frontier-checkpoint/classification.json.
- Checkpoint evidence map:
  tasks/evidence/2026-06-27-phase12-local-dual-stage-pipeline-stdin-redirection-frontier-checkpoint/evidence-map.json.
- Retained core task record:
  tasks/2026-06-27-phase12-local-dual-stage-pipeline-stdin-redirection-core.md.
- Retained core evidence:
  tasks/evidence/2026-06-27-phase12-local-dual-stage-pipeline-stdin-redirection-core/.
- Retained closeout task record:
  tasks/2026-06-27-phase12-local-dual-stage-pipeline-stdin-redirection-closeout.md.
- Retained closeout evidence:
  tasks/evidence/2026-06-27-phase12-local-dual-stage-pipeline-stdin-redirection-closeout/.
- Project docs:
  docs/src/roadmap.md, docs/src/project/phase12-networking-ssh.md, and
  docs/src/project/early-posix-shape.md.

## Reconciled Frontier

The local-only, static/unit/QEMU-substitute frontier now covers exactly
two-stage dual stdin redirection for direct path-form and fixed-/bin bare-name
pipeline commands:

~~~text
/bin/stdin </etc/banner.txt | /bin/stdin </etc/banner.txt
stdin </etc/banner.txt | stdin </etc/banner.txt
~~~

Each child receives its own read-only initramfs regular-file descriptor as
fd0. The direct path-form surface names /bin/stdin for both stages. The
bare-name surface canonicalizes both stages through fixed bounded /bin lookup
to /bin/stdin before launch. The producer fd1 remains the pipe endpoint, the
consumer owns an independent redirected fd0, loader temporary descriptors are
closed, shell descriptors restore after the pipeline, userspace stdin reads
through descriptor-backed VFS/open/read, and waitpid, laststatus,
/proc/talos/processes, zero-argument ps, and pipestatus-compatible
observations remain intact.

Mixed direct/bare dual-stage forms remain fail-closed without additional
successful process records. Live network/SSH remains paused. No Pi 5 hardware
claim is made.

## Validation

- Static inspection of task records, retained evidence paths, docs, and git
  diff: passed.
- jq empty on task-owned JSON evidence: passed.
- git diff --check: passed.
- /home/node/.cargo/bin/mdbook build: passed.
- git diff --cached --check: passed before commit.

## Result

planningNeeded: true.

planningReason: No later queued same-lane local POSIX/shell task exists after
phase12-local-dual-stage-pipeline-stdin-redirection-frontier-checkpoint-20260627
with complete objective dependencies, acceptance criteria, validation gates,
docs requirements, and evidence requirements. Supervisor planning is required
before the worker can promote more work.
