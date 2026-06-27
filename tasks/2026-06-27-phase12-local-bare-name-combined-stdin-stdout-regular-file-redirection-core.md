# Phase 12 Local Bare-Name Combined Stdin Stdout Regular-File Redirection Core

Task id: phase12-local-bare-name-combined-stdin-stdout-regular-file-redirection-core-20260627

Status: accepted; commit hash recorded in durable supervisor state after
commit.

## Scope

Implement the exact fixed-/bin bare-name combined redirection witness:

~~~text
stdin </etc/banner.txt >/tmp/stdin-report.txt
~~~

The shell resolves 'stdin' only through the accepted bounded '/bin' lookup to
'/bin/stdin', loads it through descriptor-backed VFS, binds child fd0 to
initramfs:/etc/banner.txt, binds child fd1 to
volatile-vfs:/tmp/stdin-report.txt, leaves fd2 inherited as stdio output, and
keeps shell descriptors restored after the child exits.

This task does not accept arbitrary paths, output-first ordering, spaced input
grammar, /dev/null input, explicit fd1 output, append output, stderr output,
unsupported command names, pipeline-output redirection, persistent writable
filesystem behavior, environment-backed PATH, current-directory search,
command lookup beyond bounded '/bin', live networking/SSH, generated-root
retry, Pi 5 hardware action, or phase transition.

## Findings

- fixed: Extended the bounded bare-name parser policy for exactly
  'stdin </etc/banner.txt >/tmp/stdin-report.txt'. The parser reuses the
  accepted direct combined descriptor redirection machinery but only after
  'stdin' has resolved through fixed '/bin' lookup and only for the exact
  '/tmp/stdin-report.txt' output target.
- fixed: Added a local command-loop unit regression proving fd0=regular-file,
  fd1=regular-file, fd2=stdio-output, initramfs source-route, volatile-vfs
  target-route, descriptor-backed readback with 'cat /tmp/stdin-report.txt',
  waitpid/laststatus lifecycle observation, and later normal 'stdin' shell
  fd0/fd1 restoration.
- fixed: Added task-owned QEMU/substitute smoke coverage for the exact
  bare-name command and fail-closed controls. The accepted run ended with
  classification
  qemu-local-shell-bare-name-combined-stdin-stdout-redirection-complete.
- fixed: Added the new QEMU boot scenario, local serial smoke harness branch,
  wrapper script, target-side command count, label, classification, and
  dispatch expectations.
- fixed: Updated the local command boundary string so status output names the
  accepted bounded bare-name combined stdin/stdout regular-file redirection
  surface.
- deferred: Arbitrary input/output paths, output-first ordering, append in
  combined forms, stderr combined forms, pipeline-output redirection/append,
  persistent writable filesystem behavior, environment-backed PATH,
  current-directory search, command lookup beyond bounded '/bin', arbitrary
  shell grammar, live networking/SSH, Pi 5 hardware proof, generated-root
  retry, and phase transition.

## Evidence

- static inspection: source diff confines behavior to the bounded bare-name
  parser, exact '/tmp/stdin-report.txt' target helper reuse, QEMU scenario
  plumbing, smoke harness, and docs/task evidence.
- fmt/lint/typecheck: 'cargo fmt --all -- --check' passed.
- unit tests: 'cargo -Zjson-target-spec test --quiet local_command_loop'
  passed with QEMU test runner; 862 tests passed.
- QEMU/substitute:
  'scripts/qemu-local-shell-bare-name-combined-stdin-stdout-redirection-smoke.sh'
  passed and wrote
  tasks/evidence/2026-06-27-phase12-local-bare-name-combined-stdin-stdout-regular-file-redirection-core/qemu-local-shell-bare-name-combined-stdin-stdout-redirection-smoke.log.
- QEMU/substitute retained regressions: direct combined, direct/bare stdin,
  direct/bare stdout regular-file, direct/bare stdout append, direct/bare
  stderr append, process-status VFS, ps, pipestatus, and cat-banner smokes
  passed; summary retained at
  tasks/evidence/2026-06-27-phase12-local-bare-name-combined-stdin-stdout-regular-file-redirection-core/regression-smokes-summary.txt.
- shell syntax: 'bash -n scripts/qemu-local-serial-command-loop-smoke.sh
  scripts/qemu-local-shell-bare-name-combined-stdin-stdout-redirection-smoke.sh'
  passed.
- JSON validation: 'jq empty' passed for task-owned classification and
  evidence-map JSON.
- diff validation: 'git diff --check' passed.
- docs validation: '/home/node/.cargo/bin/mdbook build' passed; the existing
  large search-index warning was retained.
- lab-controller API: not run; this local POSIX/VFS task has no Pi 5 hardware
  gate.
- serial hardware boot/output: not run; this local POSIX/VFS task has no Pi 5
  hardware gate.

## Accepted Frontier

The accepted local-only bare-name combined stdin/stdout regular-file
redirection frontier is exactly:

~~~text
stdin </etc/banner.txt >/tmp/stdin-report.txt
~~~

The evidence records bounded fixed '/bin' resolution to '/bin/stdin',
child-only fd0 source-route=initramfs:/etc/banner.txt, and child-only fd1
target-route=volatile-vfs:/tmp/stdin-report.txt. Userspace '/bin/stdin' reads
the banner through fd0 and writes its report through redirected fd1;
descriptor-backed 'cat /tmp/stdin-report.txt' reads the report back.

Unsupported bare-name combined forms fail closed without successful process
records: output-first ordering, /dev/null input, explicit 1> output, spaced
input grammar, append output, stderr output, unsupported command names, and
arbitrary output path forms.

Live network/SSH remains paused. No Pi 5 hardware claim is made.

## Result

selected_next_task:
phase12-local-bare-name-combined-stdin-stdout-regular-file-redirection-closeout-20260627.

The next task is a static closeout only; it must not broaden to arbitrary
paths, append/stderr combined forms, pipeline-output redirection, persistent
filesystem behavior, live networking/SSH, hardware proof, generated-root retry,
or phase transition before accepting this bare-name combined boundary.
