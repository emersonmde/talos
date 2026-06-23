# Phase 12.6 SSH channel-window accounting core

Task id: phase12-ssh-channel-window-accounting-core-20260623

Status: accepted

Classification: phase12-ssh-channel-window-accounting-core-accepted

## Goal

Implement exactly the bounded local modeled channel-window accounting contract
accepted by phase12-ssh-channel-window-accounting-contract-20260623.

## Scope

- Added local modeled SSH channel-window accounting labels, state, reports, and
  classifiers in src/ssh_service_readiness.rs.
- Maintained per-channel local receive-window and remote receive-window/send
  budget counters for the accepted channel-data/stdio path.
- Accounted inbound SSH_MSG_CHANNEL_DATA, outbound stdout/stderr channel-data
  reports, and inbound SSH_MSG_CHANNEL_WINDOW_ADJUST messages without retaining
  payload bytes.
- Preserved the existing channel-data/stdio classifier behavior and used it as
  the prerequisite for local modeled stdio delivery.
- Kept live socket delivery, hardware/lab action, boot publication, packet I/O,
  OpenSSH compatibility claims, broad shell command expansion, phase transition,
  and ssh-ready=true out of scope.

## Accepted Behavior

Talos now models channel-window accounting only for the already accepted local
authentication/session/channel/shell/local-stdio/open-lifecycle path.

Inbound SSH_MSG_CHANNEL_DATA first has to pass the accepted channel-data/stdio
shape and prerequisite checks. Accepted inbound data then decrements the local
receive-window counter by public data length. If the remaining local receive
window crosses the fixed low-water mark, the report emits a modeled
SSH_MSG_CHANNEL_WINDOW_ADJUST message number 93 and a public bytes-to-add count
that restores the local receive window to the fixed initial value.

Outbound stdout and stderr reports first have to pass the accepted local stdio
shape and prerequisite checks. Accepted stdout maps to SSH_MSG_CHANNEL_DATA and
accepted stderr maps to SSH_MSG_CHANNEL_EXTENDED_DATA with
SSH_EXTENDED_DATA_STDERR. Both decrement the remote receive-window/send-budget
counter by public data length.

Inbound SSH_MSG_CHANNEL_WINDOW_ADJUST accepts only message number 93 with
recipient-channel and nonzero bytes-to-add fields and no trailing data. The
bytes-to-add field increases the remote receive-window/send-budget counter only
when the addition does not overflow.

Successful local modeled reports may set channel-window-management=true.
channel-data-stdio-local=true remains limited to accepted channel-data/stdio
data paths. live-reachability=false and ssh-ready=false remain authoritative.

## Failure Behavior

Malformed, zero, over-limit, over-window, overflow, unsupported,
redaction-sensitive, prerequisite-missing, and lifecycle-invalid inputs fail
closed. Failure reports do not deliver stdio, do not mutate window counters,
and do not claim socket delivery, remote receipt, OpenSSH compatibility,
hardware reachability, phase transition, or ssh-ready=true.

## Findings

- fixed: implemented local receive-window state and decrement/adjust behavior
  for accepted inbound SSH_MSG_CHANNEL_DATA.
- fixed: implemented remote receive-window/send-budget decrement behavior for
  accepted outbound stdout/stderr reports.
- fixed: implemented inbound SSH_MSG_CHANNEL_WINDOW_ADJUST consumption with
  nonzero and no-overflow checks.
- fixed: added readiness labels and report counters that move
  channel-window-management=true only on accepted local modeled success paths.
- fixed: added fail-closed controls for over-window data, malformed
  WINDOW_ADJUST, overflow, redaction-sensitive input, and prior
  channel-data/stdio rejection paths.
- not-an-issue: live encrypted socket delivery, hardware reachability,
  OpenSSH/POSIX/Linux compatibility, broad shell command expansion, phase
  transition, and ssh-ready=true remain deferred by task scope.

## Source And Tests

- Source: src/ssh_service_readiness.rs.
- Unit tests:
  - channel_window_accounting_decrements_inbound_data_without_socket_readiness
  - channel_window_accounting_emits_local_window_adjust_at_threshold
  - channel_window_accounting_decrements_outbound_and_accepts_window_adjust
  - channel_window_accounting_rejects_over_window_data_without_mutation
  - channel_window_adjust_fails_closed_for_malformed_or_overflow
- Regression controls preserved:
  - channel_data_stdio_accepts_local_modeled_inbound_stdin
  - channel_data_stdio_reports_local_stdout_and_stderr_packet_shapes
  - channel_data_stdio_fails_closed_for_prerequisites_and_lifecycle
  - channel_data_stdio_fails_closed_for_message_shape_and_output_controls

## Evidence

- cargo fmt --all -- --check: pass.
- cargo -Zjson-target-spec test --quiet: pass with QEMU 9.2.0 on PATH, custom
  no_std harness ran 804 tests.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check: pass.

## Redaction Review

Pass. Retained evidence is limited to task ids, source/doc paths, fixed labels,
public SSH message names and numbers, public data lengths, public window
counters, boolean readiness counters, validation commands, and classifications.
It retains no private user data, channel identifiers, payload bytes, command
bytes, key/session material, live peer data, hardware data, or boot artifacts.

## Result

Accepted as the bounded local modeled SSH channel-window accounting core.

selected_next_task=phase12-ssh-channel-window-accounting-feature-smoke-20260623.
