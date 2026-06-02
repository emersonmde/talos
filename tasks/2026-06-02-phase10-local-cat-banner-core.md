# Phase 10 Local Cat Banner Core Task

Task: phase10-local-cat-banner-core-20260602

Status: accepted

## Goal

Add the smallest next file-inspection behavior: bounded
`cat /etc/banner.txt` over the accepted read-only initramfs banner fixture.

## Scope

Implemented an exact `cat /etc/banner.txt` command in the descriptor-backed
serial command loop. A user can type the command at the `talos>` prompt,
dispatch through fd0/runtime-console0 canonical-lite input, see the accepted
`Talos initramfs fixture` bytes through descriptor-backed stdout, and return
to a ready prompt.

The implementation remains prompt-local and deliberately narrow. It uses the
existing read-only initramfs fixture's regular-file bytes for only
`/etc/banner.txt`; it does not add a general `cat`, arbitrary file reads,
path traversal, descriptor-backed filesystem syscalls, or userspace execution.

Changed files:

- build.rs
- scripts/qemu-local-cat-banner-smoke.sh
- scripts/qemu-local-serial-command-loop-smoke.sh
- src/local_command_loop.rs
- src/target/qemu_virt.rs
- docs/src/roadmap.md
- tasks/2026-06-02-phase10-local-cat-banner-core.md
- tasks/evidence/2026-06-02-qemu-local-cat-banner-core/qemu-local-cat-banner-smoke.log

## Accepted Frontier

Accepted: the descriptor-backed local command loop can read
`cat /etc/banner.txt` through fd0/runtime-console0, dispatch the bounded
kernel-backed banner read, print `Talos initramfs fixture` through
descriptor-backed stdout, expose `cat` in help and status, and return to a
ready prompt.

The retained feature transcript also reruns `ls /bin` and shows visible
`init` output in the same scenario. Existing help, status, stdio, literal
echo, `pwd`, `ls /`, `ls /bin`, Backspace/Delete editing, Ctrl-C line
cancel, and Ctrl-U line kill QEMU/substitute smoke paths passed as
proportional command-loop regressions.

## Deferred Surfaces

Deferred: general `cat`, arbitrary file reads, recursive/general path
traversal, relative paths, `cd`, globbing, quoting, escaping, environment
expansion, pipes, redirection, command substitution, descriptor-backed
filesystem syscalls, POSIX read completeness, writable filesystem state,
userspace shell execution, process spawning, terminal sessions, termios, job
control, networking, SSH, RP1/PCIe, UART interrupt ownership, DMA/cache policy,
and the blocked Pi 5 `ls /bin` proof strategy.

## Evidence

- QEMU/substitute local `cat /etc/banner.txt` transcript:
  tasks/evidence/2026-06-02-qemu-local-cat-banner-core/qemu-local-cat-banner-smoke.log.
- Transcript shows `talos> cat /etc/banner.txt`, visible
  `Talos initramfs fixture`, descriptor-backed fd0/runtime-console0 input and
  descriptor-backed stdout markers, next-prompt readiness, final classification
  `qemu-local-cat-banner-complete`, and exact PASS line
  `qemu-local-cat-banner: PASS`.
- The same transcript reruns `talos> ls /bin` and shows visible `init`
  output.
- Regression gates reran retained local help, literal echo, pwd, ls-root,
  ls-bin, line editing, Ctrl-C line cancel, and Ctrl-U line kill smoke paths.
- hardwareTestLock remained unlocked/restored and unused.

## Validation

- fmt/lint: `cargo fmt --all -- --check` passed.
- unit tests: `cargo -Zjson-target-spec test --quiet` passed.
- QEMU/substitute feature gate:
  `scripts/qemu-local-cat-banner-smoke.sh --quiet` passed.
- QEMU/substitute command-loop regressions:
  `scripts/qemu-local-help-command-smoke.sh --quiet`,
  `scripts/qemu-local-literal-echo-smoke.sh --quiet`,
  `scripts/qemu-local-pwd-command-smoke.sh --quiet`,
  `scripts/qemu-local-ls-root-smoke.sh --quiet`,
  `scripts/qemu-local-ls-bin-smoke.sh --quiet`,
  `scripts/qemu-local-line-editing-smoke.sh --quiet`,
  `scripts/qemu-local-line-cancel-smoke.sh --quiet`, and
  `scripts/qemu-local-line-kill-smoke.sh --quiet` passed.
- static inspection: `git diff --check` passed.
- documentation: `mdbook build` passed.
- pre-commit static inspection: `git diff --cached --check` passed.

Acceptance commit: recorded in durable supervisor state after commit creation.
