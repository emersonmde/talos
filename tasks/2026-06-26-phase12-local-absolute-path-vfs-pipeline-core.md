# Phase 12 Local Absolute-Path VFS Pipeline Core

Task id: phase12-local-absolute-path-vfs-pipeline-core-20260626

Status: accepted; commit hash recorded in durable supervisor state after
commit.

## Scope

Implement the thinnest bounded path-form pipeline in the local shell:

~~~text
/bin/stdout | /bin/stdin
~~~

Both stages must load through the accepted VFS open/read, program-loader,
initial user stack, and userspace launch path. This task does not accept PATH
lookup, bare command lookup, arbitrary shell grammar, unbounded pipelines,
pipeline concurrency, live networking/SSH, Pi 5 hardware proof, or a phase
transition.

## Findings

- fixed: Added a direct absolute-path pipeline parser for exactly one pipe
  after an absolute producer command name.
- fixed: /bin/stdout | /bin/stdin now launches both stages through the
  accepted VFS exec/loading path and preserves the existing serialized
  two-stage pipeline byte flow.
- fixed: Preserved lifecycle/status, process-table records, explicit waitpid,
  laststatus, /proc/talos/processes, zero-argument ps, and pipestatus
  observations for the path-form pipeline.
- fixed: Added fail-closed controls for mixed diagnostic/path forms,
  unsupported paths, bare command names, and path-form multistage pipelines.
- fixed: Added focused unit coverage and a retained QEMU/substitute transcript
  for the accepted path-form pipeline and negative controls.
- fixed: Updated roadmap, Phase 12 project notes, and early POSIX notes with
  the accepted/deferred path-form pipeline frontier.
- not-an-issue: Existing exec-prefixed direct/pipeline/multistage,
  descriptor-backed VFS file I/O, waitpid/jobs, process-status VFS, ps,
  pipestatus, and redirection surfaces remain regression surfaces rather than
  new claims.
- deferred: PATH lookup, bare command lookup, arbitrary shell grammar,
  unbounded pipelines, path-form multistage pipelines, pipeline concurrency,
  scheduler concurrency, fork/signals, process groups/sessions, live
  networking/SSH, Pi 5 hardware proof, and phase transition.

## Evidence Map

- Classification and evidence JSON:
  tasks/evidence/2026-06-26-phase12-local-absolute-path-vfs-pipeline-core/classification.json
  and
  tasks/evidence/2026-06-26-phase12-local-absolute-path-vfs-pipeline-core/evidence-map.json.
- QEMU/substitute transcript:
  tasks/evidence/2026-06-26-phase12-local-absolute-path-vfs-pipeline-core/qemu-local-shell-absolute-path-vfs-pipeline-smoke.log.
- Implementation and smoke harness:
  src/local_command_loop.rs, src/target/qemu_virt.rs, src/main.rs,
  build.rs, scripts/qemu-local-serial-command-loop-smoke.sh, and
  scripts/qemu-local-shell-absolute-path-vfs-pipeline-smoke.sh.

## Accepted Frontier

The local shell accepts exactly the bounded two-stage path-form pipeline
/bin/stdout | /bin/stdin. The producer and consumer are absolute paths, have
no arguments or redirections, and load through VFS open/read and the existing
program loader.

The retained transcript proves the same pipe byte flow and bounded process
accounting already accepted for the exec-prefixed two-stage pipeline:
producer /bin/stdout, consumer /bin/stdin, one pipe endpoint handoff,
explicit waitpid records for both participants, non-consuming laststatus for
the final stage, /proc/talos/processes, zero-argument ps, and pipestatus.

Live network/SSH reachability remains paused. No Pi 5 hardware claim is made.

## Deferred Frontier

Deferred surfaces remain PATH lookup, bare command lookup, path-form arguments
or redirections, mixed diagnostic/path pipeline forms, path-form multistage
pipelines, arbitrary shell grammar, unbounded pipelines, pipeline concurrency,
scheduler concurrency, fork/signals, process groups/sessions, broad
procfs/Linux ps compatibility, PID policy expansion, persistent storage, live
networking/SSH, Pi 5 hardware proof, generated-root command-input retry, and
phase transition.

## Validation

- cargo fmt --all -- --check: passed.
- cargo -Zjson-target-spec test --quiet local_command_loop: passed.
- scripts/qemu-local-shell-absolute-path-vfs-pipeline-smoke.sh: passed.
- jq empty on task-owned JSON evidence: passed.
- git diff --check: passed.
- /home/node/.cargo/bin/mdbook build: passed; mdBook HTML emitted with the
  existing large-search-index warning.
- git diff --cached --check: passed.

## Result

selected_next_task: phase12-local-absolute-path-vfs-pipeline-closeout-20260626.

The closeout task is mechanically unblocked after this accepted core task is
committed, provided the hardware lock remains restored/unlocked and supervisor
intervention remains inactive.
