# Phase 12 Local Direct Combined Pipeline Stderr Append Redirection Core

Task id:
phase12-local-direct-combined-pipeline-stderr-append-redirection-core-20260628

Status: accepted; commit pending.

## Scope

Implemented the exact direct path-form combined pipeline stderr truncate-then-append
sequence:

~~~text
/bin/stdin </etc/banner.txt | /bin/stderr 2>/tmp/pipeline-combined-stderr-append.txt
/bin/stdin </etc/banner.txt | /bin/stderr 2>>/tmp/pipeline-combined-stderr-append.txt
~~~

The producer keeps child-only fd0 sourced from initramfs:/etc/banner.txt, fd1
bound to the serialized pipe endpoint, inherited fd2, a closed loader temporary
descriptor, and coherent userspace launch/status records. The final-stage
consumer keeps fd0 from the pipe endpoint, inherited fd1, and child-only fd2
rebound to volatile-vfs:/tmp/pipeline-combined-stderr-append.txt. The first
command uses truncate/sink semantics; the second command appends at EOF. The
descriptor-backed readback reports 0x3e bytes, proving two userspace stderr
fixture writes in order from the volatile VFS file.

## Non-Goals

This task does not accept the fixed-/bin bare-name combined pipeline stderr
append form, stdout final-stage append broadening beyond accepted controls,
arbitrary input/output paths, persistent writable filesystem behavior,
separated redirection-token grammar as an accepted form, explicit alternate fd
syntax, broad shell grammar, globbing, variables, environment-backed PATH,
current-directory lookup, unbounded or concurrent pipelines, live networking or
SSH, Pi 5 hardware action, generated-root retry, boot archive publication, or a
phase transition.

## Findings

- fixed: Added the exact /tmp/pipeline-combined-stderr-append.txt volatile VFS
  path and direct combined stdin/stderr append predicate support.
- fixed: Admitted only direct path-form compact 2> and 2>> combined stderr
  append target parsing for the absolute pipeline consumer.
- fixed: Added task-owned unit/QEMU-substitute coverage, smoke wrapper, boot
  scenario registration, qemu_virt command classification, and evidence files.
- fixed: Preserved child-only fd2 rebinding and shell fd0/fd1/fd2 restoration
  across the truncate-then-append sequence.
- not-an-issue: /bin/stderr does not consume fd0, so both accepted pipe records
  correctly show bytes-read=0 and reader-eof=false while the consumer
  descriptor table still proves fd0 is the pipe endpoint.
- not-an-issue: The long multistage negative-control line reaches the TTY line
  limit and fails closed as input-error line-complete; no successful process or
  file-write effect is recorded.
- deferred: fixed-/bin bare-name combined pipeline stderr append remains the
  queued successor task.
- deferred: arbitrary paths, persistent writable filesystem behavior,
  environment-backed PATH, current-directory search, command lookup beyond
  bounded /bin, arbitrary shell grammar, scheduler concurrency, fork/signals,
  process groups/sessions, live networking/SSH, Pi 5 hardware proof,
  generated-root retry, and phase transition remain outside this task.

## Evidence

- static inspection:
  src/local_command_loop.rs, src/target/qemu_virt.rs, src/main.rs, build.rs,
  scripts/qemu-local-serial-command-loop-smoke.sh, and
  scripts/qemu-local-shell-direct-combined-pipeline-stderr-append-redirection-smoke.sh.
- QEMU/substitute focused transcript:
  tasks/evidence/2026-06-28-phase12-local-direct-combined-pipeline-stderr-append-redirection-core/qemu-substitute-focused-test.log.
- QEMU/substitute local_command_loop regression transcript:
  tasks/evidence/2026-06-28-phase12-local-direct-combined-pipeline-stderr-append-redirection-core/local-command-loop-regression-test.log.
- QEMU/substitute local shell transcript:
  tasks/evidence/2026-06-28-phase12-local-direct-combined-pipeline-stderr-append-redirection-core/qemu-local-shell-direct-combined-pipeline-stderr-append-redirection-smoke.log.
- retained regression summary:
  tasks/evidence/2026-06-28-phase12-local-direct-combined-pipeline-stderr-append-redirection-core/retained-regression-smoke-summary.txt.
- classification:
  tasks/evidence/2026-06-28-phase12-local-direct-combined-pipeline-stderr-append-redirection-core/classification.json.
- evidence map:
  tasks/evidence/2026-06-28-phase12-local-direct-combined-pipeline-stderr-append-redirection-core/evidence-map.json.

## Validation

- cargo -Zjson-target-spec test --quiet
  local_command_loop_appends_direct_path_pipeline_combined_stdin_stderr_redirection:
  passed; the QEMU/substitute no_std harness reports 873 passed.
- cargo -Zjson-target-spec test --quiet local_command_loop: passed; the
  QEMU/substitute no_std harness reports 873 passed.
- scripts/qemu-local-shell-direct-combined-pipeline-stderr-append-redirection-smoke.sh:
  passed; the transcript reports final participants=24 expected=24 errors=0.
- bash -n on the shared smoke script and task smoke wrapper: passed.
- jq empty on task-owned classification and evidence-map JSON: passed.
- cargo fmt --all -- --check: passed.
- git diff --check: passed.
- mdbook build: passed.
- git diff --cached --check before commit: passed.

## Result

Accepted boundary: the exact direct path-form combined pipeline stderr
truncate-then-append sequence succeeds through descriptor-backed VFS/userspace
execution, not a kernel-only shim. Descriptor-backed cat
/tmp/pipeline-combined-stderr-append.txt reads two userspace stderr fixture
writes from the volatile VFS file, and later /bin/stdin and /bin/stderr
controls prove shell descriptor restoration.

Unsupported neighboring direct forms remain fail-closed without accepted
process/file-write effects: bare-name combined stderr append, wrong output
path, stdout producer with the combined stderr append target, separated
redirection tokens, explicit alternate fd syntax, /var target, multistage
combined redirection at the TTY line boundary, and unsupported stage names.

Live networking/SSH remains paused. No Pi 5 hardware claim is made.

selected_next_task:
phase12-local-bare-name-combined-pipeline-stderr-append-redirection-core-20260628.
