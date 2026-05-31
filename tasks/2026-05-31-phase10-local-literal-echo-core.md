# Phase 10 Local Literal Echo Core Task

Task: phase10-local-literal-echo-core-20260531

Status: accepted

## Goal

Add the next smallest user-visible local interactivity feature: type
`echo local serial works` at the `talos>` prompt, dispatch through the
descriptor-backed serial command loop, print `local serial works` through
descriptor-backed stdout, and return to a ready prompt.

## Scope

Implemented a bounded QEMU/substitute feature path for a longer literal echo
tail on the existing kernel-backed `echo` built-in. The accepted parser still
splits only the command word from trailing literal text; it does not implement
quoting, escaping, globbing, environment expansion, argv/envp process startup,
or general shell token vectors.

The feature exposed the previous 16-byte canonical-lite line capacity as too
small for the requested command. This task raises the canonical-lite line
capacity to 32 bytes, updates the status diagnostic expectation, and keeps the
truncation tests explicit at the new boundary.

Changed files:

- build.rs
- scripts/qemu-local-literal-echo-smoke.sh
- scripts/qemu-local-serial-command-loop-smoke.sh
- src/diagnostic_command.rs
- src/local_command_loop.rs
- src/target/qemu_virt.rs
- src/tty.rs
- docs/src/roadmap.md
- tasks/2026-05-31-phase10-local-literal-echo-core.md
- tasks/evidence/2026-05-31-qemu-local-literal-echo-core/qemu-local-literal-echo-smoke.log
- tasks/evidence/2026-05-31-qemu-local-echo-command-core/qemu-local-echo-command-smoke.log
- tasks/evidence/2026-05-31-qemu-local-pwd-command-core/qemu-local-pwd-command-smoke.log

## Accepted Frontier

Accepted: the descriptor-backed local command loop can read
`echo local serial works` through fd0/runtime-console0 canonical-lite input,
dispatch the kernel-backed `echo` built-in, write the visible
`local serial works` line through descriptor-backed stdout, preserve
descriptor-backed input/output markers, and return to a ready prompt.

The accepted local prompt remains kernel-backed and prompt-local. Existing
`echo hello`, `pwd`, `stdio`, empty-command, unknown-command,
unexpected-argument, Backspace/Delete editing, Ctrl-C line cancel, and Ctrl-U
line kill behavior remain covered by rerun QEMU/substitute regressions.

## Deferred Surfaces

Deferred: broad shell tokenization, quoting, escaping, globbing, environment
expansion, argv/envp process ABI, shell variables, pipes, redirection, command
substitution, multiline input, userspace shell execution, process spawning,
external command lookup, filesystem-backed commands, cd/path traversal,
VFS lookup, directory listing, writable filesystem state, broad POSIX read and
stdio readiness, terminal sessions, termios, job control, cursor addressing,
screen repainting, arrow keys, networking, SSH, RP1/PCIe, UART interrupt
ownership, DMA/cache-driver policy, and paused Phase 8 proof-only work.

## Evidence

- QEMU/substitute local literal echo transcript:
  tasks/evidence/2026-05-31-qemu-local-literal-echo-core/qemu-local-literal-echo-smoke.log.
- Transcript shows `talos> echo local serial works`, visible
  `local serial works`, descriptor-backed fd0/runtime-console0 input and
  descriptor-backed stdout markers in the scenario start and `stdio` output,
  next-prompt readiness, final classification
  `qemu-local-literal-echo-complete`, and exact PASS line
  `qemu-local-literal-echo: PASS`.
- Rerun QEMU/substitute regression transcripts:
  tasks/evidence/2026-05-31-qemu-local-echo-command-core/qemu-local-echo-command-smoke.log
  and
  tasks/evidence/2026-05-31-qemu-local-pwd-command-core/qemu-local-pwd-command-smoke.log.
- Rerun QEMU/substitute regression commands also passed for
  `scripts/qemu-local-line-editing-smoke.sh --quiet`,
  `scripts/qemu-local-line-cancel-smoke.sh --quiet`, and
  `scripts/qemu-local-line-kill-smoke.sh --quiet`.
- hardwareTestLock remained unlocked/restored and unused.

## Validation

- fmt/lint: `cargo fmt --all -- --check` passed.
- unit tests: `cargo -Zjson-target-spec test --quiet` passed with 348 tests.
- QEMU/substitute feature gate:
  `scripts/qemu-local-literal-echo-smoke.sh --quiet` passed.
- QEMU/substitute command-loop regressions:
  `scripts/qemu-local-echo-command-smoke.sh --quiet`,
  `scripts/qemu-local-pwd-command-smoke.sh --quiet`,
  `scripts/qemu-local-line-editing-smoke.sh --quiet`,
  `scripts/qemu-local-line-cancel-smoke.sh --quiet`, and
  `scripts/qemu-local-line-kill-smoke.sh --quiet` passed.
- static inspection: `git diff --check` passed.
- documentation: `mdbook build` passed.
- pre-commit static inspection: `git diff --cached --check` passed.

Acceptance commit: recorded in durable supervisor state after commit creation.
