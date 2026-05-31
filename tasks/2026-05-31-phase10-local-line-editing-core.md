# Phase 10 Local Line-Editing Core

Task: phase10-local-line-editing-core-20260531
Status: accepted

## Goal

Accept the smallest local interactivity edit feature after the root-only pwd
frontier: Backspace 0x08 and Delete 0x7f remove the immediately previous
editable byte before Enter dispatches a descriptor-backed local command.

## Scope

- Keep this as a QEMU/substitute core task; no Pi 5 archive publication,
  hardwareTestLock acquisition, power cycle, or hardware proof is part of this
  task.
- Preserve the accepted descriptor-backed command-loop behavior for help,
  status, stdio, echo hello, pwd, empty input, unknown commands, and unexpected
  arguments.
- Retain a feature-level transcript for corrected pwd dispatch after both
  Backspace and Delete erase bytes.

## Changed Files

- build.rs
- src/local_command_loop.rs
- src/target/qemu_virt.rs
- scripts/qemu-local-serial-command-loop-smoke.sh
- scripts/qemu-local-line-editing-smoke.sh
- tasks/2026-05-31-phase10-local-line-editing-core.md
- tasks/evidence/2026-05-31-qemu-local-line-editing-core/qemu-local-line-editing-smoke.log

## Evidence

- QEMU/substitute local line-editing transcript:
  tasks/evidence/2026-05-31-qemu-local-line-editing-core/qemu-local-line-editing-smoke.log.
- The transcript must show corrected pwd dispatch after pwx plus Backspace and
  after pwx plus Delete, visible / output, descriptor-backed fd0/stdout
  markers, ready prompt, final qemu-local-line-editing-complete
  classification, and PASS.
- Unit/regression coverage includes command-loop result telemetry for
  Backspace/Delete counts and canonical-lite erase-before-dispatch behavior.

## Validation

- cargo fmt --all -- --check: passed.
- cargo -Zjson-target-spec test --quiet: passed.
- scripts/qemu-local-line-editing-smoke.sh --quiet: passed, retained transcript
  at
  tasks/evidence/2026-05-31-qemu-local-line-editing-core/qemu-local-line-editing-smoke.log.
- scripts/qemu-local-pwd-command-smoke.sh --quiet: passed.
- scripts/qemu-local-echo-command-smoke.sh --quiet: passed.
- scripts/qemu-local-command-stdin-descriptor-smoke.sh --quiet: passed.
- scripts/qemu-local-command-stdio-bridge-smoke.sh --quiet: passed.
- git diff --check: passed.
- mdbook build: passed.
- git diff --cached --check: passed before commit.

## Acceptance

Acceptance commit: recorded in durable supervisor state for
phase10-local-line-editing-core-20260531 after commit creation.

## Deferred Surfaces

No userspace shell execution, process spawning, filesystem lookup, cd, VFS path
traversal, termios, cursor addressing, history, broad escape parsing,
networking, SSH, RP1/PCIe, UART interrupt ownership, or DMA/cache-driver policy
is accepted by this task.

hardwareTestLock remained unlocked/restored and unused.
