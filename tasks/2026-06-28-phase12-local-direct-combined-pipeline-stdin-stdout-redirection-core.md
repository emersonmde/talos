# Phase 12 Local Direct Combined Pipeline Stdin Stdout Redirection Core

Task id: phase12-local-direct-combined-pipeline-stdin-stdout-redirection-core-20260628

Status: accepted; commit hash recorded in durable supervisor state after
commit.

## Scope

Implement the exact direct path-form combined pipeline redirection witness:

~~~text
/bin/stdin </etc/banner.txt | /bin/stdin >/tmp/pipeline-combined.txt
~~~

Both programs load through descriptor-backed VFS and the accepted userspace
launch/status path. The producer gets child-only fd0 from
initramfs:/etc/banner.txt, fd1 as the pipe endpoint, and inherited fd2. The
consumer gets fd0 from the pipe endpoint, child-only fd1 to
volatile-vfs:/tmp/pipeline-combined.txt, and inherited fd2. The shell fd0/fd1
surface is restored after the pipeline.

This task does not accept fixed-/bin bare-name combined pipeline redirection,
arbitrary input or output paths, append, stderr forms, separated redirection
tokens, explicit fd1 redirection, multistage combined redirection, unsupported
stage names, PATH/current-directory lookup, persistent writable filesystem
behavior, live networking/SSH, generated-root retry, Pi 5 hardware action, or
phase transition.

## Findings

- fixed: Added the exact direct path-form pipeline policy for
  '/bin/stdin </etc/banner.txt | /bin/stdin >/tmp/pipeline-combined.txt',
  reusing the accepted descriptor-backed VFS open/read, program loader,
  userspace launch/status, pipe handoff, and volatile VFS regular-file
  redirection machinery.
- fixed: Corrected the pipeline source classifier so the producer stdin
  redirection and consumer stdout redirection are matched in their actual tuple
  slots before recording
  source=shell-pipe-producer-stdin-consumer-stdout-redirection.
- fixed: Kept '/tmp/pipeline-combined.txt' out of the older pipeline-output
  regular-file path unless the producer has the exact accepted stdin
  redirection, so neighboring stdout-producer and wrong-path forms fail closed.
- fixed: Increased the canonical TTY line capacity from 64 to 96 bytes because
  the accepted witness is longer than the previous 64-byte input ceiling; the
  diagnostic command expected text was updated to the new capacity.
- fixed: Added unit coverage, a task-owned QEMU/substitute smoke script, boot
  scenario registration, target-side smoke expectations, and retained
  regression controls for descriptor-backed cat, prior stdin/stdout
  redirection, pipeline stdin/output/stderr redirection, process-status VFS,
  ps, and pipestatus.
- deferred: Fixed-/bin bare-name combined pipeline redirection remains
  deferred to the selected closeout and the already queued bare-name core.
- deferred: Arbitrary paths, append, stderr combined pipeline forms, separated
  redirection tokens, explicit fd1 syntax, multistage combined redirection,
  unsupported stage names, persistent storage, environment-backed PATH,
  current-directory search, command lookup beyond bounded /bin, arbitrary shell
  grammar, live networking/SSH, Pi 5 hardware proof, generated-root retry, and
  phase transition.

## Evidence

- static inspection: source diff confines runtime behavior to
  src/local_command_loop.rs, the canonical TTY input capacity, QEMU boot
  scenario plumbing, the smoke harness, task evidence, and docs.
- fmt/lint/typecheck: 'cargo fmt --all -- --check' passed.
- unit tests / QEMU-substitute:
  'cargo -Zjson-target-spec test --quiet local_command_loop' passed with QEMU
  test runner; 867 tests passed.
- focused unit gate:
  'cargo -Zjson-target-spec test --quiet
  local_command_loop_runs_direct_path_pipeline_combined_stdin_stdout_redirection'
  passed with QEMU test runner; 867 tests passed.
- QEMU/substitute:
  'scripts/qemu-local-shell-direct-combined-pipeline-stdin-stdout-redirection-smoke.sh'
  passed and wrote
  tasks/evidence/2026-06-28-phase12-local-direct-combined-pipeline-stdin-stdout-redirection-core/qemu-local-shell-direct-combined-pipeline-stdin-stdout-redirection-smoke.log.
  The final line reports participants=18, expected=18, errors=0, and
  classification=qemu-local-shell-direct-combined-pipeline-stdin-stdout-redirection-complete.
- retained QEMU/substitute controls: descriptor-backed cat, accepted
  stdin/stdout redirection, accepted direct pipeline stdin redirection,
  accepted pipeline output redirection, accepted pipeline stderr redirection,
  process-status VFS, ps, and pipestatus control scripts were rerun or
  retained; the available transcripts end in PASS with matching participant
  counts.
- shell syntax:
  'bash -n scripts/qemu-local-serial-command-loop-smoke.sh
  scripts/qemu-local-shell-direct-combined-pipeline-stdin-stdout-redirection-smoke.sh'
  passed.
- JSON validation: 'jq empty' passed for task-owned classification.json and
  evidence-map.json.
- diff validation: 'git diff --check' passed.
- docs validation: '/home/node/.cargo/bin/mdbook build' passed; the existing
  large search-index warning was retained.
- lab-controller API: not run; this local POSIX/VFS task has no Pi 5 hardware
  gate.
- serial hardware boot/output: not run; this local POSIX/VFS task has no Pi 5
  hardware gate.

## Accepted Frontier

The accepted local-only direct combined pipeline stdin/stdout redirection
frontier is exactly:

~~~text
/bin/stdin </etc/banner.txt | /bin/stdin >/tmp/pipeline-combined.txt
~~~

The producer record reports fd0=regular-file from
initramfs:/etc/banner.txt, fd1=pipe-endpoint, inherited fd2, and closed loader
temporary descriptor. The consumer record reports fd0=pipe-endpoint,
fd1=regular-file targeting volatile-vfs:/tmp/pipeline-combined.txt, inherited
fd2, and closed loader temporary descriptor. Descriptor-backed
'cat /tmp/pipeline-combined.txt' reads the nested userspace stdin report
produced from the piped banner input. waitpid, laststatus,
'/proc/talos/processes', zero-argument ps, and pipestatus-compatible
observations remain coherent for the two participants.

Unsupported neighboring forms fail closed without successful process records:
bare-name combined pipeline redirection, append, wrong output path, stdout
producer with the combined path, explicit '1>', separated redirection tokens,
persistent '/var' target, unsupported stage names, PATH/current-directory
lookup, and persistent storage claims.

Live network/SSH remains paused. No Pi 5 hardware claim is made.

## Result

selected_next_task:
phase12-local-direct-combined-pipeline-stdin-stdout-redirection-closeout-20260628.

The next task is a static closeout only; it must not broaden to bare-name forms
or any deferred grammar/runtime surface before accepting this direct boundary.
