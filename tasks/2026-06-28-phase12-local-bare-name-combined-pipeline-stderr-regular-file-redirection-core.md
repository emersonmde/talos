# Phase 12 Local Bare-Name Combined Pipeline Stderr Regular-File Redirection Core

Task id:
phase12-local-bare-name-combined-pipeline-stderr-regular-file-redirection-core-20260628

Status: accepted; commit hash recorded in durable supervisor state after
commit.

## Scope

Implemented the exact fixed-/bin bare-name combined pipeline stdin/stderr
regular-file redirection witness:

~~~text
stdin </etc/banner.txt | stderr 2>/tmp/pipeline-combined-stderr.txt
~~~

Both stage names resolve only through bounded /bin lookup to /bin/stdin and
/bin/stderr before descriptor-backed VFS open/read and userspace launch/status.
The producer keeps child-only fd0 sourced from initramfs:/etc/banner.txt, fd1
bound to the serialized pipe endpoint, inherited fd2, and closed loader
temporary descriptors. The final-stage stderr consumer inherits fd0 from the
pipe endpoint, keeps fd1 inherited, and rebinds child-only fd2 to
volatile-vfs:/tmp/pipeline-combined-stderr.txt with truncate/sink semantics.

## Non-Goals

This task does not accept direct path-form runtime expansion beyond retained
controls, combined pipeline stderr append, arbitrary input or output paths,
persistent writable filesystem behavior, separated redirection-token grammar,
explicit fd2 syntax beyond compact 2>, broad shell grammar,
PATH/current-directory lookup, command lookup beyond the bounded /bin surface,
unbounded/concurrent pipelines, live networking/SSH, Pi 5 hardware action,
generated-root retry, or a phase transition.

## Findings

- fixed: Admitted the exact bare-name stderr pipeline consumer target
  /tmp/pipeline-combined-stderr.txt while keeping it routed through fixed /bin
  resolution to /bin/stderr.
- fixed: Added a task-owned unit witness proving bounded /bin resolution,
  descriptor-backed producer stdin, serialized pipe handoff, child-only fd2
  sink rebinding, descriptor-backed cat readback, and normal bare-name/direct
  descriptor restoration controls.
- fixed: Added fail-closed coverage for unsupported command names,
  path-containing mixed stage names, append form, wrong output paths, stdout
  producer, separated redirection tokens, explicit alternate fd syntax,
  persistent /var target, and multistage combined redirection without
  process-table mutation.
- fixed: Added a task-owned QEMU/substitute smoke scenario, wrapper, command
  dispatch expectations, build/cfg plumbing, docs, and evidence files.
- not-an-issue: /bin/stderr does not consume fd0; the accepted pipe record
  therefore shows bytes-read=0 and reader-eof=false while the descriptor table
  still proves fd0 is the pipe endpoint.
- not-an-issue: The QEMU-substitute cargo test harness runs the full no_std
  test binary for the focused local_command_loop filter; the focused log
  therefore reports the full suite count while exercising this test.
- deferred: combined pipeline stderr append, arbitrary paths, persistent
  storage, separated redirection-token grammar, explicit alternate fd syntax,
  mixed direct/bare path forms, broad shell grammar, live networking/SSH, Pi 5
  hardware proof, generated-root retry, and phase transition remain outside
  this task.

## Evidence

- static inspection:
  src/local_command_loop.rs, src/target/qemu_virt.rs, build.rs,
  scripts/qemu-local-serial-command-loop-smoke.sh, and
  scripts/qemu-local-shell-bare-name-combined-pipeline-stderr-regular-file-redirection-smoke.sh.
- QEMU/substitute focused transcript:
  tasks/evidence/2026-06-28-phase12-local-bare-name-combined-pipeline-stderr-regular-file-redirection-core/qemu-substitute-focused-test.log.
- QEMU/substitute local_command_loop regression transcript:
  tasks/evidence/2026-06-28-phase12-local-bare-name-combined-pipeline-stderr-regular-file-redirection-core/local-command-loop-regression-test.log.
- QEMU/substitute local shell transcript:
  tasks/evidence/2026-06-28-phase12-local-bare-name-combined-pipeline-stderr-regular-file-redirection-core/qemu-local-shell-bare-name-combined-pipeline-stderr-regular-file-redirection-smoke.log.
- wrapper transcript:
  tasks/evidence/2026-06-28-phase12-local-bare-name-combined-pipeline-stderr-regular-file-redirection-core/qemu-local-shell-bare-name-combined-pipeline-stderr-regular-file-redirection-smoke-wrapper.log.
- classification:
  tasks/evidence/2026-06-28-phase12-local-bare-name-combined-pipeline-stderr-regular-file-redirection-core/classification.json.
- evidence map:
  tasks/evidence/2026-06-28-phase12-local-bare-name-combined-pipeline-stderr-regular-file-redirection-core/evidence-map.json.

## Validation

- cargo fmt --all -- --check: passed.
- cargo -Zjson-target-spec test --quiet
  local_command_loop_runs_bare_name_pipeline_combined_stdin_stderr_redirection:
  passed.
- cargo -Zjson-target-spec test --quiet local_command_loop: passed.
- scripts/qemu-local-shell-bare-name-combined-pipeline-stderr-regular-file-redirection-smoke.sh:
  passed; the transcript reports final participants=25 expected=25 errors=0.
- jq empty on task-owned classification and evidence-map JSON: passed.
- git diff --check: passed.
- /home/node/.cargo/bin/mdbook build: passed.
- git diff --cached --check: passed before commit.

## Result

Accepted boundary: the exact fixed-/bin bare-name witness succeeds only through
bounded /bin resolution to /bin/stdin and /bin/stderr plus descriptor-backed
VFS/userspace execution, not a kernel-only shim. Descriptor-backed
cat /tmp/pipeline-combined-stderr.txt reads the userspace stderr fixture from
the volatile VFS file, and later normal stdin, direct combined stderr, and
/bin/stderr controls prove shell descriptor restoration.

Unsupported neighboring bare-name forms remain fail-closed without successful
process records or file writes: unsupported command names, path-containing
mixed stage names, append form, wrong output paths, stdout producer, separated
redirection tokens, explicit alternate fd syntax, persistent /var target,
multistage combined redirection, PATH/current-directory lookup, and
persistent-storage claims.

Live networking/SSH remains paused. No Pi 5 hardware claim is made.

selected_next_task:
phase12-local-combined-pipeline-stderr-regular-file-redirection-frontier-checkpoint-20260628.
