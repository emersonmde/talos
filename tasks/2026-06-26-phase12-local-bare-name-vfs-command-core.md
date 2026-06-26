# Phase 12 Local Bare-Name VFS Command Core

Task id: phase12-local-bare-name-vfs-command-core-20260626

Status: accepted; commit hash recorded in durable supervisor state after
commit.

## Scope

Implement the thinnest direct bare-name command path in the local shell:

~~~text
status42
~~~

The command resolves through a bounded /bin VFS lookup to /bin/status42 and
then executes through the accepted VFS open/read, program-loader, initial user
stack, userspace launch/status, and bounded process-table path.

This task does not accept POSIX PATH environment compatibility, PATH mutation,
command hashing/cache semantics, shell functions, aliases, globbing, quoting,
argument parsing for direct bare commands, redirections for direct bare
commands, bare-name pipelines, arbitrary shell grammar, live networking/SSH,
Pi 5 hardware proof, generated-root command-input retry, or a phase transition.

## Findings

- fixed: Added a direct bare-name dispatcher for the bounded /bin executable
  surface, reusing the accepted fixed-bin path resolver and VFS exec path.
- fixed: status42 now launches without the exec prefix and without an absolute
  path while still opening and reading /bin/status42 from VFS.
- fixed: waitpid, laststatus, /proc/talos/processes, zero-argument ps, and
  pipestatus/process-status observations remain consistent with accepted
  direct absolute-path /bin/status42 behavior.
- fixed: Unsupported bare arguments/redirections and still-deferred bare
  pipeline forms fail closed without successful process records.
- fixed: Relative names with slashes such as bin/status42 remain
  unknown-command; non-executable and missing absolute-path controls remain
  fail-closed.
- fixed: Added focused unit coverage and a retained QEMU/substitute transcript
  for direct bare-name command execution.
- not-an-issue: Existing exec-prefixed direct VFS exec, direct absolute-path
  commands, descriptor-backed cat/open/read, exact pipelines, multistage
  pipeline, jobs/waitpid, /proc/talos/processes, ps, pipestatus, and
  redirection surfaces remain regression surfaces rather than new claims.
- deferred: POSIX PATH environment compatibility, command lookup beyond the
  bounded /bin surface, bare-name arguments/redirections, bare-name pipelines,
  arbitrary shell grammar, unbounded pipelines, scheduler concurrency,
  fork/signals, process groups/sessions, persistent storage, live networking/
  SSH, Pi 5 hardware proof, generated-root command-input retry, and phase
  transition.

## Evidence Map

- Classification and evidence JSON:
  tasks/evidence/2026-06-26-phase12-local-bare-name-vfs-command-core/classification.json
  and
  tasks/evidence/2026-06-26-phase12-local-bare-name-vfs-command-core/evidence-map.json.
- QEMU/substitute transcript:
  tasks/evidence/2026-06-26-phase12-local-bare-name-vfs-command-core/qemu-local-shell-bare-name-vfs-command-smoke.log.
- Implementation and smoke harness:
  src/local_command_loop.rs, src/target/qemu_virt.rs,
  scripts/qemu-local-serial-command-loop-smoke.sh, and
  scripts/qemu-local-shell-bare-name-vfs-command-smoke.sh.

## Accepted Frontier

The local shell accepts direct bare-name command execution for the bounded /bin
lookup surface. The accepted proof case is status42 resolving to /bin/status42.

The command is loaded through VFS open/read and the existing program loader,
receives canonical argv0 /bin/status42, exits with status 0x2a, records the
same bounded lifecycle/process-table state as /bin/status42 and
exec /bin/status42, and remains observable through waitpid, laststatus,
/proc/talos/processes, zero-argument ps, and pipestatus/process-status
surfaces.

Live network/SSH reachability remains paused. No Pi 5 hardware claim is made.

## Deferred Frontier

Deferred surfaces remain POSIX PATH environment compatibility, PATH mutation,
command lookup beyond the bounded /bin surface, direct bare arguments or
redirections, bare-name pipelines, arbitrary shell grammar, unbounded
pipelines, pipeline concurrency, scheduler-concurrent execution, fork/signals,
process groups/sessions, broad procfs/Linux ps, PID policy expansion, waitpid
options, persistent storage, live networking/SSH, Pi 5 hardware proof,
generated-root command-input retry, and phase transition.

## Validation

- cargo fmt --all -- --check: passed after formatting.
- cargo -Zjson-target-spec test --quiet local_command_loop: passed; 839 talos
  no_std tests.
- scripts/qemu-local-shell-bare-name-vfs-command-smoke.sh: passed.
- scripts/qemu-local-shell-vfs-exec-smoke.sh: passed.
- scripts/qemu-local-shell-process-status-vfs-smoke.sh: passed.
- scripts/qemu-local-shell-pipeline-status-smoke.sh: passed.
- scripts/qemu-local-shell-absolute-path-vfs-pipeline-smoke.sh: passed.
- jq empty on task-owned JSON evidence: passed.
- git diff --check: passed.
- /home/node/.cargo/bin/mdbook build: passed; mdBook HTML emitted with the
  existing large-search-index warning.
- git diff --cached --check: passed before commit.

## Result

selected_next_task: phase12-local-bare-name-vfs-pipeline-core-20260626.

The bounded bare-name pipeline task is mechanically unblocked after this
accepted core task is committed, provided the hardware lock remains
restored/unlocked and supervisor intervention remains inactive.
