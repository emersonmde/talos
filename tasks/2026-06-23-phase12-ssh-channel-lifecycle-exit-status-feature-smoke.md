# Phase 12.6 SSH channel lifecycle EOF/exit-status feature smoke

Task id: phase12-ssh-channel-lifecycle-exit-status-feature-smoke-20260623

Status: accepted

Classification: phase12-ssh-channel-lifecycle-exit-status-feature-smoke-accepted

## Goal

Retain bounded feature smoke/regression evidence for the accepted local modeled
SSH channel lifecycle EOF, exit-status, and close behavior from
phase12-ssh-channel-lifecycle-exit-status-core-20260623.

## Scope

- Reused the accepted local no_std test surface in src/ssh_service_readiness.rs.
- Exercised accepted inbound SSH_MSG_CHANNEL_EOF receipt, outbound
  SSH_MSG_CHANNEL_REQUEST exit-status emission with want_reply=false, outbound
  SSH_MSG_CHANNEL_CLOSE, and inbound SSH_MSG_CHANNEL_CLOSE on the accepted local
  authenticated shell channel path.
- Retained fail-closed controls for duplicate EOF, late exit-status after
  close, malformed/over-limit lifecycle payloads, missing authentication,
  missing channel, missing shell attachment, missing local execution, unsupported
  message/request type, redaction-sensitive input, and prior channel-data/window
  regression surfaces.
- Added no new source behavior, no live socket delivery, no remote receipt
  proof, no hardware/lab action, no boot publication, no OpenSSH compatibility
  claim, no process wait/exit implementation, no phase transition, and no
  ssh-ready=true claim.

## Smoke Evidence

The task-owned feature evidence is the existing local no_std unit coverage:

- channel_lifecycle_models_eof_exit_status_and_close_without_socket_readiness:
  covers accepted local EOF receipt, exit-status request emission, outbound
  close, inbound close, channel_lifecycle_local=true only on local modeled
  success, live-reachability=false, and ssh-ready=false.
- channel_lifecycle_fails_closed_for_duplicate_and_invalid_ordering: covers
  duplicate EOF and lifecycle-invalid late exit-status after both close
  directions complete.
- channel_lifecycle_fails_closed_for_message_shape_and_prerequisites: covers
  missing authentication, missing channel, missing shell attachment,
  redaction-sensitive input, unsupported message, unsupported request type,
  malformed close payload, over-limit payload, and missing local execution.
- channel_lifecycle_preserves_channel_data_and_window_regression_surfaces:
  retains the prior channel-data/stdio and channel-window accounting regression
  boundary around lifecycle close state.

The local harness command was:

    cargo -Zjson-target-spec test channel_lifecycle --quiet

The repository's custom no_std harness ran the full 808-test corpus for that
command and passed. This is local unit-test evidence only; it is not live
encrypted socket delivery, remote receipt, hardware reachability,
OpenSSH/POSIX/Linux compatibility, process wait/exit implementation, a phase
transition, or ssh-ready=true.

## Findings

- not-an-issue: the accepted core task already contains the feature and
  regression tests needed for this smoke slice, so no new source behavior was
  required.
- fixed: retained explicit task-owned evidence mapping EOF, exit-status, and
  close behavior to the accepted local modeled SSH shell channel lifecycle.
- fixed: retained duplicate, invalid-ordering, malformed, over-limit,
  unsupported, missing-prerequisite, missing-local-execution, redaction, and
  prior channel-data/window regression controls.
- deferred: live encrypted socket delivery, remote receipt, hardware proof,
  OpenSSH/POSIX/Linux compatibility, process wait/exit, broad command
  expansion, phase transition, and ssh-ready=true remain for later explicit
  tasks.

## Evidence

- Feature smoke/unit tests:
  cargo -Zjson-target-spec test channel_lifecycle --quiet: pass; custom no_std
  harness ran 808 tests.
- Full unit gate:
  cargo -Zjson-target-spec test --quiet: pass with QEMU 9.2.0 on PATH, 808
  no_std tests passed.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check: pass.

## Redaction Review

Pass. Retained evidence is limited to task ids, source/doc paths, public SSH
message names and numbers, fixed test/function names, public request type names,
public exit status values, booleans, public lengths/categories, validation
commands, and classifications. It retains no private user data, channel
identifiers, request payload bytes, command payload bytes, channel data payload
bytes, key/session material, live peer data, hardware data, or boot artifacts.

## Result

Accepted as bounded local feature smoke/regression evidence for the modeled SSH
channel lifecycle EOF, exit-status, and close layer.

selected_next_task=phase12-ssh-channel-lifecycle-exit-status-closeout-20260623.
