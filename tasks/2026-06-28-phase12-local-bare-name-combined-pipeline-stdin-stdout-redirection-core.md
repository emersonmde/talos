# Phase 12 Local Bare-Name Combined Pipeline Stdin Stdout Redirection Core

Task id: phase12-local-bare-name-combined-pipeline-stdin-stdout-redirection-core-20260628

Status: accepted; commit hash recorded in durable supervisor state after
commit.

## Scope

Implement the exact fixed-/bin bare-name combined pipeline redirection witness:

~~~text
stdin </etc/banner.txt | stdin >/tmp/pipeline-combined.txt
~~~

Both stage names resolve through the bounded /bin lookup to /bin/stdin before
descriptor-backed VFS open/read, the accepted program loader, userspace
launch/status, pipe handoff, final-stage stdout redirection, and volatile VFS
readback. The producer gets child-only fd0 from initramfs:/etc/banner.txt, fd1
as the pipe endpoint, and inherited fd2. The consumer gets fd0 from that pipe,
child-only fd1 to volatile-vfs:/tmp/pipeline-combined.txt, and inherited fd2.

This task does not accept direct path-form feature expansion beyond retained
controls, arbitrary input or output paths, append, stderr forms, separated
redirection tokens, explicit fd1 redirection, multistage combined redirection,
path-containing stage names in the bare-name witness, PATH/current-directory
lookup, command lookup beyond bounded /bin, persistent writable filesystem
behavior, live networking/SSH, generated-root retry, Pi 5 hardware action, or
phase transition.

## Findings

- fixed: Added the exact bare-name consumer pipeline sink for
  'stdin </etc/banner.txt | stdin >/tmp/pipeline-combined.txt', reusing the
  accepted fixed-/bin resolution, descriptor-backed VFS open/read, program
  loader, userspace launch/status, pipe handoff, and volatile VFS regular-file
  redirection machinery.
- fixed: Added unit coverage proving the bare form resolves to /bin/stdin for
  both stages, records producer fd0 from initramfs:/etc/banner.txt, producer
  fd1 as a pipe endpoint, consumer fd0 from the pipe endpoint, consumer fd1 to
  volatile-vfs:/tmp/pipeline-combined.txt, closed loader temporaries, coherent
  lifecycle/status records, and descriptor-backed readback.
- fixed: Added a task-owned QEMU/substitute smoke scenario and script for the
  accepted witness plus retained normal bare-name stdin and direct path-form
  combined pipeline controls.
- fixed: Updated the direct combined pipeline smoke control to stop treating
  the now-accepted bare-name witness as a negative neighbor; the direct witness
  itself remains a retained control and still passes.
- deferred: Append/combined stderr pipeline redirections, arbitrary paths,
  persistent writable filesystem behavior, environment-backed PATH,
  current-directory search, command lookup beyond bounded /bin, arbitrary shell
  grammar, unbounded/concurrent pipelines, scheduler concurrency,
  fork/signals, process groups/sessions, live networking/SSH, Pi 5 hardware
  proof, generated-root retry, and phase transition remain deferred.

## Evidence

- static inspection: source diff confines runtime behavior to
  src/local_command_loop.rs, build.rs, QEMU boot scenario plumbing, shell smoke
  harnesses, task evidence, and docs.
- fmt/lint/typecheck: 'cargo fmt --all -- --check' passed.
- shell syntax: 'bash -n scripts/qemu-local-serial-command-loop-smoke.sh
  scripts/qemu-local-shell-bare-name-combined-pipeline-stdin-stdout-redirection-smoke.sh'
  passed.
- focused unit tests / QEMU-substitute:
  'cargo -Zjson-target-spec test --quiet
  local_command_loop_runs_bare_name_pipeline_combined_stdin_stdout_redirection'
  passed with the QEMU test runner; 868 tests passed.
- broad unit tests / QEMU-substitute:
  'cargo -Zjson-target-spec test --quiet local_command_loop' passed with the
  QEMU test runner; 868 tests passed.
- focused QEMU/substitute:
  'scripts/qemu-local-shell-bare-name-combined-pipeline-stdin-stdout-redirection-smoke.sh'
  passed and wrote
  tasks/evidence/2026-06-28-phase12-local-bare-name-combined-pipeline-stdin-stdout-redirection-core/qemu-local-shell-bare-name-combined-pipeline-stdin-stdout-redirection-smoke.log.
  The final line reports participants=22, expected=22, errors=0, and
  classification=qemu-local-shell-bare-name-combined-pipeline-stdin-stdout-redirection-complete.
- retained QEMU/substitute controls: descriptor-backed cat-banner,
  direct combined pipeline stdin/stdout redirection, bare-name pipeline-output
  append redirection, bare-name pipeline-stderr redirection, process-status
  VFS, ps, and pipestatus control scripts were rerun; all ended in PASS with
  matching participant counts.
- setup correction: one broad unit invocation without the Talos QEMU path
  failed with 'qemu-system-aarch64 not found'; the same gate passed after
  exporting the workspace QEMU path.
- lab-controller API: not run; this local POSIX/VFS task has no Pi 5 hardware
  gate.
- serial hardware boot/output: not run; this local POSIX/VFS task has no Pi 5
  hardware gate.

## Accepted Frontier

The accepted local-only bare-name combined pipeline stdin/stdout redirection
frontier is exactly:

~~~text
stdin </etc/banner.txt | stdin >/tmp/pipeline-combined.txt
~~~

The producer and consumer both resolve through bounded /bin lookup to
/bin/stdin. The producer record reports fd0=regular-file from
initramfs:/etc/banner.txt, fd1=pipe-endpoint, inherited fd2, and closed loader
temporary descriptor. The consumer record reports fd0=pipe-endpoint,
fd1=regular-file targeting volatile-vfs:/tmp/pipeline-combined.txt, inherited
fd2, and closed loader temporary descriptor. Descriptor-backed
'cat /tmp/pipeline-combined.txt' reads the nested userspace stdin report
produced from the piped banner input. waitpid, laststatus,
'/proc/talos/processes', zero-argument ps, and pipestatus-compatible
observations remain coherent for the two participants.

Unsupported neighboring forms fail closed without successful process records:
append to the combined path, wrong output path, stdout producer with the
combined sink, explicit '1>', separated redirection tokens, persistent '/var'
target, path-containing stage names, and multistage combined redirection.

Live network/SSH remains paused. No Pi 5 hardware claim is made.

## Result

selected_next_task:
phase12-local-bare-name-combined-pipeline-stdin-stdout-redirection-closeout-20260628.

The next task is a static closeout only; it must not broaden to append/stderr
combined pipeline forms, arbitrary paths, persistent storage, hardware, live
networking/SSH, or phase transition work.
