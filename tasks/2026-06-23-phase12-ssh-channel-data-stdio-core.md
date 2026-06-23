# Phase 12.6 SSH channel-data stdio core

Task id: phase12-ssh-channel-data-stdio-core-20260623

Status: accepted

Classification: phase12-ssh-channel-data-stdio-core-accepted

## Goal

Implement exactly the bounded local modeled channel-data/stdio bridge contract
selected by phase12-ssh-channel-data-stdio-contract-20260623.

## Scope

- Added local modeled channel-data/stdio classifier and report surfaces in
  src/ssh_service_readiness.rs.
- Added inbound SSH_MSG_CHANNEL_DATA parsing for the attached-shell stdin
  ownership boundary.
- Added local outbound stdout/stderr packet-shape reports for attached stdio
  ownership.
- Added fail-closed labels and no_std unit coverage for the accepted success
  path and representative controls.
- Kept live TCP socket delivery, Pi 5 hardware/lab runs, boot publication,
  broad command expansion, filesystem/userland feature expansion,
  channel-window management, OpenSSH/POSIX/Linux compatibility, phase
  transition, and ssh-ready=true out of scope.

## Accepted Behavior

The inbound local bridge succeeds only when all of these public model facts are
true:

- accepted local authentication success is present;
- one local modeled session channel is open;
- shell attachment is active;
- local process/session ownership is present;
- fd0/fd1/fd2 local stdio descriptor ownership is present;
- channel lifecycle remains open;
- the decrypted payload is SSH_MSG_CHANNEL_DATA with recipient-channel and
  bounded nonzero data fields and no trailing data;
- retained evidence uses only public message names or numbers, fixed labels,
  counters, and data lengths.

On that path the report sets channel-data-stdio-local=true while preserving
shell-attached=true, live-reachability=false, channel-window-management=false,
and ssh-ready=false. Payload bytes are not retained as durable evidence.

The outbound local reports are packet-construction evidence only:

- stdout maps to SSH_MSG_CHANNEL_DATA with a public bounded data length;
- stderr maps to SSH_MSG_CHANNEL_EXTENDED_DATA with
  SSH_EXTENDED_DATA_STDERR and a public bounded data length.

These reports do not prove encrypted socket writes, remote receipt, channel
window accounting, or OpenSSH interoperability.

## Failure Behavior

The classifier fails closed without channel-data/stdio delivery for missing
authentication, missing channel, missing shell attachment, missing local
process/session or stdio ownership, unsupported message families, unsupported
inbound SSH_MSG_CHANNEL_EXTENDED_DATA, malformed input, trailing data,
zero-length data, over-limit data, lifecycle violations, and
redaction-sensitive input.

Outbound reports fail closed for missing shell attachment, missing stdio
ownership, closed lifecycle, zero-length or over-limit data, and
redaction-sensitive output. Failure paths preserve ssh-ready=false and do not
claim live reachability.

## Findings

- fixed: implemented the bounded inbound SSH_MSG_CHANNEL_DATA to attached
  stdin ownership boundary with public data-length evidence only.
- fixed: implemented local stdout/stderr packet-shape reports for
  SSH_MSG_CHANNEL_DATA and SSH_MSG_CHANNEL_EXTENDED_DATA with
  SSH_EXTENDED_DATA_STDERR.
- fixed: added explicit readiness labels and counters for
  channel-data-stdio-local without changing live-reachability,
  channel-window-management, or ssh-ready.
- fixed: added fail-closed controls for missing authentication, missing
  channel, missing shell attachment, missing stdio ownership, unsupported
  message families, unsupported inbound extended data, malformed/trailing data,
  over-limit data, lifecycle violations, and redaction-sensitive input.
- not-an-issue: live socket delivery, channel-window accounting, hardware
  reachability, compatibility, and command expansion remain deferred by the
  accepted task scope.

## Evidence

- Source/unit tests: src/ssh_service_readiness.rs adds
  channel_data_stdio_accepts_local_modeled_inbound_stdin,
  channel_data_stdio_reports_local_stdout_and_stderr_packet_shapes,
  channel_data_stdio_fails_closed_for_prerequisites_and_lifecycle, and
  channel_data_stdio_fails_closed_for_message_shape_and_output_controls.
- cargo -Zjson-target-spec test channel_data_stdio --quiet: pass with QEMU
  9.2.0 on PATH, custom no_std harness ran 799 tests.
- cargo fmt --all -- --check: pass.
- cargo -Zjson-target-spec test --quiet: pass with QEMU 9.2.0 on PATH, custom
  no_std harness ran 799 tests.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check: pass.

## Redaction Review

Pass. Retained evidence is limited to task ids, source/doc paths, fixed labels,
public SSH message names and numbers, public field counts, public data lengths,
boolean readiness counters, validation commands, and classifications. It
retains no private user data, channel identifiers, request or command payload
bytes, key/session material, live peer data, hardware data, or boot artifacts.

## Result

Accepted as the bounded local modeled SSH channel-data/stdio core.

selected_next_task=phase12-ssh-channel-data-stdio-feature-smoke-20260623.
