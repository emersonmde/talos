# Phase 12.6 SSH publickey signature-verification core

Task id: phase12-ssh-publickey-verification-core-20260622

Status: accepted.

Classification: phase12-ssh-publickey-verification-core-accepted.

## Goal

Implement the bounded prerequisite-only publickey signature verifier from the
accepted contract, using the accepted private session-id handle and
authorized_keys key-match prerequisite while keeping authentication responses,
authentication success, sessions, shell attachment, live reachability,
compatibility, broad expansion, and phase transition unaccepted.

## Scope

- Reviewed the accepted verifier contract, userauth session-id handle,
  authorized_keys key-match report, preauth userauth parser, runtime KEX
  session-id surface, ssh-key verifier surface, and existing redaction rules.
- Added a prerequisite-only verifier classifier for the decrypted
  SSH_MSG_USERAUTH_REQUEST/publickey payload.
- Constructed the RFC 4252 signed-data buffer in memory from the private
  session-id handle and original request fields, then zeroized the temporary
  buffer before returning.
- Verified only ssh-ed25519 signatures when signature-present=true, the request
  key parses as ssh-ed25519, the authorized_keys report matches the current
  request length, and the session-id handle is available.
- Added focused unit coverage for verifier success as prerequisite-only and
  distinct fail-closed label families.

## Non-goals

No USERAUTH_PK_OK, SSH_MSG_USERAUTH_FAILURE, SSH_MSG_USERAUTH_SUCCESS,
partial-success behavior, authentication success, account/user authorization,
account database, sessions/channels, PTY/process/shell attachment, live socket
reachability, hardware/lab action, boot publication, OpenSSH/POSIX/Linux
compatibility claim, broad expansion, phase transition, or ssh-ready=true is
accepted.

Durable evidence must not retain session-id bytes, authorized-key bytes,
request/decoded public-key blobs, signature bytes, signed-data bytes,
fingerprints, digests, user names, comments, user/operator identity,
key-derived identifiers, stable identifiers, hardware data, or boot artifacts.

## Reviewed Inputs

- tasks/2026-06-22-phase12-ssh-publickey-verification-contract.md
- tasks/2026-06-22-phase12-ssh-userauth-session-id-core.md
- tasks/2026-06-22-phase12-ssh-authorized-keys-parser-core.md
- tasks/2026-06-22-phase12-ssh-preauth-service-userauth-core.md
- tasks/2026-06-22-phase12-ssh-runtime-kex-closeout.md
- src/ssh_service_readiness.rs
- src/ssh_key_readiness.rs
- src/ssh_runtime_crypto.rs
- docs/src/project/phase12-networking-ssh.md
- docs/src/roadmap.md

## Findings and Disposition

- fixed: implemented prerequisite-only publickey verification as a separate
  classifier instead of folding it into authentication success.
- fixed: the verifier builds the RFC 4252 signed-data buffer from the accepted
  session-id handle and original request fields, and zeroizes that temporary
  buffer before return.
- fixed: verification is limited to ssh-ed25519 request keys and ssh-ed25519
  signatures.
- fixed: signature-present=false fails closed and does not emit PK_OK.
- fixed: authorized_keys key-match must be prerequisite-only and must report
  public key lengths matching the current request before verification proceeds.
- fixed: successful verification reports only a prerequisite-only result while
  service-success=false, authentication-success=false, session-count=0,
  channel-count=0, shell-attached=false, live-reachability=false, and
  ssh-ready=false remain authoritative.
- deferred: retained smoke evidence, closeout reconciliation, authentication
  response policy, account binding, authentication success, sessions/channels,
  shell attachment, live reachability, hardware proof, compatibility, broad
  expansion, and phase transition.
- not-an-issue: no decision-log update is required because the implementation
  follows the accepted verifier policy and does not change algorithm,
  lifetime, or redaction boundaries.

## Validation

- static source/task/docs review: pass.
- cargo fmt --all -- --check: pass.
- cargo -Zjson-target-spec test --quiet: pass under configured QEMU runner;
  779 no_std tests passed.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass with existing large search-index
  warning.
- git diff --cached --check: pass.

No Pi 5 hardware run, lab-controller API action, hardwareTestLock acquisition,
boot publication, authentication response, authentication success, service
success, session/channel or shell work, live reachability claim, compatibility
claim, broad expansion, or phase transition was performed.

## Redaction Review

Pass. Retained evidence contains only task ids, file paths, fixed labels,
public byte lengths, false/zero readiness counters, validation commands, and
classifications. It retains no session-id bytes, authorized-key bytes,
request/decoded public-key blobs, signature bytes, signed-data bytes,
fingerprints, digests, user names, comments, user/operator identity,
key-derived identifiers, stable identifiers, hardware data, or boot artifacts.

## Accepted Frontier

Talos now has local prerequisite-only ssh-ed25519 publickey signature
verification for a modeled decrypted SSH_MSG_USERAUTH_REQUEST/publickey
payload. The verifier requires signature-present=true, an available private
SshUserauthSessionIdentifier handle, an authorized_keys prerequisite-only match
for the current request length, and an ssh-ed25519 request key/signature.

This clears only the cryptographic verification prerequisite. No
authentication responses, account binding, authentication success,
sessions/channels, shell attachment, live reachability, hardware proof,
OpenSSH/POSIX/Linux compatibility claim, broad expansion, phase transition, or
ssh-ready=true is accepted.

## Selected Next Task

selected_next_task=phase12-shell-ssh-publickey-verification-smoke-20260622.

The selected smoke task is objective because the implementation now has
focused source/unit coverage and needs retained smoke/regression evidence and
redaction review before closeout.

## Acceptance

Accepted as bounded publickey signature-verification core.
selected_next_task=phase12-shell-ssh-publickey-verification-smoke-20260622.
