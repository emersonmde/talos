# Phase 12 Local Bare-Name Pipeline Stderr Regular-File Redirection Core

Task id: phase12-local-bare-name-pipeline-stderr-regular-file-redirection-core-20260627

Status: accepted; commit hash recorded in durable supervisor state after
commit.

## Scope

Implement the exact fixed-/bin bare-name pipeline final-stage stderr redirection
witness:

~~~text
stdout | stderr 2>/tmp/pipeline-stderr.txt
~~~

Both stages resolve only through the accepted bounded /bin lookup to /bin/stdout
and /bin/stderr, then load through descriptor-backed VFS open/read and the
accepted userspace launch/status path. The producer writes to the accepted pipe
endpoint. The final-stage consumer inherits fd0 from that pipe endpoint and
receives only child-owned fd2 redirection to
volatile-vfs:/tmp/pipeline-stderr.txt.

## Non-Goals

This task does not accept append form for pipeline stderr redirection,
environment-backed PATH, current-directory search, command lookup beyond bounded
/bin, arbitrary output paths, input/combined pipeline redirections, persistent
filesystem behavior, live networking/SSH, Pi 5 hardware action, generated-root
retry, or phase transition.

## Findings

- fixed: Added the bare-name pipeline consumer stderr parser case for exactly
  'stdout | stderr 2>/tmp/pipeline-stderr.txt', resolving only through the
  accepted fixed /bin lookup to /bin/stderr.
- fixed: Reused the accepted final-stage volatile stderr regular-file sink path
  so only the pipeline consumer receives child-owned fd2 redirection and shell
  fd2 is restored afterward.
- fixed: Added a dedicated QEMU local serial smoke scenario and wrapper for the
  exact bare-name witness, descriptor-backed readback, shell fd2 restoration,
  lifecycle/status observations, and fail-closed neighboring forms.
- fixed: Updated the local command-loop boundary string and QEMU dispatch
  expectations to expose the accepted bounded bare-name pipeline consumer stderr
  regular-file redirection capability.
- fixed: Updated the direct path-form stderr pipeline regression expectation
  because 'stdout | stderr 2>/tmp/pipeline-stderr.txt' is now an accepted
  fixed-/bin bare-name witness rather than a negative control.
- not-an-issue: Unsupported producer command names classify as unknown-command,
  while unsupported consumer stage names with redirection classify as
  unexpected-argument; both fail closed without successful process records or
  volatile file writes.
- not-an-issue: The stderr fixture intentionally writes to stderr and does not
  read stdin; the retained pipeline record therefore reports producer
  bytes-written=0x1f, consumer bytes-read=0, and reader-eof=false while still
  proving the consumer inherited fd0 as the pipe endpoint.
- not-an-issue: The QEMU-substitute cargo test harness still runs the whole
  no_std suite when a focused filter is supplied; the retained focused
  transcript therefore reports '866 passed'.
- deferred: Stderr append forms, stdout final-stage redirection for this
  pipeline shape, input/combined pipeline redirections, arbitrary output paths,
  persistent storage, PATH/current-directory lookup, command lookup beyond
  bounded /bin, generated-root retry, live network/SSH, Pi 5 hardware proof, and
  phase transition remain outside this task.

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
  tasks/evidence/2026-06-27-phase12-local-bare-name-pipeline-stderr-regular-file-redirection-core/qemu-substitute-focused-test.log.
- Direct pipeline stderr regression transcript:
  tasks/evidence/2026-06-27-phase12-local-bare-name-pipeline-stderr-regular-file-redirection-core/direct-pipeline-stderr-regression-test.log.
- QEMU/substitute full regression transcript:
  tasks/evidence/2026-06-27-phase12-local-bare-name-pipeline-stderr-regular-file-redirection-core/full-test.log.
- QEMU/substitute local shell transcript:
  tasks/evidence/2026-06-27-phase12-local-bare-name-pipeline-stderr-regular-file-redirection-core/qemu-local-shell-bare-name-pipeline-stderr-regular-file-redirection-smoke.log.
- retained regression summary:
  tasks/evidence/2026-06-27-phase12-local-bare-name-pipeline-stderr-regular-file-redirection-core/retained-regression-smoke-summary.txt.
- classification:
  tasks/evidence/2026-06-27-phase12-local-bare-name-pipeline-stderr-regular-file-redirection-core/classification.json.
- evidence map:
  tasks/evidence/2026-06-27-phase12-local-bare-name-pipeline-stderr-regular-file-redirection-core/evidence-map.json.

## Validation

- 'cargo fmt --all -- --check': passed.
- 'cargo -Zjson-target-spec test --quiet local_command_loop_redirects_bare_name_pipeline_consumer_stderr_to_volatile_regular_file': passed; retained transcript reports 'test result: ok. 866 passed'.
- 'cargo -Zjson-target-spec test --quiet local_command_loop_redirects_direct_path_pipeline_consumer_stderr_to_volatile_regular_file': passed; retained transcript reports 'test result: ok. 866 passed'.
- 'cargo -Zjson-target-spec test --quiet': passed; retained transcript reports 'test result: ok. 866 passed'.
- 'scripts/qemu-local-shell-bare-name-pipeline-stderr-regular-file-redirection-smoke.sh': passed; QEMU/substitute transcript reports 'final participants=23 expected=23 errors=0'.
- retained local POSIX/VFS controls cover direct pipeline stderr redirection,
  pipeline-output append, process-status VFS, ps, pipestatus, and cat readback.
- 'jq empty tasks/evidence/2026-06-27-phase12-local-bare-name-pipeline-stderr-regular-file-redirection-core/classification.json tasks/evidence/2026-06-27-phase12-local-bare-name-pipeline-stderr-regular-file-redirection-core/evidence-map.json': passed.
- 'git diff --check': passed.
- '/home/node/.cargo/bin/mdbook build': passed.
- 'git diff --cached --check': passed before commit.

## Result

Accepted boundary: the fixed-/bin bare-name command
'stdout | stderr 2>/tmp/pipeline-stderr.txt' succeeds only through the accepted
bounded /bin lookup, descriptor-backed VFS/userspace execution path, pipe
handoff, and final-stage child-only stderr regular-file redirection.
Descriptor-backed 'cat /tmp/pipeline-stderr.txt' reads the userspace stderr
fixture from volatile VFS, and a later normal 'stderr' proves shell fd2
restoration.

Direct path-form pipeline stderr redirection remains accepted. Unsupported
append, stdout final-stage redirection, input redirection, unsupported command
names, path-containing stage names, malformed spacing, arbitrary/persistent
paths, PATH/current-directory lookup, and command lookup beyond bounded /bin
fail closed without successful process records or file writes.

Live network/SSH remains paused. No Pi 5 hardware claim is made.

selected_next_task:
phase12-local-bare-name-pipeline-stderr-regular-file-redirection-closeout-20260627.
