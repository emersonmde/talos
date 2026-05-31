# Phase 10 Local Command Stdio Bridge Core Task

Task: phase10-local-command-stdio-bridge-core-20260531

Status: accepted

## Scope

Feature-led Phase 10 implementation of the smallest descriptor-aware local
stdio step: type `stdio` at the serial prompt, press Enter, and receive a
visible response showing inherited fd 0/fd 1/fd 2 identities, runtime-console0
backing, descriptor-backed output routing, and readiness for the next prompt.

Changed files:

- src/local_command_loop.rs
- src/posix.rs
- src/target/qemu_virt.rs
- scripts/qemu-local-serial-command-loop-smoke.sh
- scripts/qemu-local-command-stdio-bridge-smoke.sh
- tasks/2026-05-31-phase10-local-command-stdio-bridge-core.md
- tasks/evidence/2026-05-31-qemu-local-command-stdio-bridge-core/qemu-local-command-stdio-bridge-smoke.log

## Outcome

The local command loop now advertises and dispatches a kernel-backed `stdio`
built-in. The command reports fd 0 as `stdio-input`, fd 1 and fd 2 as
`stdio-output`, `runtime-console0` as the backing console identity, and
`descriptor-backed-output=true`.

The QEMU command-loop smoke path now constructs a process-descriptor-store
owner with inherited stdio and writes prompts and command responses through a
descriptor-backed stdout bridge. runtime-console0 remains the backend helper
below the descriptor abstraction; the command client no longer writes directly
to a RuntimeConsole in the QEMU smoke path.

Help and status remain available from the same prompt. Empty input and unknown
commands remain deterministic and visible in the retained transcript.

## Accepted Frontier

Accepted: serial prompt, typed command input, Enter dispatch, descriptor-backed
visible output for the command-loop smoke path, inherited stdio identity
reporting for fd 0/fd 1/fd 2, runtime-console0 backing, and next-prompt
readiness.

Deferred: userspace shell execution, exec/spawn/wait, process lifecycle,
argv/envp, pipes, job control, terminal sessions, filesystem-backed commands,
POSIX-complete stdio, termios, descriptor inheritance across exec, networking,
SSH, RP1/PCIe, UART interrupt ownership, DMA/cache-driver policy, Pi 5
hardware proof, and paused Phase 8 proof-only work.

## Evidence

- Retained QEMU/substitute transcript:
  tasks/evidence/2026-05-31-qemu-local-command-stdio-bridge-core/qemu-local-command-stdio-bridge-smoke.log.
- Transcript includes typed `stdio`, `talos: ok stdio`, fd 0/fd 1/fd 2 stdio
  identities, `talos: runtime-console runtime-console0`,
  `talos: descriptor-backed-output=true`, `ready-for-next prompt=true`,
  classification `qemu-local-serial-command-loop-complete`, and
  `qemu-local-serial-command-loop: PASS`.
- Hardware lock: hardwareTestLock remained unlocked/restored and unused.

## Validation

- fmt/lint: cargo fmt --all -- --check passed.
- unit tests: cargo -Zjson-target-spec test --quiet passed.
- QEMU/substitute feature transcript:
  scripts/qemu-local-command-stdio-bridge-smoke.sh passed and retained the
  transcript log.
- QEMU/substitute descriptor regression:
  scripts/qemu-descriptor-write-smoke.sh passed because descriptor-to-console
  plumbing was touched.
- static inspection: git diff --check passed.
- documentation: mdbook build was not required because docs/src was not
  changed.
- staged whitespace inspection: git diff --cached --check passed before
  commit.
- hardware: no Pi 5 archive publication, power cycle, TFTP action, serial
  observation, or hardware proof was performed.

## Commit

Recorded in durable supervisor state after acceptance.
