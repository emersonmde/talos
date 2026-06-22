# Phase 12.6 SSH publickey authentication response-policy closeout

Task id: phase12-ssh-publickey-auth-response-policy-closeout-20260622

Status: accepted.

Classification: phase12-ssh-publickey-auth-response-policy-closeout-accepted.

## Goal

Reconcile the accepted publickey USERAUTH_PK_OK/USERAUTH_FAILURE response-policy
contract, implementation, retained smoke evidence, docs, validation, redaction,
deferred work, and residual risks before moving to account-binding and
authentication-success policy.

## Scope

- Reviewed the accepted response-policy contract, core implementation record,
  retained host/QEMU-substitute smoke record, project architecture notes, and
  roadmap frontier.
- Confirmed the accepted frontier is limited to prerequisite publickey
  USERAUTH_PK_OK for unsigned authorized ssh-ed25519 key probes and
  USERAUTH_FAILURE for signed, invalid, unauthorized, malformed, unsupported,
  disabled, redaction-sensitive, and prerequisite-missing paths.
- Confirmed the retained evidence keeps authentication-success=false,
  session-count=0, channel-count=0, shell-attached=false,
  live-reachability=false, and ssh-ready=false.
- Updated the Phase 12 project doc and roadmap with the reconciled closeout
  boundary and selected next task.

## Non-goals

No source behavior change, account binding, SSH_MSG_USERAUTH_SUCCESS,
authentication success, partial-success behavior, account database,
sessions/channels, PTY/process/shell attachment, live socket reachability,
hardware/lab action, boot publication, OpenSSH/POSIX/Linux compatibility claim,
broad expansion, phase transition, or ssh-ready=true is accepted.

Durable evidence must not retain session-id bytes, authorized-key bytes,
request or decoded public-key blobs, signature bytes, signed-data bytes,
fingerprints, digests, user/operator identity, peer strings, key-derived
identifiers, stable identifiers, hardware data, or boot artifacts.

## Reviewed Inputs

- tasks/2026-06-22-phase12-ssh-publickey-auth-response-policy-contract.md
- tasks/2026-06-22-phase12-ssh-publickey-auth-response-policy-core.md
- tasks/2026-06-22-phase12-ssh-publickey-auth-response-policy-smoke.md
- src/ssh_service_readiness.rs
- docs/src/project/phase12-networking-ssh.md
- docs/src/roadmap.md
- tasks/evidence/2026-06-22-ssh-publickey-auth-response-policy-smoke/qemu-shell-ssh-publickey-auth-response-policy-smoke.log

## Findings and Disposition

- fixed: reconciled contract, implementation, and retained smoke evidence into
  one accepted response-policy frontier.
- fixed: docs now state that Talos has only prerequisite publickey
  PK_OK/FAILURE response policy; account binding, USERAUTH_SUCCESS,
  authentication success, sessions/channels, shell attachment, live reachability,
  compatibility, broad expansion, phase transition, and ssh-ready=true remain
  unaccepted.
- fixed: retained smoke evidence proves authentication-success=false,
  session-count=0, channel-count=0, shell-attached=false,
  live-reachability=false, and ssh-ready=false.
- not-an-issue: no Rust source change is required for this closeout because the
  accepted response-policy behavior and retained smoke evidence already exist.
- deferred: account binding, SSH_MSG_USERAUTH_SUCCESS, authentication success,
  sessions/channels, shell attachment, live reachability, hardware proof,
  compatibility, broad expansion, phase transition, and ssh-ready=true.

## Reconciled Evidence

- Contract evidence:
  tasks/2026-06-22-phase12-ssh-publickey-auth-response-policy-contract.md.
- Source/unit evidence:
  tasks/2026-06-22-phase12-ssh-publickey-auth-response-policy-core.md and the
  publickey_auth_response focused tests.
- Retained host/QEMU-substitute smoke evidence:
  tasks/2026-06-22-phase12-ssh-publickey-auth-response-policy-smoke.md and
  tasks/evidence/2026-06-22-ssh-publickey-auth-response-policy-smoke/qemu-shell-ssh-publickey-auth-response-policy-smoke.log.
- Architecture and roadmap evidence:
  docs/src/project/phase12-networking-ssh.md and docs/src/roadmap.md.

## Validation

- static task/docs/source review: pass.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass with existing large search-index
  warning.
- git diff --cached --check: pass.

Conditional gates not run: cargo fmt --all -- --check and
cargo -Zjson-target-spec test --quiet were not required because this closeout
touched no Rust source or Cargo metadata.

No Pi 5 hardware run, lab-controller API action, hardwareTestLock acquisition,
boot publication, source behavior change, USERAUTH_SUCCESS, authentication
success, service success, session/channel or shell work, live reachability
claim, compatibility claim, broad expansion, or phase transition was performed.

## Redaction Review

Pass. Retained closeout evidence contains only task ids, source/doc/evidence
paths, fixed labels, public SSH message numbers, public byte-length/count field
names, false/zero readiness counters, validation commands, test names, and
classifications. It retains no session-id bytes, authorized-key bytes, request
or decoded public-key blobs, signature bytes, signed-data bytes, fingerprints,
digests, user/operator identity, peer strings, key-derived identifiers, stable
identifiers, hardware data, or boot artifacts.

## Accepted Frontier

Talos now has a closed publickey USERAUTH_PK_OK/USERAUTH_FAILURE response-policy
frontier. USERAUTH_PK_OK is accepted only for unsigned authorized ssh-ed25519
key probes after the accepted service/userauth, private session-id,
authorized_keys key-match, and verifier prerequisites. USERAUTH_FAILURE remains
the only accepted response for signed-valid success-deferred, invalid signature,
malformed signature, unauthorized key, malformed request, unsupported
algorithm, disabled policy, redaction-sensitive, and prerequisite-missing paths.

This accepts only prerequisite response policy. No account binding,
SSH_MSG_USERAUTH_SUCCESS, authentication success, sessions/channels, shell
attachment, live reachability, hardware proof, OpenSSH/POSIX/Linux
compatibility, broad expansion, phase transition, or ssh-ready=true is accepted.

## Selected Next Task

selected_next_task=phase12-ssh-publickey-auth-success-account-policy-contract-20260622.

The selected next task is objective because the response-policy closeout leaves
exactly one unaccepted next SSH userauth boundary: account binding and the
policy conditions for authentication success, while still excluding sessions,
channels, shell attachment, live reachability, hardware, compatibility, broad
expansion, phase transition, and ssh-ready=true.

## Acceptance

Accepted as bounded publickey USERAUTH_PK_OK/FAILURE response-policy closeout.
selected_next_task=phase12-ssh-publickey-auth-success-account-policy-contract-20260622.
