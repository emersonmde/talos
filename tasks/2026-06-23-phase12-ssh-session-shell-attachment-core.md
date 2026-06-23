# Phase 12.6 SSH session shell attachment core

Task id: phase12-ssh-session-shell-attachment-core-20260623

Status: accepted

Classification: phase12-ssh-session-shell-attachment-core-accepted

## Goal

Implement the bounded local modeled SSH shell attachment contract selected by
phase12-ssh-session-shell-attachment-contract-20260623.

## Scope

- Added a local modeled classify_ssh_session_shell_attachment surface in
  src/ssh_service_readiness.rs.
- Preserved the prior shell-request classifier and its failure/no-attachment
  behavior for request recognition only.
- Added fixed labels and counters for one authenticated, open session channel
  with one recognized shell request owning accepted local process/session and
  fd0/fd1/fd2 stdio surfaces.
- Kept live encrypted channel data, socket reachability, channel window
  management, hardware proof, compatibility, broad expansion, phase transition,
  and ssh-ready=true deferred.

## Accepted Behavior

The new attachment classifier returns SSH_MSG_CHANNEL_SUCCESS only when all of
these public model facts are true:

- authentication success is present;
- one local modeled session channel is open;
- the decrypted request has the accepted SSH_MSG_CHANNEL_REQUEST shell shape;
- shell-request and shell-attachment policies are enabled;
- no prior shell request or shell attachment exists;
- the channel lifecycle is still open;
- local process/session ownership and local stdio descriptor ownership are
  present.

For want-reply=false, the same local modeled attachment is accepted without
emitting a response packet. In both success cases,
authentication-success=true, session-count=1, channel-count=1,
shell-request-count=1, and shell-attached=true; live-reachability=false and
ssh-ready=false remain authoritative.

## Failure Behavior

The classifier fails closed without attachment for missing authentication,
missing open channel, disabled shell-request policy, duplicate shell request,
unsupported request type, malformed/trailing input, redaction-sensitive input,
disabled attachment policy, duplicate attachment, missing local execution or
stdio ownership, and lifecycle violations. Failure responses remain
SSH_MSG_CHANNEL_FAILURE only when a recognized request had want-reply=true; no
failure path reports shell-attached=true.

## Findings

- fixed: implemented the bounded local modeled CHANNEL_SUCCESS path with
  truthful shell attachment, request, session, channel, reachability, and
  readiness counters.
- fixed: retained the prior shell-request failure/no-attachment classifier as a
  regression/control surface instead of rewriting it into attachment behavior.
- fixed: added explicit failure labels for disabled attachment policy,
  duplicate attachment, missing local execution/stdio ownership, and lifecycle
  violation.
- fixed: added unit coverage for success with want-reply=true, accepted no-reply
  attachment, missing ownership, duplicate attachment, lifecycle violation, and
  shell-request fail-closed controls.
- not-an-issue: live encrypted packet I/O and socket reachability remain outside
  this local modeled attachment slice.

## Evidence

- Source/unit tests: src/ssh_service_readiness.rs adds
  session_shell_attachment_accepts_local_modeled_shell_with_channel_success,
  session_shell_attachment_fails_closed_for_attachment_ownership_and_lifecycle,
  and session_shell_attachment_preserves_shell_request_fail_closed_controls.
- cargo fmt --all -- --check: pass.
- cargo -Zjson-target-spec test --quiet: pass with QEMU 9.2.0 on PATH, 795
  no_std tests passed.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check: pass.

## Redaction Review

Pass. Retained evidence is limited to task ids, source/doc paths, fixed labels,
public SSH message names and numbers, public request/want-reply categories,
count/boolean counters, validation commands, and classifications. It retains no
private user data, channel identifiers, request payload bytes, key/session
material, hardware data, or boot artifacts.

## Result

Accepted as the bounded local modeled SSH shell attachment core.

selected_next_task=phase12-ssh-session-shell-attachment-feature-smoke-20260623.
