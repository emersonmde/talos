# Phase 12.6 SSH publickey signature-verification contract

Task id: phase12-ssh-publickey-verification-contract-20260622

Status: accepted.

Classification: phase12-ssh-publickey-verification-contract-accepted.

## Goal

Define the bounded publickey signature-verification contract after the accepted
private userauth session-id handle and authorized_keys parser/key-match
prerequisites, while keeping response emission, authentication success,
sessions, shell attachment, live reachability, compatibility, broad expansion,
and phase transition unaccepted.

## Scope

- Reviewed accepted userauth session-id, authorized_keys parser/key-match,
  pre-authentication service/userauth parser, runtime crypto, host-key
  material, and redaction evidence.
- Defined the signed-data inputs for SSH_MSG_USERAUTH_REQUEST/publickey
  verification and the accepted ssh-ed25519-only algorithm boundary.
- Defined the key-match prerequisite and signature-verification failure label
  families for the next implementation slice.
- Recorded lifetime, zeroization, and durable-evidence redaction boundaries
  for session-id, request public-key, and signature material.
- Selected one bounded verifier implementation follow-up.

## Non-goals

No Rust source implementation, Cargo dependency adoption, authentication
response emission, USERAUTH_PK_OK, SSH_MSG_USERAUTH_SUCCESS,
SSH_MSG_USERAUTH_FAILURE, partial-success behavior, authentication success,
account database, account/user authorization, sessions/channels,
PTY/process/shell attachment, live socket reachability, hardware/lab action,
boot publication, OpenSSH/POSIX/Linux compatibility claim, broad expansion,
phase transition, or ssh-ready=true is accepted.

No durable evidence may retain session-id bytes, authorized-key bytes,
decoded/request public-key blobs, signature bytes, signed-data bytes,
fingerprints, digests, user names, comments, user/operator identity,
key-derived identifiers, stable identifiers, exchange hashes, hardware data,
or boot artifacts.

## Reviewed inputs

- tasks/2026-06-22-phase12-ssh-publickey-auth-contract.md
- tasks/2026-06-22-phase12-ssh-userauth-session-id-core.md
- tasks/2026-06-22-phase12-ssh-userauth-session-id-smoke.md
- tasks/2026-06-22-phase12-ssh-userauth-session-id-closeout.md
- tasks/2026-06-22-phase12-ssh-authorized-keys-parser-policy-contract.md
- tasks/2026-06-22-phase12-ssh-authorized-keys-parser-core.md
- tasks/2026-06-22-phase12-ssh-authorized-keys-parser-smoke.md
- tasks/2026-06-22-phase12-ssh-authorized-keys-parser-closeout.md
- tasks/2026-06-22-phase12-ssh-preauth-service-userauth-closeout.md
- tasks/2026-06-22-phase12-ssh-runtime-kex-closeout.md
- src/ssh_service_readiness.rs
- src/ssh_runtime_crypto.rs
- src/ssh_key_readiness.rs
- Cargo.toml
- local registry source for ssh-key 0.7.0-rc.10, ed25519-dalek
  3.0.0-rc.0, and signature 3.0.0
- docs/src/project/phase12-networking-ssh.md
- docs/src/roadmap.md
- docs/src/decisions/README.md

## Contract

The first publickey verifier implementation may run only after these
prerequisites are true in memory for the same decrypted userauth request:

- accepted encrypted transport dispatch has delivered an
  SSH_MSG_USERAUTH_REQUEST payload;
- the modeled ssh-userauth service prerequisite is satisfied;
- the userauth request service string is ssh-connection;
- the userauth method string is publickey;
- signature-present is true;
- the algorithm string is exactly ssh-ed25519;
- the request public-key blob parses as an ssh-ed25519 public key;
- /etc/talos/ssh/authorized_keys key-match reports
  authorized-keys-match-prerequisite-only for that same request public-key
  blob;
- SshRuntimeKexReady exposes an available private
  SshUserauthSessionIdentifier handle.

If signature-present is false, this contract does not authorize PK_OK. The
next implementation must classify that path as verifier-not-requested or
equivalent fail-closed prerequisite evidence only. Response policy remains a
separate task.

The signed-data buffer for verification is the RFC 4252 publickey signature
payload:

- SSH string wrapping the accepted userauth session identifier;
- byte SSH_MSG_USERAUTH_REQUEST;
- the original request user-name string;
- the original request service string;
- the original request method string;
- boolean true;
- the original request public-key algorithm string;
- the original request public-key blob string.

The implementation must construct this buffer in memory only, verify the
decoded ssh-ed25519 signature against the parsed request public key, and
zeroize or drop temporary signed-data and signature buffers before returning.
The implementation may use the already accepted ssh-key 0.7.0-rc.10,
ed25519-dalek 3.0.0-rc.0, and signature 3.0.0 verifier surfaces. It must not
add a broader algorithm set, certificate support, account lookup, response
emission, session allocation, shell attachment, live socket claim, or
compatibility claim.

The verifier result may report only fixed labels, public byte lengths, bounded
field counts, and false/zero readiness counters. Even a successful signature
verification is only a cryptographic prerequisite. It is not account
authorization and not authentication success.

## Failure labels

The next implementation may choose exact source enum names, but retained
diagnostics must stay in these fixed-label families:

- publickey-verification-not-requested;
- publickey-verification-session-id-unavailable;
- publickey-verification-authorized-key-missing;
- publickey-verification-authorized-key-no-match;
- publickey-verification-algorithm-unsupported;
- publickey-verification-key-blob-malformed;
- publickey-verification-signature-malformed;
- publickey-verification-signed-data-malformed;
- publickey-verification-signature-rejected;
- publickey-verification-prerequisite-only;
- authentication-unimplemented;
- not-ready.

These labels do not authorize USERAUTH_PK_OK, USERAUTH_FAILURE,
USERAUTH_SUCCESS, partial-success behavior, account authorization, sessions,
shell attachment, live reachability, compatibility, or ssh-ready=true.

## Findings and disposition

- fixed: publickey verification now has an explicit bounded contract instead
  of jumping from key match and session-id availability to authentication
  behavior.
- fixed: the signed-data construction is tied to RFC 4252 publickey
  verification inputs and to the accepted private session-id handle.
- fixed: verification is limited to ssh-ed25519 and requires the already
  accepted authorized_keys key-match prerequisite for the same request public
  key.
- fixed: signature-present=false is not treated as PK_OK readiness; response
  policy remains deferred.
- fixed: a successful verifier result is prerequisite-only and keeps
  authentication-success=false, service-success=false, session-count=0,
  channel-count=0, shell-attached=false, live-reachability=false, and
  ssh-ready=false.
- deferred: verifier source implementation, focused source/unit smoke,
  authentication response policy, authentication success, account/user
  semantics, sessions/channels, shell attachment, live reachability, hardware
  proof, compatibility, broad expansion, and phase transition.
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
boot publication, signature verification implementation, authentication
response, authentication success, service success, session/channel or shell
work, live reachability claim, compatibility claim, broad expansion, or phase
transition was performed.

## Redaction review

Pass. Retained evidence contains only task ids, file paths, dependency names,
public SSH message names, fixed label families, public byte-length categories,
field counts, false/zero readiness counters, validation commands, and
classifications. It retains no session-id bytes, authorized-key bytes,
decoded/request public-key blobs, signature bytes, signed-data bytes,
fingerprints, digests, user names, comments, user/operator identity,
key-derived identifiers, stable identifiers, exchange hashes, hardware data,
or boot artifacts.

## Accepted frontier

Talos now has a bounded publickey signature-verification contract. The
accepted prerequisites for the next verifier implementation are: encrypted
transport/userauth request shape, private userauth session-id handle, and
in-memory authorized_keys key match for the same request public key.

No verifier implementation, authentication response emission, authentication
success, account authorization, sessions/channels, PTY/process/shell
attachment, live socket reachability, hardware action, OpenSSH/POSIX/Linux
compatibility claim, broad expansion, phase transition, or ssh-ready=true is
accepted. service-success=false, authentication-success=false,
session-count=0, channel-count=0, shell-attached=false,
live-reachability=false, and ssh-ready=false remain authoritative.

## Selected next task

selected_next_task=phase12-ssh-publickey-verification-core-20260622.

The selected implementation task is objective because the accepted contract
defines the required inputs, ssh-ed25519 algorithm limit, signed-data shape,
key-match prerequisite, failure labels, redaction posture, lifetime rules,
validation expectations, and unchanged false/zero readiness frontier.

## Acceptance

Accepted as bounded publickey signature-verification contract.
selected_next_task=phase12-ssh-publickey-verification-core-20260622.
