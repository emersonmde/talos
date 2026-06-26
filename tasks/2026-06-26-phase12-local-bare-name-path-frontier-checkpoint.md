# Phase 12 Local Bare-Name Path Frontier Checkpoint

Task id: phase12-local-bare-name-path-frontier-checkpoint-20260626

Status: accepted; commit hash recorded in durable supervisor state after
commit.

## Scope

Reconcile the accepted bounded bare-name command and bare-name pipeline
frontier after the direct 'status42' command and the 'stdout | stdin'
pipeline slices.

This checkpoint is static reconciliation work. It does not change source
behavior, run Pi 5 hardware, resume live networking/SSH, retry generated-root
command input, accept POSIX PATH environment compatibility, or accept a phase
transition.

## Findings

- fixed: The accepted bare-name frontier is reconciled across direct
  'status42' execution and the bounded 'stdout | stdin' bare-name pipeline.
- fixed: The evidence map cites retained bare-name command, bare-name
  pipeline, direct absolute-path command, path-form pipeline, exec-prefixed
  pipeline, multistage pipeline, process-table, waitpid/jobs,
  '/proc/talos/processes', zero-argument 'ps', and 'pipestatus' task records
  and transcripts.
- fixed: Roadmap, Phase 12 project notes, and early POSIX notes now record the
  reconciled bare-name frontier and the planning-needed result.
- not-an-issue: No source behavior change is required; the checkpoint records
  the accepted/deferred boundary after the prior implementation tasks.
- deferred: Live networking/SSH, Pi 5 hardware proof, POSIX PATH environment
  compatibility, command lookup beyond the accepted bounded /bin surface,
  path-form arguments/redirections, arbitrary shell grammar, unbounded
  pipelines, pipeline concurrency, scheduler concurrency, fork/signals,
  process groups/sessions, persistent storage, generated-root command-input
  retry, and phase transition.

## Evidence Map

- Bare-name command core:
  tasks/2026-06-26-phase12-local-bare-name-vfs-command-core.md and
  tasks/evidence/2026-06-26-phase12-local-bare-name-vfs-command-core/qemu-local-shell-bare-name-vfs-command-smoke.log.
- Bare-name pipeline core:
  tasks/2026-06-26-phase12-local-bare-name-vfs-pipeline-core.md and
  tasks/evidence/2026-06-26-phase12-local-bare-name-vfs-pipeline-core/qemu-local-shell-bare-name-vfs-pipeline-smoke.log.
- Direct absolute-path command and bounded path-form pipeline:
  tasks/2026-06-26-phase12-local-absolute-path-vfs-command-core.md,
  tasks/2026-06-26-phase12-local-absolute-path-vfs-pipeline-core.md, and
  tasks/2026-06-26-phase12-local-path-command-frontier-checkpoint.md.
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
  tasks/evidence/2026-06-26-phase12-local-bare-name-path-frontier-checkpoint/classification.json
  and
  tasks/evidence/2026-06-26-phase12-local-bare-name-path-frontier-checkpoint/evidence-map.json.

## Accepted Frontier

The accepted bare-name frontier is local-only and static/unit/QEMU-substitute
backed. It includes direct bare-name command execution for 'status42' and the
bounded two-stage bare-name pipeline 'stdout | stdin'. Every accepted
bare-name command or stage resolves only through the fixed /bin VFS lookup
and reaches userspace through descriptor-backed VFS open/read, the accepted
program loader, initial user stack, userspace launch/status, and bounded
process-table/status surfaces.

The retained evidence preserves pipeline byte flow, lifecycle/status records,
bounded process-table records, explicit waitpid, laststatus,
'/proc/talos/processes', zero-argument 'ps', and 'pipestatus' observations
using the same sources of truth accepted by the direct absolute-path,
path-form pipeline, exec-prefixed pipeline, and process-table frontiers.

Live network/SSH reachability remains paused. No Pi 5 hardware claim is made.

## Deferred Frontier

Deferred surfaces remain live networking/SSH, Pi 5 hardware proof, POSIX PATH
environment compatibility, PATH mutation, command lookup beyond the bounded
/bin surface, mixed bare/path/exec pipeline compatibility, path-form
arguments/redirections, arbitrary shell grammar, unbounded pipelines,
pipeline concurrency, scheduler concurrency, fork/signals, process
groups/sessions, broad procfs/Linux 'ps', PID policy expansion, waitpid
options, persistent storage, generated-root command-input retry, and phase
transition.

## Validation

- Static inspection of task records, retained evidence paths, docs, and git
  diff: passed.
- jq empty on task-owned JSON evidence: passed.
- git diff --check: passed.
- /home/node/.cargo/bin/mdbook build: passed.
- git diff --cached --check: passed.

## Result

selected_next_task: null.

planningNeeded: true.

Planning reason: no later queued same-lane local POSIX/shell task exists with
complete objective dependencies, acceptance criteria, validation gates, docs,
and evidence requirements. Supervisor planning is required before any next
local POSIX feature, any return to live network/SSH reachability work, any
hardware action, generated-root retry, PATH expansion, broad shell expansion,
or phase transition.
