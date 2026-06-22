# Phase 12.6 SSH publickey authentication success account-policy contract

Task id: phase12-ssh-publickey-auth-success-account-policy-contract-20260622

Status: accepted.

Classification: phase12-ssh-publickey-auth-success-account-policy-contract-accepted.

## Goal

Define the smallest account-binding and publickey authentication-success
policy after the accepted publickey USERAUTH_PK_OK/USERAUTH_FAILURE response
frontier, while keeping sessions/channels, shell attachment, live reachability,
hardware proof, OpenSSH/POSIX/Linux compatibility, broad expansion, phase
transition, and ssh-ready=true unaccepted.

## Scope

- Reviewed the accepted response-policy closeout, userauth parser,
  authorized_keys parser/key-match closeout, publickey verifier closeout,
  readiness source, Phase 12 project docs, roadmap, and decision log.
- Defined the first account-binding boundary as one reserved Talos SSH login
  account name, the ASCII literal talos, backed by the accepted global
  /etc/talos/ssh/authorized_keys key-match prerequisite.
- Defined the first modeled USERAUTH_SUCCESS policy only for signed-valid
  ssh-ed25519 publickey requests whose same request key matched the accepted
  authorized_keys prerequisite and whose request user-name matches the reserved
  account policy.
- Preserved session-count=0, channel-count=0, shell-attached=false,
  live-reachability=false, and ssh-ready=false as authoritative after modeled
  authentication success.

## Non-goals

No Rust source implementation, session/channel allocation, PTY/process/shell
attachment, live sockets, hardware/lab action, boot publication, OpenSSH/POSIX
or Linux compatibility claim, broad account database, per-user home directory,
password or keyboard-interactive authentication, writable account storage, broad
expansion, phase transition, or ssh-ready=true is accepted.

Durable evidence must not retain request user-name strings, operator identity,
authorized-key bytes, request or decoded public-key blobs, signature bytes,
signed-data bytes, fingerprints, digests, session-id bytes, peer strings,
key-derived identifiers, stable identifiers, hardware data, or boot artifacts.

## Reviewed Inputs

- tasks/2026-06-22-phase12-ssh-publickey-auth-response-policy-closeout.md
- tasks/2026-06-22-phase12-ssh-publickey-auth-response-policy-contract.md
- tasks/2026-06-22-phase12-ssh-publickey-auth-response-policy-core.md
- tasks/2026-06-22-phase12-ssh-publickey-auth-response-policy-smoke.md
- tasks/2026-06-22-phase12-ssh-preauth-service-userauth-closeout.md
- tasks/2026-06-22-phase12-ssh-authorized-keys-parser-closeout.md
- tasks/2026-06-22-phase12-ssh-publickey-verification-closeout.md
- src/ssh_service_readiness.rs
- src/ssh_key_readiness.rs
- docs/src/project/phase12-networking-ssh.md
- docs/src/roadmap.md
- docs/src/decisions/README.md

## Contract

The next implementation may add a local modeled publickey-authentication
success classifier with exactly one accepted success path:

- the encrypted transport dispatch and service/userauth prerequisite selected
  a modeled decrypted SSH_MSG_USERAUTH_REQUEST/publickey payload;
- the request service is ssh-connection and method is publickey;
- signature-present=true;
- algorithm is ssh-ed25519;
- the request public key parses as ssh-ed25519;
- the same request public key has an accepted
  authorized-keys-match-prerequisite-only result from
  /etc/talos/ssh/authorized_keys;
- the private SshUserauthSessionIdentifier handle is available;
- the accepted publickey verifier returns prerequisite-only success for the
  same request;
- account policy is enabled; and
- the request user-name matches the single reserved Talos SSH login account,
  the ASCII literal talos.

The reserved account name is a public source constant, but runtime evidence
must never retain the request user-name string. The reserved account is a local
Talos management account only. It is not a POSIX user database, UID/GID model,
home directory, shell path, login session, environment, PAM/NSS equivalent,
per-user authorized_keys lookup, or operator identity claim. The
implementation may choose source-local names for the account-policy enum and
account constant, but retained evidence must expose only fixed account-policy
labels, never request user-name strings.

The next implementation may classify only these response outcomes:

- SSH_MSG_USERAUTH_SUCCESS for the accepted success path above;
- SSH_MSG_USERAUTH_FAILURE for account mismatch, account policy disabled,
  missing account prerequisite, missing key-match/verifier/session/service
  prerequisite, unsigned publickey probes after the response-policy slice,
  invalid or malformed signatures, unauthorized keys, malformed requests,
  unsupported algorithms, redaction-sensitive paths, and every other path in
  this slice.

The implementation must not report partial success. It must not allocate a
session or channel, attach a shell, open a socket, publish a boot image, or
claim OpenSSH compatibility. For retained diagnostics and task evidence, the
fixed-label families are:

- publickey-auth-success-prerequisite-only;
- publickey-auth-success-account-match;
- publickey-auth-failure-account-mismatch;
- publickey-auth-failure-account-policy-disabled;
- publickey-auth-failure-account-prerequisite-missing;
- publickey-auth-failure-response-prerequisite-missing;
- publickey-auth-failure-signature-invalid;
- publickey-auth-failure-authorized-key-no-match;
- publickey-auth-failure-request-malformed;
- publickey-auth-failure-redaction-sensitive;
- authentication-success-local-only;
- session-unimplemented;
- not-ready.

Readiness reporting may set authentication-success=true only for the modeled
USERAUTH_SUCCESS result in this local account-policy slice. It must preserve
session-count=0, channel-count=0, shell-attached=false,
live-reachability=false, and ssh-ready=false. Any aggregate SSH service
readiness remains not-ready until a later accepted session/channel/shell/live
frontier changes those counters.

## Findings and Disposition

- fixed: defined the smallest account-binding boundary after the accepted
  PK_OK/FAILURE response frontier instead of jumping directly to sessions or
  shell attachment.
- fixed: tied USERAUTH_SUCCESS to the same request's accepted publickey
  verifier success and authorized_keys key-match prerequisite.
- fixed: limited account policy to one reserved Talos management account and
  rejected broad POSIX, per-user, writable, password, and shell account
  semantics.
- fixed: required all non-success paths to fail closed with USERAUTH_FAILURE
  and fixed labels.
- fixed: defined the readiness transition for this slice:
  authentication-success may become true, but session/channel counts,
  shell-attached, live-reachability, and ssh-ready remain false/zero.
- fixed: durable evidence redaction excludes request user names, operator
  identity, key material, key-derived identifiers, stable identifiers,
  session-id bytes, signatures, hardware data, and boot artifacts.
- deferred: Rust source implementation, retained smoke evidence, closeout,
  sessions/channels, shell attachment, live reachability, hardware proof,
  OpenSSH/POSIX/Linux compatibility, broad account database, broad expansion,
  phase transition, and ssh-ready=true.
- not-an-issue: no source change is required for this contract because it
  defines the next implementation boundary only.

## Validation

- static task/docs/source review: pass.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check: pass.

Conditional gates not run: cargo fmt --all -- --check and
cargo -Zjson-target-spec test --quiet were not required because this contract
touched no Rust source or Cargo metadata.

No Pi 5 hardware run, lab-controller API action, hardwareTestLock acquisition,
boot publication, Rust source implementation, sessions/channels, shell
attachment, live reachability claim, compatibility claim, broad expansion, or
phase transition was performed.

## Redaction Review

Pass. Retained evidence contains only task ids, source/doc paths, public SSH
message names, fixed account-policy label families, public byte-length and
count categories, false/zero readiness counters, validation commands, and
classifications. It retains no request user-name strings, operator identity,
authorized-key bytes, request or decoded public-key blobs, signature bytes,
signed-data bytes, fingerprints, digests, session-id bytes, peer strings,
key-derived identifiers, stable identifiers, hardware data, or boot artifacts.

## Accepted Frontier

Talos now has a bounded account-binding and publickey authentication-success
policy contract. The next implementation may model USERAUTH_SUCCESS only for a
signed-valid ssh-ed25519 publickey request whose same request key matched the
accepted authorized_keys prerequisite, whose signature verifier succeeded with
the accepted private session-id handle, and whose request user-name matches the
single reserved Talos SSH login account name, the ASCII literal talos.

This accepts only the account-policy/authentication-success contract. No source
implementation, session/channel allocation, shell attachment, live reachability,
hardware proof, OpenSSH/POSIX/Linux compatibility, broad expansion, phase
transition, or ssh-ready=true is accepted.

## Selected Next Task

selected_next_task=phase12-ssh-publickey-auth-success-account-core-20260622.

The selected implementation task is objective because the accepted contract
defines success prerequisites, fail-closed account-policy cases, fixed label
families, readiness boundaries, and durable-evidence redaction rules.

## Acceptance

Accepted as bounded publickey authentication-success account-policy contract.
selected_next_task=phase12-ssh-publickey-auth-success-account-core-20260622.
