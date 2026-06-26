# Phase 12 Local Bare-Name VFS Pipeline Core

Task id: phase12-local-bare-name-vfs-pipeline-core-20260626

Status: accepted; commit hash recorded in durable supervisor state after
commit.

## Scope

Implement the thinnest two-stage bare-name pipeline in the local shell:

~~~text
stdout | stdin
~~~

Each stage resolves through the bounded /bin VFS lookup to /bin/stdout and
/bin/stdin, then executes through the accepted VFS open/read, program-loader,
userspace launch/status, descriptor-backed pipe handoff, and bounded
process-table path.

This task does not accept POSIX PATH environment compatibility, lookup beyond
the bounded /bin surface, mixed bare/path/exec pipeline forms, bare-name
arguments or redirections, arbitrary shell grammar, unbounded pipelines,
pipeline concurrency, live networking/SSH, Pi 5 hardware proof,
generated-root command-input retry, or a phase transition.

## Findings

- fixed: Added a bounded bare-name pipeline dispatcher for exactly one
  two-stage /bin lookup pipeline.
- fixed: stdout | stdin now launches both stages through VFS open/read and
  the accepted loader/userspace launch path rather than a fake shell shortcut.
- fixed: Pipeline byte flow, waitpid, laststatus, /proc/talos/processes,
  zero-argument ps, and pipestatus remain consistent with accepted
  exec-prefixed and absolute-path pipeline evidence.
- fixed: Mixed path/bare forms, unsupported stage names, bare pipeline
  arguments/redirections, and bare-name multistage pipelines fail closed
  without successful process records.
- fixed: Added focused unit coverage, QEMU/substitute scenario registration,
  and a retained QEMU/substitute transcript for the accepted bare-name
  pipeline.
- not-an-issue: Existing bare-name direct command, direct absolute-path
  command, exec-prefixed direct/pipeline/multistage, descriptor-backed VFS
  file I/O, waitpid/jobs, process-status VFS, ps, pipestatus, and redirection
  surfaces remain regression surfaces rather than new claims.
- deferred: POSIX PATH environment compatibility, command lookup beyond the
  bounded /bin surface, mixed-form pipeline compatibility, path-form
  arguments/redirections, arbitrary shell grammar, unbounded pipelines,
  pipeline concurrency, scheduler-concurrent execution, fork/signals, process
  groups/sessions, persistent storage, live networking/SSH, Pi 5 hardware
  proof, generated-root command-input retry, and phase transition.

## Evidence Map

- Classification and evidence JSON:
  tasks/evidence/2026-06-26-phase12-local-bare-name-vfs-pipeline-core/classification.json
  and
  tasks/evidence/2026-06-26-phase12-local-bare-name-vfs-pipeline-core/evidence-map.json.
- QEMU/substitute transcript:
  tasks/evidence/2026-06-26-phase12-local-bare-name-vfs-pipeline-core/qemu-local-shell-bare-name-vfs-pipeline-smoke.log.
- Implementation and smoke harness:
  src/local_command_loop.rs, src/target/qemu_virt.rs, build.rs,
  scripts/qemu-local-serial-command-loop-smoke.sh, and
  scripts/qemu-local-shell-bare-name-vfs-pipeline-smoke.sh.

## Accepted Frontier

The local shell accepts one bounded bare-name VFS pipeline:

~~~text
stdout | stdin
~~~

The producer and consumer resolve through the fixed /bin lookup to
/bin/stdout and /bin/stdin. Both stages are loaded through VFS open/read and
the accepted program loader, then run through userspace launch/status and the
descriptor-backed pipe path. The accepted QEMU/substitute transcript shows
the producer and consumer lifecycle records, waitpid observations,
laststatus, /proc/talos/processes, zero-argument ps, and pipestatus.

Live network/SSH reachability remains paused. No Pi 5 hardware claim is made.

## Deferred Frontier

Deferred surfaces remain POSIX PATH environment compatibility, PATH mutation,
command lookup beyond the bounded /bin surface, mixed bare/path/exec pipeline
forms, bare-name arguments or redirections, arbitrary shell grammar,
unbounded pipelines, pipeline concurrency, scheduler-concurrent execution,
fork/signals, process groups/sessions, broad procfs/Linux ps, PID policy
expansion, waitpid options, persistent storage, live networking/SSH, Pi 5
hardware proof, generated-root command-input retry, and phase transition.

## Validation

- cargo fmt --all -- --check: passed after formatting.
- cargo -Zjson-target-spec test --quiet local_command_loop: passed.
- scripts/qemu-local-shell-bare-name-vfs-pipeline-smoke.sh: passed.
- scripts/qemu-local-shell-bare-name-vfs-command-smoke.sh: passed.
- scripts/qemu-local-shell-absolute-path-vfs-pipeline-smoke.sh: passed.
- scripts/qemu-local-shell-vfs-exec-smoke.sh: passed.
- scripts/qemu-local-shell-process-status-vfs-smoke.sh: passed.
- scripts/qemu-local-shell-pipeline-status-smoke.sh: passed.
- jq empty on task-owned JSON evidence: passed.
- git diff --check: passed.
- /home/node/.cargo/bin/mdbook build: passed; mdBook HTML emitted with the
  existing large-search-index warning.
- git diff --cached --check: passed before commit.

## Result

selected_next_task: phase12-local-bare-name-path-frontier-checkpoint-20260626.

The bare-name path frontier checkpoint is mechanically unblocked after this
accepted core task is committed, provided the hardware lock remains
restored/unlocked and supervisor intervention remains inactive.
