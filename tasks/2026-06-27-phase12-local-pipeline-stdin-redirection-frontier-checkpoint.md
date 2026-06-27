# Phase 12 Local Pipeline Stdin Redirection Frontier Checkpoint

Task id: phase12-local-pipeline-stdin-redirection-frontier-checkpoint-20260627

Status: accepted; commit hash recorded in durable supervisor state after
commit.

## Scope

Reconcile the accepted direct path-form and fixed-/bin bare-name producer-stage
read-only stdin redirection frontier after these accepted two-stage pipeline
surfaces:

~~~text
/bin/stdin </etc/banner.txt | /bin/stdin
stdin </etc/banner.txt | stdin
~~~

This checkpoint is static reconciliation work. It does not change source
behavior, run Pi 5 hardware, resume live networking/SSH, retry generated-root
command input, accept consumer-stage redirection, accept output redirection,
accept broader shell grammar, or accept a phase transition.

## Findings

- fixed: The accepted pipeline-stage stdin redirection frontier is reconciled
  against the retained direct path-form and bare-name core/closeout records,
  task-owned classification/evidence JSON, QEMU/substitute transcripts, docs,
  and regression evidence.
- fixed: The direct path-form surface remains exactly
  /bin/stdin </etc/banner.txt | /bin/stdin through descriptor-backed VFS
  open/read, the accepted loader, userspace launch/status, producer-only fd0
  replacement from initramfs:/etc/banner.txt, producer fd1 pipe handoff,
  consumer fd0 pipe input, inherited fd2, closed loader temporary descriptors,
  explicit waitpid for both participants, laststatus, bounded process-table
  observations, /proc/talos/processes, zero-argument ps, and
  pipestatus-compatible state.
- fixed: The fixed-/bin bare-name surface remains exactly
  stdin </etc/banner.txt | stdin; both stages canonicalize through bounded
  /bin lookup to /bin/stdin before using the same VFS/open/read,
  loader/userspace, descriptor redirection, pipe handoff, process-table,
  procfs, ps, and pipestatus layers.
- fixed: Existing command stdin redirection, direct/bare pipeline argv, direct
  and bare command argv, process-status VFS, zero-argument ps, pipestatus, and
  cat-banner regression surfaces remain cited as retained controls.
- fixed: Unsupported direct and bare-name pipeline redirection variants remain
  fail-closed without additional successful process records.
- not-an-issue: No implementation change is required for this checkpoint; the
  direct and bare-name pipeline stdin redirection core tasks already accepted
  the source behavior and regression evidence.
- deferred: Consumer-stage redirection, redirection on multiple pipeline
  stages, multistage pipeline redirection, output regular-file redirection,
  append/truncate, writable filesystem behavior, combined redirections beyond
  the accepted exact forms, environment-backed PATH, current-directory search,
  command lookup beyond bounded /bin, quoting, escaping, globbing, variables,
  shell functions, arbitrary shell grammar, unbounded pipelines, pipeline
  concurrency, scheduler concurrency, fork/signals, process groups/sessions,
  persistent storage, live networking/SSH, Pi 5 hardware proof,
  generated-root retry, and phase transition.

## Evidence Map

- Direct path-form pipeline stdin redirection core and closeout:
  tasks/2026-06-27-phase12-local-direct-pipeline-stdin-redirection-core.md
  and
  tasks/2026-06-27-phase12-local-direct-pipeline-stdin-redirection-closeout.md.
- Bare-name pipeline stdin redirection core and closeout:
  tasks/2026-06-27-phase12-local-bare-name-pipeline-stdin-redirection-core.md
  and
  tasks/2026-06-27-phase12-local-bare-name-pipeline-stdin-redirection-closeout.md.
- Direct path-form pipeline stdin redirection classification/evidence/transcript:
  tasks/evidence/2026-06-27-phase12-local-direct-pipeline-stdin-redirection-core/classification.json,
  tasks/evidence/2026-06-27-phase12-local-direct-pipeline-stdin-redirection-core/evidence-map.json,
  tasks/evidence/2026-06-27-phase12-local-direct-pipeline-stdin-redirection-core/qemu-local-shell-direct-pipeline-stdin-redirection-smoke.log,
  tasks/evidence/2026-06-27-phase12-local-direct-pipeline-stdin-redirection-closeout/classification.json,
  and
  tasks/evidence/2026-06-27-phase12-local-direct-pipeline-stdin-redirection-closeout/evidence-map.json.
- Bare-name pipeline stdin redirection classification/evidence/transcript:
  tasks/evidence/2026-06-27-phase12-local-bare-name-pipeline-stdin-redirection-core/classification.json,
  tasks/evidence/2026-06-27-phase12-local-bare-name-pipeline-stdin-redirection-core/evidence-map.json,
  tasks/evidence/2026-06-27-phase12-local-bare-name-pipeline-stdin-redirection-core/qemu-local-shell-bare-name-pipeline-stdin-redirection-smoke.log,
  tasks/evidence/2026-06-27-phase12-local-bare-name-pipeline-stdin-redirection-closeout/classification.json,
  and
  tasks/evidence/2026-06-27-phase12-local-bare-name-pipeline-stdin-redirection-closeout/evidence-map.json.
- Retained regression records:
  tasks/2026-06-26-phase12-local-direct-stdin-redirection-core.md,
  tasks/2026-06-26-phase12-local-bare-name-stdin-redirection-core.md,
  tasks/2026-06-26-phase12-local-direct-pipeline-stage-argv-core.md,
  tasks/2026-06-26-phase12-local-bare-name-pipeline-stage-argv-core.md,
  tasks/2026-06-26-phase12-local-command-argv-frontier-checkpoint.md,
  tasks/2026-06-26-phase12-local-process-status-vfs-core.md,
  tasks/2026-06-26-phase12-local-ps-command-vfs-backed-core.md,
  tasks/2026-06-26-phase12-local-pipefail-status-core.md, and
  tasks/2026-06-02-qemu-local-cat-banner-core.md.
- Checkpoint classification and evidence map:
  tasks/evidence/2026-06-27-phase12-local-pipeline-stdin-redirection-frontier-checkpoint/classification.json
  and
  tasks/evidence/2026-06-27-phase12-local-pipeline-stdin-redirection-frontier-checkpoint/evidence-map.json.
- Project notes:
  docs/src/roadmap.md, docs/src/project/phase12-networking-ssh.md, and
  docs/src/project/early-posix-shape.md.

## Accepted Frontier

The accepted pipeline-stage stdin redirection frontier is local-only and
static/unit/QEMU-substitute backed. Direct path-form and fixed-/bin bare-name
two-stage pipelines can redirect only the producer fd0 from one read-only
initramfs regular file:

~~~text
/bin/stdin </etc/banner.txt | /bin/stdin
stdin </etc/banner.txt | stdin
~~~

Both forms execute through descriptor-backed VFS open/read, the accepted
loader, userspace launch/status, and producer-only fd0 replacement from
initramfs:/etc/banner.txt. The direct path-form surface uses /bin/stdin for
both stages. The bare-name surface canonicalizes both stage names through
fixed bounded /bin lookup to /bin/stdin before launch. In both forms, the
producer fd1 is the pipe endpoint, the consumer fd0 is that pipe endpoint,
fd2 remains inherited, loader temporary descriptors are closed, the shell
restores fd0 after the pipeline, userspace stdin reads the redirected file and
pipe to EOF, and waitpid, laststatus, /proc/talos/processes, zero-argument ps,
and pipestatus-compatible observations remain intact.

No later queued same-lane local POSIX/shell task exists with complete
objective dependencies, acceptance criteria, validation gates, docs, and
evidence requirements, so this checkpoint records planningNeeded=true rather
than selecting a new worker task.

Live network/SSH reachability remains paused. No Pi 5 hardware claim is made.

## Deferred Frontier

Deferred surfaces remain consumer-stage redirection, redirection on multiple
pipeline stages, multistage pipeline redirection, output regular-file
redirection, append/truncate, writable filesystem behavior, combined
redirections beyond the accepted exact forms, environment-backed PATH,
current-directory search, command lookup beyond bounded /bin, quoting,
escaping, globbing, variables, shell functions, arbitrary shell grammar,
unbounded pipelines, pipeline concurrency, scheduler concurrency,
fork/signals, process groups/sessions, broad procfs/Linux ps, PID policy
expansion, waitpid options, persistent storage, live networking/SSH, Pi 5
hardware proof, generated-root command-input retry, and phase transition.

## Validation

- Static inspection of task records, retained evidence paths, docs, and git
  diff: passed.
- jq empty on task-owned JSON evidence: passed.
- git diff --check: passed.
- /home/node/.cargo/bin/mdbook build: passed; html backend written with
  existing large search-index warning.
- git diff --cached --check: passed before commit.

## Result

selected_next_task: null.

planningNeeded: true.

planningReason: No later queued same-lane local POSIX/shell task exists with
complete objective dependencies, acceptance criteria, validation gates, docs,
and evidence requirements after the accepted pipeline-stage stdin redirection
frontier.
