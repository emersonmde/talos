# Phase 12.6 SSH channel lifecycle EOF/exit-status core

Task id: phase12-ssh-channel-lifecycle-exit-status-core-20260623

Status: accepted

Classification: phase12-ssh-channel-lifecycle-exit-status-core-accepted

## Goal

Implement the bounded local modeled SSH channel lifecycle core for
SSH_MSG_CHANNEL_EOF, SSH_MSG_CHANNEL_REQUEST exit-status emission, and
SSH_MSG_CHANNEL_CLOSE around the already accepted authenticated session channel,
shell attachment, channel-data/stdio, and channel-window accounting path.

## Reviewed Inputs

- tasks/2026-06-23-phase12-ssh-session-channel-open-closeout.md.
- tasks/2026-06-23-phase12-ssh-session-shell-attachment-closeout.md.
- tasks/2026-06-23-phase12-ssh-channel-data-stdio-closeout.md.
- tasks/2026-06-23-phase12-ssh-channel-window-accounting-closeout.md.
- docs/src/project/phase12-networking-ssh.md.
- docs/src/roadmap.md.
- src/ssh_service_readiness.rs.

## Accepted Contract

Talos accepts only local modeled channel lifecycle state for the single accepted
SSH session channel after local authentication success, channel-open, shell
attachment, local process/session ownership, fd0/fd1/fd2 stdio ownership, and
open lifecycle prerequisites.

Inbound SSH_MSG_CHANNEL_EOF is accepted as local EOF receipt when the channel is
open. It records EOF state only; it does not prove remote delivery, process
exit, POSIX wait status, or socket behavior.

Local exit-status is modeled as an outbound SSH_MSG_CHANNEL_REQUEST with request
type exit-status, want_reply=false, and a public u32 exit status value supplied
by the caller. The model requires shell attachment, local process/session
ownership, an open channel lifecycle, and an available local exit status. It
does not implement process wait/exit.

SSH_MSG_CHANNEL_CLOSE is modeled in both directions. Sending close records a
local outbound close message. Receiving close records inbound close. The
channel lifecycle remains open until both directions have closed, then further
lifecycle and channel-data operations fail closed.

Malformed, over-limit, unsupported-message, unsupported-request-type, duplicate,
missing-prerequisite, lifecycle-invalid, local-execution-missing, and
redaction-sensitive inputs fail closed without claiming remote receipt,
compatibility, process exit, hardware reachability, phase transition, or
ssh-ready=true.

## Findings

- fixed: added fixed labels and SSH message constants for local modeled
  CHANNEL_EOF, CHANNEL_CLOSE, and exit-status CHANNEL_REQUEST behavior.
- fixed: added SshChannelLifecycleState and report surfaces that record EOF,
  exit-status emission, close sent, close received, and open/closed lifecycle
  state without retaining channel identifiers or payload bytes.
- fixed: added inbound lifecycle classification for EOF and CLOSE with
  fail-closed unsupported message/request, malformed, over-limit, duplicate,
  prerequisite, lifecycle, and redaction-sensitive controls.
- fixed: added local outbound exit-status and close classifiers with explicit
  local execution and lifecycle prerequisites.
- fixed: added unit coverage for successful local modeled EOF, exit-status, and
  close behavior, duplicate/ordering controls, malformed/over-limit controls,
  missing prerequisites, and channel-data/window regressions.
- deferred: live encrypted socket delivery, remote receipt proof,
  hardware/lab proof, OpenSSH/POSIX/Linux compatibility, process wait/exit,
  broad command expansion, phase transition, and ssh-ready=true remain outside
  this task.

## Source And Tests

- Source: src/ssh_service_readiness.rs.
- Unit tests:
  - channel_lifecycle_models_eof_exit_status_and_close_without_socket_readiness
  - channel_lifecycle_fails_closed_for_duplicate_and_invalid_ordering
  - channel_lifecycle_fails_closed_for_message_shape_and_prerequisites
  - channel_lifecycle_preserves_channel_data_and_window_regression_surfaces
- Regression surfaces preserved:
  - channel_data_stdio_accepts_local_modeled_inbound_stdin
  - channel_data_stdio_fails_closed_for_prerequisites_and_lifecycle
  - channel_window_accounting_decrements_inbound_data_without_socket_readiness
  - channel_window_accounting_rejects_over_window_data_without_mutation

## Evidence

- Focused lifecycle/unit gate:
  cargo -Zjson-target-spec test channel_lifecycle --quiet: pass; custom no_std
  harness ran 808 tests.
- Full unit gate:
  cargo -Zjson-target-spec test --quiet: pass with QEMU 9.2.0 on PATH, 808
  no_std tests passed.
- cargo fmt --all -- --check: pass.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check: pass.

## Redaction Review

Pass. Retained evidence is limited to task ids, source/doc paths, fixed labels,
public SSH message names and numbers, public request type names, public exit
status values, booleans, validation commands, and classifications. It retains
no private user data, channel identifiers, command payload bytes, channel data
payload bytes, key/session material, live peer data, hardware data, or boot
artifacts.

## Result

Accepted as the bounded local modeled SSH channel lifecycle EOF/exit-status
core.

selected_next_task=phase12-ssh-channel-lifecycle-exit-status-feature-smoke-20260623.
