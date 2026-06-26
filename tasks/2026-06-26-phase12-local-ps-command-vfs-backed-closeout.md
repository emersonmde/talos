# Phase 12 Local Ps Command VFS-Backed Closeout

Task id: phase12-local-ps-command-vfs-backed-closeout-20260626

Status: accepted; commit hash recorded in durable supervisor state after
commit.

## Scope

Close out the accepted zero-argument `ps` shell view as a thin presentation
of the Talos-private `/proc/talos/processes` VFS status file.

This task is static closeout work. It does not add code, resume live
networking, run Pi 5 hardware, retry generated-root command input, or advance a
phase transition.

## Findings

- fixed: Reconciled the accepted `ps` command against the retained
  descriptor-backed `/proc/talos/processes` status-file evidence.
- fixed: Recorded that `ps` stays on the same VFS data path as
  `cat /proc/talos/processes` and does not directly dump the process table.
- fixed: Added closeout classification and evidence-map records that point to
  the accepted core task, retained QEMU/substitute transcript, process-status
  VFS regression transcript, and updated docs.
- fixed: Updated roadmap and Phase 12 project notes with the closeout
  boundary and selected checkpoint follow-up.
- not-an-issue: The accepted `ps` output intentionally matches
  `talos-processes-v1`; Linux formatting and options are outside this
  boundary.
- deferred: Linux `ps` compatibility, Linux procfs compatibility,
  `/proc/self`, `/proc/<pid>`, public process enumeration ABI, `ps`
  arguments/options, sorting/filtering, scheduler concurrency, fork/signals,
  process groups/sessions, waitpid options, PID reuse/zombie policy,
  multi-stage pipelines, pipefail, persistent storage, live networking, SSH,
  Pi 5 hardware proof, and phase transition.

## Evidence Map

- Core task record:
  `tasks/2026-06-26-phase12-local-ps-command-vfs-backed-core.md`.
- Core classification:
  `tasks/evidence/2026-06-26-phase12-local-ps-command-vfs-backed-core/classification.json`.
- Core evidence map:
  `tasks/evidence/2026-06-26-phase12-local-ps-command-vfs-backed-core/evidence-map.json`.
- QEMU/substitute `ps` transcript:
  `tasks/evidence/2026-06-26-phase12-local-ps-command-vfs-backed-core/qemu-local-shell-ps-command-vfs-smoke.log`.
- Process-status VFS regression transcript:
  `tasks/evidence/2026-06-26-phase12-local-process-status-vfs-core/qemu-local-shell-process-status-vfs-smoke.log`.
- Closeout classification:
  `tasks/evidence/2026-06-26-phase12-local-ps-command-vfs-backed-closeout/classification.json`.
- Closeout evidence map:
  `tasks/evidence/2026-06-26-phase12-local-ps-command-vfs-backed-closeout/evidence-map.json`.

## Accepted Frontier

`ps` is accepted only as a zero-argument Talos-private shell view over the
descriptor-backed `/proc/talos/processes` file. It reports the accepted
bounded `talos-processes-v1` records for direct foreground VFS exec, exact
two-stage pipeline producer/consumer records, and accepted background jobs.

The closeout preserves `cat /proc/talos/processes` as the underlying
regression/control surface and rejects a direct process-table dump or fake
command expansion as progress.

Live network/SSH reachability remains paused. No Pi 5 hardware claim is made.

## Deferred Frontier

Deferred surfaces remain Linux `ps` compatibility, Linux procfs
compatibility, `/proc/self`, `/proc/<pid>`, public process enumeration ABI,
`ps` arguments/options, sorting/filtering, scheduler concurrency,
fork/signals, process groups/sessions, waitpid options, PID reuse/zombie
policy, multi-stage pipelines, pipefail, persistent storage, live networking,
SSH, Pi 5 hardware proof, and phase transition.

## Selected Follow-Up

selected_next_task is
`phase12-local-posix-frontier-checkpoint-20260626` because that queued
checkpoint is mechanically objective after this accepted closeout and remains
in the same local POSIX/VFS/userspace continuation slice.

## Validation

- passed: static inspection of retained core task record, classification,
  evidence map, QEMU/substitute transcript paths, roadmap, and Phase 12 project
  notes.
- passed: `jq empty` on closeout classification/evidence-map JSON.
- passed: `git diff --check`.
- passed: `/home/node/.cargo/bin/mdbook build`.
- passed: `git diff --cached --check`.

## Result

The VFS-backed `ps` frontier is closed out as a Talos-private presentation
over `/proc/talos/processes`. It does not authorize fake command expansion,
Linux `ps`/procfs compatibility, hardware work, live networking, SSH, or a
phase transition.
