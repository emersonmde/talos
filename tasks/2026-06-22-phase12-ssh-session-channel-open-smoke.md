# Phase 12.6 SSH session channel-open smoke

Task id: phase12-shell-ssh-session-channel-open-smoke-20260622

Status: accepted.

Classification: phase12-shell-ssh-session-channel-open-smoke-accepted.

## Goal

Retain host/QEMU-substitute smoke evidence for the accepted local modeled SSH
session channel-open classifier without expanding into shell attachment, live
reachability, hardware proof, compatibility, phase transition, or
ssh-ready=true.

## Scope

- Added scripts/qemu-shell-ssh-session-channel-open-smoke.sh as the retained
  fixed-label smoke command for this slice.
- Exercised the accepted source/unit session_channel_open cases through the
  configured target cargo test runner.
- Covered the modeled authenticated session-channel success path and
  representative fail-closed paths for missing authentication, wrong message,
  unsupported channel type, malformed shape, duplicate/existing channel,
  disabled policy, and redaction-sensitive input.
- Retained only fixed labels, public SSH message numbers, public field-count
  and public channel-type length categories, readiness counters, validation
  commands, paths, task ids, and classifications.

## Non-goals

No source feature expansion, PTY/TTY/process/shell attachment, channel data,
EOF/close/window flow control, shell/pty/exec/subsystem request behavior, live
network reachability, hardware/lab action, boot publication, OpenSSH/POSIX/Linux
compatibility claim, broad expansion, phase transition, or ssh-ready=true is
accepted.

Durable evidence must not retain request payload bytes, channel identifiers,
window sizes, packet sizes, user/operator identity, key material,
key-derived identifiers, stable identifiers, session-id bytes, signatures,
hardware data, or boot artifacts.

## Findings and Disposition

- fixed: retained a dedicated smoke script and transcript path for the
  session channel-open frontier instead of relying only on implementation-task
  evidence.
- fixed: retained smoke evidence covers the modeled authenticated
  SSH_MSG_CHANNEL_OPEN success path and records session-count=1 and
  channel-count=1 only for that success state.
- fixed: retained smoke evidence covers SSH_MSG_CHANNEL_OPEN_FAILURE for
  missing authentication, wrong message, unsupported channel type, malformed
  shape, duplicate/existing channel, disabled policy, and redaction-sensitive
  paths.
- fixed: retained smoke evidence records shell-attached=false,
  live-reachability=false, and ssh-ready=false for both success and failure
  paths.
- not-an-issue: the smoke task adds no new protocol behavior; it reruns the
  accepted session_channel_open source/unit coverage through the retained
  host/QEMU-substitute command.
- deferred: closeout reconciliation, shell attachment, PTY/process ownership,
  channel data, EOF/close/window handling, shell/pty/exec/subsystem requests,
  live reachability, hardware proof, compatibility, broad expansion, phase
  transition, and ssh-ready=true.

## Smoke Evidence

Retained transcript:

- tasks/evidence/2026-06-22-ssh-session-channel-open-smoke/qemu-shell-ssh-session-channel-open-smoke.log
- tasks/evidence/2026-06-22-ssh-session-channel-open-smoke/evidence-map.md

The transcript records:

- Success state:
  sshservicediag-authentication-success-local-only,
  sshservicediag-session-channel-open-prerequisite-only,
  sshservicediag-session-channel-open-session-type,
  sshservicediag-session-open-local-only,
  sshservicediag-channel-open-local-only,
  sshservicediag-shell-unattached, SSH_MSG_CHANNEL_OPEN=90,
  SSH_MSG_CHANNEL_OPEN_CONFIRMATION=91, authentication-success=true,
  session-count=1, channel-count=1, shell-attached=false,
  live-reachability=false, and ssh-ready=false.
- Failure message:
  SSH_MSG_CHANNEL_OPEN_FAILURE=92, session-count=0, channel-count=0,
  shell-attached=false, live-reachability=false, and ssh-ready=false.
- Fail-closed states for authentication-missing, wrong-message,
  unsupported-channel-type, request-malformed, duplicate-existing-channel,
  policy-disabled, and redaction-sensitive paths.

The retained source/unit test filter is session_channel_open, covering:

- session_channel_open_accepts_one_modeled_authenticated_session_only.
- session_channel_open_fails_closed_for_prerequisites_and_policy.
- session_channel_open_fails_closed_for_message_type_and_shape.

## Validation

- scripts/qemu-shell-ssh-session-channel-open-smoke.sh: pass under the
  configured host/QEMU-substitute cargo test runner; retained transcript ends
  with classification=host-qemu-substitute-shell-ssh-session-channel-open-smoke-complete.
- cargo fmt --all -- --check: pass.
- cargo -Zjson-target-spec test session_channel_open --quiet: pass.
- cargo -Zjson-target-spec test --quiet: pass.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check: pass.

No Pi 5 hardware run, lab-controller API action, hardwareTestLock acquisition,
boot publication, shell attachment, live reachability claim, compatibility
claim, broad expansion, or phase transition was performed.

## Redaction Review

Pass. Retained smoke evidence contains only task ids, paths, fixed labels,
public SSH message names/numbers, public field-count and public channel-type
length categories, readiness counters, validation commands, test names, and
classifications. It retains no request payload bytes, channel identifiers,
window sizes, packet sizes, user/operator identity, key material,
key-derived identifiers, stable identifiers, session-id bytes, signatures,
hardware data, or boot artifacts.

## Acceptance

Accepted as bounded retained host/QEMU-substitute smoke evidence for the local
modeled SSH session channel-open frontier.

selected_next_task=phase12-ssh-session-channel-open-closeout-20260622.
