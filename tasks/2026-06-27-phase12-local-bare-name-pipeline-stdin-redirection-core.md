# Phase 12 Local Bare-Name Pipeline Stdin Redirection Core

Task id: phase12-local-bare-name-pipeline-stdin-redirection-core-20260627

Status: accepted; commit hash recorded in durable supervisor state after
commit.

## Scope

Implement the thinnest fixed-/bin bare-name two-stage pipeline producer stdin
redirection path:

~~~text
stdin </etc/banner.txt | stdin
~~~

Both stages resolve only through the accepted fixed bounded /bin lookup to
/bin/stdin before descriptor-backed VFS open/read, loader execution,
userspace launch/status, pipe descriptor handoff, process-table recording, ps,
and pipestatus observations. The producer opens fd0 from
initramfs:/etc/banner.txt, preserves fd1 as the pipe endpoint, inherits fd2,
and writes the userspace stdin fixture output through the pipe. The consumer
reads fd0 from that pipe and writes to inherited fd1.

This task does not accept consumer-stage redirection, redirection on multiple
pipeline stages, output redirection, append/truncate, writable filesystem
behavior, environment-backed PATH, current-directory search, command lookup
beyond the accepted bounded /bin surface, broad shell grammar, live
networking/SSH, Pi 5 hardware proof, generated-root retry, or a phase
transition.

## Findings

- fixed: Bare-name two-stage pipelines now accept exactly
  'stdin </etc/banner.txt | stdin' without admitting general redirection
  grammar or treating the redirection token as argv.
- fixed: The bare-name pipeline parser now applies the producer-stage argument
  policy already used by the direct path-form pipeline, so one leading
  '</etc/banner.txt' token is allowed only on the producer before any literal
  argv.
- fixed: Producer execution records canonical argv0=/bin/stdin, fd0 as the
  read-only initramfs regular file, fd1 as the pipe endpoint, inherited fd2,
  a closed loader temporary descriptor, and successful userspace stdin read.
- fixed: Consumer execution records canonical argv0=/bin/stdin, fd0 as the
  pipe endpoint from the producer, inherited fd1/fd2, a closed loader
  temporary descriptor, successful launch/status, and successful userspace
  pipe read.
- fixed: Pipeline lifecycle/status, explicit waitpid for producer and
  consumer, laststatus, /proc/talos/processes, zero-argument ps, and
  pipestatus-compatible observations remain attached to the two participants.
- fixed: Unsupported variants such as consumer command changes, producer argv
  plus redirection, consumer argv plus redirection, unsupported redirection
  paths, separated '<' syntax, and multistage pipeline redirection fail closed
  without additional successful process records.
- fixed: The QEMU/substitute smoke harness now has a dedicated
  qemu_local_shell_bare_name_pipeline_stdin_redirection boot scenario and
  task-owned transcript.
- not-an-issue: The accepted direct path-form pipeline-stage stdin
  redirection surface remains a separate regression/control path.
- deferred: Consumer-stage redirection, redirection on multiple pipeline
  stages, combined input/output redirection, output regular-file redirection,
  append/truncate, writable filesystem behavior, environment-backed PATH,
  current-directory search, command lookup beyond existing bounded surfaces,
  quoting, escaping, globbing, variables, arbitrary shell grammar, unbounded
  pipelines, pipeline concurrency, scheduler concurrency, fork/signals,
  process groups/sessions, persistent storage, live networking/SSH, Pi 5
  hardware proof, generated-root retry, and phase transition.

## Evidence Map

- Classification and evidence JSON:
  tasks/evidence/2026-06-27-phase12-local-bare-name-pipeline-stdin-redirection-core/classification.json
  and
  tasks/evidence/2026-06-27-phase12-local-bare-name-pipeline-stdin-redirection-core/evidence-map.json.
- QEMU/substitute transcript:
  tasks/evidence/2026-06-27-phase12-local-bare-name-pipeline-stdin-redirection-core/qemu-local-shell-bare-name-pipeline-stdin-redirection-smoke.log.
- Task-owned regression transcripts:
  tasks/evidence/2026-06-27-phase12-local-bare-name-pipeline-stdin-redirection-core/regressions/.
- Implementation and smoke harness:
  build.rs, src/local_command_loop.rs, src/target/qemu_virt.rs,
  scripts/qemu-local-serial-command-loop-smoke.sh, and
  scripts/qemu-local-shell-bare-name-pipeline-stdin-redirection-smoke.sh.
- Project notes:
  docs/src/roadmap.md, docs/src/project/phase12-networking-ssh.md, and
  docs/src/project/early-posix-shape.md.

## Accepted Frontier

The accepted bare-name pipeline stdin redirection frontier is local-only and
static/unit/QEMU-substitute backed. A fixed-/bin bare-name two-stage pipeline
can now redirect the producer's fd0 from a read-only initramfs file while
preserving fd1 as the pipe endpoint:

~~~text
stdin </etc/banner.txt | stdin
~~~

Both stages canonicalize argv0 to /bin/stdin. The producer records fd0
source-route=initramfs:/etc/banner.txt, fd1 as the pipe endpoint, inherited
fd2, loader-temp-open=false, and a regular-file EOF after read. The consumer
records fd0 as the pipe endpoint, inherited fd1/fd2, loader-temp-open=false,
and pipe EOF after writer close. The shell restores fd0, the pipeline
lifecycle/status record is coherent, explicit waitpid observes both
participants, and laststatus, /proc/talos/processes, zero-argument ps, and
pipestatus remain coherent.

Live network/SSH reachability remains paused. No Pi 5 hardware claim is made.

## Deferred Frontier

Deferred surfaces remain consumer-stage redirection, redirection on multiple
pipeline stages, combined input/output redirection, output regular-file
redirection, append/truncate, writable filesystem behavior, environment-backed
PATH, current-directory search, command lookup beyond existing bounded
surfaces, quoting, escaping, globbing, variables, arbitrary shell grammar,
unbounded pipelines, pipeline concurrency, scheduler concurrency,
fork/signals, process groups/sessions, broad procfs/Linux ps, PID policy
expansion, waitpid options, persistent storage, live networking/SSH, Pi 5
hardware proof, generated-root command-input retry, and phase transition.

## Validation

- cargo fmt --all -- --check: passed.
- cargo -Zjson-target-spec test --quiet local_command_loop: passed.
- QEMU/substitute bare-name pipeline-stage stdin redirection smoke using
  scripts/qemu-local-shell-bare-name-pipeline-stdin-redirection-smoke.sh with
  task-owned evidence path: passed.
- Task-owned QEMU/substitute regressions passed: direct path-form
  pipeline-stage stdin redirection, direct and bare-name stdin redirection,
  direct and bare-name pipeline argv, absolute path command argv, bare-name
  command argv, process-status VFS, zero-argument ps, pipestatus, and
  cat-banner.
- jq empty on task-owned JSON evidence: passed.
- git diff --check: passed.
- /home/node/.cargo/bin/mdbook build: passed.
- git diff --cached --check: passed before commit.

## Result

selected_next_task: phase12-local-bare-name-pipeline-stdin-redirection-closeout-20260627.

The bare-name pipeline stdin redirection closeout is mechanically unblocked
after this accepted core task is committed, provided the hardware lock remains
restored/unlocked and supervisor intervention remains inactive.
