# Phase 12 Local Process Status VFS Core

- Task: `phase12-local-process-status-vfs-core-20260626`
- Status: accepted
- Classification: `local-process-status-vfs-core-accepted`
- Commit: recorded in supervisor state after accepted commit

## Scope

This task accepts the first narrow, Talos-private process-status VFS surface: `/proc/talos/processes`.

The file is read-only, versioned as `talos-processes-v1`, and consumed through the existing descriptor-backed `cat`/open/read path. It reports only the accepted bounded local process-table records for direct foreground VFS exec, exact two-stage pipeline producer/consumer records, and accepted background job fixtures.

## Findings

- fixed: Added `/proc/talos/processes` as a Talos-private VFS path routed through descriptor-backed local `cat`/read instead of adding a fake shell command.
- fixed: Added deterministic `talos-processes-v1` records with `slot`, `capacity`, `pid`, `parent`, `owner`, `path`, `state`, `status`, `observed-status`, `reaped`, `wait-consumed`, `job-state`, and `source` fields.
- fixed: Direct `/bin/status42`, exact `/bin/stdout | /bin/stdin` pipeline records, and background `/bin/status42` records are visible through `cat /proc/talos/processes`.
- fixed: Explicit `waitpid` consumption changes the process-status view from `wait-consumed=false` to `wait-consumed=true` for accepted direct, pipeline, and background fixtures.
- fixed: Unsupported `/proc/talos` fails closed with the accepted command error path, while existing non-proc VFS `cat /etc/banner.txt` behavior remains accepted.
- fixed: Added a dedicated QEMU/substitute process-status VFS smoke wrapper and boot scenario so the evidence is task-owned and labeled.
- not-an-issue: The schema is intentionally Talos-private and versioned; it is not Linux procfs compatibility or a stable public process enumeration ABI.
- deferred: `ps`, `/proc/self`, `/proc/<pid>`, mutable procfs, public process enumeration ABI, scheduler-concurrent execution, fork/signals, process groups/sessions, waitpid options, PID reuse policy, multi-stage pipelines, pipefail, persistent storage, live networking, SSH, Pi 5 hardware proof, and phase transition.

## Schema

```text
talos-processes-v1
slot=<decimal> capacity=<decimal> pid=<fixed-hex> parent=shell owner=<fixed-hex> path=<absolute-vfs-path> state=<state> status=<fixed-hex> observed-status=<fixed-hex> reaped=<bool> wait-consumed=<bool> job-state=<foreground|running|completed> source=bounded-process-table
```

The file intentionally has no Linux procfs compatibility promise. Field names and values are chosen to support the next local POSIX/VFS work without pretending to expose a complete process model.

## Evidence

- static inspection: `src/local_command_loop.rs` adds the process-status VFS file, schema writer, descriptor-backed read helper, exact `cat /proc/talos/processes` routing, unsupported `/proc` negative control, and focused unit tests.
- static inspection: `src/target/qemu_virt.rs`, `build.rs`, and `scripts/qemu-local-serial-command-loop-smoke.sh` add the dedicated `qemu_local_shell_process_status_vfs` scenario and expected command dispatch.
- QEMU/substitute: `./scripts/qemu-local-shell-process-status-vfs-smoke.sh` passed with `participants=19 expected=19 errors=0` and classification `qemu-local-shell-process-status-vfs-complete`.
- QEMU/substitute transcript: `tasks/evidence/2026-06-26-phase12-local-process-status-vfs-core/qemu-local-shell-process-status-vfs-smoke.log`.
- evidence map: `tasks/evidence/2026-06-26-phase12-local-process-status-vfs-core/evidence-map.json`.
- classification: `tasks/evidence/2026-06-26-phase12-local-process-status-vfs-core/classification.json`.

## Validation

- passed: `cargo fmt --all -- --check`
- passed: `cargo -Zjson-target-spec test --quiet local_command_loop_cats_proc_talos_processes`
- passed: `./scripts/qemu-local-shell-process-status-vfs-smoke.sh`
- passed: `cargo -Zjson-target-spec test --quiet local_command_loop`
- passed: `cargo -Zjson-target-spec test --quiet`
- passed: `jq empty tasks/evidence/2026-06-26-phase12-local-process-status-vfs-core/classification.json tasks/evidence/2026-06-26-phase12-local-process-status-vfs-core/evidence-map.json`
- passed: `git diff --check`
- passed: `/home/node/.cargo/bin/mdbook build`
- pending: `git diff --cached --check`

## Result

`/proc/talos/processes` is accepted as the first local process-status VFS view. Live network/SSH reachability remains paused, no Pi 5 hardware claim is made, and this task does not accept a phase transition.
