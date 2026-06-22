# Phase 12.6 SSH publickey authentication response-policy contract

Task id: phase12-ssh-publickey-auth-response-policy-contract-20260622

Status: accepted.

Classification: phase12-ssh-publickey-auth-response-policy-contract-accepted.

## Goal

Define the bounded publickey USERAUTH_PK_OK/USERAUTH_FAILURE response policy
after the accepted prerequisite-only publickey verifier closeout, while keeping
authentication success, account binding, sessions, shell attachment, live
reachability, compatibility, broad expansion, phase transition, and
ssh-ready=true unaccepted.

## Scope

- Reviewed the accepted pre-authentication service/userauth parser, private
  userauth session-id prerequisite, authorized_keys parser/key-match frontier,
  prerequisite-only publickey verifier, source readiness boundaries, project
  docs, roadmap, and prior decision records.
- Defined the first response-policy boundary for modeled decrypted
  SSH_MSG_USERAUTH_REQUEST/publickey payloads.
- Limited accepted outcomes to USERAUTH_PK_OK and SSH_MSG_USERAUTH_FAILURE
  classifications only.
- Preserved authentication-success=false, service-success=false,
  session-count=0, channel-count=0, shell-attached=false,
  live-reachability=false, and ssh-ready=false as authoritative.
- Selected one bounded implementation follow-up.

## Non-goals

No Rust source implementation, response serialization, packet encryption,
authentication success, SSH_MSG_USERAUTH_SUCCESS, partial-success behavior,
account database, account/user authorization, sessions/channels,
PTY/process/shell attachment, live socket reachability, hardware/lab action,
boot publication, OpenSSH/POSIX/Linux compatibility claim, broad expansion,
phase transition, or ssh-ready=true is accepted.

Durable evidence must not retain session-id bytes, authorized-key bytes,
request/decoded public-key blobs, signature bytes, signed-data bytes,
fingerprints, digests, user names, comments, user/operator identity, peer
strings, key-derived identifiers, stable identifiers, exchange hashes,
hardware data, or boot artifacts.

## Reviewed Inputs

- tasks/2026-06-22-phase12-ssh-preauth-service-userauth-closeout.md
- tasks/2026-06-22-phase12-ssh-userauth-session-id-closeout.md
- tasks/2026-06-22-phase12-ssh-authorized-keys-parser-closeout.md
- tasks/2026-06-22-phase12-ssh-publickey-verification-contract.md
- tasks/2026-06-22-phase12-ssh-publickey-verification-core.md
- tasks/2026-06-22-phase12-ssh-publickey-verification-smoke.md
- tasks/2026-06-22-phase12-ssh-publickey-verification-closeout.md
- src/ssh_service_readiness.rs
- src/ssh_key_readiness.rs
- src/ssh_runtime_crypto.rs
- docs/src/project/phase12-networking-ssh.md
- docs/src/roadmap.md
- docs/src/decisions/README.md

## Contract

The next implementation may classify only these publickey authentication
response outcomes for modeled decrypted SSH_MSG_USERAUTH_REQUEST/publickey
payloads:

- USERAUTH_PK_OK, for an unsigned publickey probe only when all accepted
  prerequisites are present: encrypted userauth dispatch, modeled ssh-userauth
  service prerequisite, service string ssh-connection, method publickey,
  signature-present=false, algorithm ssh-ed25519, a parsed ssh-ed25519 request
  public-key blob, an authorized-keys-match-prerequisite-only result for the
  same request public-key blob, and response policy enabled.
- USERAUTH_FAILURE, for every signed or fail-closed publickey path in this
  slice, including signature-present=true with a verifier prerequisite-only
  success, signature-present=true with rejected or malformed signature,
  unauthorized key, malformed request, unsupported algorithm, missing
  service/userauth/session/key/verifier prerequisite, disabled policy, and
  redaction-sensitive paths.

USERAUTH_PK_OK is only a key-probe acknowledgement. It is not authentication
success, account authorization, session allocation, shell attachment, live
reachability, compatibility, or readiness. USERAUTH_FAILURE in this slice may
represent success-deferred even when the prerequisite-only verifier has
accepted a valid signature, because account binding and USERAUTH_SUCCESS are
separate unaccepted work.

The next implementation may choose exact enum/source names, but retained
diagnostics must stay in these fixed-label families:

- publickey-auth-response-pk-ok-prerequisite-only;
- publickey-auth-response-failure-signature-valid-success-deferred;
- publickey-auth-response-failure-signature-rejected;
- publickey-auth-response-failure-signature-malformed;
- publickey-auth-response-failure-authorized-key-missing;
- publickey-auth-response-failure-authorized-key-no-match;
- publickey-auth-response-failure-request-malformed;
- publickey-auth-response-failure-algorithm-unsupported;
- publickey-auth-response-failure-prerequisite-missing;
- publickey-auth-response-failure-policy-disabled;
- publickey-auth-response-failure-redaction-sensitive;
- authentication-success-unaccepted;
- not-ready.

The implementation must retain only fixed labels, public byte lengths, bounded
field counts, validation command names, task ids, source/doc paths, and
classifications. It must not retain user strings, peer strings, key material,
session-id material, signatures, signed-data material, fingerprints, digests,
or stable identifiers in durable evidence.

## Findings and Disposition

- fixed: defined the smallest response-policy boundary after the verifier
  closeout instead of jumping directly to authentication success.
- fixed: limited signature-absent behavior to USERAUTH_PK_OK only when the
  accepted publickey request shape and authorized_keys key-match prerequisites
  are present.
- fixed: signature-present-valid remains success-deferred and maps only to
  USERAUTH_FAILURE until account binding and USERAUTH_SUCCESS are accepted.
- fixed: malformed, unsupported, unauthorized, disabled, prerequisite-missing,
  and redaction-sensitive cases fail closed with fixed labels.
- fixed: service-success=false, authentication-success=false, session-count=0,
  channel-count=0, shell-attached=false, live-reachability=false, and
  ssh-ready=false remain authoritative.
- deferred: response serialization/packet emission, source implementation,
  retained smoke evidence, account binding, authentication success,
  SSH_MSG_USERAUTH_SUCCESS, sessions/channels, shell attachment, live
  reachability, hardware proof, compatibility, broad expansion, and phase
  transition.
- not-an-issue: no Rust source change is required for this contract because it
  defines the next implementation slice.

## Validation

- static task/docs/source review: pass.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass with existing large search-index
  warning.
- git diff --cached --check: pass.

Conditional gates not run: cargo fmt --all -- --check and
cargo -Zjson-target-spec test --quiet were not required because this contract
touched no Rust source or Cargo metadata.

No Pi 5 hardware run, lab-controller API action, hardwareTestLock acquisition,
boot publication, response implementation, response serialization,
authentication success, service success, session/channel or shell work, live
reachability claim, compatibility claim, broad expansion, or phase transition
was performed.

## Redaction Review

Pass. Retained evidence contains only task ids, source/doc paths, public SSH
message names, fixed label families, public byte-length and count categories,
false/zero readiness counters, validation commands, and classifications. It
retains no session-id bytes, authorized-key bytes, request/decoded public-key
blobs, signature bytes, signed-data bytes, fingerprints, digests, user names,
comments, user/operator identity, peer strings, key-derived identifiers,
stable identifiers, exchange hashes, hardware data, or boot artifacts.

## Accepted Frontier

Talos now has a bounded publickey authentication response-policy contract. The
only accepted response classifications for the next source slice are
USERAUTH_PK_OK for accepted unsigned ssh-ed25519 publickey key probes and
USERAUTH_FAILURE for signed, invalid, unauthorized, malformed, unsupported,
disabled, prerequisite-missing, and redaction-sensitive paths.

This accepts only the response-policy boundary. No response implementation,
authentication success, SSH_MSG_USERAUTH_SUCCESS, account binding,
sessions/channels, shell attachment, live reachability, hardware proof,
OpenSSH/POSIX/Linux compatibility claim, broad expansion, phase transition, or
ssh-ready=true is accepted.

## Selected Next Task

selected_next_task=phase12-ssh-publickey-auth-response-policy-core-20260622.

The selected implementation task is objective because the accepted contract
defines PK_OK versus FAILURE cases, failure-label families, redaction
boundaries, validation expectations, and unchanged false/zero readiness
frontier.

## Acceptance

Accepted as bounded publickey USERAUTH_PK_OK/FAILURE response-policy contract.
selected_next_task=phase12-ssh-publickey-auth-response-policy-core-20260622.
