# Phase 12 Local Direct Pipeline Output Regular-File Redirection Core

Task id: phase12-local-direct-pipeline-output-regular-file-redirection-core-20260627

Status: accepted; commit hash recorded in durable supervisor state after
commit.

## Scope

Implement the exact direct path-form pipeline-output redirection witness:

~~~text
/bin/stdout | /bin/stdin >/tmp/pipeline-report.txt
~~~

Both stages load through descriptor-backed VFS open/read and the accepted
userspace launch/status path. The producer writes to the accepted pipe endpoint,
the consumer reads from that pipe on fd0, and only the consumer fd1 is rebound
to volatile-vfs:/tmp/pipeline-report.txt. Shell descriptors are restored after
the pipeline exits.

This task does not accept bare-name pipeline-output redirection, append
pipeline-output forms, pipeline input redirection expansion, stderr or combined
pipeline redirection forms, arbitrary output paths, persistent writable
filesystem behavior, generated-root retry, live networking/SSH, Pi 5 hardware
action, or phase transition.

## Findings

- fixed: Added an exact /tmp/pipeline-report.txt volatile-path helper for the
  direct path-form pipeline-output witness.
- fixed: Added a direct absolute-path pipeline consumer parser path that accepts
  only '/bin/stdout | /bin/stdin >/tmp/pipeline-report.txt' as the new
  output-redirection form. The existing bare-name parser and non-pipeline
  direct command parser remain fail-closed for this target.
- fixed: Reused the accepted descriptor machinery for final-stage stdout sink
  setup, pipe fd0 handoff, process-table lifecycle records, waitpid,
  laststatus, ps, and pipestatus observations.
- fixed: Added a QEMU/substitute local command-loop regression that proves VFS
  open/read for both stages, fd0=pipe-endpoint and fd1=regular-file on the
  consumer, exec-redirection op=sink, descriptor-backed
  'cat /tmp/pipeline-report.txt' readback, shell fd1 restoration through a
  later normal '/bin/stdout', and fail-closed neighboring direct forms.
- deferred: Fixed-/bin bare-name pipeline-output redirection is separate queued
  work and remains outside this direct core.
- deferred: Append pipeline-output forms, input/stderr/combined pipeline
  redirections, arbitrary paths, persistent writable filesystem behavior,
  generated-root retry, live networking/SSH, Pi 5 hardware proof, and phase
  transition remain outside this task.

## Evidence

- static inspection: source diff confines runtime behavior to
  src/local_command_loop.rs parser/path-policy changes and a focused
  local_command_loop regression.
- fmt/lint/typecheck: 'cargo fmt --all -- --check' passed.
- unit tests / QEMU-substitute: 'cargo -Zjson-target-spec test --quiet
  local_command_loop' passed with QEMU test runner; 863 tests passed.
- task-owned QEMU/substitute transcript: the
  local_command_loop_redirects_direct_path_pipeline_consumer_stdout_to_volatile_regular_file
  regression retains the exact witness and fail-closed negatives in
  src/local_command_loop.rs, with transcript excerpts summarized in
  tasks/evidence/2026-06-27-phase12-local-direct-pipeline-output-regular-file-redirection-core/evidence-map.json.
- diff validation: 'git diff --check' passed.
- docs validation: '/home/node/.cargo/bin/mdbook build' passed.
- lab-controller API: not run; this local POSIX/VFS task has no Pi 5 hardware
  gate.
- serial hardware boot/output: not run; this local POSIX/VFS task has no Pi 5
  hardware gate.

## Accepted Frontier

The accepted local-only direct pipeline-output regular-file redirection
frontier is exactly:

~~~text
/bin/stdout | /bin/stdin >/tmp/pipeline-report.txt
~~~

The retained transcript proves both stages load through descriptor-backed VFS
open/read, the producer writes 0x1f bytes to pipe:stdout-to-stdin, the consumer
inherits fd0 as the pipe endpoint and fd1 as a volatile regular file, and the
consumer writes a 0x44-byte userspace stdin report to
volatile-vfs:/tmp/pipeline-report.txt. Descriptor-backed
'cat /tmp/pipeline-report.txt' reads the report back. A later normal
'/bin/stdout' proves shell fd1 restoration.

Unsupported direct forms fail closed without accepting a broader path policy:
'/tmp/stdout.txt' as the pipeline output target, append syntax, and a wrong
final-stage program. Bare-name pipeline-output redirection and all deferred
pipeline redirection shapes remain outside this task.

Live network/SSH remains paused. No Pi 5 hardware claim is made.

## Result

selected_next_task:
phase12-local-direct-pipeline-output-regular-file-redirection-closeout-20260627.

The next task is a static closeout only; it must not broaden to bare-name forms
or any deferred grammar/runtime surface before accepting this direct boundary.
