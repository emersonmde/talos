# Phase 10 Local Line-Kill Core

Task: phase10-local-line-kill-core-20260531
Status: accepted

## Goal

Accept the smallest local interactivity line-kill feature after Ctrl-C prompt
cancel: Ctrl-U 0x15 discards the current editable prompt line, prints a short
kernel-local line-killed response, then dispatches the following command
normally through descriptor-backed stdout.

## Scope

- Keep this as a QEMU/substitute core task; no Pi 5 archive publication,
  hardwareTestLock acquisition, power cycle, or hardware proof is part of this
  task.
- Preserve descriptor-backed command-loop behavior for help, status, stdio,
  echo hello, pwd, empty input, unknown commands, unexpected arguments,
  Backspace/Delete correction, and Ctrl-C prompt-local cancellation.
- Keep Ctrl-U semantics local to the kernel-backed prompt loop. This does not
  accept POSIX signal delivery, process interruption, job control, termios,
  shell history, kill/yank editing, cursor addressing, or userspace shell
  behavior.
- Retain a feature-level transcript for partial input, Ctrl-U line kill,
  line-killed response, following pwd dispatch, visible slash output, and a
  ready prompt.

## Changed Files

- build.rs
- src/tty.rs
- src/local_command_loop.rs
- src/target/qemu_virt.rs
- scripts/qemu-local-serial-command-loop-smoke.sh
- scripts/qemu-local-line-kill-smoke.sh
- docs/src/roadmap.md
- docs/src/decisions/README.md
- tasks/2026-05-31-phase10-local-line-kill-core.md
- tasks/evidence/2026-05-31-qemu-local-line-kill-core/qemu-local-line-kill-smoke.log

## Evidence

- QEMU/substitute local line-kill transcript:
  tasks/evidence/2026-05-31-qemu-local-line-kill-core/qemu-local-line-kill-smoke.log.
- The transcript shows partial `bogus` input, Ctrl-U 0x15 line kill, visible
  `talos: line-killed`, following `pwd` dispatch, visible `/` output,
  descriptor-backed fd0/stdout markers, one clear-line control telemetry event,
  qemu-local-line-kill-complete classification, and PASS.
- Regression evidence for accepted Ctrl-C prompt-local cancellation:
  tasks/evidence/2026-05-31-qemu-local-line-cancel-core/qemu-local-line-cancel-smoke.log.
- Regression evidence for accepted Backspace/Delete line editing:
  tasks/evidence/2026-05-31-qemu-local-line-editing-core/qemu-local-line-editing-smoke.log.
- Unit coverage includes canonical-lite Ctrl-U line kill without termination
  and descriptor-backed command-loop line kill followed by successful `pwd`
  dispatch.

## Validation

- cargo fmt --all -- --check: passed.
- cargo -Zjson-target-spec test --quiet: passed.
- scripts/qemu-local-line-kill-smoke.sh --quiet: passed, retained transcript
  at
  tasks/evidence/2026-05-31-qemu-local-line-kill-core/qemu-local-line-kill-smoke.log.
- scripts/qemu-local-line-cancel-smoke.sh --quiet: passed as the Ctrl-C
  regression gate.
- scripts/qemu-local-line-editing-smoke.sh --quiet: passed as the
  Backspace/Delete regression gate.
- git diff --check: passed.
- mdbook build: passed.
- git diff --cached --check: passed before commit.

## Acceptance

Acceptance commit: recorded in durable supervisor state for
phase10-local-line-kill-core-20260531 after commit creation.

## Deferred Surfaces

No POSIX signal delivery, process interruption, userspace shell execution,
process spawning, job control, terminal sessions, termios, filesystem-backed
command lookup, broad escape-sequence parsing, arrow keys, shell history,
kill/yank editing, cursor addressing, screen repainting, networking, SSH,
RP1/PCIe, UART interrupt ownership, or DMA/cache-driver policy is accepted by
this task.

No Pi 5 hardware action, boot archive publication, or hardwareTestLock
acquisition occurred. hardwareTestLock remained unlocked/restored and unused.
