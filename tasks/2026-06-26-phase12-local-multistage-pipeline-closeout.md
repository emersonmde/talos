# Phase 12 Local Multistage Pipeline Closeout

Task id: phase12-local-multistage-pipeline-closeout-20260626

Status: accepted; commit hash recorded in durable supervisor state after
commit.

## Scope

Close out the accepted exact three-stage local pipeline:

```text
exec stdout | exec stdin | exec stdin
```

This task reconciles the retained byte-flow, process-table, waitpid/jobs,
`/proc/talos/processes`, and zero-argument `ps` evidence from the core task.
It is static closeout work. It does not change source behavior, run Pi 5
hardware, resume live networking/SSH, retry generated-root command input, or
advance a phase transition.

## Findings

- fixed: Mapped the accepted three-stage byte-flow evidence to the retained
  core task record, core classification/evidence map, and multistage
  QEMU/substitute transcript.
- fixed: Recorded that the first fixture's stdout reaches the final fixture
  through two descriptor-backed pipe handoffs and not through a direct shell
  shortcut.
- fixed: Reconciled the three bounded process-table records with explicit
  waitpid observations, `/proc/talos/processes`, and zero-argument `ps`
  output.
- fixed: Added closeout classification and evidence-map records that preserve
  the accepted/deferred frontier and selected follow-up.
- fixed: Updated roadmap and Phase 12 project notes with the closeout boundary
  and dependency-gated pipeline status follow-up.
- not-an-issue: The accepted middle and final stages intentionally reuse the
  existing `/bin/stdin` fixture; the proof target is descriptor-backed byte
  propagation through the accepted VFS/userspace fixture path.
- deferred: arbitrary shell grammar, unbounded or arbitrary-length pipelines,
  pipefail, scheduler concurrency, fork/signals, process groups/sessions,
  broad procfs/Linux `ps` compatibility, PID policy expansion, persistent
  storage, live networking, SSH, Pi 5 hardware proof, generated-root
  command-input retry, and phase transition.

## Evidence Map

- Core task record:
  `tasks/2026-06-26-phase12-local-multistage-pipeline-core.md`.
- Core classification:
  `tasks/evidence/2026-06-26-phase12-local-multistage-pipeline-core/classification.json`.
- Core evidence map:
  `tasks/evidence/2026-06-26-phase12-local-multistage-pipeline-core/evidence-map.json`.
- QEMU/substitute multistage transcript:
  `tasks/evidence/2026-06-26-phase12-local-multistage-pipeline-core/qemu-local-shell-multistage-pipeline-smoke.log`.
- Process-status VFS regression transcript:
  `tasks/evidence/2026-06-26-phase12-local-process-status-vfs-core/qemu-local-shell-process-status-vfs-smoke.log`.
- VFS-backed `ps` regression transcript:
  `tasks/evidence/2026-06-26-phase12-local-ps-command-vfs-backed-core/qemu-local-shell-ps-command-vfs-smoke.log`.
- Closeout classification:
  `tasks/evidence/2026-06-26-phase12-local-multistage-pipeline-closeout/classification.json`.
- Closeout evidence map:
  `tasks/evidence/2026-06-26-phase12-local-multistage-pipeline-closeout/evidence-map.json`.

## Accepted Frontier

The accepted multistage pipeline frontier remains exactly:

```text
exec stdout | exec stdin | exec stdin
```

The producer, middle, and final fixtures are loaded through the accepted VFS
exec path. The first pipe transfers 31 bytes from `/bin/stdout` fd1 to the
middle `/bin/stdin` fd0; the second pipe transfers 68 bytes from the middle
`/bin/stdin` fd1 to the final `/bin/stdin` fd0. The final visible output is:

```text
Talos userspace stdin fixture read: Talos userspace stdin fixture read: Talos userspace stdout fixture
```

The retained transcript records the two pipe summaries, the
`phase12-local-multistage-pipeline-lifecycle-status-record-v1` lifecycle
record, explicit waitpid for pids `0x100001`, `0x100002`, and `0x100003`,
and matching `/proc/talos/processes` plus zero-argument `ps` output for all
three bounded process-table records.

Live network/SSH reachability remains paused. No Pi 5 hardware claim is made.

## Deferred Frontier

Deferred surfaces remain arbitrary shell grammar, unbounded or arbitrary-length
pipelines, pipefail, scheduler-concurrent execution, fork/signals, process
groups/sessions, waitpid options, broad procfs/Linux `ps` compatibility,
`/proc/self`, `/proc/<pid>`, public process enumeration ABI, PID policy
expansion, persistent storage, live networking, SSH, Pi 5 hardware proof,
generated-root command-input retry, and phase transition.

## Selected Follow-Up

selected_next_task is `phase12-local-pipefail-status-core-20260626` because
the core task was accepted and the queued bounded pipeline status task is
mechanically objective only after this closeout. The follow-up remains
local/static/unit/QEMU-substitute scoped and does not authorize live
networking, SSH, hardware proof, or POSIX shell compatibility claims.

## Validation

- passed: static inspection of retained core task record, classification,
  evidence map, QEMU/substitute transcript paths, roadmap, and Phase 12 project
  notes.
- passed: `jq empty` on closeout classification/evidence-map JSON.
- passed: `git diff --check`.
- passed: `/home/node/.cargo/bin/mdbook build`.
- passed: `git diff --cached --check`.

## Result

The exact three-stage local pipeline frontier is closed out as a
VFS/userspace/descriptor-backed pipeline proof with three bounded process-table
records. It does not authorize fake command expansion, unbounded pipelines,
pipefail, hardware work, live networking, SSH, or a phase transition.
