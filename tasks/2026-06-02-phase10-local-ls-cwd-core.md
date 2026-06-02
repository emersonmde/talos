# Phase 10 Local Ls Cwd Core Task

Task: phase10-local-ls-cwd-core-20260602

Status: accepted

## Goal

Add the smallest cwd-aware listing behavior to the descriptor-backed serial
command loop: bare `ls` lists the accepted command-context current directory.

## Scope

Implemented kernel-backed bare `ls` dispatch for the existing prompt-local cwd
state. The command now resolves against `/`, `/etc`, or `/bin` after the
accepted fixed-directory `cd` feature and prints the bounded immutable
initramfs fixture entries for that directory.

The existing exact `ls /` and `ls /bin` paths remain accepted. Bare `ls` adds
the `/etc` fixture listing only when cwd is `/etc`; it does not accept
`ls /etc` as a new explicit path form.

Changed files:

- build.rs
- scripts/qemu-local-ls-cwd-smoke.sh
- scripts/qemu-local-serial-command-loop-smoke.sh
- src/local_command_loop.rs
- src/target/qemu_virt.rs
- docs/src/roadmap.md
- tasks/2026-06-02-phase10-local-ls-cwd-core.md
- tasks/evidence/2026-06-02-qemu-local-ls-cwd-core/qemu-local-ls-cwd-smoke.log

## Accepted Frontier

Accepted: through fd0/runtime-console0 canonical-lite input and
descriptor-backed stdout, bare `ls` lists root cwd as `bin`, `dir`, `empty`,
and `etc`; after `cd /etc`, bare `ls` lists `banner.txt`; after `cd /bin`,
bare `ls` lists `init`; after `cd /`, bare `ls` returns to the root listing.
The feature returns to a ready prompt and preserves a regression `bogus`
unknown-command response.

The feature is still command-context cwd behavior. It does not add POSIX
`chdir`, relative path traversal, `.` or `..`, arbitrary path listing,
descriptor-backed filesystem syscalls, userspace shell execution, process cwd
inheritance, globbing, quoting, pipes, redirection, networking, SSH, RP1/PCIe,
UART interrupt ownership, or DMA/cache policy.

## Evidence

- QEMU/substitute local bare `ls` cwd transcript:
  tasks/evidence/2026-06-02-qemu-local-ls-cwd-core/qemu-local-ls-cwd-smoke.log.
- Transcript shows `pwd`, bare `ls` at `/`, `cd /etc`, bare `ls` with
  `banner.txt`, `cd /bin`, bare `ls` with `init`, `cd /`, bare `ls` with the
  root entries, `bogus` regression, next-prompt readiness, final
  classification `qemu-local-ls-cwd-complete`, and exact PASS line
  `qemu-local-ls-cwd: PASS`.
- Unit tests cover bare `ls` against `/`, `/etc`, and `/bin` through the
  descriptor-backed local command loop.
- hardwareTestLock remained unlocked/restored and unused.

## Validation

- fmt/lint: `cargo fmt --all -- --check` passed.
- unit tests: `cargo -Zjson-target-spec test --quiet` passed.
- QEMU/substitute feature gate: `scripts/qemu-local-ls-cwd-smoke.sh` passed.
- static inspection: `git diff --check` passed.
- documentation: `mdbook build` passed.
- pre-commit static inspection: `git diff --cached --check` passed.

Acceptance commit: recorded in durable supervisor state after commit creation.
