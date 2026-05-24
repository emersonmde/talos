# Phase 5 Local Diagnostic Command Channel Source Inventory

Task: phase5-local-diagnostic-command-channel-source-inventory-20260524

Status: accepted as the Milestone 5.3 source inventory before command-channel
implementation.

## Scope

This task inventoried the accepted console, TTY, and diagnostic surfaces that a
future kernel-owned local diagnostic command channel may consume. It made no
Rust code changes and did not implement a parser, dispatcher, registry, hardware
run, descriptor table, syscall ABI, userspace shell, filesystem, networking,
SSH, SMP, UART interrupt path, or scheduler blocking I/O.

## Inventory Summary

- Accepted input/output boundary: runtime-console0, ConsoleWriteOutcome,
  ConsoleInputPollOutcome, TtyLineDiscipline, and PollingTtyRxResult.
- Accepted input evidence: QEMU PL011 polling RX and Pi 5 firmware-preserved
  UART10 polling RX both pass through the same runtime-console/TTY boundary and
  share the same canonical-lite byte sequence evidence.
- Candidate command providers: bounded help/list, status, timer/tick,
  scheduler, and memory/runtime status providers, subject to the next contract
  task choosing exact names and response framing.
- Boot-only regression surfaces: QEMU architecture smokes and Pi 5 hardware
  diagnostic images remain validation gates, not first command-channel
  interfaces.
- Deferred surfaces: shell grammar, descriptor/syscall/POSIX semantics,
  filesystem-backed commands, networking, SSH, SMP, UART interrupts, scheduler
  blocking I/O, RP1 UART0, and destructive fault/allocator-trigger commands.

## Recommended Next Task

phase5-diagnostic-command-channel-contract-20260524 should define the durable
contract and minimal target-independent parser/dispatcher shape.

Recommended acceptance criteria:

- consume complete TTY lines, not UART bytes;
- provide deterministic bounded unknown, help/list, and status behavior;
- keep parser/dispatcher response framing kernel-diagnostic-only;
- preserve separation from descriptor, syscall, shell, filesystem, networking,
  SSH, SMP, UART interrupt, and scheduler blocking behavior;
- add focused parser/dispatcher tests if Rust code changes are made.

## Validation

- static inspection: git status --short was clean before edits.
- static inspection: git diff --check passed after documentation edits.
- static inspection: mdbook build was not run because mdbook is unavailable in
  this container.
- Rust fmt/tests were not required because this task changed only Markdown
  documentation and durable task state.
