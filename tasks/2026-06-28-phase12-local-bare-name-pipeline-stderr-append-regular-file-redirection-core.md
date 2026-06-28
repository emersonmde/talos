# Phase 12 Local Bare-Name Pipeline Stderr Append Regular-File Redirection Core

Task id: phase12-local-bare-name-pipeline-stderr-append-regular-file-redirection-core-20260628

Status: accepted; commit hash recorded in durable supervisor state after
commit.

## Scope

Accept the smallest fixed-/bin bare-name pipeline final-stage stderr append
regular-file redirection witness:

~~~text
stdout | stderr 2>/tmp/pipeline-stderr.txt
stdout | stderr 2>>/tmp/pipeline-stderr.txt
~~~

Both stages resolve only through the accepted fixed bounded /bin lookup to
/bin/stdout and /bin/stderr, then load through descriptor-backed VFS open/read
and the accepted userspace launch/status path. The producer writes fd1 to the
pipe endpoint. The final-stage consumer inherits fd0 from that pipe endpoint
and receives only a child-owned fd2 redirection to
volatile-vfs:/tmp/pipeline-stderr.txt. The first pipeline truncates/sinks the
stderr fixture into the volatile file; the second pipeline appends the same
fixture at EOF.

## Non-Goals

This task does not accept environment-backed PATH, current-directory search,
command lookup beyond bounded /bin, path-containing bare-name stages,
input/combined pipeline redirections, stdout final-stage redirection for this
pipeline shape, arbitrary output paths, persistent writable filesystem
behavior, generated-root retry, live networking/SSH, Pi 5 hardware action, or
a phase transition.

## Findings

- fixed: Added fixed-/bin bare-name pipeline consumer parsing for exactly
  'stdout | stderr 2>>/tmp/pipeline-stderr.txt'.
- fixed: Reused the accepted stderr volatile regular-file append descriptor
  path for the final stage only, preserving child-only fd2 redirection and
  shell fd2 restoration.
- fixed: Extended the bare-name pipeline stderr QEMU/substitute smoke to prove
  truncate followed by append, descriptor-backed readback of two stderr fixture
  writes, waitpid/laststatus/proc/ps/pipestatus coherence, and fail-closed
  neighboring forms.
- fixed: Updated the direct pipeline stderr retained control so its unsupported
  bare-name negative now uses an unsupported append target rather than the
  newly accepted bare-name witness.
- fixed: Added a boundary string for the fixed-/bin bare-name pipeline
  consumer stderr append capability in the local command-loop status surface.
- not-an-issue: The stderr fixture intentionally writes to stderr and does not
  read stdin; the retained pipeline record therefore reports producer
  bytes-written=0x1f, consumer bytes-read=0, and reader-eof=false for both
  truncate and append runs while still proving the consumer inherited fd0 as
  the pipe endpoint.
- deferred: Input/combined pipeline redirections, stdout final-stage
  redirection for this pipeline shape, arbitrary paths, persistent storage,
  generated-root retry, live network/SSH, Pi 5 hardware proof, and phase
  transition remain outside this task.

## Evidence

- static inspection:
  - src/local_command_loop.rs parser/path-policy/runtime and regression diff.
  - src/target/qemu_virt.rs QEMU scenario dispatch expectations.
  - scripts/qemu-local-serial-command-loop-smoke.sh command driver and grep
    assertions.
  - docs/src/roadmap.md.
  - docs/src/project/phase12-networking-ssh.md.
  - docs/src/project/early-posix-shape.md.
- focused QEMU/substitute transcript:
  tasks/evidence/2026-06-28-phase12-local-bare-name-pipeline-stderr-append-regular-file-redirection-core/qemu-local-shell-bare-name-pipeline-stderr-regular-file-redirection-smoke.log.
- retained regression controls:
  tasks/evidence/2026-06-28-phase12-local-bare-name-pipeline-stderr-append-regular-file-redirection-core/retained-regression-smoke-summary.txt.
- classification:
  tasks/evidence/2026-06-28-phase12-local-bare-name-pipeline-stderr-append-regular-file-redirection-core/classification.json.
- evidence map:
  tasks/evidence/2026-06-28-phase12-local-bare-name-pipeline-stderr-append-regular-file-redirection-core/evidence-map.json.

## Validation

- 'cargo fmt --all -- --check': passed.
- 'cargo -Zjson-target-spec test --quiet': passed; no_std test runner reported
  'test result: ok. 866 passed'.
- 'scripts/qemu-local-shell-bare-name-pipeline-stderr-regular-file-redirection-smoke.sh':
  passed; transcript reports 'final participants=23 expected=23 errors=0'.
- Retained local POSIX/VFS controls passed for direct pipeline stderr append,
  pipeline-output append, process-status VFS, ps, pipestatus, and
  descriptor-backed cat-banner readback.
- 'jq empty' on task-owned classification/evidence JSON: passed.
- 'git diff --check': passed.
- '/home/node/.cargo/bin/mdbook build': passed.
- 'git diff --cached --check': passed before commit.

## Result

Accepted boundary: the fixed-/bin bare-name sequence
'stdout | stderr 2>/tmp/pipeline-stderr.txt' then
'stdout | stderr 2>>/tmp/pipeline-stderr.txt' succeeds only through bounded
/bin lookup, descriptor-backed VFS/userspace execution, and child-only
final-stage fd2 redirection. Descriptor-backed 'cat /tmp/pipeline-stderr.txt'
reads two userspace stderr fixture writes bytes=0x3e in order, and a later
normal 'stderr' proves shell fd2 restoration.

Unsupported command names, path-containing stage names, stdout final-stage
redirection, input redirection, unsupported append targets, malformed spacing,
and arbitrary/persistent paths fail closed without successful process records
or file writes.

Live network/SSH remains paused. No Pi 5 hardware claim is made.

selected_next_task:
phase12-local-bare-name-pipeline-stderr-append-regular-file-redirection-closeout-20260628.
