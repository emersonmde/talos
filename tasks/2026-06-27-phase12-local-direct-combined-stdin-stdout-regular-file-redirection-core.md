# Phase 12 Local Direct Combined Stdin Stdout Regular-File Redirection Core

Task id: phase12-local-direct-combined-stdin-stdout-regular-file-redirection-core-20260627

Status: accepted; commit hash recorded in durable supervisor state after
commit.

## Scope

Implement the exact direct path-form combined redirection witness:

~~~text
/bin/stdin </etc/banner.txt >/tmp/stdin-report.txt
~~~

The command loads /bin/stdin through descriptor-backed VFS, binds child fd0 to
initramfs:/etc/banner.txt, binds child fd1 to
volatile-vfs:/tmp/stdin-report.txt, leaves fd2 inherited as stdio output, and
keeps shell descriptors restored after the child exits.

This task does not accept bare-name combined redirection, output-first
ordering, spaced input grammar, /dev/null input, explicit fd1 output, append
output, stderr output, arbitrary output paths, pipeline-output redirection,
persistent writable filesystem behavior, live networking/SSH, generated-root
retry, Pi 5 hardware action, or phase transition.

## Findings

- fixed: Added the direct absolute-path parser policy for exactly
  '/bin/stdin </etc/banner.txt >/tmp/stdin-report.txt'. The policy reuses the
  accepted descriptor redirection machinery while keeping the output target
  narrowed to '/tmp/stdin-report.txt' for this combined direct witness.
- fixed: Added a local command-loop unit regression proving fd0=regular-file,
  fd1=regular-file, fd2=stdio-output, initramfs source-route, volatile-vfs
  target-route, descriptor-backed readback with 'cat /tmp/stdin-report.txt',
  waitpid/laststatus lifecycle observation, and later normal '/bin/stdin'
  shell fd0/fd1 restoration.
- fixed: Added task-owned QEMU/substitute smoke coverage for the exact direct
  command and its fail-closed controls. The accepted run ended with
  classification
  qemu-local-shell-direct-combined-stdin-stdout-redirection-complete.
- fixed: Added the new QEMU boot scenario, local serial smoke harness branch,
  target-side command count, label, classification, and dispatch expectations.
- deferred: Fixed-/bin bare-name combined redirection is explicitly deferred
  to phase12-local-direct-combined-stdin-stdout-regular-file-redirection-closeout-20260627
  followed by the queued bare-name core if the closeout accepts this boundary.
- deferred: Arbitrary input/output paths, output-first ordering, append in
  combined forms, stderr combined forms, pipeline-output redirection/append,
  persistent writable filesystem behavior, environment-backed PATH,
  current-directory search, command lookup beyond bounded /bin, arbitrary shell
  grammar, live networking/SSH, Pi 5 hardware proof, generated-root retry, and
  phase transition.

## Evidence

- static inspection: source diff confines behavior to the direct absolute-path
  parser, exact '/tmp/stdin-report.txt' target helper, QEMU scenario plumbing,
  smoke harness, and docs/task evidence.
- fmt/lint/typecheck: 'cargo fmt --all -- --check' passed.
- unit tests: 'cargo -Zjson-target-spec test --quiet local_command_loop'
  passed with QEMU test runner; 861 tests passed.
- QEMU/substitute:
  'scripts/qemu-local-shell-direct-combined-stdin-stdout-redirection-smoke.sh'
  passed and wrote
  tasks/evidence/2026-06-27-phase12-local-direct-combined-stdin-stdout-regular-file-redirection-core/qemu-local-shell-direct-combined-stdin-stdout-redirection-smoke.log.
- shell syntax: 'bash -n scripts/qemu-local-serial-command-loop-smoke.sh
  scripts/qemu-local-shell-direct-combined-stdin-stdout-redirection-smoke.sh'
  passed.
- diff validation: 'git diff --check' passed.
- docs validation: '/home/node/.cargo/bin/mdbook build' passed; the existing
  large search-index warning was retained.
- lab-controller API: not run; this local POSIX/VFS task has no Pi 5 hardware
  gate.
- serial hardware boot/output: not run; this local POSIX/VFS task has no Pi 5
  hardware gate.

## Accepted Frontier

The accepted local-only direct combined stdin/stdout regular-file redirection
frontier is exactly:

~~~text
/bin/stdin </etc/banner.txt >/tmp/stdin-report.txt
~~~

The evidence records child-only fd0 source-route=initramfs:/etc/banner.txt and
fd1 target-route=volatile-vfs:/tmp/stdin-report.txt. Userspace /bin/stdin reads
the banner through fd0 and writes its report through redirected fd1; descriptor
backed 'cat /tmp/stdin-report.txt' reads the report back. A later normal
'/bin/stdin' unit control proves shell fd0/fd1 restoration and the descriptor
record reports the loader temporary descriptor closed.

Unsupported direct combined forms fail closed without successful process
records: output-first ordering, spaced input grammar, /dev/null input, explicit
1> output, append output, stderr output, and arbitrary output path forms.

Live network/SSH remains paused. No Pi 5 hardware claim is made.

## Result

selected_next_task:
phase12-local-direct-combined-stdin-stdout-regular-file-redirection-closeout-20260627.

The next task is a static closeout only; it must not broaden to bare-name forms
or any deferred grammar/runtime surface before accepting this direct boundary.
