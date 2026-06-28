# Phase 12 Local Direct Pipeline Stderr Append Regular-File Redirection Core

Task id: phase12-local-direct-pipeline-stderr-append-regular-file-redirection-core-20260628

Status: accepted; commit hash recorded in durable supervisor state after
commit.

## Scope

Accept the smallest direct path-form pipeline final-stage stderr append
regular-file redirection witness:

~~~text
/bin/stdout | /bin/stderr 2>/tmp/pipeline-stderr.txt
/bin/stdout | /bin/stderr 2>>/tmp/pipeline-stderr.txt
~~~

Both stages load through descriptor-backed VFS open/read and the accepted
userspace launch/status path. The producer writes fd1 to the pipe endpoint. The
final-stage consumer inherits fd0 from that pipe endpoint and receives only a
child-owned fd2 redirection to volatile-vfs:/tmp/pipeline-stderr.txt. The first
pipeline truncates/sinks the stderr fixture into the volatile file; the second
pipeline appends the same fixture at EOF.

## Non-Goals

This task does not accept fixed-/bin bare-name pipeline stderr append,
input/combined pipeline redirections, stdout final-stage redirection for this
pipeline shape, arbitrary output paths, persistent writable filesystem
behavior, generated-root retry, live networking/SSH, Pi 5 hardware action, or a
phase transition.

## Findings

- fixed: Added direct absolute-path pipeline consumer parsing for exactly
  '/bin/stdout | /bin/stderr 2>>/tmp/pipeline-stderr.txt'.
- fixed: Reused the existing volatile stderr regular-file append descriptor path
  for the final stage only, preserving child-only fd2 redirection and shell fd2
  restoration.
- fixed: Extended the direct pipeline stderr support predicate and pipeline
  source label so append records as
  'shell-pipe-consumer-stderr-append-redirection'.
- fixed: Extended the focused QEMU local serial scenario and command-loop
  regression to prove truncate followed by append, descriptor-backed readback
  of two stderr fixture writes, waitpid/laststatus/proc/ps/pipestatus
  coherence, and fail-closed neighboring forms.
- fixed: Added a boundary string for the direct path-form pipeline consumer
  stderr append capability in the local command-loop status surface.
- not-an-issue: The stderr fixture intentionally writes to stderr and does not
  read stdin; the retained pipeline record therefore reports producer
  bytes-written=0x1f, consumer bytes-read=0, and reader-eof=false for both
  truncate and append runs while still proving the consumer inherited fd0 as the
  pipe endpoint.
- deferred: Fixed-/bin bare-name pipeline stderr append is separate future work.
- deferred: Input/combined pipeline redirections, stdout final-stage redirection
  for this pipeline shape, arbitrary paths, persistent storage,
  generated-root retry, live network/SSH, Pi 5 hardware proof, and phase
  transition remain outside this task.

## Evidence

- static inspection:
  - src/local_command_loop.rs parser/path-policy/runtime and regression diff.
  - src/target/qemu_virt.rs QEMU scenario dispatch and transcript assertions.
  - scripts/qemu-local-serial-command-loop-smoke.sh command driver and grep
    assertions.
  - docs/src/roadmap.md.
  - docs/src/project/phase12-networking-ssh.md.
  - docs/src/project/early-posix-shape.md.
- focused QEMU/substitute transcript:
  tasks/evidence/2026-06-28-phase12-local-direct-pipeline-stderr-append-regular-file-redirection-core/qemu-local-shell-direct-pipeline-stderr-append-regular-file-redirection-smoke.log.
- retained regression controls:
  - scripts/qemu-local-shell-direct-pipeline-output-append-regular-file-redirection-smoke.sh.
  - scripts/qemu-local-shell-direct-stdout-regular-file-append-redirection-smoke.sh.
  - scripts/qemu-local-shell-direct-stderr-regular-file-append-redirection-smoke.sh.
  - scripts/qemu-local-shell-process-status-vfs-smoke.sh.
  - scripts/qemu-local-shell-ps-command-vfs-smoke.sh.
  - scripts/qemu-local-shell-pipeline-status-smoke.sh.
  - scripts/qemu-local-cat-banner-smoke.sh.
- classification:
  tasks/evidence/2026-06-28-phase12-local-direct-pipeline-stderr-append-regular-file-redirection-core/classification.json.
- evidence map:
  tasks/evidence/2026-06-28-phase12-local-direct-pipeline-stderr-append-regular-file-redirection-core/evidence-map.json.

## Validation

- 'cargo fmt --all -- --check': passed.
- 'cargo -Zjson-target-spec test --quiet': passed; no_std test runner reported
  'test result: ok. 866 passed'.
- 'scripts/qemu-local-shell-direct-pipeline-stderr-regular-file-redirection-smoke.sh':
  passed; transcript reports 'final participants=19 expected=19 errors=0'.
- Retained local POSIX/VFS controls passed for pipeline-output append, direct
  stdout append, direct stderr append, process-status VFS, ps, pipestatus, and
  descriptor-backed cat-banner readback.
- 'jq empty' on task-owned classification/evidence JSON: passed.
- 'git diff --check': passed.
- '/home/node/.cargo/bin/mdbook build': passed.
- 'git diff --cached --check': passed before commit.

## Result

Accepted boundary: the direct path-form sequence
'/bin/stdout | /bin/stderr 2>/tmp/pipeline-stderr.txt' then
'/bin/stdout | /bin/stderr 2>>/tmp/pipeline-stderr.txt' succeeds only through
the accepted descriptor-backed VFS/userspace execution path. Descriptor-backed
'cat /tmp/pipeline-stderr.txt' reads two userspace stderr fixture writes
bytes=0x3e in order, and a later normal '/bin/stderr' proves shell fd2
restoration.

Unsupported stdout final-stage redirection, input redirection, unsupported
append targets, malformed spacing, arbitrary/persistent paths, and fixed-/bin
bare-name append fail closed without successful process records or file writes.

Live network/SSH remains paused. No Pi 5 hardware claim is made.

selected_next_task:
phase12-local-direct-pipeline-stderr-append-regular-file-redirection-closeout-20260628.
