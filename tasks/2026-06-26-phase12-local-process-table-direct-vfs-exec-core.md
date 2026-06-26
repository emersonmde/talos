# Phase 12 Local Process Table Direct VFS Exec Core

Task id: phase12-local-process-table-direct-vfs-exec-core-20260626

Status: accepted; commit hash recorded in durable supervisor state after commit.

## Scope

Implement the thinnest kernel-owned process-table substrate for direct
foreground VFS exec records. The accepted surface is limited to fixed initramfs
fixtures /bin/init, /bin/zero, and /bin/status42.

No live Ethernet/TCP/SSH work, Pi 5 hardware action, fake command expansion,
pipeline/background process-table migration, broad process enumeration, procfs,
scheduler concurrency, fork/signals, process groups/sessions, or phase
transition is included.

## Findings

- fixed: Direct foreground VFS exec now records a bounded internal
  LocalCommandProcessTableRecord for /bin/init, /bin/zero, and /bin/status42.
- fixed: The process-table record captures stable pid 0x100001, shell parent,
  owner 0x1, path, exited state, status, observed-status, and reaped state.
- fixed: Focused tests prove no-argument waitpid consumes the direct waitable
  lifecycle record while the process-table record remains available for
  non-consuming laststatus continuity.
- fixed: Rejected exec targets and rejected literal argv syntax do not create
  successful process-table records.
- not-an-issue: Existing shell-visible waitpid source labels remain
  lifecycle-record for compatibility with accepted transcripts; the
  process-table substrate is verified by unit inspection in this task.
- deferred: Exact pipeline and background jobs remain on their previously
  accepted record sources for the queued pipeline/background process-table
  integration task.
- deferred: Broad process table APIs, public enumeration, procfs/ps, true
  scheduler concurrency, fork/signals, process groups/sessions, waitpid
  options, PID reuse/zombie policy, live networking, SSH, Pi 5 hardware proof,
  and phase transition.

## Evidence

- Static inspection: src/local_command_loop.rs adds the bounded record type,
  storage slot, direct record creation, and focused tests without changing
  accepted shell-visible output.
- Unit tests: cargo test -Zjson-target-spec process_table --quiet passed.
- Unit regression: cargo test -Zjson-target-spec local_command_loop --quiet
  passed.
- QEMU/substitute: ./scripts/qemu-local-shell-vfs-exec-smoke.sh passed and
  retained tasks/evidence/2026-06-26-phase12-local-process-table-direct-vfs-exec-core/qemu-local-shell-vfs-exec-smoke.log.
- QEMU/substitute: ./scripts/qemu-local-shell-waitpid-lifecycle-smoke.sh
  passed and retained
  tasks/evidence/2026-06-26-phase12-local-process-table-direct-vfs-exec-core/qemu-local-shell-waitpid-lifecycle-smoke.log.

## Validation

- cargo fmt --all -- --check: passed.
- jq -e empty on task-owned JSON evidence: passed.
- git diff --check: passed.
- /home/node/.cargo/bin/mdbook build: passed.
- git diff --cached --check: pending pre-commit gate.

## Result

Accepted frontier: bounded internal direct foreground VFS exec process-table
lifecycle/status record only.

Selected next task after commit: phase12-local-process-table-direct-vfs-exec-closeout-20260626.

Live network/SSH reachability remains paused. No Pi 5 hardware claim is made.
