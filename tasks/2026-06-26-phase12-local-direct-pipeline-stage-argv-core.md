# Phase 12 Local Direct Pipeline Stage Argv Core

Task id: phase12-local-direct-pipeline-stage-argv-core-20260626

Status: accepted; commit hash recorded in durable supervisor state after
commit.

## Scope

Implement the thinnest direct path-form two-stage pipeline argv path:

~~~text
/bin/stdout alpha | /bin/stdin beta
~~~

Both stages remain absolute VFS executable paths. Each stage opens and reads
its executable through descriptor-backed VFS, launches through the accepted
userspace startup/status path, and uses the existing serialized pipe descriptor
handoff. This task only accepts one bounded literal argument per direct
path-form pipeline stage.

This task does not accept bare-name pipeline argv, multistage pipeline argv,
redirections, environment-backed PATH, current-directory search, quoting,
escaping, globbing, variables, arbitrary shell grammar, pipeline concurrency,
live networking/SSH, Pi 5 hardware proof, generated-root retry, or a phase
transition.

## Findings

- fixed: Direct path-form two-stage pipelines now parse one bounded literal
  argument before and after the pipe separator.
- fixed: /bin/stdout alpha | /bin/stdin beta executes both stages through VFS
  open/read, loader, userspace startup/status, descriptor inheritance, and the
  accepted pipe handoff rather than fake command dispatch.
- fixed: Producer evidence records argc=2, argv0=/bin/stdout, argv1=alpha,
  deterministic empty envp, inherited fd0/fd2, fd1 as the pipe endpoint, a
  closed loader temporary descriptor, and a bounded process-table entry.
- fixed: Consumer evidence records argc=2, argv0=/bin/stdin, argv1=beta,
  deterministic empty envp, fd0 as the pipe endpoint, inherited fd1/fd2, a
  closed loader temporary descriptor, and a bounded process-table entry.
- fixed: Pipeline lifecycle/status, waitpid, laststatus,
  /proc/talos/processes, zero-argument ps, and pipestatus observations remain
  attached to the producer and consumer participants.
- fixed: Existing direct and bare-name command argv, no-argument direct and
  bare-name commands, direct and bare-name no-argument pipelines, multistage
  pipeline, process-status VFS, ps, and pipestatus regression smokes remain
  passing.
- fixed: Too many path-form pipeline stage arguments and unsupported literal
  characters fail closed without accepted process records.
- fixed: The QEMU/substitute smoke harness now has a dedicated
  qemu_local_shell_direct_pipeline_stage_argv boot scenario with a 15-command
  transcript and task-owned classification.
- not-an-issue: No-argument direct path-form pipelines continue to use the
  same accepted pipeline execution path.
- deferred: Bare-name pipeline argv, multistage pipeline argv, redirections,
  environment-backed PATH, current-directory search, arbitrary shell grammar,
  unbounded pipelines, pipeline concurrency, scheduler concurrency,
  fork/signals, process groups/sessions, persistent storage, live
  networking/SSH, Pi 5 hardware proof, generated-root retry, and phase
  transition.

## Evidence Map

- Classification and evidence JSON:
  tasks/evidence/2026-06-26-phase12-local-direct-pipeline-stage-argv-core/classification.json
  and
  tasks/evidence/2026-06-26-phase12-local-direct-pipeline-stage-argv-core/evidence-map.json.
- QEMU/substitute transcript:
  tasks/evidence/2026-06-26-phase12-local-direct-pipeline-stage-argv-core/qemu-local-shell-direct-pipeline-stage-argv-smoke.log.
- Implementation and smoke harness:
  build.rs, src/main.rs, src/local_command_loop.rs, src/target/qemu_virt.rs,
  scripts/qemu-local-serial-command-loop-smoke.sh, and
  scripts/qemu-local-shell-direct-pipeline-stage-argv-smoke.sh.
- Project notes:
  docs/src/roadmap.md, docs/src/project/phase12-networking-ssh.md, and
  docs/src/project/early-posix-shape.md.

## Accepted Frontier

The accepted direct path-form pipeline stage argv frontier is local-only and
static/unit/QEMU substitute backed. A direct path-form two-stage pipeline can
now carry one bounded literal argument per stage:

~~~text
/bin/stdout alpha | /bin/stdin beta
~~~

Both stages still execute through descriptor-backed VFS open/read, the accepted
loader, userspace launch/status, and the serialized pipe descriptor handoff.
The producer records argc=2, argv0=/bin/stdout, argv1=alpha, empty envp, fd1
as the pipe endpoint, and status 0. The consumer records argc=2,
argv0=/bin/stdin, argv1=beta, empty envp, fd0 as the pipe endpoint, and status
0. The bounded process table, waitpid, laststatus, /proc/talos/processes,
zero-argument ps, and pipestatus surfaces remain intact.

Live network/SSH reachability remains paused. No Pi 5 hardware claim is made.

## Deferred Frontier

Deferred surfaces remain bare-name pipeline argv, multistage pipeline argv,
redirections, environment-backed PATH, current-directory search, command lookup
beyond existing bounded surfaces, quoting, escaping, globbing, variables,
arbitrary shell grammar, unbounded pipelines, pipeline concurrency, scheduler
concurrency, fork/signals, process groups/sessions, broad procfs/Linux ps, PID
policy expansion, waitpid options, persistent storage, live networking/SSH,
Pi 5 hardware proof, generated-root command-input retry, and phase transition.

## Validation

- cargo fmt --all -- --check: passed.
- cargo -Zjson-target-spec test --quiet local_command_loop: passed.
- QEMU/substitute direct pipeline stage argv smoke using
  scripts/qemu-local-shell-direct-pipeline-stage-argv-smoke.sh with task-owned
  evidence paths: passed.
- scripts/qemu-local-shell-absolute-path-vfs-command-smoke.sh: passed.
- scripts/qemu-local-shell-bare-name-command-argv-smoke.sh: passed.
- scripts/qemu-local-shell-absolute-path-vfs-pipeline-smoke.sh: passed.
- scripts/qemu-local-shell-bare-name-vfs-pipeline-smoke.sh: passed.
- scripts/qemu-local-shell-multistage-pipeline-smoke.sh: passed.
- scripts/qemu-local-shell-process-status-vfs-smoke.sh: passed.
- scripts/qemu-local-shell-ps-command-vfs-smoke.sh: passed.
- scripts/qemu-local-shell-pipeline-status-smoke.sh: passed.
- jq empty on task-owned JSON evidence: passed.
- git diff --check: passed.
- /home/node/.cargo/bin/mdbook build: passed.
- git diff --cached --check: passed before commit.

## Result

selected_next_task: phase12-local-direct-pipeline-stage-argv-closeout-20260626.

The direct pipeline stage argv closeout is mechanically unblocked after this
accepted core task is committed, provided the hardware lock remains
restored/unlocked and supervisor intervention remains inactive.
