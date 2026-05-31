# Phase 10 Local Line-Cancel Core

Task: phase10-local-line-cancel-core-20260531
Status: accepted

## Goal

Accept the smallest local interactivity cancel feature after Backspace/Delete
line editing: Ctrl-C 0x03 discards the current editable prompt line, prints a
short kernel-local cancellation response, returns to a fresh prompt, and allows
the next command to dispatch normally.

## Scope

- Keep this as a QEMU/substitute core task; no Pi 5 archive publication,
  hardwareTestLock acquisition, power cycle, or hardware proof is part of this
  task.
- Preserve descriptor-backed command-loop behavior for help, status, stdio,
  echo hello, pwd, empty input, unknown commands, unexpected arguments, and
  Backspace/Delete correction.
- Keep Ctrl-C semantics local to the kernel-backed command loop. This does not
  accept POSIX signal delivery, process interruption, job control, termios, or
  userspace shell behavior.
- Retain a feature-level transcript for partial input, Ctrl-C cancellation,
  fresh prompt readiness, and a following successful pwd dispatch.

## Changed Files

- build.rs
- src/tty.rs
- src/local_command_loop.rs
- src/target/qemu_virt.rs
- scripts/qemu-local-serial-command-loop-smoke.sh
- scripts/qemu-local-line-cancel-smoke.sh
- tasks/2026-05-31-phase10-local-line-cancel-core.md
- tasks/evidence/2026-05-31-qemu-local-line-cancel-core/qemu-local-line-cancel-smoke.log

## Evidence

- QEMU/substitute local line-cancel transcript:
  tasks/evidence/2026-05-31-qemu-local-line-cancel-core/qemu-local-line-cancel-smoke.log.
- The transcript shows partial `bogus` input, Ctrl-C cancellation response
  `talos: line-canceled`, an empty canceled command line, control telemetry,
  a fresh prompt, following `pwd` dispatch, visible `/` output,
  descriptor-backed fd0/stdout markers, ready prompt, final
  qemu-local-line-cancel-complete classification, and PASS.
- Regression evidence for accepted Backspace/Delete line editing:
  tasks/evidence/2026-05-31-qemu-local-line-editing-core/qemu-local-line-editing-smoke.log.
- Unit coverage includes canonical-lite Ctrl-C line cancellation and
  descriptor-backed command-loop cancellation followed by successful `pwd`
  dispatch on the same IO bridge.

## Validation

- cargo fmt --all -- --check: passed.
- cargo -Zjson-target-spec test --quiet: passed.
- scripts/qemu-local-line-cancel-smoke.sh --quiet: passed, retained transcript
  at
  tasks/evidence/2026-05-31-qemu-local-line-cancel-core/qemu-local-line-cancel-smoke.log.
- scripts/qemu-local-line-editing-smoke.sh --quiet: passed as the
  Backspace/Delete regression gate.
- git diff --check: passed.
- mdbook build: passed.
- git diff --cached --check: passed before commit.

## Acceptance

Acceptance commit: recorded in durable supervisor state for
phase10-local-line-cancel-core-20260531 after commit creation.

## Deferred Surfaces

No POSIX signal delivery, process interruption, userspace shell execution,
process spawning, job control, terminal sessions, termios, filesystem-backed
command lookup, broad escape-sequence parsing, arrow keys, shell history,
kill/yank editing, cursor addressing, screen repainting, networking, SSH,
RP1/PCIe, UART interrupt ownership, or DMA/cache-driver policy is accepted by
this task.

No Pi 5 hardware action, boot archive publication, or hardwareTestLock
acquisition occurred. hardwareTestLock remained unlocked/restored and unused.
