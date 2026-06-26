# Phase 12 Local POSIX Frontier Checkpoint

Task id: phase12-local-posix-frontier-checkpoint-20260626

Status: accepted; commit hash recorded in durable supervisor state after
commit.

## Scope

Reconcile the accepted local POSIX/VFS/process frontier after the descriptor
table, VFS/open/read/cat, direct VFS exec, userspace launch, process-table
lifecycle/status, waitpid/jobs, `/proc/talos/processes`, and VFS-backed `ps`
slices.

This checkpoint is static reconciliation work. It does not change source
behavior, run Pi 5 hardware, resume live networking/SSH, retry generated-root
command input, or accept a phase transition.

## Findings

- fixed: The accepted local frontier is reconciled across descriptor-backed
  file I/O, VFS program loading, direct userspace process launch, bounded
  process-table lifecycle/status records, exact two-stage pipeline records,
  accepted background job records, explicit-pid waitpid, no-argument waitpid,
  non-consuming laststatus, jobs accounting, `/proc/talos/processes`, and
  zero-argument VFS-backed `ps`.
- fixed: The evidence map cites retained process-table, process-status VFS,
  VFS-backed `ps`, waitpid/jobs, descriptor-backed VFS, and local POSIX docs.
- fixed: Roadmap, Phase 12 project notes, and early POSIX notes now record the
  reconciled local POSIX frontier and the planning-needed result.
- not-an-issue: No source behavior change is required; the checkpoint records
  the accepted/deferred boundary after the prior implementation and closeout
  tasks.
- deferred: Live networking/SSH, Pi 5 hardware proof, scheduler concurrency,
  fork/signals, process groups/sessions, broad procfs, `/proc/self`,
  `/proc/<pid>`, Linux `ps` compatibility, PID reuse/zombie policy beyond the
  bounded deterministic controls, waitpid options, persistent storage,
  multi-stage pipelines, pipefail, generated-root command-input retry, and
  phase transition.

## Evidence Map

- Process-table frontier checkpoint:
  `tasks/2026-06-26-phase12-local-process-table-frontier-checkpoint.md`.
- Process-status VFS core and closeout:
  `tasks/2026-06-26-phase12-local-process-status-vfs-core.md`,
  `tasks/2026-06-26-phase12-local-process-status-vfs-closeout.md`, and
  `tasks/evidence/2026-06-26-phase12-local-process-status-vfs-core/qemu-local-shell-process-status-vfs-smoke.log`.
- VFS-backed `ps` core and closeout:
  `tasks/2026-06-26-phase12-local-ps-command-vfs-backed-core.md`,
  `tasks/2026-06-26-phase12-local-ps-command-vfs-backed-closeout.md`, and
  `tasks/evidence/2026-06-26-phase12-local-ps-command-vfs-backed-core/qemu-local-shell-ps-command-vfs-smoke.log`.
- Descriptor/VFS/POSIX docs:
  `docs/src/project/early-posix-shape.md`,
  `docs/src/project/phase12-networking-ssh.md`, and
  `docs/src/roadmap.md`.
- Checkpoint classification and evidence map:
  `tasks/evidence/2026-06-26-phase12-local-posix-frontier-checkpoint/classification.json`
  and
  `tasks/evidence/2026-06-26-phase12-local-posix-frontier-checkpoint/evidence-map.json`.

## Accepted Frontier

The accepted local POSIX/VFS/process frontier is still bounded and
substitute-backed. It covers process-local descriptor-backed file I/O over the
accepted initramfs VFS, direct VFS program loading for the accepted fixtures,
the local userspace launch/status path, internal process-table lifecycle/status
records for direct VFS exec, exact two-stage pipelines, and accepted background
jobs, shell-visible explicit/no-argument waitpid, laststatus and jobs
accounting, the Talos-private `/proc/talos/processes` VFS status file, and the
zero-argument `ps` presentation over that same VFS status surface.

Retained evidence remains static inspection, unit-test references from
accepted task records, and QEMU/substitute transcripts. Live network/SSH
reachability remains paused. No Pi 5 hardware claim is made.

## Deferred Frontier

Deferred surfaces remain live networking/SSH, Pi 5 hardware proof,
scheduler-concurrent execution, fork/signals, process groups/sessions, broad
procfs, `/proc/self`, `/proc/<pid>`, Linux `ps` compatibility, public process
enumeration ABI, PID reuse/zombie policy beyond bounded deterministic
controls, waitpid options, persistent storage, multi-stage pipelines,
pipefail, generated-root command-input retry, and phase transition.

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

Planning reason: no later queued same-lane local POSIX task exists with
complete objective dependencies, acceptance criteria, validation gates, docs,
and evidence requirements. Supervisor planning is required before any next
local POSIX feature, any return to live network/SSH reachability work, or any
phase transition.
