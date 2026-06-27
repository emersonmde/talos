# Phase 12 Local Bare-Name Pipeline-Output Append Regular-File Redirection Core

Task id: phase12-local-bare-name-pipeline-output-append-regular-file-redirection-core-20260627

Status: accepted; commit hash recorded in durable supervisor state after
commit.

## Scope

Implement the exact fixed-/bin bare-name pipeline-output append witness:

~~~text
stdout | stdin >/tmp/pipeline-report.txt
stdout | stdin >>/tmp/pipeline-report.txt
~~~

Both pipeline stages resolve only through the accepted bounded /bin lookup to
/bin/stdout and /bin/stdin, then load through descriptor-backed VFS open/read
and the accepted userspace launch/status path. The producer writes to the
accepted pipe endpoint, the consumer reads from that pipe on fd0, and only the
final stage receives child-only fd1 redirection to
volatile-vfs:/tmp/pipeline-report.txt. The first command uses truncate/sink
semantics; the second records append-at-EOF semantics.

## Non-Goals

This task does not accept stderr pipeline-output append, input or combined
pipeline redirections, arbitrary output paths, persistent writable filesystem
behavior, environment-backed PATH, current-directory search, command lookup
beyond bounded /bin, arbitrary shell grammar, generated-root retry, live
networking/SSH, Pi 5 hardware action, or a phase transition.

## Findings

- fixed: Added the bare-name pipeline consumer append parser case for exactly
  'stdout | stdin >>/tmp/pipeline-report.txt', resolving only through the
  fixed /bin lookup to /bin/stdin.
- fixed: Reused the accepted final-stage volatile regular-file append path so
  the first bare-name pipeline truncates/sinks and the second appends at EOF.
- fixed: Added a dedicated QEMU local serial smoke scenario and wrapper for the
  exact bare-name truncate-then-append sequence, descriptor-backed readback,
  shell fd1 restoration, lifecycle/status observations, and fail-closed
  neighboring forms.
- fixed: Updated the local command-loop boundary string and QEMU dispatch
  expectations to expose the accepted bounded bare-name pipeline consumer
  stdout append capability.
- fixed: Updated the direct path-form append regression expectations because
  'stdout | stdin >>/tmp/pipeline-report.txt' is now an accepted fixed-/bin
  bare-name witness rather than a negative control.
- not-an-issue: Direct absolute path-form pipeline-output append remains
  accepted and continues to run through descriptor-backed VFS/userspace
  execution.
- not-an-issue: The QEMU-substitute cargo test harness still runs the whole
  no_std suite when a focused filter is supplied; the retained focused
  transcript therefore reports '864 passed'.
- deferred: Stderr pipeline append, input/combined pipeline redirections,
  arbitrary paths, persistent writable storage, generated-root retry, live
  network/SSH, Pi 5 hardware proof, and phase transition remain outside this
  task.

## Evidence

- static inspection:
  - src/local_command_loop.rs parser/path-policy/runtime and regression diff.
  - src/target/qemu_virt.rs QEMU scenario dispatch and transcript assertions.
  - build.rs and src/main.rs scenario registration.
  - scripts/qemu-local-serial-command-loop-smoke.sh and the task-owned wrapper.
  - docs/src/roadmap.md.
  - docs/src/project/phase12-networking-ssh.md.
  - docs/src/project/early-posix-shape.md.
- QEMU/substitute focused transcript:
  tasks/evidence/2026-06-27-phase12-local-bare-name-pipeline-output-append-regular-file-redirection-core/qemu-substitute-focused-test.log.
- QEMU/substitute local shell transcript:
  tasks/evidence/2026-06-27-phase12-local-bare-name-pipeline-output-append-regular-file-redirection-core/qemu-local-shell-bare-name-pipeline-output-append-regular-file-redirection-smoke.log.
- retained regression summary:
  tasks/evidence/2026-06-27-phase12-local-bare-name-pipeline-output-append-regular-file-redirection-core/retained-regression-smoke-summary.txt.
- classification:
  tasks/evidence/2026-06-27-phase12-local-bare-name-pipeline-output-append-regular-file-redirection-core/classification.json.
- evidence map:
  tasks/evidence/2026-06-27-phase12-local-bare-name-pipeline-output-append-regular-file-redirection-core/evidence-map.json.

## Validation

- 'cargo -Zjson-target-spec test --quiet local_command_loop_redirects_bare_name_pipeline_consumer_stdout_to_volatile_regular_file': passed; retained transcript reports 'test result: ok. 864 passed'.
- 'cargo fmt --all -- --check': passed.
- 'cargo -Zjson-target-spec test --quiet local_command_loop': passed; retained transcript reports 'test result: ok. 864 passed'.
- 'scripts/qemu-local-shell-bare-name-pipeline-output-append-regular-file-redirection-smoke.sh': passed; QEMU/substitute transcript reports 'final participants=23 expected=23 errors=0'.
- retained local POSIX QEMU/substitute smoke gates passed for cat-banner, absolute-path VFS command, process-status VFS/ps, pipestatus, direct stdin redirection, direct stdout append, direct stderr append, bare-name stderr append, direct combined stdin/stdout redirection, existing pipeline-output regular-file redirection, and direct pipeline-output append regular-file redirection.
- 'jq empty tasks/evidence/2026-06-27-phase12-local-bare-name-pipeline-output-append-regular-file-redirection-core/classification.json tasks/evidence/2026-06-27-phase12-local-bare-name-pipeline-output-append-regular-file-redirection-core/evidence-map.json': passed.
- 'git diff --check': passed.
- '/home/node/.cargo/bin/mdbook build': passed.
- 'git diff --cached --check': passed before commit.

## Result

Accepted boundary: the fixed-/bin bare-name sequence
'stdout | stdin >/tmp/pipeline-report.txt' followed by
'stdout | stdin >>/tmp/pipeline-report.txt' succeeds only through the accepted
bounded /bin lookup, descriptor-backed VFS/userspace execution path, pipe
handoff, and final-stage child-only stdout regular-file redirection.
Descriptor-backed 'cat /tmp/pipeline-report.txt' reads the two userspace stdin
reports in order, and a later normal 'stdout' proves shell fd1 restoration.

Direct path-form pipeline-output append remains accepted. Unsupported
PATH/current-directory lookup, unsupported command names, path-containing
consumer names, explicit '1>', spaced output grammar, malformed append grammar,
stderr forms, input redirection on pipelines, arbitrary output paths, and
persistent-storage claims fail closed.

Live network/SSH remains paused. No Pi 5 hardware claim is made.

selected_next_task:
phase12-local-bare-name-pipeline-output-append-regular-file-redirection-closeout-20260627.
