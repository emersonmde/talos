# Phase 12.6 SSH publickey authentication success account core

Task id: phase12-ssh-publickey-auth-success-account-core-20260622

Status: accepted.

Classification: phase12-ssh-publickey-auth-success-account-core-accepted.

## Goal

Implement the bounded local modeled SSH publickey USERAUTH_SUCCESS
account-policy core for the single accepted Talos account while keeping
sessions/channels, shell attachment, live reachability, compatibility, broad
expansion, phase transition, and ssh-ready=false out of scope.

## Scope

- Added a local source classifier for the USERAUTH_SUCCESS account-policy
  boundary accepted by
  phase12-ssh-publickey-auth-success-account-policy-contract-20260622.
- Modeled only the single reserved Talos SSH login account policy for signed
  ssh-ed25519 publickey requests after accepted service/userauth, private
  session-id, authorized_keys key-match, and verifier prerequisites.
- Preserved USERAUTH_FAILURE for account mismatch, disabled or missing account
  policy, missing prerequisites, malformed and unsupported inputs, unsigned
  probes after the PK_OK slice, invalid signatures, unauthorized keys, and
  redaction-sensitive cases.
- Exposed fixed labels, public SSH message numbers, public byte-length fields,
  test names, validation commands, and false/zero session/channel/shell/live
  readiness counters only.

## Non-goals preserved

No account database, UID/GID model, home directory, shell path, login session,
per-user authorized_keys lookup, writable account store, operator identity
claim, password or keyboard-interactive authentication, sessions/channels,
PTY/process/shell attachment, live socket reachability, hardware/lab action,
boot publication, OpenSSH/POSIX/Linux compatibility claim, broad expansion,
phase transition, or ssh-ready=true is accepted by this task.

## Implementation

Source changed:

- src/ssh_service_readiness.rs

The new SshPublickeyAuthSuccessAccountInput and
classify_ssh_publickey_auth_success_account_policy path accepts exactly one
modeled success message:

- SSH_MSG_USERAUTH_SUCCESS for a signed-valid ssh-ed25519 publickey request
  whose encrypted userauth request shape, private session-id handle,
  same-request authorized_keys key-match prerequisite, prerequisite-only
  verifier success, enabled account policy, and reserved Talos SSH login
  account name match the accepted policy.

All other paths in this slice return SSH_MSG_USERAUTH_FAILURE with fixed labels
and without partial success. The report permits authentication_success=true
only for the modeled USERAUTH_SUCCESS result and keeps service_success=false,
session_count=0, channel_count=0, shell_attached=false,
reachability_accepted=false, and ssh_ready=false.

Docs changed:

- docs/src/project/phase12-networking-ssh.md
- docs/src/roadmap.md

docs/src/decisions/README.md was not changed because the implementation follows
the already accepted account-policy decision rather than changing architecture
policy.

## Findings and disposition

- fixed: signed-valid ssh-ed25519 publickey requests for the reserved Talos
  account now classify as USERAUTH_SUCCESS and
  publickey-auth-success-prerequisite-only/account-match without accepting any
  session/channel/shell/live readiness.
- fixed: account mismatch, disabled policy, missing account prerequisite, and
  redaction-sensitive cases fail closed with USERAUTH_FAILURE and fixed labels.
- fixed: unsigned probes after the PK_OK response slice, invalid signatures,
  unauthorized keys, missing service/session prerequisites, unsupported
  algorithms, and malformed requests fail closed with USERAUTH_FAILURE.
- fixed: focused tests prove authentication_success=true only for the modeled
  success case and prove session-count=0, channel-count=0,
  shell-attached=false, reachability_accepted=false, and ssh-ready=false.
- deferred: retained smoke evidence, closeout reconciliation,
  sessions/channels, shell attachment, live reachability, compatibility,
  hardware proof, broad expansion, and phase transition remain future tasks.

## Evidence

- Source/unit evidence:
  - publickey_auth_success_account_accepts_reserved_account_only
  - publickey_auth_success_account_fails_closed_for_account_policy_cases
  - publickey_auth_success_account_fails_closed_for_signature_and_prerequisites
- Validation:
  - cargo fmt --all -- --check: pass
  - cargo -Zjson-target-spec test publickey_auth_success_account --quiet: pass
  - cargo -Zjson-target-spec test publickey --quiet: pass
  - cargo -Zjson-target-spec test --quiet: pass
  - git diff --check: pass
  - /home/node/.cargo/bin/mdbook build: pass
  - git diff --cached --check: pass
- Redaction review: pass. Durable evidence retains only task ids, source/doc
  paths, fixed labels, public SSH message numbers, public byte-length/count
  fields, validation commands, test names, and classifications. It retains no
  request user-name strings except the public literal account constant already
  in source/docs, no private user/operator identity, authorized-key bytes,
  request/decoded public-key blobs, signature bytes, signed-data bytes,
  fingerprints, digests, session-id bytes, peer strings, key-derived
  identifiers, stable identifiers, hardware data, or boot artifacts.

## Acceptance

Accepted as bounded publickey USERAUTH_SUCCESS single-account source
implementation.

selected_next_task=phase12-shell-ssh-publickey-auth-success-account-smoke-20260622.
