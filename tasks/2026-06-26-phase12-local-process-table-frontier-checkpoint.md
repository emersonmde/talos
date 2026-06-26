# Phase 12 Local Process Table Frontier Checkpoint

Task id: phase12-local-process-table-frontier-checkpoint-20260626

Status: accepted; commit hash recorded in durable supervisor state after commit.

## Scope

Reconcile the accepted local process-table frontier after the direct exec,
exact pipeline, background job, explicit-pid waitpid, no-argument waitpid,
laststatus, jobs, descriptor-backed VFS, and redirection slices.

This checkpoint does not change source behavior and does not add public process
enumeration, procfs/ps, scheduler-concurrent execution, fork/signals, process
groups/sessions, waitpid options, PID reuse/zombie policy, multi-stage
pipelines, pipefail, persistent storage, live networking, SSH, Pi 5 hardware
proof, or a phase transition.

## Findings

- fixed: The accepted frontier is now reconciled in one checkpoint covering
  direct foreground VFS exec, exact two-stage pipeline producer/consumer
  records, accepted background VFS exec job records, explicit pid waitpid,
  no-argument waitpid, non-consuming laststatus, jobs accounting,
  descriptor-backed VFS controls, and redirection controls.
- fixed: The evidence map cites retained direct exec, pipeline, background,
  explicit-pid waitpid, no-argument waitpid, jobs, stale-entry,
  descriptor-backed VFS, and redirection evidence from accepted task records and
  QEMU/substitute transcripts.
- fixed: The roadmap and Phase 12 project notes record that this checkpoint is
  a static/QEMU-substitute local process-table boundary and that supervisor
  planning is needed for any next local POSIX feature.
- not-an-issue: No source change is required; the checkpoint only records the
  accepted/deferred boundary after the previous implementation and closeout
  tasks.
- deferred: Public process enumeration/procfs, scheduler-concurrent execution,
  fork/signals, process groups/sessions, waitpid options, PID reuse/zombie
  policy, multi-stage pipelines, pipefail, persistent storage, live networking,
  SSH, Pi 5 hardware proof, and phase transition.

## Evidence Map

- Direct VFS exec process-table evidence:
  tasks/2026-06-26-phase12-local-process-table-direct-vfs-exec-core.md and
  tasks/evidence/2026-06-26-phase12-local-process-table-direct-vfs-exec-core/.
- Exact pipeline and background process-table evidence:
  tasks/2026-06-26-phase12-local-process-table-pipeline-background-core.md,
  tasks/evidence/2026-06-26-phase12-local-process-table-pipeline-background-core/classification.json,
  tasks/evidence/2026-06-26-phase12-local-process-table-pipeline-background-core/evidence-map.json,
  and
  tasks/evidence/2026-06-26-phase12-local-process-table-pipeline-background-core/qemu-local-shell-waitpid-any-completed-child-smoke.log.
- Process-observation closeout evidence:
  tasks/2026-06-26-phase12-local-waitpid-process-observation-closeout.md and
  tasks/evidence/2026-06-26-phase12-local-waitpid-any-completed-child-observation-core/qemu-local-shell-waitpid-any-completed-child-smoke.log.
- Pipeline/background closeout evidence:
  tasks/2026-06-26-phase12-local-process-table-pipeline-background-closeout.md
  and
  tasks/evidence/2026-06-26-phase12-local-process-table-pipeline-background-closeout/evidence-map.json.
- Documentation evidence: docs/src/roadmap.md,
  docs/src/project/phase12-networking-ssh.md, and
  docs/src/project/early-posix-shape.md.

## Accepted Frontier

The accepted local frontier is a bounded internal process-table
lifecycle/status substrate for direct foreground VFS exec, exact two-stage
pipeline producer/consumer records, and accepted background VFS exec job
records. Shell-visible observation covers explicit-pid waitpid, no-argument
waitpid over one completed child, non-consuming laststatus, and jobs accounting
for the accepted local fixtures.

Retained evidence is static inspection, unit-test references from accepted task
records, and QEMU/substitute transcripts only. Live network/SSH reachability
remains paused. No Pi 5 hardware claim is made.

## Deferred Frontier

Deferred process surfaces remain public process enumeration/procfs,
scheduler-concurrent execution, fork/signals, process groups/sessions, waitpid
options, PID reuse/zombie policy beyond bounded deterministic controls,
multi-stage pipelines, pipefail, persistent storage, live networking, SSH, Pi 5
hardware proof, and phase transition.

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

Planning reason: no queued same-lane local POSIX/process-table follow-up exists
after this checkpoint with explicit dependencies, acceptance criteria,
validation gates, docs, and evidence requirements. Supervisor planning is
required before any next local POSIX feature or any return to live
network/SSH reachability work.
