# Phase 12 Local Bare-Name Command Argv Core

Task id: phase12-local-bare-name-command-argv-core-20260626

Status: accepted; commit hash recorded in durable supervisor state after
commit.

## Scope

Implement the thinnest bare-name command argv path in the local shell:

~~~text
status42 alpha beta
~~~

The command resolves only through the accepted bounded fixed /bin lookup to
/bin/status42. It then reaches /bin/status42 through descriptor-backed VFS
open/read, the accepted loader, userspace launch/status, and bounded
process-table observations. The new surface is only a small literal argv
vector on the accepted direct bare-name command form.

This task does not accept pipeline stage argv, redirections, environment-backed
PATH, current-directory search, command lookup beyond the bounded /bin surface,
quoting, escaping, globbing, variables, arbitrary shell grammar, live
networking/SSH, Pi 5 hardware proof, generated-root retry, or a phase
transition.

## Findings

- fixed: Direct bare-name commands now preserve a bounded literal argv vector
  instead of rejecting all arguments.
- fixed: status42 alpha beta resolves through the fixed /bin lookup to
  /bin/status42 and records argc=3, argv0=/bin/status42, argv1=alpha,
  argv2=beta, deterministic empty envp, inherited fd0/fd1/fd2, closed loader
  temporary descriptor, and status 0x2a through the accepted userspace startup
  path.
- fixed: Existing no-argument bare-name command behavior, bare-name pipeline
  behavior, direct absolute-path argv, exec-prefixed literal argv,
  process-status VFS, ps, and pipestatus regression surfaces remain passing.
- fixed: Too many bare-name literal arguments, unsupported literal characters,
  and unsupported bare commands fail closed without accepted process records.
- fixed: The QEMU/substitute smoke harness now has a dedicated
  qemu_local_shell_bare_name_command_argv boot scenario with a 13-command
  transcript and task-owned classification.
- not-an-issue: Bare-name pipelines still use the existing no-argv stage parser
  and remain a separate bounded surface.
- deferred: Pipeline stage argv, redirections, PATH compatibility,
  current-directory search, command lookup beyond existing bounded surfaces,
  arbitrary shell grammar, unbounded pipelines, scheduler concurrency,
  fork/signals, process groups/sessions, persistent storage, live
  networking/SSH, Pi 5 hardware proof, generated-root retry, and phase
  transition.

## Evidence Map

- Classification and evidence JSON:
  tasks/evidence/2026-06-26-phase12-local-bare-name-command-argv-core/classification.json
  and
  tasks/evidence/2026-06-26-phase12-local-bare-name-command-argv-core/evidence-map.json.
- QEMU/substitute transcript:
  tasks/evidence/2026-06-26-phase12-local-bare-name-command-argv-core/qemu-local-shell-bare-name-command-argv-smoke.log.
- Implementation and smoke harness:
  build.rs, src/local_command_loop.rs, src/target/qemu_virt.rs,
  scripts/qemu-local-serial-command-loop-smoke.sh, and
  scripts/qemu-local-shell-bare-name-command-argv-smoke.sh.
- Project notes:
  docs/src/roadmap.md, docs/src/project/phase12-networking-ssh.md, and
  docs/src/project/early-posix-shape.md.

## Accepted Frontier

The accepted command argv frontier is local-only and static/unit/QEMU
substitute backed. A direct bare-name command can now carry a small literal argv
vector:

~~~text
status42 alpha beta
~~~

The shell resolves the command name only through the fixed /bin lookup, then
builds the same canonical resolved-path startup ABI already accepted for
exec-prefixed and direct path argv: argc=3, argv0=/bin/status42, argv1=alpha,
argv2=beta, empty envp, standard descriptor inheritance, and a closed loader
temporary descriptor. The executable still comes from VFS open/read and exits
with status 0x2a. waitpid, laststatus, /proc/talos/processes, zero-argument ps,
and existing pipestatus regression surfaces remain intact.

Live network/SSH reachability remains paused. No Pi 5 hardware claim is made.

## Deferred Frontier

Deferred surfaces remain pipeline stage argv, redirections, environment-backed
PATH, current-directory search, command lookup beyond existing bounded
surfaces, quoting, escaping, globbing, variables, arbitrary shell grammar,
unbounded pipelines, pipeline concurrency, scheduler concurrency, fork/signals,
process groups/sessions, broad procfs/Linux ps, PID policy expansion, waitpid
options, persistent storage, live networking/SSH, Pi 5 hardware proof,
generated-root command-input retry, and phase transition.

## Validation

- cargo fmt --all -- --check: passed.
- cargo -Zjson-target-spec test --quiet local_command_loop: passed.
- QEMU/substitute bare-name command argv smoke using
  scripts/qemu-local-shell-bare-name-command-argv-smoke.sh with task-owned
  evidence paths: passed.
- scripts/qemu-local-shell-bare-name-vfs-command-smoke.sh: passed; retained
  log shows qemu-local-shell-absolute-path-vfs-command-complete and includes
  status42 no-argument bare-name regression.
- scripts/qemu-local-shell-bare-name-vfs-pipeline-smoke.sh: passed.
- scripts/qemu-local-shell-absolute-path-vfs-command-smoke.sh: passed.
- scripts/qemu-local-shell-literal-argv-smoke.sh: passed.
- scripts/qemu-local-shell-process-status-vfs-smoke.sh: passed.
- scripts/qemu-local-shell-ps-command-vfs-smoke.sh: passed.
- scripts/qemu-local-shell-pipeline-status-smoke.sh: passed.
- jq empty on task-owned JSON evidence: passed.
- git diff --check: passed.
- /home/node/.cargo/bin/mdbook build: passed.
- git diff --cached --check: passed before commit.

## Result

selected_next_task: phase12-local-command-argv-frontier-checkpoint-20260626.

The command argv frontier checkpoint is mechanically unblocked after this
accepted core task is committed, provided the hardware lock remains
restored/unlocked and supervisor intervention remains inactive.
