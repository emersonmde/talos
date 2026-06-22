# Phase 12.6 SSH publickey authentication response-policy core

Task id: phase12-ssh-publickey-auth-response-policy-core-20260622

Status: accepted.

Classification: phase12-ssh-publickey-auth-response-policy-core-accepted.

## Goal

Implement the accepted publickey USERAUTH_PK_OK/USERAUTH_FAILURE response-policy
boundary in the local modeled SSH service while keeping authentication success,
sessions/channels, shell attachment, live reachability, compatibility, broad
expansion, phase transition, and ssh-ready=false out of scope.

## Scope

- Added a local source classifier for the response-policy boundary accepted by
  phase12-ssh-publickey-auth-response-policy-contract-20260622.
- Modeled only response selection after accepted service/userauth, private
  session-id, authorized_keys key-match, and publickey verifier prerequisites.
- Exposed fixed labels, public message numbers, public byte-length fields, and
  false/zero readiness counters only.
- Added focused source/unit coverage for PK_OK, signed-valid success-deferred
  failure, invalid and malformed signatures, unauthorized keys, malformed
  requests, unsupported algorithms, disabled policy, prerequisite-missing, and
  redaction-sensitive cases.

## Non-goals preserved

No SSH_MSG_USERAUTH_SUCCESS, authentication success, partial-success behavior,
account database, durable user/account authorization, sessions/channels,
PTY/process/shell attachment, live socket reachability, hardware/lab action,
OpenSSH/POSIX/Linux compatibility claim, broad expansion, phase transition, or
ssh-ready=true is accepted by this task.

## Implementation

Source changed:

- src/ssh_service_readiness.rs

The new SshPublickeyAuthResponsePolicyInput and
classify_ssh_publickey_auth_response_policy path accepts only two modeled
response message numbers:

- SSH_MSG_USERAUTH_PK_OK for an unsigned ssh-ed25519 publickey key probe when
  accepted prerequisites are present: encrypted userauth dispatch has selected
  ssh-userauth, the request shape is ssh-connection/publickey, the request key
  parses as ssh-ed25519, the authorized_keys match is prerequisite-only for the
  same request public-key blob length, and a private userauth session-id handle
  is available.
- SSH_MSG_USERAUTH_FAILURE for every signed or fail-closed path in this slice,
  including valid signatures that remain success-deferred until account binding
  and USERAUTH_SUCCESS are explicitly accepted.

The report keeps service_success=false, authentication_success=false,
session_count=0, channel_count=0, shell_attached=false,
reachability_accepted=false, and ssh_ready=false.

## Findings and disposition

- fixed: unsigned authorized ssh-ed25519 probes now classify as
  publickey-auth-response-pk-ok-prerequisite-only and message 60 without
  accepting authentication success.
- fixed: valid signed publickey requests now classify as
  publickey-auth-response-failure-signature-valid-success-deferred and message
  51 until account binding and USERAUTH_SUCCESS are explicitly accepted.
- fixed: invalid signatures, malformed signatures, unauthorized keys,
  malformed requests, unsupported algorithms, disabled policy, missing
  prerequisites, and redaction-sensitive cases fail closed as USERAUTH_FAILURE.
- fixed: focused tests prove response message selection and false/zero readiness
  boundaries.
- deferred: retained smoke evidence, closeout reconciliation, account binding,
  authentication success, SSH_MSG_USERAUTH_SUCCESS, sessions/channels, shell
  attachment, live reachability, compatibility, and hardware proof remain future
  tasks.

## Evidence

- Source/unit evidence:
  - publickey_auth_response_pk_ok_for_unsigned_authorized_probe_only
  - publickey_auth_response_defers_valid_signature_to_failure_until_success_policy
  - publickey_auth_response_fails_closed_for_invalid_and_malformed_signatures
  - publickey_auth_response_fails_closed_for_policy_and_prerequisite_cases
- Validation:
  - cargo fmt --all -- --check: pass
  - cargo -Zjson-target-spec test publickey --quiet: pass
  - cargo -Zjson-target-spec test --quiet: pass
  - git diff --check: pass
  - /home/node/.cargo/bin/mdbook build: pass
  - git diff --cached --check: pass
- Redaction review: pass. Durable evidence retains only task ids, source/doc
  paths, fixed labels, public SSH message numbers, public byte-length/count
  fields, false/zero readiness counters, validation commands, test names, and
  classifications. It retains no session-id bytes, authorized-key bytes,
  request/decoded public-key blobs, signature bytes, signed-data bytes,
  fingerprints, digests, user names, comments, peer strings, user/operator
  identity, key-derived identifiers, stable identifiers, hardware data, or boot
  artifacts.

## Acceptance

Accepted as bounded publickey USERAUTH_PK_OK/USERAUTH_FAILURE response-policy
source implementation.

selected_next_task=phase12-shell-ssh-publickey-auth-response-policy-smoke-20260622.
