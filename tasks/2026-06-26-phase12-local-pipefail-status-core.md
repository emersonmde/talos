# Phase 12 Local Pipeline Status Core

Task id: phase12-local-pipefail-status-core-20260626

Status: accepted; commit hash recorded in durable supervisor state after commit.

## Scope

Implement a bounded shell-visible pipeline status surface over accepted local
VFS-backed pipeline records. The accepted surface is `pipestatus`: it reads the
current bounded process-table lifecycle records and reports participant
statuses, default final-stage pipeline status, and a labeled pipefail-style
observation.

No POSIX shell compatibility, shell option framework, arbitrary shell grammar,
unbounded pipelines, scheduler concurrency, fork/signals, process
groups/sessions, live networking, SSH, Pi 5 hardware proof, or phase
transition is accepted.

## Findings

- fixed: The shell had process-table lifecycle records for two-stage and
  three-stage pipelines, but no bounded status command that reported all
  participant statuses from that source of truth.
- fixed: The two-stage pipeline executor did not have a bounded nonzero
  pipeline participant case. It now supports exactly
  `exec status42 | exec stdin`, producing a nonzero producer and zero-status
  final stage without changing default `laststatus` semantics.
- fixed: QEMU/local command-loop metadata and smoke scripts did not know the
  new pipeline-status scenario. A dedicated scenario and smoke wrapper retain
  the transcript.
- fixed: Roadmap, Phase 12, and early POSIX notes now describe the accepted
  `pipestatus` boundary and deferred pipefail/POSIX scope.
- not-an-issue: `laststatus` and no-argument `waitpid` continue to use the
  final stage/current lifecycle record; the new `pipefail-status` field is an
  observation only, labeled `bounded-observation-not-posix-shell`.
- deferred: POSIX shell compatibility, `set -o pipefail`, arbitrary pipeline
  grammar, unbounded pipeline length, concurrent execution, fork/signals,
  process groups/sessions, networking, SSH, hardware proof, and phase
  transition.

## Accepted Behavior

`pipestatus` reports no pipeline before a pipeline has populated at least two
bounded process-table records. After `exec stdout | exec stdin`, it reports two
zero-status participants with default status 0 and pipefail-status 0. After
`exec status42 | exec stdin`, it reports producer status 42, final-stage status
0, default status 0, and pipefail-status 42. After the accepted
`exec stdout | exec stdin | exec stdin` form, it reports all three zero-status
participants.

The source of truth is the bounded process-table lifecycle records; the shell
parser does not recompute participant status.

Live network/SSH reachability remains paused. No Pi 5 hardware claim is made.

## Evidence Map

- Classification:
  `tasks/evidence/2026-06-26-phase12-local-pipefail-status-core/classification.json`.
- Evidence map:
  `tasks/evidence/2026-06-26-phase12-local-pipefail-status-core/evidence-map.json`.
- QEMU/substitute transcript:
  `tasks/evidence/2026-06-26-phase12-local-pipefail-status-core/qemu-local-shell-pipeline-status-smoke.log`.
- Code paths: `src/local_command_loop.rs`, `src/target/qemu_virt.rs`,
  `build.rs`, `scripts/qemu-local-serial-command-loop-smoke.sh`, and
  `scripts/qemu-local-shell-pipeline-status-smoke.sh`.
- Docs: `docs/src/roadmap.md`, `docs/src/project/phase12-networking-ssh.md`,
  and `docs/src/project/early-posix-shape.md`.

## Validation

- passed: `cargo fmt --all -- --check`.
- passed: `cargo -Zjson-target-spec test --quiet local_command_loop`
  (834 Talos no_std tests).
- passed: `scripts/qemu-local-shell-pipeline-status-smoke.sh`.
- passed: `jq empty` on task-owned JSON evidence.
- passed: `git diff --check`.
- passed: `/home/node/.cargo/bin/mdbook build` (existing large-search-index
  warning only).
- passed: `git diff --cached --check` before commit.

## Selected Follow-Up

selected_next_task is `phase12-local-pipefail-status-closeout-20260626`.
That follow-up is documentation/evidence reconciliation only if this core
remains accepted and committed.
