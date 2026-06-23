# Phase 12.6 SSH session shell-request core

Task id: phase12-ssh-session-shell-request-core-20260622
Status: accepted
Owner: worker
Classification: phase12-ssh-session-shell-request-core-accepted

## Goal

Implement only the accepted local modeled SSH_MSG_CHANNEL_REQUEST shell
classifier after the accepted authentication-success and session channel-open
prerequisites, while preserving failure/no-attachment behavior.

## Scope Completed

- Added a bounded shell-request classifier in src/ssh_service_readiness.rs.
- Recognized only request type shell on an already-open local modeled session
  channel, with authentication-success=true, policy enabled, no previous shell
  request or attachment, non-sensitive redaction state, exact four-field public
  shape, and no trailing request-specific payload.
- Modeled want-reply=true as SSH_MSG_CHANNEL_FAILURE with fixed
  failure/no-attachment labels.
- Modeled want-reply=false as fixed no-reply failure/no-attachment labels.
- Added unit coverage for recognized want-reply true/false behavior,
  readiness counters, fail-closed controls, and channel-open regression
  continuity.
- Updated docs/src/project/phase12-networking-ssh.md and docs/src/roadmap.md
  with the accepted source frontier.

## Reviewed Inputs

- tasks/2026-06-22-phase12-ssh-session-shell-request-contract.md.
- tasks/2026-06-22-phase12-ssh-session-channel-open-closeout.md.
- src/ssh_service_readiness.rs.
- docs/src/project/phase12-networking-ssh.md.
- docs/src/roadmap.md.

## Accepted Behavior

The classifier accepts only the prerequisite shell-request recognition slice:

- prerequisite counters: authentication-success=true, session-count=1, and
  channel-count=1;
- recognized shell request counter: shell-request-count=1;
- want-reply=true: response_message_number=SSH_MSG_CHANNEL_FAILURE;
- want-reply=false: no response message is modeled;
- shell-attached=false, live-reachability=false, and ssh-ready=false remain
  authoritative.

Fail-closed cases cover missing authentication, missing open session channel,
disabled policy, duplicate/existing shell request or attachment,
redaction-sensitive input, wrong message number, unsupported request type,
malformed/over-limit shape, and trailing data.

## Findings

- fixed: implemented the bounded local classifier for SSH_MSG_CHANNEL_REQUEST
  request type shell.
- fixed: preserved failure/no-attachment behavior for both want-reply
  categories without introducing CHANNEL_SUCCESS.
- fixed: added readiness counters for shell-request-count=1 while preserving
  shell-attached=false, live-reachability=false, and ssh-ready=false.
- fixed: added unit coverage for recognized want-reply=true and
  want-reply=false paths.
- fixed: added unit coverage for missing authentication, missing channel,
  disabled policy, duplicate request or attachment, redaction-sensitive input,
  wrong message number, unsupported type, malformed request, over-limit shape,
  and trailing data.
- not-an-issue: the existing session-channel-open regression coverage remains
  in the same source test module and continues to cover the accepted
  prerequisite surface.
- deferred: CHANNEL_SUCCESS, PTY allocation, TTY/session ownership, process
  ownership, shell execution, descriptor handoff, channel data,
  EOF/close/window flow control, exec/subsystem/pty/env/signal behavior, live
  reachability, hardware proof, OpenSSH/POSIX/Linux compatibility, broad
  expansion, phase transition, and ssh-ready=true.

## Validation

- static source/docs review: pass.
- cargo fmt --all -- --check: pass.
- cargo -Zjson-target-spec test --quiet: pass.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.

## Redaction Review

Pass. Durable evidence retains only task ids, source/doc paths, fixed labels,
public SSH message/request names or numbers, public want-reply categories,
public field-count and length categories, validation commands, readiness
counters, and classifications. It retains no private user data, channel
identifiers, request payload bytes, key/session material, hardware data, or
boot artifacts.

## Result

Accepted as the bounded local modeled shell-request source frontier.

selected_next_task=phase12-shell-ssh-session-shell-request-smoke-20260622.
