# Phase 12 Local Direct Combined Pipeline Stdout Append Redirection Core

Task id:
phase12-local-direct-combined-pipeline-stdout-append-redirection-core-20260628

Status: accepted; commit hash recorded in durable supervisor state after
commit.

## Scope

Implemented the exact direct path-form combined pipeline stdout append
witness:

~~~text
/bin/stdin </etc/banner.txt | /bin/stdin >/tmp/pipeline-combined-append.txt
/bin/stdin </etc/banner.txt | /bin/stdin >>/tmp/pipeline-combined-append.txt
~~~

Both producer stages keep child-only fd0 sourced from
initramfs:/etc/banner.txt, fd1 bound to the serialized pipe endpoint, inherited
fd2, closed loader temporaries, and accepted userspace launch/status records.
The final-stage consumers keep fd0 from the pipe endpoint, inherited fd2, and
child-only fd1 rebound to
volatile-vfs:/tmp/pipeline-combined-append.txt. The first consumer uses
truncate/sink semantics and the second records append-at-EOF semantics.

## Non-Goals

This task does not accept fixed-/bin bare-name combined pipeline append,
combined stderr append, arbitrary paths, persistent writable filesystem
behavior, separated redirection-token grammar, explicit fd1 syntax, PATH or
current-directory lookup, unbounded/concurrent pipelines, live networking/SSH,
Pi 5 hardware action, generated-root retry, or a phase transition.

## Findings

- fixed: Added the exact
  '/tmp/pipeline-combined-append.txt' volatile path and allowed it only for
  the direct combined stdin/stdout pipeline append witness.
- fixed: Reused the existing child-only final-stage stdout sink/append
  descriptor path so the first command truncates the volatile file and the
  second appends at EOF without persistent storage claims.
- fixed: Kept ordinary pipeline-output append from broadening into the combined
  append target by rejecting '/bin/stdout | /bin/stdin
  >>/tmp/pipeline-combined-append.txt' before any process records or file
  writes.
- fixed: Increased bounded test capture space for the larger combined append
  regression transcript.
- fixed: Added a task-owned QEMU/substitute smoke scenario, wrapper, dispatch
  expectations, harness assertions, classification JSON, evidence map,
  retained regression summary, and docs updates.
- not-an-issue: The QEMU-substitute cargo test harness still runs the full
  no_std test binary for a focused local_command_loop filter; the retained
  focused transcript therefore reports the whole suite count while exercising
  this test.
- deferred: fixed-/bin bare-name combined pipeline append is the next queued
  task.
- deferred: combined stderr append, arbitrary paths, persistent writable
  filesystem behavior, separated redirection-token grammar, explicit fd1
  syntax, broad shell grammar, live networking/SSH, Pi 5 hardware proof,
  generated-root retry, and phase transition remain outside this task.

## Evidence

- static inspection:
  src/local_command_loop.rs, src/target/qemu_virt.rs, build.rs,
  scripts/qemu-local-serial-command-loop-smoke.sh, and
  scripts/qemu-local-shell-direct-combined-pipeline-stdout-append-redirection-smoke.sh.
- QEMU/substitute focused transcript:
  tasks/evidence/2026-06-28-phase12-local-direct-combined-pipeline-stdout-append-redirection-core/qemu-substitute-focused-test.log.
- QEMU/substitute local shell transcript:
  tasks/evidence/2026-06-28-phase12-local-direct-combined-pipeline-stdout-append-redirection-core/qemu-local-shell-direct-combined-pipeline-stdout-append-redirection-smoke.log.
- retained regression summary:
  tasks/evidence/2026-06-28-phase12-local-direct-combined-pipeline-stdout-append-redirection-core/retained-regression-smoke-summary.txt.
- classification:
  tasks/evidence/2026-06-28-phase12-local-direct-combined-pipeline-stdout-append-redirection-core/classification.json.
- evidence map:
  tasks/evidence/2026-06-28-phase12-local-direct-combined-pipeline-stdout-append-redirection-core/evidence-map.json.

## Validation

- 'cargo fmt --all -- --check': passed.
- 'cargo -Zjson-target-spec test --quiet
  local_command_loop_appends_direct_path_pipeline_combined_stdin_stdout_redirection':
  passed.
- 'cargo -Zjson-target-spec test --quiet local_command_loop': passed.
- 'scripts/qemu-local-shell-direct-combined-pipeline-stdout-append-redirection-smoke.sh
  --quiet': passed; the transcript reports 'final participants=23 expected=23
  errors=0'.
- retained local POSIX/VFS QEMU/substitute smoke gates passed for combined
  pipeline stdin/stdout, pipeline stdout append, pipeline stderr redirection,
  process-status VFS, ps, pipestatus, waitpid, and laststatus.
- 'jq empty' on task-owned classification and evidence-map JSON passed.
- 'git diff --check' passed.
- '/home/node/.cargo/bin/mdbook build' passed.
- 'git diff --cached --check' passed before commit.

## Result

Accepted boundary: the direct path-form sequence above succeeds through
descriptor-backed VFS/userspace execution, not a kernel-only shim.
Descriptor-backed 'cat /tmp/pipeline-combined-append.txt' reads two nested
userspace stdin reports in order, and later normal
'/bin/stdin </etc/banner.txt' proves shell fd0/fd1 restoration.

Unsupported neighboring direct forms remain fail-closed without successful
process records or file writes: bare-name combined append, combined stderr
append, wrong output path, stdout producer with the combined append target,
explicit fd1 syntax, separated redirection tokens, persistent /var target,
multistage combined redirection, unsupported stage names, PATH/current-directory
lookup, and persistent-storage claims.

Live networking/SSH remains paused. No Pi 5 hardware claim is made.

selected_next_task:
phase12-local-bare-name-combined-pipeline-stdout-append-redirection-core-20260628.
