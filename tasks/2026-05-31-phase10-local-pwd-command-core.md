# Phase 10 Local Pwd Command Core Task

Task: phase10-local-pwd-command-core-20260531

Status: accepted

## Goal

Add the next feature-led local built-in: type `pwd` at the serial prompt,
dispatch it through the descriptor-backed command loop, print `/` through
descriptor-backed stdout, and return to a ready prompt.

## Scope

Implemented only the bounded kernel-backed `pwd` built-in on the accepted
local command-loop parser and dispatch path. The current directory is a
root-only placeholder exposed as `LOCAL_COMMAND_CURRENT_DIRECTORY`; it is
process-local in shape because it lives below the descriptor-backed local
command context, but it does not claim real VFS, path traversal, or `cd`
semantics.

Changed files:

- build.rs
- scripts/qemu-local-pwd-command-smoke.sh
- scripts/qemu-local-serial-command-loop-smoke.sh
- src/local_command_loop.rs
- src/target/qemu_virt.rs
- tasks/2026-05-31-phase10-local-pwd-command-core.md
- tasks/evidence/2026-05-31-qemu-local-pwd-command-core/qemu-local-pwd-command-smoke.log
- refreshed local command regression evidence under tasks/evidence/2026-05-31-qemu-local-*-core/

## Accepted Frontier

Accepted: the descriptor-backed local command loop can read `pwd` through
fd0/runtime-console0 canonical-lite input, dispatch a kernel-backed built-in,
write the visible `/` line through descriptor-backed stdout, preserve
descriptor-backed input/output markers, and return to a ready prompt.

Existing local command behavior remains covered: `help`, `status`, `stdio`,
`echo hello`, empty input, and unknown command behavior remain deterministic.
`help` now lists `pwd` with the other kernel-backed local built-ins.

## Deferred Surfaces

Deferred: `cd`, path traversal/normalization, VFS lookup, directory listing,
userspace shell execution, process spawning, exec/wait/exit, filesystem-backed
command lookup, pipes, redirection, globbing, environment expansion,
quoting/escaping semantics, termios/job control, networking, SSH, RP1/PCIe,
UART interrupt ownership, DMA/cache-driver policy, hardware proof, and broader
POSIX read/stdio behavior outside this descriptor-backed local command-loop
path.

## Evidence

- QEMU/substitute pwd command transcript:
  tasks/evidence/2026-05-31-qemu-local-pwd-command-core/qemu-local-pwd-command-smoke.log.
- Transcript includes `talos> pwd`, visible output `/`,
  fd0/runtime-console0 descriptor-backed input in the scenario start line,
  `talos: descriptor-backed-output=true`, ready-for-next prompt=true,
  final classification `qemu-local-pwd-command-complete`, and
  `qemu-local-pwd-command: PASS`.
- Unit coverage added for root-only `pwd` dispatch through
  `DescriptorBackedLocalCommandIo`.
- Regression transcripts refreshed by existing scripts:
  tasks/evidence/2026-05-31-qemu-local-echo-command-core/qemu-local-echo-command-smoke.log,
  tasks/evidence/2026-05-31-qemu-local-command-stdin-descriptor-core/qemu-local-command-stdin-descriptor-smoke.log,
  and
  tasks/evidence/2026-05-31-qemu-local-command-stdio-bridge-core/qemu-local-command-stdio-bridge-smoke.log.
- hardwareTestLock remained unlocked/restored and unused.

## Validation

- fmt/lint: `cargo fmt --all -- --check` passed.
- unit tests: `cargo -Zjson-target-spec test --quiet` passed with 342 tests
  after rerunning with the Talos QEMU 9.2.0 path exported.
- QEMU/substitute feature smoke:
  `scripts/qemu-local-pwd-command-smoke.sh --quiet` passed.
- QEMU/substitute echo regression:
  `scripts/qemu-local-echo-command-smoke.sh --quiet` passed.
- QEMU/substitute stdin descriptor regression:
  `scripts/qemu-local-command-stdin-descriptor-smoke.sh --quiet` passed.
- QEMU/substitute stdio bridge regression:
  `scripts/qemu-local-command-stdio-bridge-smoke.sh --quiet` passed.
- static inspection: `git diff --check` passed.
- documentation: `mdbook build` not run because docs/src was not touched.
- staged static inspection: `git diff --cached --check` passed before commit.

## Commit

Acceptance commit: recorded in durable supervisor state for
phase10-local-pwd-command-core-20260531 after commit creation.
