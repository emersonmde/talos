# Phase 12 Local Multistage Pipeline Core

Task id: phase12-local-multistage-pipeline-core-20260626

Status: accepted; commit hash recorded in durable supervisor state after commit.

## Scope

Add the first bounded shell-visible three-stage pipeline:

```text
exec stdout | exec stdin | exec stdin
```

The path stays local and VFS-backed. The producer, middle stage, and final
stage are loaded through the accepted VFS exec path and exchange bytes through
two pipe-backed stdio descriptor handoffs. The task also records three bounded
process-table lifecycle/status entries and preserves waitpid, jobs,
`/proc/talos/processes`, zero-argument `ps`, and existing pipeline/direct exec
regressions.

No live Ethernet/TCP, SSH, Pi 5 hardware proof, generated-root retry, fake
kernel-backed command expansion, arbitrary pipeline grammar, unbounded pipeline
length, concurrent scheduler execution, pipefail, fork/signals, process
groups/sessions, or phase transition is accepted.

## Findings

- fixed: Added parser and dispatch support for exactly one three-stage
  pipeline form with two pipe separators while preserving existing two-stage
  pipeline parsing and unsupported-form rejection.
- fixed: Added a second bounded pipe endpoint and routed fd1 of the first
  fixture to fd0 of the middle fixture, then fd1 of the middle fixture to fd0
  of the final fixture through descriptor-backed pipe objects.
- fixed: Added a three-stage lifecycle/status record identity,
  `phase12-local-multistage-pipeline-lifecycle-status-record-v1`, and three
  process-table entries with pids `0x100001`, `0x100002`, and `0x100003`.
- fixed: Increased the local command loop scratch/status buffer sizes only far
  enough for the accepted three-stage transcript and three process-status
  records.
- fixed: Added focused unit coverage and a QEMU/substitute smoke scenario for
  `exec stdout | exec stdin | exec stdin`, explicit waitpid of all three pids,
  `cat /proc/talos/processes`, `ps`, and `cat /etc/banner.txt`.
- fixed: Refreshed `/proc/talos/processes` and `ps` QEMU/substitute regression
  transcripts after the process-status capacity increase.
- not-an-issue: The middle and final stages intentionally reuse the existing
  `/bin/stdin` fixture; it is the accepted VFS/userspace stdin/stdout fixture
  and proves byte propagation through both pipe handoffs without adding a fake
  command.
- deferred: arbitrary shell grammar, unbounded pipeline length, pipeline
  concurrency, pipefail, fork/signals, process groups/sessions, waitpid options,
  public procfs/Linux `ps` compatibility, persistent/block-backed storage, live
  networking, SSH, Pi 5 hardware proof, and phase transition.

## Evidence Map

- Classification:
  `tasks/evidence/2026-06-26-phase12-local-multistage-pipeline-core/classification.json`.
- Evidence map:
  `tasks/evidence/2026-06-26-phase12-local-multistage-pipeline-core/evidence-map.json`.
- QEMU/substitute multistage transcript:
  `tasks/evidence/2026-06-26-phase12-local-multistage-pipeline-core/qemu-local-shell-multistage-pipeline-smoke.log`.
- Process-status VFS regression transcript:
  `tasks/evidence/2026-06-26-phase12-local-process-status-vfs-core/qemu-local-shell-process-status-vfs-smoke.log`.
- VFS-backed `ps` regression transcript:
  `tasks/evidence/2026-06-26-phase12-local-ps-command-vfs-backed-core/qemu-local-shell-ps-command-vfs-smoke.log`.

## Accepted Frontier

The accepted three-stage path is exactly
`exec stdout | exec stdin | exec stdin`. The first pipe records 31 bytes
written by `/bin/stdout` and read by the middle `/bin/stdin`; the second pipe
records 68 bytes written by the middle fixture and read by the final
`/bin/stdin`. The final visible output nests the middle fixture's read output,
proving the original stdout payload crossed both pipe handoffs:

```text
Talos userspace stdin fixture read: Talos userspace stdin fixture read: Talos userspace stdout fixture
```

The bounded process table reports all three records through
`/proc/talos/processes` and zero-argument `ps`, and explicit waitpid observes
each pid exactly once.

Live network/SSH reachability remains paused. No Pi 5 hardware claim is made.

## Deferred Frontier

Deferred surfaces remain arbitrary shell grammar, unbounded or arbitrary-length
pipelines, concurrent pipeline scheduling, pipefail, fork/signals, process
groups/sessions, waitpid options, public procfs/Linux `ps` compatibility,
persistent/block-backed storage, live networking, SSH, Pi 5 hardware proof,
generated-root command-input retry, and phase transition.

## Validation

- passed: `cargo fmt --all -- --check`
- passed:
  `cargo -Zjson-target-spec test --quiet local_command_loop_runs_three_stage_pipeline_through_bounded_vfs_processes`
- passed: `cargo -Zjson-target-spec test --quiet local_command_loop` with 832
  no_std tests reported.
- passed: `./scripts/qemu-local-shell-multistage-pipeline-smoke.sh` with
  `participants=10 expected=10 errors=0` and classification
  `qemu-local-shell-multistage-pipeline-complete`.
- passed: `./scripts/qemu-local-shell-process-status-vfs-smoke.sh` with
  `participants=19 expected=19 errors=0`.
- passed: `./scripts/qemu-local-shell-ps-command-vfs-smoke.sh` with
  `participants=23 expected=23 errors=0`.
- passed: `jq empty` on task-owned JSON evidence.
- passed: `git diff --check`.
- passed: `/home/node/.cargo/bin/mdbook build`.
- passed: `git diff --cached --check`.

## Result

selected_next_task: phase12-local-multistage-pipeline-closeout-20260626.

The accepted work remains local QEMU/substitute evidence only and does not
authorize live network/SSH work, hardware/lab action, fake command expansion,
or a phase transition.
