# Phase 12 Local Direct Combined Pipeline Stderr Regular-File Redirection Core

Task id:
phase12-local-direct-combined-pipeline-stderr-regular-file-redirection-core-20260628

Status: accepted; committed at
209de86c201762e5e18a258b1dbe7e0217654ade.

## Scope

Implemented the exact direct path-form combined pipeline stderr regular-file
redirection witness:

~~~text
/bin/stdin </etc/banner.txt | /bin/stderr 2>/tmp/pipeline-combined-stderr.txt
~~~

The producer keeps child-only fd0 sourced from initramfs:/etc/banner.txt, fd1
bound to the serialized pipe endpoint, inherited fd2, closed loader temporary
descriptor, and coherent userspace launch/status records. The final-stage
consumer keeps fd0 from the pipe endpoint, inherited fd1, and child-only fd2
rebound to volatile-vfs:/tmp/pipeline-combined-stderr.txt with truncate/sink
semantics. The stderr fixture writes 0x1f bytes to the volatile VFS file; it
does not read fd0, so the pipe record correctly keeps bytes-read=0 and
reader-eof=false.

## Non-Goals

This task does not accept fixed-/bin bare-name combined pipeline stderr
redirection, append form, arbitrary input or output paths, persistent writable
filesystem behavior, separated redirection-token grammar, explicit fd2 syntax
beyond compact 2>, broad shell grammar, PATH/current-directory lookup,
unbounded or concurrent pipelines, live networking/SSH, Pi 5 hardware action,
generated-root retry, or a phase transition.

## Findings

- fixed: Added the exact /tmp/pipeline-combined-stderr.txt volatile path and
  direct combined stdin/stderr pipeline predicate.
- fixed: Admitted only the direct path-form witness in the absolute pipeline
  parser; fixed-/bin bare-name combined stderr remains unsupported in this
  slice.
- fixed: Preserved the existing standalone pipeline stderr path boundary so
  /bin/stdout | /bin/stderr cannot write to the new combined output path.
- fixed: Added task-owned unit/QEMU-substitute coverage, QEMU smoke wrapper,
  boot scenario registration, command-count/dispatch expectations, and
  evidence files.
- not-an-issue: /bin/stderr does not consume fd0; the accepted pipe record
  therefore shows bytes-read=0 and reader-eof=false while the descriptor table
  still proves fd0 is the pipe endpoint.
- deferred: fixed-/bin bare-name combined pipeline stderr redirection is the
  queued successor task.
- deferred: combined pipeline stderr append, arbitrary paths, persistent
  storage, separated redirection-token grammar, explicit alternate fd syntax,
  broad shell grammar, live networking/SSH, Pi 5 hardware proof,
  generated-root retry, and phase transition remain outside this task.

## Evidence

- static inspection:
  src/local_command_loop.rs, src/target/qemu_virt.rs, src/main.rs, build.rs,
  scripts/qemu-local-serial-command-loop-smoke.sh, and
  scripts/qemu-local-shell-direct-combined-pipeline-stderr-regular-file-redirection-smoke.sh.
- QEMU/substitute focused transcript:
  tasks/evidence/2026-06-28-phase12-local-direct-combined-pipeline-stderr-regular-file-redirection-core/qemu-substitute-focused-test.log.
- QEMU/substitute local_command_loop regression transcript:
  tasks/evidence/2026-06-28-phase12-local-direct-combined-pipeline-stderr-regular-file-redirection-core/local-command-loop-regression-test.log.
- QEMU/substitute local shell transcript:
  tasks/evidence/2026-06-28-phase12-local-direct-combined-pipeline-stderr-regular-file-redirection-core/qemu-local-shell-direct-combined-pipeline-stderr-regular-file-redirection-smoke.log.
- retained regression summary:
  tasks/evidence/2026-06-28-phase12-local-direct-combined-pipeline-stderr-regular-file-redirection-core/retained-regression-smoke-summary.txt.
- classification:
  tasks/evidence/2026-06-28-phase12-local-direct-combined-pipeline-stderr-regular-file-redirection-core/classification.json.
- evidence map:
  tasks/evidence/2026-06-28-phase12-local-direct-combined-pipeline-stderr-regular-file-redirection-core/evidence-map.json.

## Validation

- cargo -Zjson-target-spec test --quiet
  local_command_loop_runs_direct_path_pipeline_combined_stdin_stderr_redirection:
  passed.
- cargo -Zjson-target-spec test --quiet local_command_loop: passed.
- scripts/qemu-local-shell-direct-combined-pipeline-stderr-regular-file-redirection-smoke.sh
  --quiet: passed; the transcript reports final participants=23 expected=23
  errors=0.
- bash -n on the shared smoke script and task smoke wrapper: passed.
- jq empty on task-owned classification and evidence-map JSON: passed.
- cargo fmt --all -- --check: passed.
- git diff --check: passed.
- mdbook build: passed.
- git diff --cached --check: passed before commit.

## Result

Accepted boundary: the exact direct path-form witness succeeds through
descriptor-backed VFS/userspace execution, not a kernel-only shim.
Descriptor-backed cat /tmp/pipeline-combined-stderr.txt reads the userspace
stderr fixture from the volatile VFS file, and later normal
/bin/stdin </etc/banner.txt, /bin/stdout, and /bin/stderr controls prove shell
descriptor restoration.

Unsupported neighboring direct forms remain fail-closed without accepted
process/file-write effects: bare-name combined stderr, append form, wrong
pipeline stderr output path, stdout producer with the combined stderr target,
separated redirection tokens, /var target, multistage combined redirection,
and unsupported stage names.

Live networking/SSH remains paused. No Pi 5 hardware claim is made.

selected_next_task:
phase12-local-bare-name-combined-pipeline-stderr-regular-file-redirection-core-20260628.
