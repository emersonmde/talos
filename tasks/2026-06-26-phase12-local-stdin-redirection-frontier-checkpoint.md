# Phase 12 Local Stdin Redirection Frontier Checkpoint

Task id: phase12-local-stdin-redirection-frontier-checkpoint-20260626

Status: accepted; commit hash recorded in durable supervisor state after
commit.

## Scope

Reconcile the accepted direct path-form and bare-name read-only stdin
redirection frontier after these accepted surfaces:

~~~text
/bin/stdin </etc/banner.txt
stdin </etc/banner.txt
~~~

This checkpoint is static reconciliation work. It does not change source
behavior, run Pi 5 hardware, resume live networking/SSH, retry generated-root
command input, accept broader shell grammar, accept output or pipeline-stage
redirection, or accept a phase transition.

## Findings

- fixed: The accepted stdin redirection frontier is reconciled against the
  retained direct path-form and bare-name core/closeout records, task-owned
  classification/evidence JSON, QEMU/substitute transcripts, docs, and
  regression evidence.
- fixed: The direct path-form surface remains exactly
  /bin/stdin </etc/banner.txt through descriptor-backed VFS open/read, the
  accepted loader, userspace startup/status, child-only fd0 replacement from
  initramfs:/etc/banner.txt, shell fd0 restoration, inherited fd1/fd2, a
  closed loader temporary descriptor, waitpid, laststatus, bounded
  process-table observations, /proc/talos/processes, zero-argument ps, and
  pipestatus-compatible state.
- fixed: The bare-name surface remains exactly stdin </etc/banner.txt; stdin
  resolves only through fixed bounded /bin lookup to /bin/stdin before using
  the same VFS/open/read, loader, userspace startup/status, child-only fd0
  redirection, descriptor restoration, process-table, procfs, ps, and
  pipestatus layers.
- fixed: Existing exec-prefixed stdin redirection, direct/bare command argv,
  direct/bare pipeline argv, process-status VFS, zero-argument ps,
  pipestatus, and cat-banner regression surfaces remain cited as retained
  controls.
- fixed: Unsupported direct and bare-name redirection variants remain
  fail-closed without accepted process records.
- not-an-issue: No implementation change is required for this checkpoint; the
  direct and bare-name stdin redirection core tasks already accepted the
  source behavior and regression evidence.
- deferred: Output regular-file redirection, append/truncate, writable
  filesystem behavior, pipeline-stage redirection, combined redirections
  beyond the accepted exact forms, environment-backed PATH, current-directory
  search, command lookup beyond bounded /bin, quoting, escaping, globbing,
  variables, shell functions, arbitrary shell grammar, unbounded pipelines,
  pipeline concurrency, scheduler concurrency, fork/signals, process
  groups/sessions, persistent storage, live networking/SSH, Pi 5 hardware
  proof, generated-root retry, and phase transition.

## Evidence Map

- Direct path-form stdin redirection core and closeout:
  tasks/2026-06-26-phase12-local-direct-stdin-redirection-core.md and
  tasks/2026-06-26-phase12-local-direct-stdin-redirection-closeout.md.
- Bare-name stdin redirection core and closeout:
  tasks/2026-06-26-phase12-local-bare-name-stdin-redirection-core.md and
  tasks/2026-06-26-phase12-local-bare-name-stdin-redirection-closeout.md.
- Direct path-form stdin redirection classification/evidence/transcript:
  tasks/evidence/2026-06-26-phase12-local-direct-stdin-redirection-core/classification.json,
  tasks/evidence/2026-06-26-phase12-local-direct-stdin-redirection-core/evidence-map.json,
  tasks/evidence/2026-06-26-phase12-local-direct-stdin-redirection-core/qemu-local-shell-direct-stdin-redirection-smoke.log,
  tasks/evidence/2026-06-26-phase12-local-direct-stdin-redirection-closeout/classification.json,
  and
  tasks/evidence/2026-06-26-phase12-local-direct-stdin-redirection-closeout/evidence-map.json.
- Bare-name stdin redirection classification/evidence/transcript:
  tasks/evidence/2026-06-26-phase12-local-bare-name-stdin-redirection-core/classification.json,
  tasks/evidence/2026-06-26-phase12-local-bare-name-stdin-redirection-core/evidence-map.json,
  tasks/evidence/2026-06-26-phase12-local-bare-name-stdin-redirection-core/qemu-local-shell-bare-name-stdin-redirection-smoke.log,
  tasks/evidence/2026-06-26-phase12-local-bare-name-stdin-redirection-closeout/classification.json,
  and
  tasks/evidence/2026-06-26-phase12-local-bare-name-stdin-redirection-closeout/evidence-map.json.
- Retained regression records:
  tasks/2026-06-04-phase10-readonly-regular-file-stdin-redirection-core.md,
  tasks/2026-06-26-phase12-local-absolute-path-vfs-command-core.md,
  tasks/2026-06-26-phase12-local-bare-name-command-argv-core.md,
  tasks/2026-06-26-phase12-local-direct-pipeline-stage-argv-core.md,
  tasks/2026-06-26-phase12-local-bare-name-pipeline-stage-argv-core.md,
  tasks/2026-06-26-phase12-local-process-status-vfs-core.md,
  tasks/2026-06-26-phase12-local-ps-command-vfs-backed-core.md,
  tasks/2026-06-26-phase12-local-pipefail-status-core.md, and
  tasks/2026-06-02-qemu-local-cat-banner-core.md.
- Checkpoint classification and evidence map:
  tasks/evidence/2026-06-26-phase12-local-stdin-redirection-frontier-checkpoint/classification.json
  and
  tasks/evidence/2026-06-26-phase12-local-stdin-redirection-frontier-checkpoint/evidence-map.json.
- Project notes:
  docs/src/roadmap.md, docs/src/project/phase12-networking-ssh.md, and
  docs/src/project/early-posix-shape.md.

## Accepted Frontier

The accepted stdin redirection frontier is local-only and
static/unit/QEMU-substitute backed. Direct path-form and bare-name commands
can read fd0 from one read-only initramfs regular file:

~~~text
/bin/stdin </etc/banner.txt
stdin </etc/banner.txt
~~~

Both forms execute through descriptor-backed VFS open/read, the accepted
loader, userspace launch/status, and child-only fd0 replacement from
initramfs:/etc/banner.txt. The direct path-form surface uses /bin/stdin as
argv0. The bare-name surface canonicalizes stdin through fixed bounded /bin
lookup to /bin/stdin before launch. In both forms, fd1/fd2 are inherited, the
loader temporary descriptor is closed, the shell restores fd0 after child
exit, userspace stdin reads the redirected file to EOF, and waitpid,
laststatus, /proc/talos/processes, zero-argument ps, and
pipestatus-compatible observations remain intact.

No later queued same-lane local POSIX/shell task exists with complete
objective dependencies, acceptance criteria, validation gates, docs, and
evidence requirements, so this checkpoint records planningNeeded=true rather
than selecting a new worker task.

Live network/SSH reachability remains paused. No Pi 5 hardware claim is made.

## Deferred Frontier

Deferred surfaces remain output regular-file redirection, append/truncate,
writable filesystem behavior, pipeline-stage redirection, combined
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
and evidence requirements after the accepted stdin redirection frontier.
