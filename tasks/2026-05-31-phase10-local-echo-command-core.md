# Phase 10 Local Echo Command Core Task

Task: phase10-local-echo-command-core-20260531

Status: accepted

## Goal

Add the next smallest user-visible local command feature: type `echo hello`,
dispatch a kernel-backed built-in, print `hello` through descriptor-backed
stdout, and return to a ready prompt.

## Scope

Implemented only the bounded kernel-backed `echo` built-in on the accepted
local command-loop path. The command parser now accepts a command word plus
simple trailing argument text for `echo`; other accepted built-ins still reject
arguments deterministically. The canonical-lite line buffer is 16 bytes so the
feature input `echo hello` fits without truncation.

Changed files:

- build.rs
- scripts/qemu-local-echo-command-smoke.sh
- scripts/qemu-local-serial-command-loop-smoke.sh
- src/diagnostic_command.rs
- src/local_command_loop.rs
- src/target/qemu_virt.rs
- src/tty.rs
- tasks/2026-05-31-phase10-local-echo-command-core.md
- tasks/evidence/2026-05-31-qemu-local-echo-command-core/qemu-local-echo-command-smoke.log

## Accepted Frontier

Accepted: the descriptor-backed local command loop can read `echo hello`
through fd0/runtime-console0 canonical-lite input, dispatch the kernel-backed
`echo` built-in, write the visible `hello` line through descriptor-backed
stdout, preserve descriptor-backed input/output markers, and return to a ready
prompt.

Existing local command behavior remains covered: `help`, `status`, `stdio`,
empty input, and unknown command behavior remain deterministic. `stdio` still
reports fd 0/fd 1/fd 2 identities, runtime-console0 backing,
descriptor-backed-input=true, and descriptor-backed-output=true.

## Deferred Surfaces

Deferred: userspace shell execution, process spawning, exec/wait/exit,
filesystem-backed command lookup, pipes, redirection, globbing, environment
expansion, quoting/escaping semantics, termios/job control, networking, SSH,
RP1/PCIe, UART interrupt ownership, DMA/cache-driver policy, and broader POSIX
read/stdio behavior outside this descriptor-backed local command-loop path.

## Evidence

- QEMU/substitute echo command transcript:
  tasks/evidence/2026-05-31-qemu-local-echo-command-core/qemu-local-echo-command-smoke.log.
- Transcript includes `talos> echo hello`, fd0/runtime-console0
  descriptor-backed input in the scenario start line, visible output `hello`,
  `talos: descriptor-backed-output=true`, ready-for-next prompt=true,
  final classification `qemu-local-echo-command-complete`, and
  `qemu-local-echo-command: PASS`.
- Regression transcripts retained by the existing scripts:
  tasks/evidence/2026-05-31-qemu-local-command-stdin-descriptor-core/qemu-local-command-stdin-descriptor-smoke.log
  and
  tasks/evidence/2026-05-31-qemu-local-command-stdio-bridge-core/qemu-local-command-stdio-bridge-smoke.log.
- hardwareTestLock remained unlocked/restored and unused.

## Validation

- fmt/lint: `cargo fmt --all -- --check` passed.
- unit tests: `cargo -Zjson-target-spec test --quiet` passed with 341 tests.
- QEMU/substitute feature smoke:
  `scripts/qemu-local-echo-command-smoke.sh --quiet` passed.
- QEMU/substitute stdin descriptor regression:
  `scripts/qemu-local-command-stdin-descriptor-smoke.sh --quiet` passed.
- QEMU/substitute stdio bridge regression:
  `scripts/qemu-local-command-stdio-bridge-smoke.sh --quiet` passed.
- static inspection: `git diff --check` passed.
- documentation: `mdbook build` not run because docs/src was not touched.
- staged static inspection: `git diff --cached --check` passed before commit.

## Commit

Implementation and retained QEMU/substitute evidence commit:
33fcf235bf3528c75085c568404f3e7a3bd1d1ea.
