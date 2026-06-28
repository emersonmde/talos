# Phase 12 Local Bare-Name Combined Pipeline Stdout Append Redirection Core

Task id:
phase12-local-bare-name-combined-pipeline-stdout-append-redirection-core-20260628

Status: accepted; commit hash recorded in durable supervisor state after
commit.

## Scope

Implemented the exact fixed-/bin bare-name combined pipeline stdout append
witness:

~~~text
stdin </etc/banner.txt | stdin >/tmp/pipeline-combined-append.txt
stdin </etc/banner.txt | stdin >>/tmp/pipeline-combined-append.txt
~~~

Both stages resolve only through the bounded /bin lookup to /bin/stdin before
using descriptor-backed VFS open/read, the accepted loader/userspace
launch/status path, serialized pipe handoff, and volatile VFS regular-file
rebinding. The first consumer truncates/sinks to
volatile-vfs:/tmp/pipeline-combined-append.txt; the second appends at EOF.

## Non-Goals

This task does not accept combined stderr append, arbitrary input/output paths,
persistent writable filesystem behavior, separated redirection-token grammar,
explicit fd1 syntax, broad shell grammar, environment-backed PATH,
current-directory search, command lookup beyond the bounded /bin surface,
unbounded/concurrent pipelines, live networking/SSH, Pi 5 hardware action,
generated-root retry, or a phase transition.

## Findings

- fixed: Added the exact bare-name consumer stdout truncate/append parser
  allowance for /tmp/pipeline-combined-append.txt while keeping it routed
  through fixed /bin resolution to /bin/stdin.
- fixed: Added a task-owned unit witness that proves truncate then
  append-at-EOF behavior, descriptor-backed cat readback of two nested stdin
  reports, normal bare-name/direct fd restoration controls, and fail-closed
  unsupported neighbors without process-table mutation.
- fixed: Added a task-owned QEMU/substitute smoke scenario, wrapper, command
  dispatch expectations, and harness assertions for the 24-command bare-name
  append transcript.
- fixed: Updated the retained direct combined pipeline stdout append smoke so
  its unsupported neighbor remains a mixed direct/bare form now that the pure
  bare-name append witness is accepted.
- fixed: Added build/cfg plumbing and docs/evidence updates for the accepted
  fixed-/bin bare-name append frontier.
- not-an-issue: The QEMU-substitute cargo test harness still runs the full
  no_std test binary for a focused local_command_loop filter; the focused log
  therefore reports the full suite count while exercising this test.
- deferred: combined stderr append, arbitrary paths, persistent writable
  filesystem behavior, separated redirection-token grammar, explicit fd1
  syntax, broad shell grammar, live networking/SSH, Pi 5 hardware proof,
  generated-root retry, and phase transition remain outside this task.

## Evidence

- static inspection:
  src/local_command_loop.rs, src/target/qemu_virt.rs, build.rs, src/main.rs,
  scripts/qemu-local-serial-command-loop-smoke.sh, and
  scripts/qemu-local-shell-bare-name-combined-pipeline-stdout-append-redirection-smoke.sh.
- QEMU/substitute focused transcript:
  tasks/evidence/2026-06-28-phase12-local-bare-name-combined-pipeline-stdout-append-redirection-core/qemu-substitute-focused-test.log.
- QEMU/substitute local_command_loop transcript:
  tasks/evidence/2026-06-28-phase12-local-bare-name-combined-pipeline-stdout-append-redirection-core/qemu-substitute-local-command-loop.log.
- QEMU/substitute local shell transcript:
  tasks/evidence/2026-06-28-phase12-local-bare-name-combined-pipeline-stdout-append-redirection-core/qemu-local-shell-bare-name-combined-pipeline-stdout-append-redirection-smoke.log.
- retained regression summary:
  tasks/evidence/2026-06-28-phase12-local-bare-name-combined-pipeline-stdout-append-redirection-core/retained-regression-smoke-summary.txt.
- classification:
  tasks/evidence/2026-06-28-phase12-local-bare-name-combined-pipeline-stdout-append-redirection-core/classification.json.
- evidence map:
  tasks/evidence/2026-06-28-phase12-local-bare-name-combined-pipeline-stdout-append-redirection-core/evidence-map.json.

## Validation

- 'cargo fmt --all -- --check': passed.
- 'cargo -Zjson-target-spec test --quiet
  local_command_loop_appends_bare_name_pipeline_combined_stdin_stdout_redirection':
  passed.
- 'cargo -Zjson-target-spec test --quiet local_command_loop': passed.
- 'scripts/qemu-local-shell-bare-name-combined-pipeline-stdout-append-redirection-smoke.sh
  --quiet': passed; the transcript reports 'final participants=24 expected=24
  errors=0'.
- retained local POSIX/VFS QEMU/substitute smoke gates passed for direct and
  bare-name combined pipeline stdin/stdout, direct combined pipeline stdout
  append, direct and bare-name pipeline stdout append, direct and bare-name
  pipeline stderr redirection, process-status VFS, ps, pipestatus, waitpid,
  and laststatus.
- 'jq empty' on task-owned classification and evidence-map JSON passed.
- 'git diff --check' passed.
- '/home/node/.cargo/bin/mdbook build' passed.
- 'git diff --cached --check' passed before commit.

## Result

Accepted boundary: the fixed-/bin bare-name sequence above succeeds through
bounded /bin resolution to /bin/stdin and descriptor-backed VFS/userspace
execution, not a kernel-only shim. Descriptor-backed
'cat /tmp/pipeline-combined-append.txt' reads two nested userspace stdin
reports in order, and later normal bare-name/direct controls prove shell fd0
and fd1 restoration.

Unsupported neighboring bare-name forms remain fail-closed without successful
process records or file writes: unsupported command names, path-containing
mixed stage names, combined stderr append, wrong output paths, explicit fd1
syntax, separated redirection tokens, persistent /var target, multistage
combined redirection, PATH/current-directory lookup, and persistent-storage
claims.

Live networking/SSH remains paused. No Pi 5 hardware claim is made.

selected_next_task:
phase12-local-combined-pipeline-stdout-append-redirection-frontier-checkpoint-20260628.
