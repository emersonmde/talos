# Phase 12 Local Direct Pipeline Consumer Stdin Redirection Core

Task id: phase12-local-direct-pipeline-consumer-stdin-redirection-core-20260627

Status: accepted; commit hash recorded in durable supervisor state after
commit.

## Scope

Implement the thinnest direct path-form two-stage pipeline consumer stdin
redirection path:

~~~text
/bin/stdin | /bin/stdin </etc/banner.txt
~~~

Both stages load from descriptor-backed VFS through the accepted loader and
userspace launch/status path. The producer keeps inherited fd0, writes fd1 to
the pipe endpoint, and inherits fd2. The consumer starts from the accepted
pipeline fd0 handoff, then replaces only the child fd0 with
initramfs:/etc/banner.txt before launch and restores shell fd0 after launch.

This task does not accept bare-name consumer-stage redirection, redirection on
multiple pipeline stages, multistage pipeline redirection, output redirection,
append/truncate, writable filesystem behavior, environment-backed PATH,
current-directory search, command lookup beyond bounded /bin, broad shell
grammar, live networking/SSH, Pi 5 hardware proof, generated-root retry, or a
phase transition.

## Findings

- fixed: Direct path-form two-stage pipelines now accept exactly
  '/bin/stdin | /bin/stdin </etc/banner.txt' without admitting general
  redirection grammar or treating the redirection token as argv.
- fixed: Consumer-stage argument policy now permits one leading
  '</etc/banner.txt' token for the consumer parser while the execution policy
  still narrows acceptance to the exact direct /bin/stdin producer and
  consumer shape.
- fixed: Producer execution records argv0=/bin/stdin, fd0 as inherited stdio
  input, fd1 as the pipe endpoint, inherited fd2, a closed loader temporary
  descriptor, and a no-data runtime stdin observation when no console byte is
  ready under QEMU.
- fixed: Scheduler stdin wait diagnostics are no longer written through a
  child stdout pipe endpoint; the child payload remains the only pipe payload.
- fixed: Consumer execution records argv0=/bin/stdin, fd0 as the read-only
  initramfs regular file from child-only redirection, inherited fd1/fd2, a
  closed loader temporary descriptor, successful launch/status, and regular
  file EOF after reading /etc/banner.txt.
- fixed: Pipeline lifecycle/status, explicit waitpid for producer and
  consumer, laststatus, /proc/talos/processes, zero-argument ps, and
  pipestatus-compatible observations remain attached to the two participants.
- fixed: Unsupported direct variants such as stdout producer, producer argv,
  consumer argv plus redirection, /dev/null consumer redirection, separated
  '<' syntax, multistage consumer redirection, and redirection on both stages
  fail closed without additional successful process records.
- fixed: The QEMU/substitute smoke harness now has a dedicated
  qemu_local_shell_direct_pipeline_consumer_stdin_redirection boot scenario
  and task-owned transcript.
- not-an-issue: Existing producer-stage pipeline stdin redirection remains a
  separate accepted regression/control path; this task only accepts the direct
  consumer-stage form.
- deferred: Bare-name consumer-stage stdin redirection, redirection on
  multiple pipeline stages, multistage pipeline redirection, combined
  input/output redirection, output regular-file redirection, append/truncate,
  writable filesystem behavior, environment-backed PATH, current-directory
  search, command lookup beyond existing bounded surfaces, quoting, escaping,
  globbing, variables, arbitrary shell grammar, unbounded pipelines, pipeline
  concurrency, scheduler concurrency, fork/signals, process groups/sessions,
  persistent storage, live networking/SSH, Pi 5 hardware proof,
  generated-root retry, and phase transition.

## Evidence Map

- Classification and evidence JSON:
  tasks/evidence/2026-06-27-phase12-local-direct-pipeline-consumer-stdin-redirection-core/classification.json
  and
  tasks/evidence/2026-06-27-phase12-local-direct-pipeline-consumer-stdin-redirection-core/evidence-map.json.
- QEMU/substitute transcript:
  tasks/evidence/2026-06-27-phase12-local-direct-pipeline-consumer-stdin-redirection-core/qemu-local-shell-direct-pipeline-consumer-stdin-redirection-smoke.log.
- Task-owned regression transcripts:
  tasks/evidence/2026-06-27-phase12-local-direct-pipeline-consumer-stdin-redirection-core/regressions/.
- Implementation and smoke harness:
  build.rs, src/main.rs, src/local_command_loop.rs, src/target/qemu_virt.rs,
  scripts/qemu-local-serial-command-loop-smoke.sh, and
  scripts/qemu-local-shell-direct-pipeline-consumer-stdin-redirection-smoke.sh.
- Project notes:
  docs/src/roadmap.md, docs/src/project/phase12-networking-ssh.md, and
  docs/src/project/early-posix-shape.md.

## Accepted Frontier

The accepted direct path-form consumer-stage stdin redirection frontier is
local-only and static/unit/QEMU-substitute backed. A direct path-form
two-stage pipeline can now redirect the consumer's fd0 from a read-only
initramfs file while the producer still launches through VFS and keeps fd1 as
the pipe endpoint:

~~~text
/bin/stdin | /bin/stdin </etc/banner.txt
~~~

The producer records fd0 as stdio input, fd1 as the pipe endpoint, inherited
fd2, loader-temp-open=false, and a readiness/no-data stdin observation when no
console byte is available. The consumer records fd0
source-route=initramfs:/etc/banner.txt, inherited fd1/fd2,
loader-temp-open=false, and regular-file EOF after read. The shell restores
fd0, the pipeline lifecycle/status record is coherent, explicit waitpid
observes both participants, and laststatus, /proc/talos/processes,
zero-argument ps, and pipestatus remain coherent.

Live network/SSH reachability remains paused. No Pi 5 hardware claim is made.

## Deferred Frontier

Deferred surfaces remain bare-name consumer-stage stdin redirection,
redirection on multiple pipeline stages, multistage pipeline redirection,
combined input/output redirection, output regular-file redirection,
append/truncate, writable filesystem behavior, environment-backed PATH,
current-directory search, command lookup beyond existing bounded surfaces,
quoting, escaping, globbing, variables, arbitrary shell grammar, unbounded
pipelines, pipeline concurrency, scheduler concurrency, fork/signals, process
groups/sessions, broad procfs/Linux ps, PID policy expansion, waitpid options,
persistent storage, live networking/SSH, Pi 5 hardware proof,
generated-root command-input retry, and phase transition.

## Validation

- cargo fmt --all -- --check: passed.
- cargo -Zjson-target-spec test --quiet local_command_loop: passed.
- QEMU/substitute direct path-form consumer-stage stdin redirection smoke using
  scripts/qemu-local-shell-direct-pipeline-consumer-stdin-redirection-smoke.sh
  with task-owned evidence path: passed.
- Task-owned QEMU/substitute regressions passed: direct and bare-name stdin
  redirection, direct and bare-name producer-stage pipeline stdin redirection,
  direct and bare-name command argv, direct and bare-name pipeline argv,
  process-status VFS, zero-argument ps, pipestatus, and cat-banner.
- jq empty on task-owned JSON evidence: passed.
- git diff --check: passed.
- /home/node/.cargo/bin/mdbook build: passed.
- git diff --cached --check: passed before commit.

## Result

selected_next_task: phase12-local-direct-pipeline-consumer-stdin-redirection-closeout-20260627.

The direct consumer-stage stdin redirection closeout is mechanically unblocked
after this accepted core task is committed, provided the hardware lock remains
restored/unlocked and supervisor intervention remains inactive.
