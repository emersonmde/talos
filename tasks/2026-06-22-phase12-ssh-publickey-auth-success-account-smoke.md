# Phase 12.6 SSH publickey authentication success account smoke

Task id: phase12-shell-ssh-publickey-auth-success-account-smoke-20260622

Status: accepted.

Classification: phase12-shell-ssh-publickey-auth-success-account-smoke-accepted.

## Goal

Retain host/QEMU-substitute smoke evidence for the accepted publickey
USERAUTH_SUCCESS single-account policy boundary while keeping sessions,
channels, shell attachment, live reachability, compatibility, broad expansion,
phase transition, and ssh-ready=true unaccepted.

## Scope

- Added scripts/qemu-shell-ssh-publickey-auth-success-account-smoke.sh as the
  retained fixed-label smoke command for this slice.
- Exercise the accepted source/unit account-success cases through the
  configured target cargo test runner.
- Cover the modeled signed-valid accepted-account USERAUTH_SUCCESS case,
  account mismatch, disabled account policy, missing account/response
  prerequisites, invalid signature, unauthorized key, malformed request,
  unsupported algorithm, redaction-sensitive failure, and unsigned probe PK_OK
  behavior retained from the prior response-policy slice.
- Retain only fixed labels, public SSH message numbers, public byte-length/count
  field names, false/zero readiness counters, validation commands, paths, task
  ids, and classifications.

## Non-goals

No source feature expansion, account database, UID/GID model, home directory,
shell path, login session, per-user authorized_keys lookup, sessions/channels,
PTY/process/shell attachment, live sockets, hardware/lab action, boot
publication, OpenSSH/POSIX/Linux compatibility claim, broad expansion, phase
transition, or ssh-ready=true is accepted.

Durable evidence must not retain session-id bytes, authorized-key bytes,
request or decoded public-key blobs, signature bytes, signed-data bytes,
fingerprints, digests, private user/operator identity, peer strings,
key-derived identifiers, stable identifiers, hardware data, or boot artifacts.

## Findings and Disposition

- fixed: retained a dedicated smoke script and transcript path for the
  USERAUTH_SUCCESS account-policy frontier instead of relying only on
  implementation-task evidence.
- fixed: retained smoke evidence covers the modeled signed-valid accepted
  account USERAUTH_SUCCESS result and records authentication-success=true only
  for that success state.
- fixed: retained smoke evidence covers USERAUTH_FAILURE for account mismatch,
  disabled account policy, missing account/response prerequisites, invalid
  signature, unauthorized key, malformed request, unsupported algorithm, and
  redaction-sensitive paths.
- fixed: retained smoke evidence records session-count=0, channel-count=0,
  shell-attached=false, live-reachability=false, and ssh-ready=false for both
  success and failure paths.
- not-an-issue: the smoke task adds no new protocol behavior; it reruns the
  accepted publickey_auth_success_account source/unit coverage through the
  retained host/QEMU-substitute command.
- deferred: closeout reconciliation, sessions/channels, shell attachment, live
  reachability, hardware proof, compatibility, broad expansion, phase
  transition, and ssh-ready=true.

## Smoke Evidence

Retained transcript:

- tasks/evidence/2026-06-22-ssh-publickey-auth-success-account-smoke/qemu-shell-ssh-publickey-auth-success-account-smoke.log

The transcript records:

- Success state:
  sshservicediag-publickey-auth-success-prerequisite-only,
  sshservicediag-publickey-auth-success-account-match,
  sshservicediag-authentication-success-local-only,
  SSH_MSG_USERAUTH_SUCCESS=52, authentication-success=true, and false/zero
  session/channel/shell/live/ssh-ready counters.
- Unsigned probe state:
  SSH_MSG_USERAUTH_PK_OK=60, authentication-success=false, and false/zero
  readiness counters.
- Failure message:
  SSH_MSG_USERAUTH_FAILURE=51, authentication-success=false, and false/zero
  readiness counters.
- Fail-closed states for account-mismatch, account-policy-disabled,
  account-prerequisite-missing, response-prerequisite-missing,
  signature-invalid, authorized-key-no-match, request-malformed,
  algorithm-unsupported, and redaction-sensitive paths.

The retained source/unit test filter is publickey_auth_success_account,
covering:

- publickey_auth_success_account_accepts_reserved_account_only.
- publickey_auth_success_account_fails_closed_for_account_policy_cases.
- publickey_auth_success_account_fails_closed_for_signature_and_prerequisites.

## Validation

- scripts/qemu-shell-ssh-publickey-auth-success-account-smoke.sh: pass under
  the configured host/QEMU-substitute cargo test runner; retained transcript
  ends with
  classification=host-qemu-substitute-shell-ssh-publickey-auth-success-account-smoke-complete.
- cargo fmt --all -- --check: pass.
- cargo -Zjson-target-spec test publickey_auth_success_account --quiet: pass.
- cargo -Zjson-target-spec test publickey --quiet: pass.
- cargo -Zjson-target-spec test --quiet: pass.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check: pass.

No Pi 5 hardware run, lab-controller API action, hardwareTestLock acquisition,
boot publication, service/session/channel/shell work, live reachability claim,
compatibility claim, broad expansion, or phase transition was performed.

## Redaction Review

Pass. Retained smoke evidence contains only task ids, paths, fixed labels,
public SSH message numbers, public byte-length/count field names, false/zero
readiness counters, validation commands, test names, and classifications. It
retains no session-id bytes, authorized-key bytes, request or decoded
public-key blobs, signature bytes, signed-data bytes, fingerprints, digests,
private user/operator identity, peer strings, key-derived identifiers, stable
identifiers, hardware data, or boot artifacts.

## Acceptance

Accepted as bounded retained host/QEMU-substitute smoke evidence for the
publickey USERAUTH_SUCCESS single-account policy frontier.

selected_next_task=phase12-ssh-publickey-auth-success-account-closeout-20260622.
