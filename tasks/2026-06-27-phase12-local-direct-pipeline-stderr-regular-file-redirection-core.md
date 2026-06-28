# Phase 12 Local Direct Pipeline Stderr Regular-File Redirection Core

Task id: phase12-local-direct-pipeline-stderr-regular-file-redirection-core-20260627

Status: accepted; commit hash recorded in durable supervisor state after
commit.

## Scope

Implement the exact direct path-form pipeline final-stage stderr redirection
witness:

~~~text
/bin/stdout | /bin/stderr 2>/tmp/pipeline-stderr.txt
~~~

Both pipeline stages load through descriptor-backed VFS open/read and the
accepted userspace launch/status path. The producer writes to the accepted pipe
endpoint. The consumer receives fd0 from that pipe endpoint and receives only a
child-owned fd2 redirection to volatile-vfs:/tmp/pipeline-stderr.txt. The
stderr fixture does not consume stdin, so the accepted pipeline record preserves
bytes_read=0 and reader_eof=false while still recording the producer write and
consumer fd0 pipe endpoint.

## Non-Goals

This task does not accept stderr append, stdout final-stage redirection in this
pipeline shape, input or combined pipeline redirections, fixed-/bin bare-name
pipeline stderr redirection, arbitrary output paths, persistent writable
filesystem behavior, generated-root retry, live networking/SSH, Pi 5 hardware
action, or a phase transition.

## Findings

- fixed: Added direct absolute-path pipeline consumer parsing for exactly
  '/bin/stdout | /bin/stderr 2>/tmp/pipeline-stderr.txt'.
- fixed: Reused the existing volatile stderr regular-file sink path for the
  final stage only, preserving child-only fd2 redirection and shell fd2
  restoration.
- fixed: Added a bounded pipeline source label for consumer stderr file
  redirection so QEMU/substitute evidence distinguishes this path from prior
  stdout pipeline-output redirection work.
- fixed: Added a dedicated QEMU local serial smoke scenario and command-loop
  regression for the exact direct stderr-redirection witness, descriptor-backed
  readback, shell fd2 restoration, lifecycle/status observations, and
  fail-closed neighboring forms.
- fixed: Added a boundary string for the direct path-form pipeline consumer
  stderr redirection capability in the local command-loop status surface.
- not-an-issue: The stderr fixture intentionally writes to stderr and does not
  read stdin; the retained pipeline record therefore reports producer
  bytes-written=0x1f, consumer bytes-read=0, and reader-eof=false while still
  proving the consumer inherited fd0 as the pipe endpoint.
- not-an-issue: The QEMU-substitute cargo test harness still runs the whole
  no_std suite when a focused filter is supplied; the retained focused
  transcript therefore reports '865 passed'.
- deferred: Fixed-/bin bare-name pipeline stderr redirection is separate
  future work.
- deferred: Stderr append forms, stdout final-stage redirection for this
  pipeline shape, input/combined pipeline redirections, arbitrary output paths,
  persistent storage, generated-root retry, live network/SSH, Pi 5 hardware
  proof, and phase transition remain outside this task.

## Evidence

- static inspection:
  - src/local_command_loop.rs parser/path-policy/runtime and regression diff.
  - src/target/qemu_virt.rs QEMU scenario dispatch and transcript assertions.
  - scripts/qemu-local-serial-command-loop-smoke.sh and the task-owned wrapper.
  - docs/src/roadmap.md.
  - docs/src/project/phase12-networking-ssh.md.
  - docs/src/project/early-posix-shape.md.
- QEMU/substitute focused transcript:
  tasks/evidence/2026-06-27-phase12-local-direct-pipeline-stderr-regular-file-redirection-core/qemu-substitute-focused-test.log.
- QEMU/substitute local shell transcript:
  tasks/evidence/2026-06-27-phase12-local-direct-pipeline-stderr-regular-file-redirection-core/qemu-local-shell-direct-pipeline-stderr-regular-file-redirection-smoke.log.
- local command-loop regression transcript:
  tasks/evidence/2026-06-27-phase12-local-direct-pipeline-stderr-regular-file-redirection-core/local-command-loop-test.log.
- full unit/QEMU-substitute regression transcript:
  tasks/evidence/2026-06-27-phase12-local-direct-pipeline-stderr-regular-file-redirection-core/full-test.log.
- retained regression summary:
  tasks/evidence/2026-06-27-phase12-local-direct-pipeline-stderr-regular-file-redirection-core/retained-regression-smoke-summary.txt.
- classification:
  tasks/evidence/2026-06-27-phase12-local-direct-pipeline-stderr-regular-file-redirection-core/classification.json.
- evidence map:
  tasks/evidence/2026-06-27-phase12-local-direct-pipeline-stderr-regular-file-redirection-core/evidence-map.json.

## Validation

- 'cargo -Zjson-target-spec test --quiet local_command_loop_redirects_direct_path_pipeline_consumer_stderr_to_volatile_regular_file': passed; retained transcript reports 'test result: ok. 865 passed'.
- 'cargo -Zjson-target-spec test --quiet local_command_loop': passed; retained transcript reports 'test result: ok. 865 passed'.
- 'cargo fmt --all -- --check': passed.
- 'cargo -Zjson-target-spec test --quiet': passed; retained transcript reports 'test result: ok. 865 passed'.
- 'scripts/qemu-local-shell-direct-pipeline-stderr-regular-file-redirection-smoke.sh': passed; QEMU/substitute transcript reports 'final participants=19 expected=19 errors=0'.
- retained local POSIX/VFS controls cover pipeline-output append, direct stdout/stderr append redirection, process-status VFS, ps, pipestatus, and cat readback.
- 'jq empty tasks/evidence/2026-06-27-phase12-local-direct-pipeline-stderr-regular-file-redirection-core/classification.json tasks/evidence/2026-06-27-phase12-local-direct-pipeline-stderr-regular-file-redirection-core/evidence-map.json': passed.
- 'git diff --check': passed.
- '/home/node/.cargo/bin/mdbook build': passed.
- 'git diff --cached --check': passed before commit.

## Result

Accepted boundary: the direct path-form command
'/bin/stdout | /bin/stderr 2>/tmp/pipeline-stderr.txt' succeeds only through
the accepted descriptor-backed VFS/userspace execution path. Descriptor-backed
'cat /tmp/pipeline-stderr.txt' reads the userspace stderr fixture from
volatile VFS, and a later normal '/bin/stderr' proves shell fd2 restoration.

Unsupported direct append, stdout final-stage redirection, input redirection,
command-level stderr target reuse, malformed spacing, arbitrary/persistent
paths, and fixed-/bin bare-name forms fail closed without successful process
records or file writes.

Live network/SSH remains paused. No Pi 5 hardware claim is made.

selected_next_task:
phase12-local-direct-pipeline-stderr-regular-file-redirection-closeout-20260627.
