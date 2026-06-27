# Phase 12 Local Direct Pipeline-Output Append Regular-File Redirection Core

Task id: phase12-local-direct-pipeline-output-append-regular-file-redirection-core-20260627

Status: accepted; commit hash recorded in durable supervisor state after
commit.

## Scope

Implement the exact direct path-form pipeline-output append witness:

~~~text
/bin/stdout | /bin/stdin >/tmp/pipeline-report.txt
/bin/stdout | /bin/stdin >>/tmp/pipeline-report.txt
~~~

Both pipeline stages load through descriptor-backed VFS open/read and the
accepted userspace launch/status path. The producer writes to the accepted pipe
endpoint, the consumer reads from that pipe on fd0, and only the final stage
receives child-only fd1 redirection to
volatile-vfs:/tmp/pipeline-report.txt. The first command uses truncate/sink
semantics; the second records append-at-EOF semantics.

## Non-Goals

This task does not accept fixed-/bin bare-name pipeline-output append, stderr
pipeline-output append, input or combined pipeline redirections, arbitrary
output paths, persistent writable filesystem behavior, generated-root retry,
live networking/SSH, Pi 5 hardware action, or a phase transition.

## Findings

- fixed: Added direct absolute-path pipeline consumer parsing for exactly
  '/bin/stdout | /bin/stdin >>/tmp/pipeline-report.txt'.
- fixed: Reused the existing volatile regular-file append path for final-stage
  stdout only, preserving the first command's truncate/sink behavior and the
  second command's append-at-EOF behavior.
- fixed: Increased the bounded volatile file backing from 128 to 256 bytes so
  the two 0x44-byte pipeline consumer reports fit without changing the accepted
  path policy.
- fixed: Added a dedicated QEMU local serial smoke scenario and command-loop
  regression for the exact direct append sequence, descriptor-backed readback,
  shell fd1 restoration, lifecycle/status observations, and fail-closed
  neighboring forms.
- fixed: Added a boundary string for the direct path-form pipeline consumer
  stdout append capability in the local command-loop status surface.
- not-an-issue: The QEMU-substitute cargo test harness still runs the whole
  no_std suite when a focused filter is supplied; the retained focused
  transcript therefore reports '864 passed'.
- deferred: Fixed-/bin bare-name pipeline-output append is separate queued
  work.
- deferred: Stderr append forms, input/combined pipeline redirections,
  arbitrary output paths, persistent storage, generated-root retry, live
  network/SSH, Pi 5 hardware proof, and phase transition remain outside this
  task.

## Evidence

- static inspection:
  - src/local_command_loop.rs parser/path-policy/runtime and regression diff.
  - src/target/qemu_virt.rs QEMU scenario dispatch and transcript assertions.
  - scripts/qemu-local-serial-command-loop-smoke.sh and the task-owned wrapper.
  - docs/src/roadmap.md.
  - docs/src/project/phase12-networking-ssh.md.
  - docs/src/project/early-posix-shape.md.
- QEMU/substitute focused transcript:
  tasks/evidence/2026-06-27-phase12-local-direct-pipeline-output-append-regular-file-redirection-core/qemu-substitute-focused-test.log.
- QEMU/substitute local shell transcript:
  tasks/evidence/2026-06-27-phase12-local-direct-pipeline-output-append-regular-file-redirection-core/qemu-local-shell-direct-pipeline-output-append-regular-file-redirection-smoke.log.
- retained regression summary:
  tasks/evidence/2026-06-27-phase12-local-direct-pipeline-output-append-regular-file-redirection-core/retained-regression-smoke-summary.txt.
- classification:
  tasks/evidence/2026-06-27-phase12-local-direct-pipeline-output-append-regular-file-redirection-core/classification.json.
- evidence map:
  tasks/evidence/2026-06-27-phase12-local-direct-pipeline-output-append-regular-file-redirection-core/evidence-map.json.

## Validation

- 'cargo -Zjson-target-spec test --quiet local_command_loop_redirects_direct_path_pipeline_consumer_stdout_to_volatile_regular_file': passed; retained transcript reports 'test result: ok. 864 passed'.
- 'cargo fmt --all -- --check': passed.
- 'cargo -Zjson-target-spec test --quiet local_command_loop': passed; retained transcript reports 'test result: ok. 864 passed'.
- 'scripts/qemu-local-shell-direct-pipeline-output-append-regular-file-redirection-smoke.sh': passed; QEMU/substitute transcript reports 'final participants=20 expected=20 errors=0'.
- retained local POSIX QEMU/substitute smoke gates passed for cat-banner, VFS exec, process-status VFS/ps, pipestatus, stdin redirection, stdout append, stderr append, combined redirection, and existing pipeline-output regular-file redirection.
- 'jq empty tasks/evidence/2026-06-27-phase12-local-direct-pipeline-output-append-regular-file-redirection-core/classification.json tasks/evidence/2026-06-27-phase12-local-direct-pipeline-output-append-regular-file-redirection-core/evidence-map.json': passed.
- 'git diff --check': passed.
- '/home/node/.cargo/bin/mdbook build': passed.
- 'git diff --cached --check': passed before commit.

## Result

Accepted boundary: the direct path-form sequence
'/bin/stdout | /bin/stdin >/tmp/pipeline-report.txt' followed by
'/bin/stdout | /bin/stdin >>/tmp/pipeline-report.txt' succeeds only through the
accepted descriptor-backed VFS/userspace execution path. Descriptor-backed
'cat /tmp/pipeline-report.txt' reads the two userspace stdin reports in order,
and a later normal '/bin/stdout' proves shell fd1 restoration.

Unsupported direct forms, fixed-/bin bare-name append, wrong final-stage
programs, stderr append forms, input redirection on pipelines, malformed append
grammar, and arbitrary/persistent paths fail closed without successful process
records or file writes.

Live network/SSH remains paused. No Pi 5 hardware claim is made.

selected_next_task:
phase12-local-direct-pipeline-output-append-regular-file-redirection-closeout-20260627.
