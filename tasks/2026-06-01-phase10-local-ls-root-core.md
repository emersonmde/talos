# Phase 10 Local Ls Root Core Task

Task: phase10-local-ls-root-core-20260601

Status: accepted

## Goal

Add the smallest useful filesystem-visible local command: bounded `ls /` over
the accepted read-only initramfs root.

## Scope

Implemented a kernel-backed `ls /` command on the descriptor-backed serial
command loop. A user can type `ls /` at the `talos>` prompt, dispatch through
fd0/runtime-console0 canonical-lite input, see the accepted root fixture
entries through descriptor-backed stdout, and return to a ready prompt.

The implementation is deliberately root-only. It checks the accepted
read-only initramfs fixture paths for `/bin`, `/dir`, `/empty`, and `/etc`,
then prints the stable user-visible root listing:

- `bin`
- `dir`
- `empty`
- `etc`

Changed files:

- build.rs
- scripts/qemu-local-ls-root-smoke.sh
- scripts/qemu-local-serial-command-loop-smoke.sh
- src/local_command_loop.rs
- src/target/qemu_virt.rs
- docs/src/roadmap.md
- tasks/2026-06-01-phase10-local-ls-root-core.md
- tasks/evidence/2026-06-01-qemu-local-ls-root-core/qemu-local-ls-root-smoke.log

## Accepted Frontier

Accepted: the descriptor-backed local command loop can read `ls /` through
fd0/runtime-console0, dispatch the bounded kernel-backed root-listing built-in,
print `bin`, `dir`, `empty`, and `etc` through descriptor-backed stdout, retain
descriptor-backed input/output markers, and return to a ready prompt.

Existing help, literal echo, `echo hello`, `pwd`, `stdio`, empty-command,
unknown-command, unexpected-argument, Backspace/Delete editing, Ctrl-C line
cancel, and Ctrl-U line kill behavior passed rerun QEMU/substitute regression
gates during this task.

## Deferred Surfaces

Deferred: broad shell parser/tokenization, quoting, escaping, globbing,
recursive listing, general path traversal, writable filesystem state,
descriptor-backed filesystem syscalls, userspace shell execution, process
lifecycle, terminal sessions, termios, networking, SSH, RP1/PCIe, UART
interrupts, DMA/cache policy, and paused Phase 8 proof-only work.

## Evidence

- QEMU/substitute local `ls /` transcript:
  tasks/evidence/2026-06-01-qemu-local-ls-root-core/qemu-local-ls-root-smoke.log.
- Transcript shows `talos> ls /`, visible root entries `bin`, `dir`, `empty`,
  and `etc`, descriptor-backed fd0/runtime-console0 input and
  descriptor-backed stdout markers in the scenario start and `stdio` output,
  next-prompt readiness, final classification `qemu-local-ls-root-complete`,
  and exact PASS line `qemu-local-ls-root: PASS`.
- Regression gates reran the retained local help, literal echo, pwd, line
  editing, Ctrl-C line cancel, and Ctrl-U line kill smoke paths.
- hardwareTestLock remained unlocked/restored and unused.

## Validation

- fmt/lint: `cargo fmt --all -- --check` passed.
- unit tests: `cargo -Zjson-target-spec test --quiet` passed.
- QEMU/substitute feature gate:
  `scripts/qemu-local-ls-root-smoke.sh --quiet` passed.
- QEMU/substitute command-loop regressions:
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
