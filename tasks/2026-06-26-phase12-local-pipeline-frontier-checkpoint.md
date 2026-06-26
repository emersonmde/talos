# Phase 12 Local Pipeline Frontier Checkpoint

Task id: phase12-local-pipeline-frontier-checkpoint-20260626

Status: accepted; commit hash recorded in durable supervisor state after
commit.

## Scope

Reconcile the accepted local pipeline/POSIX frontier after the descriptor-backed
VFS file I/O, VFS exec/userspace launch, process-table, waitpid/laststatus/jobs,
`/proc/talos/processes`, VFS-backed `ps`, exact two-stage pipeline, accepted
three-stage pipeline, and bounded `pipestatus` slices.

This checkpoint is static reconciliation work. It does not change source
behavior, run Pi 5 hardware, resume live networking/SSH, retry generated-root
command input, or accept a phase transition.

## Findings

- fixed: Reconciled the accepted local pipeline frontier across direct VFS exec,
  exact two-stage pipeline records, the accepted three-stage pipeline
  `exec stdout | exec stdin | exec stdin`, process-table lifecycle/status
  records, explicit/no-argument `waitpid`, non-consuming `laststatus`, jobs
  accounting, `/proc/talos/processes`, zero-argument VFS-backed `ps`, and
  bounded `pipestatus`.
- fixed: The evidence map cites retained multistage pipeline, pipeline-status,
  process-status VFS, VFS-backed `ps`, process-table, and prior POSIX frontier
  checkpoint records.
- fixed: Roadmap, Phase 12 project notes, and early POSIX notes now record the
  reconciled pipeline frontier and the planning-needed result.
- not-an-issue: No implementation change is required; the checkpoint only
  records the accepted/deferred boundary after the previous implementation and
  closeout tasks.
- deferred: Live networking/SSH, Pi 5 hardware proof, scheduler concurrency,
  fork/signals, process groups/sessions, broad procfs/Linux `ps`, PID policy
  expansion, waitpid options, persistent storage, arbitrary shell grammar,
  unbounded pipelines, pipeline concurrency, POSIX pipefail compatibility,
  generated-root command-input retry, and phase transition.

## Evidence Map

- Multistage pipeline core and closeout:
  `tasks/2026-06-26-phase12-local-multistage-pipeline-core.md`,
  `tasks/2026-06-26-phase12-local-multistage-pipeline-closeout.md`, and
  `tasks/evidence/2026-06-26-phase12-local-multistage-pipeline-core/qemu-local-shell-multistage-pipeline-smoke.log`.
- Pipeline-status core and closeout:
  `tasks/2026-06-26-phase12-local-pipefail-status-core.md`,
  `tasks/2026-06-26-phase12-local-pipefail-status-closeout.md`, and
  `tasks/evidence/2026-06-26-phase12-local-pipefail-status-core/qemu-local-shell-pipeline-status-smoke.log`.
- Process-status VFS and VFS-backed `ps` evidence:
  `tasks/2026-06-26-phase12-local-process-status-vfs-closeout.md`,
  `tasks/2026-06-26-phase12-local-ps-command-vfs-backed-closeout.md`,
  `tasks/evidence/2026-06-26-phase12-local-process-status-vfs-core/qemu-local-shell-process-status-vfs-smoke.log`, and
  `tasks/evidence/2026-06-26-phase12-local-ps-command-vfs-backed-core/qemu-local-shell-ps-command-vfs-smoke.log`.
- Process-table and prior POSIX frontier checkpoints:
  `tasks/2026-06-26-phase12-local-process-table-frontier-checkpoint.md` and
  `tasks/2026-06-26-phase12-local-posix-frontier-checkpoint.md`.
- Checkpoint classification and evidence map:
  `tasks/evidence/2026-06-26-phase12-local-pipeline-frontier-checkpoint/classification.json`
  and
  `tasks/evidence/2026-06-26-phase12-local-pipeline-frontier-checkpoint/evidence-map.json`.

## Accepted Frontier

The accepted local pipeline/POSIX frontier is still bounded and
substitute-backed. It covers process-local descriptor-backed file I/O over the
accepted initramfs VFS, direct VFS program loading for accepted fixtures, local
userspace launch/status, internal process-table lifecycle/status records for
direct VFS exec, exact two-stage pipelines, accepted background jobs, and the
accepted exact three-stage pipeline:

```text
exec stdout | exec stdin | exec stdin
```

Shell-visible observation covers explicit/no-argument `waitpid`,
non-consuming `laststatus`, jobs accounting, the Talos-private
`/proc/talos/processes` VFS status file, zero-argument VFS-backed `ps`, and the
bounded `pipestatus` surface. `pipestatus` reads accepted process-table
records for exact two-stage and accepted three-stage pipelines, reports default
final-stage status, and exposes `pipefail-status` only as the labeled
`bounded-observation-not-posix-shell` field.

Retained evidence remains static inspection, unit-test references from
accepted task records, and QEMU/substitute transcripts. Live network/SSH
reachability remains paused. No Pi 5 hardware claim is made.

## Deferred Frontier

Deferred surfaces remain live networking/SSH, Pi 5 hardware proof,
scheduler-concurrent execution, fork/signals, process groups/sessions, broad
procfs, `/proc/self`, `/proc/<pid>`, Linux `ps` compatibility, public process
enumeration ABI, PID reuse/zombie policy beyond bounded deterministic
controls, waitpid options, persistent storage, arbitrary shell grammar,
unbounded or arbitrary-length pipelines, pipeline-concurrent execution, POSIX
pipefail compatibility or `set -o pipefail`, generated-root command-input
retry, and phase transition.

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

Planning reason: no later queued same-lane local POSIX/pipeline task exists
with complete objective dependencies, acceptance criteria, validation gates,
docs, and evidence requirements. Supervisor planning is required before any
next local POSIX feature, any return to live network/SSH reachability work, or
any phase transition.
