# Phase 5 Console Input Result Contract

Task: `phase5-console-input-result-contract-20260524`

Status: accepted as the Milestone 5.2 internal console input result contract.

## Scope

This task names the runtime-console polling input outcomes before Pi 5 input
evidence or descriptor-facing stdin work depends on them. `ConsoleInputPollOutcome`
now distinguishes:

- `ByteAvailable { device, byte }`: the selected backend produced one input byte.
- `NoData { device }`: the backend is present and RX is empty for this poll.
- `BackendUnavailable { device }`: no accepted input backend is attached to the
  console identity.
- `BackendError { device }`: a future backend error distinct from ordinary RX
  empty polling.

QEMU PL011 currently uses only byte-available and no-data outcomes. The current
backend has no recoverable error channel, so backend error remains a named
compatibility slot rather than a fabricated condition.

`PollingTtyRxResult` separately names diagnostic completion as `line-complete`,
`timeout`, `input-unavailable`, or `backend-error`. Timeout is produced by the
diagnostic after repeated `NoData` polls exceed its wait limit; it is not a
runtime-console backend outcome.

## Deferred POSIX Mapping

These names are internal kernel contracts. They are not POSIX `read`, EOF,
errno, readiness, nonblocking I/O, descriptor lifetime, scheduler blocking, or a
syscall ABI. Later descriptor work may translate these outcomes only after the
descriptor table, readiness policy, and scheduler sleep/wakeup behavior exist.

## Evidence

The QEMU RX diagnostic still consumes the target-owned PL011 through the
runtime-console input boundary. Its summary now reports the diagnostic outcome:

```text
qemu-tty-rx-diagnostic: raw-len=15 line-len=8 terminated=true timeout=false outcome=line-complete truncated=true backspaces=1 deletes=1 controls=1
qemu-tty-rx-diagnostic: line-hex=61 62 63 64 65 66 67 68
qemu-tty-rx-diagnostic: echo-hex=61 62 58 08 20 08 63 59 08 20 08 64 65 66 67 68 0d 0a
qemu-tty-rx-diagnostic: control-events=ctrl-c
qemu-tty-rx-diagnostic: PASS
```

Unit coverage includes byte-available/no-data input polling, named unavailable
and backend-error outcomes, line-complete diagnostic completion, and bounded
timeout after repeated no-data polls.

## Validation

- fmt/lint/typecheck: `cargo fmt --all -- --check` passed.
- unit tests: `cargo -Zjson-target-spec test` passed 84 no_std tests,
  including the new runtime-console input outcome coverage and diagnostic
  completion outcome coverage.
- QEMU/substitute: `scripts/qemu-smoke.sh` passed and preserved the accepted
  generic QEMU smoke behavior.
- QEMU/substitute: `scripts/qemu-tty-rx-diagnostic.sh` passed with
  `outcome=line-complete`, exact line bytes, exact echo bytes,
  `control-events=ctrl-c`, and PASS.
- static inspection: `git diff --check` passed.
- static inspection: `mdbook build` was not run because `mdbook` is
  unavailable in this container.

## Next Work

The next queued Milestone 5.2 task may run the serialized Pi 5 UART10 polling RX
proof and interpret hardware evidence against this internal input contract.
