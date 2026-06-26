# Phase 12 Local Absolute-Path VFS Command Core

Task id: phase12-local-absolute-path-vfs-command-core-20260626

Status: accepted; commit hash recorded in durable supervisor state after
commit.

## Scope

Implement the thinnest direct absolute-path command path in the local shell:
/bin/status42 dispatches through the existing VFS open/read, program-loader,
initial user stack, and userspace launch path without requiring the diagnostic
exec prefix.

This task does not accept PATH lookup, bare command lookup, path-form
pipelines, arbitrary shell grammar, direct path arguments/redirections,
networking, SSH, Pi 5 hardware proof, or a phase transition.

## Findings

- fixed: Added a direct absolute-path dispatcher that recognizes shell command
  names beginning with / and hands them to the accepted VFS exec path.
- fixed: /bin/status42 now launches without the exec prefix and preserves the
  accepted lifecycle/status, waitpid, laststatus, /proc/talos/processes, and
  zero-argument ps observations.
- fixed: Unsupported direct paths fail closed: /missing is not found, /bin and
  /etc/banner.txt are not executable, and direct path arguments are invalid.
- fixed: Relative path and bare command controls, bin/status42 and status42,
  remain unknown-command; no PATH or bare-name lookup is accepted.
- fixed: Added focused unit coverage and a retained QEMU/substitute transcript
  for direct absolute-path execution and negative controls.
- deferred: PATH lookup, bare command lookup, direct absolute-path arguments,
  redirections, path-form pipelines, arbitrary shell grammar, unbounded
  pipelines, scheduler concurrency, fork/signals, process groups/sessions,
  live networking/SSH, Pi 5 hardware proof, and phase transition.

## Evidence Map

- Classification and evidence JSON:
  tasks/evidence/2026-06-26-phase12-local-absolute-path-vfs-command-core/classification.json
  and
  tasks/evidence/2026-06-26-phase12-local-absolute-path-vfs-command-core/evidence-map.json.
- QEMU/substitute transcript:
  tasks/evidence/2026-06-26-phase12-local-absolute-path-vfs-command-core/qemu-local-shell-absolute-path-vfs-command-smoke.log.
- Implementation and smoke harness:
  src/local_command_loop.rs, src/target/qemu_virt.rs, src/main.rs, build.rs,
  scripts/qemu-local-serial-command-loop-smoke.sh, and
  scripts/qemu-local-shell-absolute-path-vfs-command-smoke.sh.

## Accepted Frontier

The local shell accepts exactly direct absolute-path command execution for the
supported executable VFS fixtures covered by the existing VFS exec path. The
accepted proof case is /bin/status42.

The command is loaded through VFS open/read and the existing program loader,
receives argv0 /bin/status42, exits with status 0x2a, records the same bounded
lifecycle/process-table state as exec /bin/status42, and remains observable
through waitpid, laststatus, /proc/talos/processes, and zero-argument ps.

Live network/SSH reachability remains paused. No Pi 5 hardware claim is made.

## Deferred Frontier

Deferred surfaces remain PATH lookup, bare command lookup, direct
absolute-path arguments/redirections, path-form pipelines, arbitrary shell
grammar, unbounded pipelines, pipeline concurrency, scheduler-concurrent
execution, fork/signals, process groups/sessions, broad procfs/Linux ps, PID
policy expansion, waitpid options, persistent storage, live networking/SSH,
Pi 5 hardware proof, generated-root command-input retry, and phase transition.

## Validation

- cargo fmt --all -- --check: passed.
- cargo -Zjson-target-spec test --quiet local_command_loop: passed.
- scripts/qemu-local-shell-absolute-path-vfs-command-smoke.sh: passed.
- jq empty on task-owned JSON evidence: passed.
- git diff --check: passed.
- /home/node/.cargo/bin/mdbook build: passed; mdBook HTML emitted with the
  existing large-search-index warning.
- git diff --cached --check: passed.

## Result

selected_next_task: phase12-local-absolute-path-vfs-command-closeout-20260626.

The closeout task is mechanically unblocked after this accepted core task is
committed, provided the hardware lock remains restored/unlocked and supervisor
intervention remains inactive.
