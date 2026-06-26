# Phase 12 Local Absolute-Path VFS Pipeline Closeout

Task id: phase12-local-absolute-path-vfs-pipeline-closeout-20260626

Status: accepted; commit hash recorded in durable supervisor state after
commit.

## Scope

Reconcile the accepted bounded path-form pipeline:

~~~text
/bin/stdout | /bin/stdin
~~~

against the retained direct absolute-path command, exec-prefixed pipeline,
multistage, process-table, waitpid, /proc/talos/processes, ps, and pipestatus
evidence. This closeout makes no implementation change beyond task/evidence/docs
reconciliation.

## Findings

- fixed: Mapped the accepted bounded path-form pipeline behavior to the core
  task record, classification JSON, evidence map, and retained QEMU/substitute
  transcript.
- fixed: Reconciled the path-form pipeline with the direct absolute-path
  command frontier and the accepted exec-prefixed two-stage/multistage pipeline
  evidence.
- fixed: Preserved the process-table, waitpid, /proc/talos/processes,
  zero-argument ps, and pipestatus observations as the source of truth for
  accepted pipeline participant status.
- fixed: Recorded that the compact local path-command frontier checkpoint is
  the next mechanically objective task after this closeout.
- removed: No stale implication that path-form pipelines require the old
  diagnostic exec prefix remains in the accepted frontier summary.
- not-an-issue: Existing direct path-command, exec-prefixed direct/pipeline,
  descriptor-backed VFS file I/O, process-status VFS, ps, pipestatus, and
  redirection evidence remains retained regression evidence, not new behavior
  accepted by this closeout.
- deferred: PATH lookup, bare-name lookup, arbitrary shell grammar, unbounded
  pipelines, pipeline concurrency, scheduler concurrency, fork/signals, process
  groups/sessions, live networking/SSH, Pi 5 hardware proof, generated root
  retry, and phase transition.

## Evidence Map

- Core task record:
  tasks/2026-06-26-phase12-local-absolute-path-vfs-pipeline-core.md.
- Core classification and evidence JSON:
  tasks/evidence/2026-06-26-phase12-local-absolute-path-vfs-pipeline-core/classification.json
  and
  tasks/evidence/2026-06-26-phase12-local-absolute-path-vfs-pipeline-core/evidence-map.json.
- Core QEMU/substitute transcript:
  tasks/evidence/2026-06-26-phase12-local-absolute-path-vfs-pipeline-core/qemu-local-shell-absolute-path-vfs-pipeline-smoke.log.
- Closeout classification and evidence JSON:
  tasks/evidence/2026-06-26-phase12-local-absolute-path-vfs-pipeline-closeout/classification.json
  and
  tasks/evidence/2026-06-26-phase12-local-absolute-path-vfs-pipeline-closeout/evidence-map.json.
- Updated project docs:
  docs/src/roadmap.md and docs/src/project/phase12-networking-ssh.md.

## Accepted Frontier

The accepted local shell frontier now includes the bounded two-stage path-form
pipeline /bin/stdout | /bin/stdin. Both stages are absolute VFS paths loaded
through descriptor-backed VFS open/read and the accepted program-loader and
userspace launch path. The evidence remains static/unit/QEMU-substitute only.

The retained transcript proves serialized byte flow, lifecycle/status records
for producer and consumer, explicit waitpid observations, laststatus for the
final stage, /proc/talos/processes, zero-argument ps, and pipestatus
participant/status reporting. The same accepted process-table sources of truth
also preserve direct absolute-path command and exec-prefixed pipeline regression
coverage.

Live network/SSH reachability remains paused. No Pi 5 hardware claim is made.

## Deferred Frontier

Deferred surfaces remain PATH lookup, bare-name lookup, path-form arguments or
redirections, mixed diagnostic/path forms beyond the accepted fail-closed
controls, path-form multistage pipelines, arbitrary shell grammar, unbounded
pipelines, pipeline concurrency, scheduler concurrency, fork/signals, process
groups/sessions, broad procfs/Linux ps compatibility, persistent storage, live
networking/SSH, Pi 5 hardware proof, generated-root command-input retry, and
phase transition.

## Validation

- static inspection of task records, retained evidence paths, docs, and git
  diff: passed.
- jq empty on task-owned JSON evidence: passed.
- git diff --check: passed.
- /home/node/.cargo/bin/mdbook build: passed; mdBook HTML emitted with the
  existing large-search-index warning.
- git diff --cached --check: passed.

## Result

selected_next_task: phase12-local-path-command-frontier-checkpoint-20260626.

The checkpoint task is mechanically unblocked after this accepted closeout is
committed, provided the hardware lock remains restored/unlocked and supervisor
intervention remains inactive.
