# Phase 12.6 SSH publickey authentication response-policy smoke

Task id: phase12-shell-ssh-publickey-auth-response-policy-smoke-20260622

Status: accepted.

Classification: phase12-shell-ssh-publickey-auth-response-policy-smoke-accepted.

## Goal

Retain host/QEMU-substitute smoke evidence for the accepted publickey
USERAUTH_PK_OK/USERAUTH_FAILURE response-policy boundary without claiming live
reachability, authentication success, compatibility, broad expansion, phase
transition, or ssh-ready=true.

## Scope

- Added scripts/qemu-shell-ssh-publickey-auth-response-policy-smoke.sh as the
  retained fixed-label smoke command for this slice.
- Exercised the accepted source/unit response-policy cases through the
  configured target cargo test runner.
- Covered acceptable unsigned PK_OK, signed-valid success-deferred FAILURE,
  invalid and malformed signatures, unauthorized key failures, malformed
  request failure, unsupported algorithm failure, disabled policy,
  redaction-sensitive, and prerequisite-missing cases.
- Retained only fixed labels, public message numbers, public byte-length/count
  field names, false/zero readiness counters, validation commands, paths, task
  ids, and classifications.

## Non-goals

No source feature expansion, SSH_MSG_USERAUTH_SUCCESS, authentication success,
account database, account/user authorization, sessions/channels,
PTY/process/shell attachment, live sockets, hardware/lab action, boot
publication, OpenSSH/POSIX/Linux compatibility claim, broad expansion, phase
transition, or ssh-ready=true is accepted.

Durable evidence must not retain session-id bytes, authorized-key bytes,
request or decoded public-key blobs, signature bytes, signed-data bytes,
fingerprints, digests, user/operator identity, peer strings, key-derived
identifiers, stable identifiers, hardware data, or boot artifacts.

## Findings and Disposition

- fixed: retained a dedicated smoke script and transcript path for the
  response-policy frontier instead of relying only on implementation-task
  evidence.
- fixed: retained smoke evidence covers PK_OK for unsigned authorized
  ssh-ed25519 probes and USERAUTH_FAILURE for signed-valid success-deferred,
  invalid signature, malformed signature, unauthorized key, malformed request,
  unsupported algorithm, disabled, redaction-sensitive, and prerequisite-missing
  paths.
- fixed: smoke evidence records authentication-success=false,
  session-count=0, channel-count=0, shell-attached=false,
  live-reachability=false, and ssh-ready=false.
- not-an-issue: the smoke task adds no new protocol behavior; it reruns the
  accepted publickey_auth_response source/unit coverage through the retained
  host/QEMU-substitute command.
- deferred: account binding, SSH_MSG_USERAUTH_SUCCESS, authentication success,
  sessions/channels, shell attachment, live reachability, hardware proof,
  compatibility, broad expansion, phase transition, and ssh-ready=true.

## Smoke Evidence

Retained transcript:

- tasks/evidence/2026-06-22-ssh-publickey-auth-response-policy-smoke/qemu-shell-ssh-publickey-auth-response-policy-smoke.log

The transcript records:

- PK_OK state:
  sshservicediag-publickey-auth-response-pk-ok-prerequisite-only,
  SSH_MSG_USERAUTH_PK_OK=60, and false/zero readiness counters.
- Signed-valid deferred state:
  sshservicediag-publickey-auth-response-failure-signature-valid-success-deferred,
  SSH_MSG_USERAUTH_FAILURE=51, authentication-success=false, and ssh-ready=false.
- Fail-closed states for signature-rejected, signature-malformed,
  authorized-key-missing, authorized-key-no-match, request-malformed,
  algorithm-unsupported, prerequisite-missing, policy-disabled, and
  redaction-sensitive paths.

The retained source/unit test filter is publickey_auth_response, covering:

- publickey_auth_response_pk_ok_for_unsigned_authorized_probe_only.
- publickey_auth_response_defers_valid_signature_to_failure_until_success_policy.
- publickey_auth_response_fails_closed_for_invalid_and_malformed_signatures.
- publickey_auth_response_fails_closed_for_policy_and_prerequisite_cases.

## Validation

- scripts/qemu-shell-ssh-publickey-auth-response-policy-smoke.sh: pass under the
  configured host/QEMU-substitute cargo test runner; retained transcript ends
  with classification=host-qemu-substitute-shell-ssh-publickey-auth-response-policy-smoke-complete.
- cargo fmt --all -- --check: pass.
- cargo -Zjson-target-spec test publickey --quiet: pass.
- cargo -Zjson-target-spec test --quiet: pass.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass with existing large search-index
  warning.
- git diff --cached --check: pass.

No Pi 5 hardware run, lab-controller API action, hardwareTestLock acquisition,
boot publication, authentication success, service success, session/channel or
shell work, live reachability claim, compatibility claim, broad expansion, or
phase transition was performed.

## Redaction Review

Pass. Retained smoke evidence contains only task ids, paths, fixed labels,
public SSH message numbers, public byte-length/count field names, false/zero
readiness counters, validation commands, test names, and classifications. It
retains no session-id bytes, authorized-key bytes, request or decoded
public-key blobs, signature bytes, signed-data bytes, fingerprints, digests,
user/operator identity, peer strings, key-derived identifiers, stable
identifiers, hardware data, or boot artifacts.

## Acceptance

Accepted as bounded retained host/QEMU-substitute smoke evidence for the
publickey USERAUTH_PK_OK/USERAUTH_FAILURE response-policy frontier.

selected_next_task=phase12-ssh-publickey-auth-response-policy-closeout-20260622.
