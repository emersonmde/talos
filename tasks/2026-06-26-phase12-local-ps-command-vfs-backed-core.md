# Phase 12 Local Ps Command VFS-Backed Core

Task id: phase12-local-ps-command-vfs-backed-core-20260626

Status: accepted; commit hash recorded in durable supervisor state after commit.

## Scope

Add a bounded zero-argument `ps` shell view only as a presentation of the
accepted Talos-private `/proc/talos/processes` VFS status file.

The command remains local QEMU/substitute work. It does not resume live
networking, SSH, Pi 5 hardware proof, generated-root retries, or phase
transition work.

## Findings

- fixed: Added `ps` to the bounded local shell inventory and status boundary.
- fixed: `ps` rejects any argument or option with the existing
  `talos: unexpected-argument` path.
- fixed: zero-argument `ps` calls `write_initramfs_text_file` with
  `/proc/talos/processes`, which reaches the accepted descriptor-backed
  `read_process_status_file_via_descriptor` helper instead of directly dumping
  the process table.
- fixed: Added the task-owned `qemu_local_shell_ps_command_vfs` boot scenario,
  wrapper script, transcript, and classification for direct, exact pipeline,
  background, `cat /proc/talos/processes`, and unsupported-argument controls.
- fixed: Updated roadmap, Phase 12 project notes, and early POSIX guardrails to
  record `ps` as Talos-private process status only.
- not-an-issue: The `ps` output currently matches the accepted
  `talos-processes-v1` status-file text exactly; the command is intentionally a
  thin view rather than a Linux-format process table.
- deferred: Linux `ps` compatibility, Linux procfs compatibility, `/proc/self`,
  `/proc/<pid>`, public process enumeration ABI, `ps` arguments/options,
  sorting/filtering, scheduler-concurrent execution, fork/signals, process
  groups/sessions, waitpid options, PID reuse policy, multi-stage pipelines,
  pipefail, persistent storage, live networking, SSH, Pi 5 hardware proof, and
  phase transition.

## VFS Backing

The accepted data path is:

```text
dispatch_local_command("ps")
  -> write_initramfs_text_file(LOCAL_COMMAND_PROC_TALOS_PROCESSES_PATH)
  -> LocalCommandSink::read_initramfs_file_via_syscall
  -> read_process_status_file_via_descriptor
  -> process_status_file_bytes / talos-processes-v1
```

That is the same backing surface used by `cat /proc/talos/processes`. A direct
process-table dump bypassing the VFS status file remains rejected.

## Evidence Map

- Classification:
  `tasks/evidence/2026-06-26-phase12-local-ps-command-vfs-backed-core/classification.json`.
- Evidence map:
  `tasks/evidence/2026-06-26-phase12-local-ps-command-vfs-backed-core/evidence-map.json`.
- QEMU/substitute `ps` transcript:
  `tasks/evidence/2026-06-26-phase12-local-ps-command-vfs-backed-core/qemu-local-shell-ps-command-vfs-smoke.log`.
- Process-status VFS regression transcript:
  `tasks/evidence/2026-06-26-phase12-local-process-status-vfs-core/qemu-local-shell-process-status-vfs-smoke.log`.

## Accepted Frontier

`ps` is accepted only as a zero-argument Talos-private shell view over
`/proc/talos/processes`. It reports the accepted bounded process-table records
for direct foreground VFS exec, exact two-stage pipeline producer/consumer
records, and accepted background jobs with the same `talos-processes-v1` fields
as the VFS file.

Live network/SSH reachability remains paused. No Pi 5 hardware claim is made.

## Deferred Frontier

Deferred surfaces remain Linux `ps` compatibility, Linux procfs compatibility,
`/proc/self`, `/proc/<pid>`, public process enumeration ABI, `ps`
arguments/options, sorting/filtering, scheduler concurrency, fork/signals,
process groups/sessions, waitpid options, PID reuse policy, multi-stage
pipelines, pipefail, persistent storage, live networking, SSH, Pi 5 hardware
proof, and phase transition.

## Validation

- passed: `cargo fmt --all -- --check`
- passed:
  `cargo -Zjson-target-spec test --quiet local_command_loop_ps_reads_proc_talos_processes_status_file`
- passed: `cargo -Zjson-target-spec test --quiet local_command_loop` with 831
  no_std tests reported.
- passed: `./scripts/qemu-local-shell-ps-command-vfs-smoke.sh --quiet` with
  `participants=23 expected=23 errors=0` and classification
  `qemu-local-shell-ps-command-vfs-complete`.
- passed: `./scripts/qemu-local-shell-process-status-vfs-smoke.sh --quiet`
  with `participants=19 expected=19 errors=0` and classification
  `qemu-local-shell-process-status-vfs-complete`.
- passed: `jq empty` on task-owned JSON evidence.
- passed: `git diff --check`.
- passed: `/home/node/.cargo/bin/mdbook build` with existing large
  search-index warning.
- passed: `git diff --cached --check`.

## Result

`ps` is accepted as a VFS-backed presentation layer over the accepted
`/proc/talos/processes` surface. It does not authorize fake command expansion,
Linux `ps`/procfs compatibility, hardware work, live networking, SSH, or a
phase transition.
