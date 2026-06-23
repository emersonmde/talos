# Phase 12.6 SSH channel-data stdio feature smoke

Task id: phase12-ssh-channel-data-stdio-feature-smoke-20260623

Status: accepted

Classification: phase12-ssh-channel-data-stdio-feature-smoke-accepted

## Goal

Retain bounded feature smoke/regression evidence for the accepted local modeled
SSH channel-data/stdio bridge from
phase12-ssh-channel-data-stdio-core-20260623.

## Scope

- Reused the accepted local no_std test surface in src/ssh_service_readiness.rs.
- Exercised the accepted successful inbound SSH_MSG_CHANNEL_DATA to attached
  stdin boundary and outbound stdout/stderr packet-shape reports.
- Retained regression coverage for missing shell attachment, malformed or
  over-limit channel data, lifecycle violation, and prior authentication,
  channel, and shell-attachment fail-closed controls.
- Added no new source behavior, no live socket path, no hardware/lab action, no
  channel-window implementation, no compatibility claim, no phase transition,
  and no ssh-ready=true claim.

## Smoke Evidence

The task-owned feature evidence is the existing local no_std unit coverage:

- channel_data_stdio_accepts_local_modeled_inbound_stdin: covers the successful
  inbound SSH_MSG_CHANNEL_DATA path to the attached-shell stdin ownership
  boundary with channel-data-stdio-local=true, shell-attached=true,
  live-reachability=false, channel-window-management=false, and ssh-ready=false.
- channel_data_stdio_reports_local_stdout_and_stderr_packet_shapes: covers
  local stdout as SSH_MSG_CHANNEL_DATA and stderr as
  SSH_MSG_CHANNEL_EXTENDED_DATA with SSH_EXTENDED_DATA_STDERR from accepted
  attached stdio ownership.
- channel_data_stdio_fails_closed_for_prerequisites_and_lifecycle: covers
  missing authentication, missing channel, missing shell attachment, missing
  local stdio ownership, lifecycle violation, and redaction-sensitive input.
- channel_data_stdio_fails_closed_for_message_shape_and_output_controls: covers
  unsupported messages, unsupported inbound extended data, malformed/trailing
  data, zero-length data, over-limit data, missing outbound shell attachment,
  and over-limit outbound stderr data.

The local harness command was:

    cargo -Zjson-target-spec test channel_data_stdio --quiet

The repository's custom no_std harness ran the full 799-test corpus for that
command and passed. This is local unit-test evidence only; it is not live SSH
reachability, hardware reachability, OpenSSH/POSIX/Linux compatibility,
channel-window management, a phase transition, or ssh-ready=true.

## Findings

- not-an-issue: the accepted core task already contains the feature and
  regression tests needed for this smoke slice, so no new source behavior was
  required.
- fixed: retained explicit task-owned evidence mapping the local modeled
  channel-data/stdio success path to representative outbound stdout/stderr
  packet-shape evidence.
- fixed: retained regression evidence for missing shell attachment,
  malformed/over-limit channel data, lifecycle violation, and previous
  authentication/channel/shell-attachment fail-closed paths.
- deferred: live encrypted socket delivery, channel-window accounting, remote
  receipt, hardware proof, OpenSSH/POSIX/Linux compatibility, broader
  EOF/close/exit-status behavior, phase transition, and ssh-ready=true remain
  for later explicit tasks.

## Evidence

- Feature smoke/unit tests:
  cargo -Zjson-target-spec test channel_data_stdio --quiet: pass; custom
  no_std harness ran 799 tests.
- Full unit gate:
  cargo -Zjson-target-spec test --quiet: pass with QEMU 9.2.0 on PATH, 799
  no_std tests passed.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check: pass.

## Redaction Review

Pass. Retained evidence is limited to task ids, source/doc paths, public SSH
message names and numbers, fixed test/function names, public counters,
booleans, public lengths/categories, validation commands, and classifications.
It retains no private user data, channel identifiers, request payload bytes,
command payload bytes beyond fixed public test labels/lengths, key/session
material, hardware data, or boot artifacts.

## Result

Accepted as bounded local feature smoke/regression evidence for the modeled SSH
channel-data/stdio bridge.

selected_next_task=phase12-ssh-channel-data-stdio-closeout-20260623.
