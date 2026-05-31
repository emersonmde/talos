# Phase 10 Local Command Stdin Descriptor Core Task

Task: phase10-local-command-stdin-descriptor-core-20260531

## Goal

Route the local command-loop input side through an fd0/runtime-console-backed
descriptor read path while preserving the visible serial prompt, Enter
dispatch, descriptor-backed stdout response, and next-prompt readiness.

## Changed Files

- src/local_command_loop.rs
- src/target/qemu_virt.rs
- src/target/rpi5.rs
- src/runtime_console.rs
- build.rs
- scripts/qemu-local-serial-command-loop-smoke.sh
- scripts/qemu-local-command-stdin-descriptor-smoke.sh
- tasks/2026-05-31-phase10-local-command-stdin-descriptor-core.md
- tasks/evidence/2026-05-31-qemu-local-command-stdin-descriptor-core/qemu-local-command-stdin-descriptor-smoke.log

## Accepted Frontier

Accepted: the local command loop now has a descriptor-backed stdio I/O adapter
for the feature path. The adapter resolves inherited fd 0 as stdio-input before
polling runtime-console0/canonical-lite input, and it resolves fd 1 as
stdio-output before writing prompts and command responses. The retained
QEMU/substitute transcript shows typed stdio input, fd0 descriptor input,
visible stdio response, fd 0/fd 1/fd 2 identities, runtime-console0 backing,
descriptor-backed output, empty and unknown command behavior, next-prompt
readiness, final classification, and PASS.

Deferred: userspace shell execution, exec/spawn/wait, process lifecycle,
argv/envp, pipes, terminal sessions, job control, signals, networking, SSH,
writable filesystem behavior, filesystem-backed commands, POSIX-complete
read(2), blocking readiness, termios, async UART interrupts, descriptor
inheritance across exec, and general stdin for arbitrary processes.

## Evidence

- QEMU/substitute stdin descriptor transcript:
  tasks/evidence/2026-05-31-qemu-local-command-stdin-descriptor-core/qemu-local-command-stdin-descriptor-smoke.log.
- Transcript includes typed stdio input, `talos: descriptor-backed-input=true`,
  `talos: ok stdio`, fd 0/fd 1/fd 2 stdio identity lines,
  `talos: runtime-console runtime-console0`,
  `talos: descriptor-backed-output=true`,
  `qemu-local-command-stdin-descriptor: ready-for-next prompt=true`,
  `classification=qemu-local-command-stdin-descriptor-complete`, and
  `qemu-local-command-stdin-descriptor: PASS`.
- hardwareTestLock remained unlocked/restored and unused.

## Validation

- fmt/lint: `cargo fmt --all -- --check` passed.
- unit tests: `cargo -Zjson-target-spec test --quiet` passed.
- QEMU/substitute feature smoke:
  `scripts/qemu-local-command-stdin-descriptor-smoke.sh` passed.
- QEMU/substitute adjacent read smoke:
  `scripts/qemu-read-stdin-smoke.sh` passed; this is the existing descriptor
  read smoke because `scripts/qemu-descriptor-read-smoke.sh` is not present.
- QEMU/substitute adjacent write smoke:
  `scripts/qemu-descriptor-write-smoke.sh` passed.
- QEMU/substitute compatibility smoke:
  `scripts/qemu-local-command-stdio-bridge-smoke.sh` passed.
- static inspection: `git diff --check` passed.
- documentation: `mdbook build` not run because docs/src was not touched.
- staged static inspection: pending before commit.

## Commit

Pending.
