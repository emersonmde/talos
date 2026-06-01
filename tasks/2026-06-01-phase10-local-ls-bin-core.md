# Phase 10 Local Ls Bin Core Task

Task: phase10-local-ls-bin-core-20260601

Status: accepted

## Goal

Add the smallest next filesystem-visible local command step: bounded
`ls /bin` over the accepted read-only initramfs directory.

## Scope

Implemented an exact `ls /bin` extension to the kernel-backed `ls` built-in
on the descriptor-backed serial command loop. A user can type `ls /bin` at
the `talos>` prompt, dispatch through fd0/runtime-console0 canonical-lite
input, see `init` through descriptor-backed stdout, and return to a ready
prompt.

The implementation remains deliberately narrow. It preserves the existing
`ls /` root listing, checks the accepted read-only initramfs fixture for
`/bin` and `/bin/init`, and does not add a general path walker or shell
parser.

Changed files:

- build.rs
- scripts/qemu-local-ls-bin-smoke.sh
- scripts/qemu-local-serial-command-loop-smoke.sh
- src/local_command_loop.rs
- src/target/qemu_virt.rs
- docs/src/roadmap.md
- tasks/2026-06-01-phase10-local-ls-bin-core.md
- tasks/evidence/2026-06-01-qemu-local-ls-bin-core/qemu-local-ls-bin-smoke.log

## Accepted Frontier

Accepted: the descriptor-backed local command loop can read `ls /bin` through
fd0/runtime-console0, dispatch the bounded kernel-backed `/bin` listing,
print `init` through descriptor-backed stdout, retain descriptor-backed
input/output markers, and return to a ready prompt.

The retained feature transcript also reruns `ls /` and shows `bin`, `dir`,
`empty`, and `etc` through descriptor-backed stdout. Existing help, literal
echo, `pwd`, Backspace/Delete editing, Ctrl-C line cancel, and Ctrl-U line kill
QEMU/substitute smoke paths passed as proportional command-loop regressions.

## Deferred Surfaces

Deferred: recursive/general path listing, relative paths, `cd`,
current-directory mutation, file reads, `cat`, writable filesystem state,
descriptor-backed filesystem syscalls, userspace shell execution, process
spawning, filesystem-backed command execution, terminal sessions, termios,
foreground process groups, job control, networking, SSH, RP1/PCIe, UART
interrupt ownership, DMA/cache policy, and paused Phase 8 proof-only work.

## Evidence

- QEMU/substitute local `ls /bin` transcript:
  tasks/evidence/2026-06-01-qemu-local-ls-bin-core/qemu-local-ls-bin-smoke.log.
- Transcript shows `talos> ls /bin`, visible `init`, descriptor-backed
  fd0/runtime-console0 input and descriptor-backed stdout markers, next-prompt
  readiness, final classification `qemu-local-ls-bin-complete`, and exact
  PASS line `qemu-local-ls-bin: PASS`.
- The same transcript reruns `talos> ls /` and shows visible `bin`, `dir`,
  `empty`, and `etc` output.
- Regression gates reran retained local help, literal echo, pwd, line editing,
  Ctrl-C line cancel, and Ctrl-U line kill smoke paths.
- hardwareTestLock remained unlocked/restored and unused.

## Validation

- fmt/lint: `cargo fmt --all -- --check` passed after rustfmt applied one
  test-array wrapping change.
- unit tests: `cargo -Zjson-target-spec test --quiet` passed.
- QEMU/substitute feature gate:
  `scripts/qemu-local-ls-bin-smoke.sh --quiet` passed.
- QEMU/substitute command-loop regressions:
  `scripts/qemu-local-ls-root-smoke.sh --quiet`,
  `scripts/qemu-local-help-command-smoke.sh --quiet`,
  `scripts/qemu-local-literal-echo-smoke.sh --quiet`,
  `scripts/qemu-local-pwd-command-smoke.sh --quiet`,
  `scripts/qemu-local-line-editing-smoke.sh --quiet`,
  `scripts/qemu-local-line-cancel-smoke.sh --quiet`, and
  `scripts/qemu-local-line-kill-smoke.sh --quiet` passed.
- static inspection: `git diff --check` passed.
- documentation: `mdbook build` passed.
- pre-commit static inspection: `git diff --cached --check` passed.

Acceptance commit: recorded in durable supervisor state after commit creation.
