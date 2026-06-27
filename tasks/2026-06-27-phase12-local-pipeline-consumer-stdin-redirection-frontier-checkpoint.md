# Phase 12 Local Pipeline Consumer Stdin Redirection Frontier Checkpoint

Task id: phase12-local-pipeline-consumer-stdin-redirection-frontier-checkpoint-20260627

Status: accepted; commit hash recorded in durable supervisor state after
commit.

## Scope

Reconcile the accepted direct path-form and fixed-/bin bare-name
consumer-stage read-only stdin redirection frontier after these accepted
two-stage pipeline surfaces:

~~~text
/bin/stdin | /bin/stdin </etc/banner.txt
stdin | stdin </etc/banner.txt
~~~

This checkpoint is static reconciliation work. It does not change source
behavior, run Pi 5 hardware, resume live networking/SSH, retry generated-root
command input, accept output redirection, accept broader shell grammar, or
accept a phase transition.

## Findings

- fixed: The accepted consumer-stage pipeline stdin redirection frontier is
  reconciled against the retained direct path-form and bare-name
  core/closeout records, task-owned classification/evidence JSON,
  QEMU/substitute transcripts, docs, and regression evidence.
- fixed: The direct path-form surface remains exactly
  /bin/stdin | /bin/stdin </etc/banner.txt through descriptor-backed VFS
  open/read, the accepted loader, userspace launch/status, producer fd1 pipe
  handoff, consumer-only fd0 replacement from initramfs:/etc/banner.txt,
  inherited fd2, closed loader temporary descriptors, explicit waitpid for
  both participants, laststatus, bounded process-table observations,
  /proc/talos/processes, zero-argument ps, and pipestatus-compatible state.
- fixed: The fixed-/bin bare-name surface remains exactly
  stdin | stdin </etc/banner.txt; both stages canonicalize through bounded
  /bin lookup to /bin/stdin before using the same VFS/open/read,
  loader/userspace, child-only descriptor redirection, pipe handoff,
  process-table, procfs, ps, and pipestatus layers.
- fixed: Existing direct and bare-name producer-stage pipeline stdin
  redirection, direct and bare-name command stdin redirection, command argv,
  pipeline argv, process-status VFS, zero-argument ps, pipestatus, and
  cat-banner regression surfaces remain cited as retained controls.
- fixed: Unsupported direct and bare-name consumer-stage pipeline redirection
  variants remain fail-closed without additional successful process records.
- not-an-issue: No implementation change is required for this checkpoint; the
  direct and bare-name consumer-stage pipeline stdin redirection core tasks
  already accepted the source behavior and regression evidence.
- deferred: Redirection on multiple pipeline stages, multistage pipeline
  redirection, output regular-file redirection, append/truncate, writable
  filesystem behavior, combined redirections beyond the accepted exact forms,
  environment-backed PATH, current-directory search, command lookup beyond
  bounded /bin, quoting, escaping, globbing, variables, shell functions,
  arbitrary shell grammar, unbounded pipelines, pipeline concurrency,
  scheduler concurrency, fork/signals, process groups/sessions, persistent
  storage, live networking/SSH, Pi 5 hardware proof, generated-root retry, and
  phase transition.

## Evidence Map

- Direct path-form consumer-stage pipeline stdin redirection core and
  closeout:
  tasks/2026-06-27-phase12-local-direct-pipeline-consumer-stdin-redirection-core.md
  and
  tasks/2026-06-27-phase12-local-direct-pipeline-consumer-stdin-redirection-closeout.md.
- Bare-name consumer-stage pipeline stdin redirection core and closeout:
  tasks/2026-06-27-phase12-local-bare-name-pipeline-consumer-stdin-redirection-core.md
  and
  tasks/2026-06-27-phase12-local-bare-name-pipeline-consumer-stdin-redirection-closeout.md.
- Direct path-form consumer-stage evidence:
  tasks/evidence/2026-06-27-phase12-local-direct-pipeline-consumer-stdin-redirection-core/
  and
  tasks/evidence/2026-06-27-phase12-local-direct-pipeline-consumer-stdin-redirection-closeout/.
- Bare-name consumer-stage evidence:
  tasks/evidence/2026-06-27-phase12-local-bare-name-pipeline-consumer-stdin-redirection-core/
  and
  tasks/evidence/2026-06-27-phase12-local-bare-name-pipeline-consumer-stdin-redirection-closeout/.
- Retained regression records include the accepted producer-stage pipeline
  stdin redirection checkpoint, direct/bare command stdin redirection, direct
  and bare command argv, direct and bare pipeline argv, process-status VFS,
  zero-argument ps, pipestatus, and cat-banner task records.
- Checkpoint classification and evidence map:
  tasks/evidence/2026-06-27-phase12-local-pipeline-consumer-stdin-redirection-frontier-checkpoint/classification.json
  and
  tasks/evidence/2026-06-27-phase12-local-pipeline-consumer-stdin-redirection-frontier-checkpoint/evidence-map.json.
- Project notes:
  docs/src/roadmap.md, docs/src/project/phase12-networking-ssh.md, and
  docs/src/project/early-posix-shape.md.

## Accepted Frontier

The accepted consumer-stage pipeline stdin redirection frontier is local-only
and static/unit/QEMU-substitute backed. Direct path-form and fixed-/bin
bare-name two-stage pipelines can redirect only the consumer fd0 from one
read-only initramfs regular file:

~~~text
/bin/stdin | /bin/stdin </etc/banner.txt
stdin | stdin </etc/banner.txt
~~~

Both forms execute through descriptor-backed VFS open/read, the accepted
loader, userspace launch/status, producer fd1 pipe handoff, and consumer-only
fd0 replacement from initramfs:/etc/banner.txt. The direct path-form surface
uses /bin/stdin for both stages. The bare-name surface canonicalizes both
stage names through fixed bounded /bin lookup to /bin/stdin before launch. In
both forms, the producer fd0/fd2 remain inherited, the producer fd1 is the
pipe endpoint, the consumer fd0 is the redirected initramfs file, the consumer
fd1/fd2 remain inherited, loader temporary descriptors are closed, the shell
restores fd0 after the pipeline, userspace stdin reads the redirected file to
EOF, and waitpid, laststatus, /proc/talos/processes, zero-argument ps, and
pipestatus-compatible observations remain intact.

No later queued same-lane local POSIX/shell task exists with complete
objective dependencies, acceptance criteria, validation gates, docs, and
evidence requirements, so this checkpoint records planningNeeded=true rather
than selecting a new worker task.

Live network/SSH reachability remains paused. No Pi 5 hardware claim is made.

## Deferred Frontier

Deferred surfaces remain redirection on multiple pipeline stages, multistage
pipeline redirection, output regular-file redirection, append/truncate,
writable filesystem behavior, combined redirections beyond the accepted exact
forms, environment-backed PATH, current-directory search, command lookup
beyond bounded /bin, quoting, escaping, globbing, variables, shell functions,
arbitrary shell grammar, unbounded pipelines, pipeline concurrency, scheduler
concurrency, fork/signals, process groups/sessions, broad procfs/Linux ps, PID
policy expansion, waitpid options, persistent storage, live networking/SSH, Pi
5 hardware proof, generated-root command-input retry, and phase transition.

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
and evidence requirements after the accepted consumer-stage pipeline stdin
redirection frontier.
