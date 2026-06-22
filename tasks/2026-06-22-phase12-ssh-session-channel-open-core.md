# Phase 12.6 SSH session channel-open core

Task id: phase12-ssh-session-channel-open-core-20260622

Status: accepted.

Classification: phase12-ssh-session-channel-open-core-accepted.

## Goal

Implement the bounded local modeled SSH session channel-open source slice after
the accepted publickey USERAUTH_SUCCESS account-policy prerequisite, while
keeping shell attachment, PTY/process ownership, live reachability, hardware
proof, compatibility, broad expansion, phase transition, and ssh-ready=false
out of scope.

## Scope

- Added a local source classifier for one SSH_MSG_CHANNEL_OPEN request after
  the accepted authentication-success prerequisite.
- Accepted SSH_MSG_CHANNEL_OPEN_CONFIRMATION only when authentication succeeded,
  channel-open policy is enabled, no modeled session/channel already exists,
  redaction is not sensitive, the public message number is SSH_MSG_CHANNEL_OPEN,
  the public channel type string is session, and the bounded five-field shape
  is exact.
- Kept missing authentication, disabled policy, duplicate/existing channel,
  redaction-sensitive paths, wrong message number, unsupported channel type,
  malformed packets, over-limit channel type/shape, and trailing data fail
  closed with SSH_MSG_CHANNEL_OPEN_FAILURE.
- Exposed fixed labels, public SSH message names/numbers, public field counts,
  public channel-type byte length, validation commands, test names, and
  readiness counters only.

## Non-goals preserved

No smoke script retention, packet I/O, live socket reachability, hardware/lab
action, boot publication, PTY/TTY/process/shell attachment, channel data,
EOF/close/window flow control, shell/pty/exec/subsystem request handling,
process launch, filesystem-backed command execution, OpenSSH/POSIX/Linux
compatibility claim, broad expansion, phase transition, or ssh-ready=true is
accepted by this task.

## Implementation

Source changed:

- src/ssh_service_readiness.rs

The new SshSessionChannelOpenInput and classify_ssh_session_channel_open path
models one post-authentication session channel-open boundary. The success path
returns SSH_MSG_CHANNEL_OPEN_CONFIRMATION and fixed labels:

- authentication-success-local-only;
- session-channel-open-prerequisite-only;
- session-channel-open-session-type;
- session-open-local-only;
- channel-open-local-only;
- shell-unattached;
- not-ready.

Failure paths return SSH_MSG_CHANNEL_OPEN_FAILURE with fixed labels for
authentication-missing, policy-disabled, unsupported-message,
unsupported-channel-type, request-malformed, existing-channel, and
redaction-sensitive cases.

Only the modeled success case reports authentication_success=true,
session_count=1, and channel_count=1. shell_attached=false,
reachability_accepted=false, and ssh_ready=false remain authoritative for every
case.

Docs changed:

- docs/src/project/phase12-networking-ssh.md
- docs/src/roadmap.md

docs/src/decisions/README.md was not changed because this task implements the
already accepted session channel-open contract without adding new architecture
policy.

## Findings and disposition

- fixed: authenticated session channel-open now has a bounded local classifier
  that accepts exactly one public session channel bookkeeping handle.
- fixed: missing authentication, disabled policy, duplicate/existing channel,
  redaction-sensitive path, wrong message number, unsupported channel type,
  malformed packet, over-limit channel type/shape, and trailing data fail
  closed with SSH_MSG_CHANNEL_OPEN_FAILURE.
- fixed: focused tests prove the success and representative fail-closed paths,
  including readiness counters and shell/live/ssh-ready false boundaries.
- deferred: retained smoke evidence, closeout reconciliation, shell request
  handling, PTY/process ownership, channel data, EOF/close/window handling,
  live reachability, hardware proof, compatibility, broad expansion, and phase
  transition remain future tasks.

## Evidence

- Source/unit evidence:
  - session_channel_open_accepts_one_modeled_authenticated_session_only
  - session_channel_open_fails_closed_for_prerequisites_and_policy
  - session_channel_open_fails_closed_for_message_type_and_shape
- Validation:
  - cargo fmt --all -- --check: pass
  - cargo -Zjson-target-spec test --quiet: pass
  - git diff --check: pass
  - /home/node/.cargo/bin/mdbook build: pass
  - git diff --cached --check: pass
- Redaction review: pass. Durable evidence retains only task ids, source/doc
  paths, fixed labels, public SSH message names/numbers, public field-count and
  public channel-type length fields, validation commands, test names, readiness
  counters, and classifications. It retains no request payload bytes, channel
  identifiers, window sizes, packet sizes, user/operator identity, key material,
  key-derived identifiers, stable identifiers, session-id bytes, signatures,
  hardware data, or boot artifacts.

## Acceptance

Accepted as bounded local modeled SSH session channel-open source
implementation.

selected_next_task=phase12-shell-ssh-session-channel-open-smoke-20260622.
