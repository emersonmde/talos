# Phase 12 Local Bare-Name Combined Pipeline Stderr Append Redirection Core

Task id:
phase12-local-bare-name-combined-pipeline-stderr-append-redirection-core-20260628

Status: accepted; final commit recorded in durable supervisor state.

## Scope

Implemented the exact fixed-/bin bare-name combined pipeline stderr
truncate-then-append sequence:

~~~text
stdin </etc/banner.txt | stderr 2>/tmp/pipeline-combined-stderr-append.txt
stdin </etc/banner.txt | stderr 2>>/tmp/pipeline-combined-stderr-append.txt
~~~

The producer resolves through bounded /bin lookup to /bin/stdin, keeps
child-only fd0 sourced from initramfs:/etc/banner.txt, and writes fd1 to the
serialized pipe endpoint. The final-stage consumer resolves through bounded
/bin lookup to /bin/stderr, keeps fd0 from that pipe endpoint, inherited fd1,
and child-only fd2 rebound to
volatile-vfs:/tmp/pipeline-combined-stderr-append.txt. The first command uses
truncate/sink semantics; the second command appends at EOF. The
descriptor-backed readback reports 0x3e bytes, proving two userspace stderr
fixture writes in order from the volatile VFS file.

## Non-Goals

This task does not accept mixed direct/bare path forms as a new surface,
command lookup beyond fixed /bin for the exact witness, arbitrary input/output
paths, persistent writable filesystem behavior, separated redirection-token
grammar as an accepted form, explicit alternate fd syntax, broad shell grammar,
globbing, variables, environment-backed PATH, current-directory lookup,
unbounded or concurrent pipelines, live networking or SSH, Pi 5 hardware
action, generated-root retry, boot archive publication, or a phase transition.

## Findings

- fixed: Added the exact bare-name combined pipeline stderr append parser
  admission by reusing the already accepted volatile VFS combined stderr
  append path.
- fixed: Added task-owned unit/QEMU-substitute coverage, QEMU smoke wrapper,
  boot scenario registration, qemu_virt command classification, and evidence
  files for the fixed-/bin bare-name witness.
- fixed: Preserved bounded /bin argv0 resolution to /bin/stdin and
  /bin/stderr while keeping child-only fd2 rebinding and shell fd0/fd1/fd2
  restoration across the truncate-then-append sequence.
- fixed: Retained the direct path-form combined stderr append witness as a
  regression/control surface in the smoke transcript.
- not-an-issue: /bin/stderr does not consume fd0, so accepted pipe records
  correctly show bytes-read=0 and reader-eof=false while the consumer
  descriptor table proves fd0 is the pipe endpoint.
- not-an-issue: The direct path-form control writes the third stderr fixture
  to the volatile file after the accepted readback; the task-owned smoke
  intentionally checks only the first two bare-name writes through
  descriptor-backed cat before that control.
- deferred: arbitrary paths, persistent writable filesystem behavior,
  environment-backed PATH, current-directory search, command lookup beyond
  bounded /bin, arbitrary shell grammar, scheduler concurrency, fork/signals,
  process groups/sessions, live networking/SSH, Pi 5 hardware proof,
  generated-root retry, and phase transition remain outside this task.

## Evidence

- static inspection:
  src/local_command_loop.rs, src/target/qemu_virt.rs, src/main.rs, build.rs,
  scripts/qemu-local-serial-command-loop-smoke.sh, and
  scripts/qemu-local-shell-bare-name-combined-pipeline-stderr-append-redirection-smoke.sh.
- QEMU/substitute focused transcript:
  tasks/evidence/2026-06-28-phase12-local-bare-name-combined-pipeline-stderr-append-redirection-core/qemu-substitute-focused-test.log.
- QEMU/substitute local_command_loop regression transcript:
  tasks/evidence/2026-06-28-phase12-local-bare-name-combined-pipeline-stderr-append-redirection-core/local-command-loop-regression-test.log.
- QEMU/substitute local shell transcript:
  tasks/evidence/2026-06-28-phase12-local-bare-name-combined-pipeline-stderr-append-redirection-core/qemu-local-shell-bare-name-combined-pipeline-stderr-append-redirection-smoke.log.
- retained regression summary:
  tasks/evidence/2026-06-28-phase12-local-bare-name-combined-pipeline-stderr-append-redirection-core/retained-regression-smoke-summary.txt.
- classification:
  tasks/evidence/2026-06-28-phase12-local-bare-name-combined-pipeline-stderr-append-redirection-core/classification.json.
- evidence map:
  tasks/evidence/2026-06-28-phase12-local-bare-name-combined-pipeline-stderr-append-redirection-core/evidence-map.json.

## Validation

- cargo -Zjson-target-spec test --quiet
  local_command_loop_appends_bare_name_pipeline_combined_stdin_stderr_redirection:
  passed; the QEMU/substitute no_std harness reports 874 passed.
- cargo -Zjson-target-spec test --quiet local_command_loop: passed; the
  QEMU/substitute no_std harness reports 874 passed.
- scripts/qemu-local-shell-bare-name-combined-pipeline-stderr-append-redirection-smoke.sh:
  passed on alternate port 54453 after the default port was occupied; the
  transcript reports final participants=26 expected=26 errors=0.
- bash -n on the shared smoke script and task smoke wrapper: passed.
- jq empty on task-owned classification and evidence-map JSON: passed.
- cargo fmt --all -- --check: passed.
- git diff --check: passed.
- mdbook build: passed with the existing large search-index warning.
- git diff --cached --check before commit: passed.

## Result

Accepted boundary: the exact fixed-/bin bare-name combined pipeline stderr
truncate-then-append sequence succeeds through bounded /bin resolution and
descriptor-backed VFS/userspace execution, not a kernel-only shim.
Descriptor-backed cat /tmp/pipeline-combined-stderr-append.txt reads two
userspace stderr fixture writes from the volatile VFS file, and later stdin,
stderr, and direct path-form controls prove shell descriptor restoration and
retained direct-path behavior.

Unsupported neighboring forms remain fail-closed without accepted process or
file-write effects: mixed direct/bare stage names, command lookup beyond
bounded /bin, wrong output paths, separated redirection tokens, explicit
alternate fd syntax, persistent /var targets, arbitrary paths, multistage
combined redirection, unsupported producer names, and unsupported consumer
names.

Live networking/SSH remains paused. No Pi 5 hardware claim is made.

selected_next_task:
phase12-local-combined-pipeline-stderr-append-redirection-frontier-checkpoint-20260628.
