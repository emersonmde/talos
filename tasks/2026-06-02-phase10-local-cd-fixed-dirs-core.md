# Phase 10 Local Cd Fixed Dirs Core Task

Task: phase10-local-cd-fixed-dirs-core-20260602

Status: accepted

## Goal

Add the smallest current-directory feature to the local serial command loop:
bounded kernel-backed cd for /, /etc, and /bin, with pwd reflecting the
command-loop cwd state.

## Scope

Implemented command-context current-directory state in the descriptor-backed
local command loop. A user can type pwd, cd /etc, pwd, cd /bin, pwd, cd /, pwd,
cd /missing, and pwd at the talos> prompt; the accepted directories update the
prompt-local cwd, the nonexistent absolute directory is rejected, and the final
pwd remains /.

This is a kernel-backed placeholder for future process-local cwd. It is not a
POSIX chdir syscall, a userspace shell, a process-owned cwd model, relative path
support, broad path traversal, or descriptor-backed filesystem syscall behavior.

Changed files:

- build.rs
- scripts/qemu-local-cd-fixed-dirs-smoke.sh
- scripts/qemu-local-serial-command-loop-smoke.sh
- src/local_command_loop.rs
- src/target/qemu_virt.rs
- docs/src/roadmap.md
- tasks/2026-06-02-phase10-local-cd-fixed-dirs-core.md
- tasks/evidence/2026-06-02-qemu-local-cd-fixed-dirs-core/qemu-local-cd-fixed-dirs-smoke.log

## Accepted Frontier

Accepted: the descriptor-backed local command loop can read bounded cd commands
through fd0/runtime-console0, update command-loop cwd state for /etc, /bin, and
/, print the updated cwd through descriptor-backed stdout with pwd, reject cd
/missing visibly with talos: not-directory, preserve the previous cwd on
rejection, and return to a ready prompt.

Help and status now include cd in the command vocabulary. The implementation
checks the accepted read-only initramfs fixture for the bounded directories but
does not add a general path walker or relative path resolver.

## Deferred Surfaces

Deferred: POSIX chdir, process-local cwd ownership, relative paths, . and ..
cwd mutation through the prompt, broad path traversal, arbitrary directory
targets, descriptor-backed filesystem syscalls, userspace shell execution,
process lifecycle, filesystem-backed command execution, terminal sessions,
termios, pipes/redirection, networking, SSH, RP1/PCIe, UART interrupt ownership,
DMA/cache policy, blocked ls /bin Pi 5 proof strategy, and paused Phase 8
proof-only work.

## Evidence

- QEMU/substitute local cd fixed-directories transcript:
  tasks/evidence/2026-06-02-qemu-local-cd-fixed-dirs-core/qemu-local-cd-fixed-dirs-smoke.log.
- Transcript shows initial pwd output /, cd /etc followed by pwd output /etc,
  cd /bin followed by pwd output /bin, cd / followed by pwd output /, rejected
  cd /missing, final pwd output /, next-prompt readiness, final classification
  qemu-local-cd-fixed-dirs-complete, and exact PASS line
  qemu-local-cd-fixed-dirs: PASS.
- Regression gates reran local cat-banner, ls /, ls /bin, pwd, and Ctrl-U
  line-kill QEMU/substitute smoke paths after the cd help/status vocabulary
  update.
- Unit tests cover the bounded cwd state machine and retained local command
  behavior.
- hardwareTestLock remained unlocked/restored and unused.

## Validation

- fmt/lint: cargo fmt --all -- --check passed.
- unit tests: cargo -Zjson-target-spec test --quiet passed.
- QEMU/substitute feature gate:
  scripts/qemu-local-cd-fixed-dirs-smoke.sh --quiet passed.
- QEMU/substitute command-loop regressions:
  scripts/qemu-local-cat-banner-smoke.sh --quiet,
  scripts/qemu-local-ls-root-smoke.sh --quiet,
  scripts/qemu-local-ls-bin-smoke.sh --quiet,
  scripts/qemu-local-pwd-command-smoke.sh --quiet, and
  scripts/qemu-local-line-kill-smoke.sh --quiet passed.
- static inspection: git diff --check passed.
- documentation: mdbook build passed.
- pre-commit static inspection: git diff --cached --check passed.

Acceptance commit: recorded in durable supervisor state after commit creation.
