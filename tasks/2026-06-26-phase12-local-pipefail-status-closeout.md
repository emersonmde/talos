# Phase 12 Local Pipeline Status Closeout

Task id: phase12-local-pipefail-status-closeout-20260626

Status: accepted; commit hash recorded in durable supervisor state after
commit.

## Scope

Close out the accepted bounded pipeline status surface:

~~~text
pipestatus
~~~

This task reconciles the retained status evidence from the core task against
the accepted multistage pipeline, process-table, `waitpid`,
`laststatus`,
`/proc/talos/processes`, and zero-argument `ps` frontier. It is static closeout
work. It does not change source behavior, run Pi 5 hardware, resume live
networking/SSH, retry generated-root command input, or advance a phase
transition.

## Findings

- fixed: Mapped the accepted `pipestatus` surface to the retained core task
  record, core classification/evidence map, and QEMU/substitute transcript.
- fixed: Recorded that `pipestatus` reads participant statuses from the
  accepted bounded process-table lifecycle records for exact two-stage
  pipelines, the accepted three-stage pipeline, and the bounded nonzero
  producer case `exec status42 | exec stdin`.
- fixed: Reconciled default pipeline status with the final-stage lifecycle
  record and preserved `laststatus` behavior as a separate non-consuming
  latest-process observation.
- fixed: Added closeout classification and evidence-map records that preserve
  the accepted/deferred frontier and selected checkpoint follow-up.
- fixed: Updated roadmap and Phase 12 project notes with the closeout boundary
  and dependency-gated local pipeline frontier checkpoint.
- not-an-issue: The reported `pipefail-status` remains a labeled bounded
  observation, `bounded-observation-not-posix-shell`, rather than POSIX
  `set -o pipefail` compatibility or a shell option framework.
- deferred: POSIX shell compatibility, arbitrary shell grammar, unbounded or
  arbitrary-length pipelines, pipeline concurrency, scheduler concurrency,
  fork/signals, process groups/sessions, waitpid options, broad procfs/Linux
  `ps` compatibility, public process enumeration ABI, PID policy expansion,
  persistent storage, live networking, SSH, Pi 5 hardware proof,
  generated-root command-input retry, and phase transition.

## Evidence Map

- Core task record:
  `tasks/2026-06-26-phase12-local-pipefail-status-core.md`.
- Core classification:
  `tasks/evidence/2026-06-26-phase12-local-pipefail-status-core/classification.json`.
- Core evidence map:
  `tasks/evidence/2026-06-26-phase12-local-pipefail-status-core/evidence-map.json`.
- QEMU/substitute pipeline-status transcript:
  `tasks/evidence/2026-06-26-phase12-local-pipefail-status-core/qemu-local-shell-pipeline-status-smoke.log`.
- Multistage pipeline closeout:
  `tasks/2026-06-26-phase12-local-multistage-pipeline-closeout.md`.
- Process-status VFS regression transcript:
  `tasks/evidence/2026-06-26-phase12-local-process-status-vfs-core/qemu-local-shell-process-status-vfs-smoke.log`.
- VFS-backed `ps` regression transcript:
  `tasks/evidence/2026-06-26-phase12-local-ps-command-vfs-backed-core/qemu-local-shell-ps-command-vfs-smoke.log`.
- Closeout classification:
  `tasks/evidence/2026-06-26-phase12-local-pipefail-status-closeout/classification.json`.
- Closeout evidence map:
  `tasks/evidence/2026-06-26-phase12-local-pipefail-status-closeout/evidence-map.json`.

## Accepted Frontier

The accepted pipeline-status frontier is the bounded `pipestatus` observation
surface. It reports the participant count, default final-stage pipeline status,
and a labeled `pipefail-status` over the process-table records for the last
accepted two-stage or three-stage pipeline.

The retained transcript proves:

- no prior pipeline reports `talos: pipestatus none`;
- `exec stdout | exec stdin` reports two zero-status participants with
  default status `0x0` and `pipefail-status=0x0`;
- `exec status42 | exec stdin` reports producer status `0x2a`, final-stage
  default status `0x0`, and `pipefail-status=0x2a`;
- `exec stdout | exec stdin | exec stdin` reports three zero-status
  participants with default status `0x0` and `pipefail-status=0x0`.

Default pipeline status remains the final-stage lifecycle status. `laststatus`
continues to report the latest process lifecycle record and is not changed into
a pipefail or all-participants surface.

Live network/SSH reachability remains paused. No Pi 5 hardware claim is made.

## Deferred Frontier

Deferred surfaces remain POSIX shell compatibility, `set -o pipefail`,
arbitrary shell grammar, unbounded or arbitrary-length pipelines,
pipeline-concurrent execution, scheduler concurrency, fork/signals, process
groups/sessions, waitpid options, broad procfs/Linux `ps` compatibility,
`/proc/self`, `/proc/<pid>`, public process enumeration ABI, PID policy
expansion, persistent storage, live networking, SSH, Pi 5 hardware proof,
generated-root command-input retry, and phase transition.

## Selected Follow-Up

selected_next_task is `phase12-local-pipeline-frontier-checkpoint-20260626`
because the pipeline-status core was accepted and committed, this closeout
reconciles its evidence, and the queued checkpoint is mechanically objective
only after the accepted multistage and pipeline-status closeouts.

The follow-up remains local static/docs/evidence reconciliation scoped. It does
not authorize live networking, SSH, hardware proof, POSIX shell compatibility,
or a phase transition.

## Validation

- passed: static inspection of retained core task record, classification,
  evidence map, QEMU/substitute transcript paths, multistage closeout,
  roadmap, Phase 12 project notes, and early POSIX notes.
- passed: `jq empty` on closeout classification/evidence-map JSON.
- passed: `git diff --check`.
- passed: `/home/node/.cargo/bin/mdbook build`.
- passed: `git diff --cached --check`.

## Result

The bounded pipeline status frontier is closed out as a process-table-backed
status observation over accepted two-stage and three-stage local pipeline
records. It preserves default final-stage status and `laststatus`, does not
claim POSIX pipefail compatibility, and does not authorize fake command
expansion, unbounded pipelines, hardware work, live networking, SSH, or a phase
transition.
