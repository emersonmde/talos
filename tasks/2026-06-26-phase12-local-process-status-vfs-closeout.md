# Phase 12 Local Process Status VFS Closeout

Task id: phase12-local-process-status-vfs-closeout-20260626

Status: accepted; commit hash recorded in durable supervisor state after commit.

## Scope

Reconcile the accepted Talos-private process-status VFS surface and decide
whether the queued VFS-backed `ps` shell view is mechanically objective.

This closeout does not change source behavior. It only records the accepted
and deferred boundary after `/proc/talos/processes` was accepted as a
read-only, descriptor-backed VFS file.

## Findings

- fixed: The accepted process-status VFS core evidence is reconciled against
  the retained bounded process-table, descriptor-backed VFS, waitpid/jobs,
  exact pipeline/background, non-proc VFS cat, and unsupported proc-path
  negative-control evidence.
- fixed: The closeout records that `/proc/talos/processes` is the accepted
  first process-status surface: Talos-private, read-only, versioned as
  `talos-processes-v1`, and consumed through the existing `cat`/open/read
  descriptor path.
- fixed: The queued `phase12-local-ps-command-vfs-backed-core-20260626` task
  is mechanically objective because its dependency is exactly this accepted
  VFS file and its non-goals forbid a direct process-table dump that bypasses
  the VFS process-status surface.
- not-an-issue: No source change is required in closeout; source behavior was
  accepted by the core task at commit
  `21cda14fb10afabf8f0dfc52fc306302f62a7ba4`.
- deferred: Linux procfs compatibility, `/proc/self`, `/proc/<pid>`, public
  stable process enumeration ABI, scheduler-concurrent execution, fork/signals,
  process groups/sessions, waitpid options, PID reuse policy, multi-stage
  pipelines, pipefail, persistent storage, live networking, SSH, Pi 5 hardware
  proof, and phase transition.

## Evidence Map

- Core task record:
  `tasks/2026-06-26-phase12-local-process-status-vfs-core.md`.
- Core classification:
  `tasks/evidence/2026-06-26-phase12-local-process-status-vfs-core/classification.json`.
- Core evidence map:
  `tasks/evidence/2026-06-26-phase12-local-process-status-vfs-core/evidence-map.json`.
- Core QEMU/substitute transcript:
  `tasks/evidence/2026-06-26-phase12-local-process-status-vfs-core/qemu-local-shell-process-status-vfs-smoke.log`.
- Closeout classification:
  `tasks/evidence/2026-06-26-phase12-local-process-status-vfs-closeout/classification.json`.
- Closeout evidence map:
  `tasks/evidence/2026-06-26-phase12-local-process-status-vfs-closeout/evidence-map.json`.

## Accepted Frontier

The accepted process-status frontier is `/proc/talos/processes`, a
Talos-private read-only VFS file backed by the bounded process-table substrate
and read through the descriptor-backed local VFS path. The versioned
`talos-processes-v1` text reports only the accepted direct foreground VFS
exec, exact two-stage pipeline producer/consumer, and accepted background job
records, including consumed/reaped state where the accepted waitpid fixtures
observe it.

The next bounded shell feature may present this information as `ps` only if
it reads `/proc/talos/processes` through the accepted VFS path or an
explicitly documented equivalent process-local VFS read helper. A `ps`
implementation that directly dumps the process table is not accepted.

## Deferred Frontier

Deferred surfaces remain Linux procfs compatibility, `/proc/self`,
`/proc/<pid>`, public stable process enumeration ABI, scheduler-concurrent
execution, fork/signals, process groups/sessions, waitpid options, PID reuse
policy, multi-stage pipelines, pipefail, persistent storage, live networking,
SSH, Pi 5 hardware proof, and phase transition.

Live network/SSH reachability remains paused. No Pi 5 hardware claim is made.

## Validation

- Static inspection of task records, retained evidence paths, docs, and git
  diff: passed.
- jq empty on task-owned JSON evidence: passed.
- git diff --check: passed.
- /home/node/.cargo/bin/mdbook build: passed.
- git diff --cached --check: passed.

## Result

selected_next_task: phase12-local-ps-command-vfs-backed-core-20260626.

planningNeeded: false.

The queued `ps` task is selected only as a VFS-backed presentation layer over
the accepted `/proc/talos/processes` surface. It does not authorize fake
command expansion, Linux `ps`/procfs compatibility, hardware work, live
networking, SSH, or a phase transition.
