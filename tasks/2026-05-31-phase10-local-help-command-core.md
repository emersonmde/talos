# Phase 10 Local Help Command Core Task

Task: phase10-local-help-command-core-20260531

Status: accepted

## Goal

Make the serial `help` command an accurate user-visible guide to the currently
accepted kernel-backed local command loop.

## Scope

Implemented a bounded QEMU/substitute feature path for `help` on the
descriptor-backed serial command loop. A user can type `help` at the `talos>`
prompt, dispatch through fd0/runtime-console0 canonical-lite input, see concise
help output through descriptor-backed stdout, and return to a ready prompt.

The help output now names the accepted local command frontier: `help`,
`status`, `stdio`, `pwd`, `echo`, the accepted `echo hello` and
`echo local serial works` forms, and prompt-local Backspace/Delete, Ctrl-C, and
Ctrl-U editing controls.

Changed files:

- build.rs
- scripts/qemu-local-help-command-smoke.sh
- scripts/qemu-local-serial-command-loop-smoke.sh
- src/local_command_loop.rs
- src/target/qemu_virt.rs
- docs/src/roadmap.md
- tasks/2026-05-31-phase10-local-help-command-core.md
- tasks/evidence/2026-05-31-qemu-local-help-command-core/qemu-local-help-command-smoke.log
- tasks/evidence/2026-05-31-qemu-local-literal-echo-core/qemu-local-literal-echo-smoke.log
- tasks/evidence/2026-05-31-qemu-local-pwd-command-core/qemu-local-pwd-command-smoke.log
- tasks/evidence/2026-05-31-qemu-local-line-editing-core/qemu-local-line-editing-smoke.log
- tasks/evidence/2026-05-31-qemu-local-line-cancel-core/qemu-local-line-cancel-smoke.log
- tasks/evidence/2026-05-31-qemu-local-line-kill-core/qemu-local-line-kill-smoke.log

## Accepted Frontier

Accepted: the descriptor-backed local command loop can read `help` through
fd0/runtime-console0, dispatch the kernel-backed help built-in, print accurate
help output through descriptor-backed stdout, preserve descriptor-backed
input/output markers, and return to a ready prompt.

The accepted local prompt remains kernel-backed and prompt-local. Existing
`pwd`, `stdio`, `echo hello`, bounded literal echo tail, empty-command,
unknown-command, unexpected-argument, Backspace/Delete editing, Ctrl-C line
cancel, and Ctrl-U line kill behavior remain covered by rerun QEMU/substitute
regressions.

## Deferred Surfaces

Deferred: broad shell tokenization, quoting, escaping, globbing, environment
expansion, shell variables, command substitution, multiline input, pipes,
redirection, userspace shell execution, process spawning, external command
lookup, filesystem-backed commands, cd/path traversal, VFS lookup, directory
listing, writable filesystem state, broad POSIX read and stdio readiness,
terminal sessions, termios, job control, cursor addressing, screen repainting,
arrow keys, networking, SSH, RP1/PCIe, UART interrupt ownership,
DMA/cache-driver policy, and paused Phase 8 proof-only work.

## Evidence

- QEMU/substitute local help command transcript:
  tasks/evidence/2026-05-31-qemu-local-help-command-core/qemu-local-help-command-smoke.log.
- Transcript shows `talos> help`, `talos: ok help`, accepted command names,
  accepted echo forms, accepted prompt-local editing controls, descriptor-backed
  fd0/runtime-console0 input and descriptor-backed stdout markers in the
  scenario start and `stdio` output, next-prompt readiness, final classification
  `qemu-local-help-command-complete`, and exact PASS line
  `qemu-local-help-command: PASS`.
- Rerun QEMU/substitute regression transcripts:
  tasks/evidence/2026-05-31-qemu-local-literal-echo-core/qemu-local-literal-echo-smoke.log,
  tasks/evidence/2026-05-31-qemu-local-pwd-command-core/qemu-local-pwd-command-smoke.log,
  tasks/evidence/2026-05-31-qemu-local-line-editing-core/qemu-local-line-editing-smoke.log,
  tasks/evidence/2026-05-31-qemu-local-line-cancel-core/qemu-local-line-cancel-smoke.log,
  and
  tasks/evidence/2026-05-31-qemu-local-line-kill-core/qemu-local-line-kill-smoke.log.
- hardwareTestLock remained unlocked/restored and unused.

## Validation

- fmt/lint: `cargo fmt --all -- --check` passed.
- unit tests: `cargo -Zjson-target-spec test --quiet` passed with 348 tests.
- QEMU/substitute feature gate:
  `scripts/qemu-local-help-command-smoke.sh --quiet` passed.
- QEMU/substitute command-loop regressions:
  `scripts/qemu-local-literal-echo-smoke.sh --quiet`,
  `scripts/qemu-local-pwd-command-smoke.sh --quiet`,
  `scripts/qemu-local-line-editing-smoke.sh --quiet`,
  `scripts/qemu-local-line-cancel-smoke.sh --quiet`, and
  `scripts/qemu-local-line-kill-smoke.sh --quiet` passed.
- static inspection: `git diff --check` passed.
- documentation: `mdbook build` passed.
- pre-commit static inspection: `git diff --cached --check` passed.

Acceptance commit: recorded in durable supervisor state after commit creation.
