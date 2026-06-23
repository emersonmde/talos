# Phase 12.6 SSH session shell attachment feature smoke

Task id: phase12-ssh-session-shell-attachment-feature-smoke-20260623

Status: accepted

Classification: phase12-ssh-session-shell-attachment-feature-smoke-accepted

## Goal

Retain bounded feature smoke/regression evidence for the accepted local modeled
SSH shell attachment path from
phase12-ssh-session-shell-attachment-core-20260623.

## Scope

- Reused the accepted local no_std test surface in src/ssh_service_readiness.rs.
- Exercised the accepted shell attachment success path through
  session_shell_attachment_accepts_local_modeled_shell_with_channel_success.
- Retained regression coverage for attachment ownership/lifecycle failures and
  prior shell-request failure/no-attachment controls.
- Added no new source behavior, no live socket path, no hardware/lab action, no
  compatibility claim, no phase transition, and no ssh-ready=true claim.

## Smoke Evidence

The task-owned feature evidence is the existing local no_std unit coverage:

- session_shell_attachment_accepts_local_modeled_shell_with_channel_success:
  covers want-reply=true CHANNEL_SUCCESS plus shell-attached=true and the
  want-reply=false accepted no-reply attachment path.
- session_shell_attachment_fails_closed_for_attachment_ownership_and_lifecycle:
  covers disabled attachment policy, duplicate attachment, missing local
  process/session ownership, missing stdio ownership, and lifecycle violation.
- session_shell_attachment_preserves_shell_request_fail_closed_controls:
  covers missing authentication, missing open channel, duplicate shell request,
  unsupported request type, malformed/trailing request data, and
  redaction-sensitive input.

The local harness command was:

    cargo -Zjson-target-spec test session_shell_attachment --quiet

The repository's custom no_std harness ran the full 795-test corpus for that
command and passed. This is local unit-test evidence only; it is not live SSH
reachability, hardware reachability, OpenSSH/POSIX/Linux compatibility, a phase
transition, or ssh-ready=true.

## Findings

- not-an-issue: the accepted core task already contains the feature and
  regression tests needed for this smoke slice, so no new source behavior was
  required.
- fixed: retained explicit task-owned evidence mapping the local modeled
  CHANNEL_SUCCESS/shell-attached path to regression controls for missing
  prerequisites, duplicate/lifecycle failure, redaction-sensitive input, and
  previous shell-request failure/no-attachment behavior.
- deferred: live encrypted channel data, socket reachability, hardware proof,
  compatibility, broader PTY/window/channel flow behavior, phase transition,
  and ssh-ready=true remain for later explicit tasks.

## Evidence

- Feature smoke/unit tests:
  cargo -Zjson-target-spec test session_shell_attachment --quiet: pass; custom
  no_std harness ran 795 tests.
- Full unit gate:
  cargo -Zjson-target-spec test --quiet: pass with QEMU 9.2.0 on PATH, 795
  no_std tests passed.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check: pass.

## Redaction Review

Pass. Retained evidence is limited to task ids, source/doc paths, public SSH
message names, fixed test/function names, public counters/booleans, validation
commands, and classifications. It retains no private user data, channel
identifiers, request payload bytes, key/session material, hardware data, or boot
artifacts.

## Result

Accepted as bounded local feature smoke/regression evidence for the modeled SSH
shell attachment path.

selected_next_task=phase12-ssh-session-shell-attachment-closeout-20260623.
