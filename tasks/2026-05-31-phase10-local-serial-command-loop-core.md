# Phase 10 Local Serial Command Loop Core Task

Task: phase10-local-serial-command-loop-core-20260531

Status: accepted

## Scope

Feature-led Phase 10 implementation of the smallest local serial
interactivity path: type a line over serial, press Enter, dispatch a command,
print a visible response, and expose a prompt/ready marker for another
command.

Changed files:

- build.rs
- src/local_command_loop.rs
- src/main.rs
- src/pl011.rs
- src/runtime_console.rs
- src/target/qemu_virt.rs
- scripts/qemu-local-serial-command-loop-smoke.sh
- docs/src/roadmap.md
- docs/src/decisions/README.md
- tasks/2026-05-31-phase10-local-serial-command-loop-core.md
- tasks/evidence/2026-05-31-qemu-local-serial-command-loop-core/qemu-local-serial-command-loop-smoke.log

## Outcome

The implementation adds a bounded kernel-backed local command-loop boundary on
top of runtime-console0 and the accepted canonical-lite TTY line discipline.
It writes a `talos> ` prompt, reads one completed serial line, dispatches
kernel-backed built-ins, and reports visible responses for accepted, empty,
unknown, argument, parse, and input-error cases.

Accepted built-ins are intentionally minimal: `help` lists available commands
and `status` reports the command-loop version, runtime-console0, and that the
current built-ins are kernel-backed. Empty input and unknown input are visible
and deterministic. This task does not claim userspace shell execution,
descriptor-backed filesystem commands, process spawning, networking, SSH, or
Pi 5 hardware behavior.

The QEMU feature transcript script injects `help`, an empty line, and `bogus`
over the serial socket. The retained transcript shows the typed lines, Enter
dispatch, visible responses, and `ready-for-next prompt=true`.

## Evidence

- Implementation: src/local_command_loop.rs.
- QEMU/substitute feature transcript route:
  scripts/qemu-local-serial-command-loop-smoke.sh and src/target/qemu_virt.rs.
- Retained QEMU/substitute transcript:
  tasks/evidence/2026-05-31-qemu-local-serial-command-loop-core/qemu-local-serial-command-loop-smoke.log.
- Final retained classification:
  qemu-local-serial-command-loop-complete.
- Final retained PASS line:
  qemu-local-serial-command-loop: PASS.
- Hardware lock: hardwareTestLock remained unlocked/restored and unused.

## Validation

- fmt/lint: cargo fmt --all -- --check passed.
- unit tests: cargo -Zjson-target-spec test --quiet passed.
- QEMU/substitute:
  scripts/qemu-local-serial-command-loop-smoke.sh passed and retained the
  transcript log.
- static inspection: git diff --check passed.
- documentation: mdbook build passed.
- staged whitespace inspection: git diff --cached --check passed before
  commit.
- hardware: no Pi 5 archive publication, power cycle, TFTP action, serial
  observation, or hardware proof was performed.

## Commit

Recorded in durable supervisor state after acceptance.
