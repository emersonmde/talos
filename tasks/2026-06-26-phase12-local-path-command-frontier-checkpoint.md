# Phase 12 Local Path-Command Frontier Checkpoint

Task id: phase12-local-path-command-frontier-checkpoint-20260626

Status: accepted; commit hash recorded in durable supervisor state after
commit.

## Scope

Reconcile the accepted direct absolute-path command and bounded path-form
pipeline frontier after the direct '/bin/status42' command and the
'/bin/stdout | /bin/stdin' pipeline slices.

This checkpoint is static reconciliation work. It does not change source
behavior, run Pi 5 hardware, resume live networking/SSH, retry generated-root
command input, accept PATH or bare-name lookup, or accept a phase transition.

## Findings

- fixed: The accepted path-command frontier is reconciled across direct
  '/bin/status42' execution and the bounded '/bin/stdout | /bin/stdin'
  path-form pipeline.
- fixed: The evidence map cites retained direct path-command, path-form
  pipeline, exec-prefixed pipeline, multistage pipeline, process-table,
  waitpid/jobs, '/proc/talos/processes', zero-argument 'ps', and 'pipestatus'
  task records and transcripts.
- fixed: Roadmap, Phase 12 project notes, and early POSIX notes now record the
  reconciled path-command frontier and the planning-needed result.
- not-an-issue: No source behavior change is required; the checkpoint records
  the accepted/deferred boundary after the prior implementation and closeout
  tasks.
- deferred: Live networking/SSH, Pi 5 hardware proof, PATH lookup, bare-name
  lookup, path-form arguments/redirections, arbitrary shell grammar,
  unbounded pipelines, pipeline concurrency, scheduler concurrency,
  fork/signals, process groups/sessions, persistent storage,
  generated-root command-input retry, and phase transition.

## Evidence Map

- Direct absolute-path command core and closeout:
  tasks/2026-06-26-phase12-local-absolute-path-vfs-command-core.md,
  tasks/2026-06-26-phase12-local-absolute-path-vfs-command-closeout.md, and
  tasks/evidence/2026-06-26-phase12-local-absolute-path-vfs-command-core/qemu-local-shell-absolute-path-vfs-command-smoke.log.
- Bounded path-form pipeline core and closeout:
  tasks/2026-06-26-phase12-local-absolute-path-vfs-pipeline-core.md,
  tasks/2026-06-26-phase12-local-absolute-path-vfs-pipeline-closeout.md, and
  tasks/evidence/2026-06-26-phase12-local-absolute-path-vfs-pipeline-core/qemu-local-shell-absolute-path-vfs-pipeline-smoke.log.
- Retained pipeline/process regression frontier:
  tasks/2026-06-26-phase12-local-multistage-pipeline-core.md,
  tasks/2026-06-26-phase12-local-pipefail-status-core.md,
  tasks/2026-06-26-phase12-local-process-table-frontier-checkpoint.md,
  tasks/2026-06-26-phase12-local-posix-frontier-checkpoint.md, and
  tasks/2026-06-26-phase12-local-pipeline-frontier-checkpoint.md.
- Descriptor/VFS/POSIX docs:
  docs/src/project/early-posix-shape.md,
  docs/src/project/phase12-networking-ssh.md, and docs/src/roadmap.md.
- Checkpoint classification and evidence map:
  tasks/evidence/2026-06-26-phase12-local-path-command-frontier-checkpoint/classification.json
  and
  tasks/evidence/2026-06-26-phase12-local-path-command-frontier-checkpoint/evidence-map.json.

## Accepted Frontier

The accepted path-command frontier is local-only and
static/unit/QEMU-substitute backed. It includes direct absolute-path command
execution for '/bin/status42' and the bounded two-stage path-form pipeline
'/bin/stdout | /bin/stdin'. Every accepted stage reaches userspace through the
descriptor-backed VFS open/read path, existing program loader, initial user
stack, and local userspace launch/status path.

The retained evidence preserves pipe byte flow, lifecycle/status records,
bounded process-table records, explicit waitpid, laststatus,
'/proc/talos/processes', zero-argument 'ps', and 'pipestatus' observations
using the same sources of truth accepted by the exec-prefixed pipeline and
process-table frontier.

Live network/SSH reachability remains paused. No Pi 5 hardware claim is made.

## Deferred Frontier

Deferred surfaces remain live networking/SSH, Pi 5 hardware proof, PATH
lookup, bare-name lookup, path-form arguments/redirections, arbitrary shell
grammar, unbounded pipelines, pipeline concurrency, scheduler concurrency,
fork/signals, process groups/sessions, broad procfs/Linux 'ps', PID policy
expansion, persistent storage, generated-root command-input retry, and phase
transition.

## Validation

- Static inspection of task records, retained evidence paths, docs, and git
  diff: passed.
- jq empty on task-owned JSON evidence: passed.
- git diff --check: passed.
- /home/node/.cargo/bin/mdbook build: passed; mdBook emitted the existing
  large-search-index warning.
- git diff --cached --check: passed.

## Result

selected_next_task: null.

planningNeeded: true.

Planning reason: no later queued same-lane local POSIX/shell task exists with
complete objective dependencies, acceptance criteria, validation gates, docs,
and evidence requirements. Supervisor planning is required before any next
local POSIX feature, any return to live network/SSH reachability work, any
hardware action, generated-root retry, PATH/bare-name expansion, or phase
transition.
