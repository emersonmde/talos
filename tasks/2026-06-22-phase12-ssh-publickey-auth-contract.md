# Phase 12.6 SSH publickey authentication contract

Task id: phase12-ssh-publickey-auth-contract-20260622

Status: accepted

Classification: phase12-ssh-publickey-auth-contract-prerequisites-blocked.

## Goal

Define or block the next publickey authentication boundary after the accepted
pre-authentication service/userauth parser, without accepting authentication
success, sessions, shell behavior, live reachability, hardware behavior,
compatibility, broad expansion, or a phase transition.

## Scope

- Reviewed the accepted service/userauth parser closeout, publickey request
  shape modeling, authorized-key source policy, host-key private-material
  parsing/signing boundary, runtime KEX/packet crypto boundary, service
  readiness boundary, and redaction rules.
- Defined the prerequisites for a real publickey authentication implementation.
- Kept this task to contract work only. No Rust implementation, dependency
  adoption, authentication response, authorized-key parser, signature
  verification, account/user model, session/channel allocation, shell
  attachment, hardware action, reachability claim, compatibility claim, broad
  expansion, or phase transition is accepted here.

## Non-goals

- No implementation, authentication success, SSH_MSG_USERAUTH_SUCCESS,
  SSH_MSG_USERAUTH_FAILURE, SSH_MSG_USERAUTH_PK_OK, partial-success behavior,
  account/user authorization, authorized-key byte parsing, public-key blob
  retention, signature verification, session/channel allocation,
  PTY/process/shell attachment, live socket connection, hardware/lab action,
  boot publication, OpenSSH/POSIX/Linux compatibility claim, broad expansion,
  phase transition, or ssh-ready=true.
- No retention of packet payload bytes, parsed usernames, peer service/method
  strings, authorized-key bytes, public-key blobs, signatures, fingerprints,
  digests, session identifiers, ciphertext, plaintext, keys, IVs, exchange
  hashes, shared secrets, peer addresses, operator identity, stable
  transport/session identifiers, live hardware data, or boot artifacts.

## Reviewed Inputs

- tasks/2026-06-22-phase12-ssh-preauth-service-userauth-closeout.md
- tasks/2026-06-22-phase12-ssh-preauth-service-userauth-core.md
- tasks/2026-06-21-phase12-ssh-authorized-key-policy-contract.md
- tasks/2026-06-21-phase12-ssh-authorized-key-vfs-metadata-core.md
- tasks/2026-06-22-phase12-ssh-host-key-private-material-contract.md
- tasks/2026-06-22-phase12-ssh-host-key-private-material-core.md
- tasks/2026-06-22-phase12-ssh-runtime-kex-core.md
- tasks/2026-06-22-phase12-ssh-newkeys-packet-crypto-closeout.md
- tasks/2026-06-22-phase12-ssh-service-readiness-closeout.md
- src/ssh_service_readiness.rs
- src/ssh_key_readiness.rs
- src/ssh_runtime_crypto.rs
- docs/src/project/phase12-networking-ssh.md
- docs/src/roadmap.md
- docs/src/decisions/README.md

## Contract

The accepted service/userauth parser already recognizes
SSH_MSG_USERAUTH_REQUEST (50) with service ssh-connection and method publickey
after the modeled ssh-userauth service prerequisite. That recognition is only
a diagnostic shape. It may parse the public SSH publickey fields enough to
classify malformed, missing, unsupported, and present shapes, but it does not
authenticate a user.

A real publickey authentication implementation is not objective yet. The
blocking prerequisites are:

- Runtime KEX must expose a bounded session identifier for authentication
  verification. Today src/ssh_runtime_crypto.rs computes an exchange hash for
  host-key signing and key derivation, then zeroizes it. That is correct for
  the current KEX evidence, but SSH publickey verification needs the session
  identifier in the signed data and no accepted contract currently exposes or
  redacts that value.
- Authorized-key material is still metadata-only. The accepted
  /etc/talos/ssh/authorized_keys policy and core classify presence and byte
  length only; no accepted parser, key-format policy, key comparison boundary,
  operator identity binding, or user/account authorization rule exists.
- Publickey authentication response policy is unaccepted. Talos has no accepted
  behavior for signature-present=false probes, PK_OK responses,
  USERAUTH_FAILURE, partial-success, user-name handling, or final
  USERAUTH_SUCCESS.
- Session and channel allocation remain unaccepted. Even after a future auth
  success, ssh-ready must remain false until session/channel/shell and
  reachability work is separately accepted.

The smallest safe next planning slice should choose between the missing
prerequisites rather than implementing authentication success. Acceptable
follow-up scopes include a session-identifier retention/redaction contract or
an authorized_keys parser/key-match contract, but the supervisor must enqueue
one explicit task with its own scope, gates, docs, and evidence before worker
promotion.

## Failure Labels

Any later publickey-auth implementation must remain fail-closed and fixed-label
only until all prerequisites are accepted. The minimum label families to define
before implementation are:

- session-identifier-unavailable;
- authorized-key-parser-unaccepted;
- authorized-key-missing-or-invalid;
- publickey-algorithm-unsupported;
- publickey-blob-malformed;
- publickey-signature-missing-or-deferred;
- publickey-signature-invalid;
- authentication-unimplemented;
- not-ready.

The exact label spelling is intentionally left to the planned implementation
task. This contract does not add labels or source behavior.

## Findings

- fixed: the publickey authentication boundary now has concrete prerequisites
  instead of an implicit jump from parser recognition to authentication
  success.
- fixed: identified session identifier exposure as a required prerequisite;
  current runtime KEX deliberately zeroizes the exchange hash and exposes no
  accepted session-id value for userauth signature verification.
- fixed: identified authorized_keys parsing/key-match policy as a required
  prerequisite; current authorized-key readiness is metadata-only.
- fixed: signature-present=false probes, PK_OK, USERAUTH_FAILURE,
  partial-success, USERAUTH_SUCCESS, user/account binding, and final
  authentication state remain unaccepted response-policy work.
- fixed: redaction boundaries forbid retaining public-key blobs, signatures,
  authorized-key bytes, fingerprints, digests, session identifiers, exchange
  hashes, user names, peer strings, operator identity, and stable identifiers
  in durable evidence.
- deferred: session-id retention contract, authorized_keys parser/key-match
  contract, publickey verifier implementation, authentication response policy,
  account/user model, authentication success, session/channel allocation, shell
  attachment, live reachability, compatibility, hardware proof, broad
  expansion, and phase transition.
- not-an-issue: no Rust source change is required for this contract because
  the accepted parser already proves only diagnostic publickey shape modeling.

## Accepted Frontier

The accepted frontier remains local publickey authentication contract work only.
Talos may recognize that the current parser reached the publickey method shape,
but authentication-success=false, service-success=false, session-count=0,
channel-count=0, shell-attached=false, live reachability=false, and
ssh-ready=false remain authoritative.

No publickey authentication implementation, authorized-key parser, signature
verification, authentication response emission, user/account policy,
session/channel allocation, shell attachment, live reachability,
OpenSSH/POSIX/Linux compatibility, hardware/lab action, broad expansion, or
phase transition is accepted.

## Selected Next Task

selected_next_task=null.

planningNeeded=true. The supervisor must plan exactly one bounded prerequisite
task before any worker promotion. The planning discriminator is whether to
first expose a bounded, redacted runtime KEX session identifier for userauth
verification, or first define authorized_keys parsing/key-match policy. Both
are prerequisites to a real publickey authentication implementation, and
neither is already queued with explicit acceptance criteria.

## Evidence

- static task/docs/source review: pass; accepted pre-auth parser evidence stops
  at diagnostic publickey method shape with false/zero readiness.
- static source review: pass; src/ssh_runtime_crypto.rs computes and zeroizes
  the exchange hash and does not expose an accepted session identifier.
- static source/task review: pass; authorized-key policy/core remains
  metadata-only and accepts no authorized-key parsing or key matching.
- docs update: docs/src/project/phase12-networking-ssh.md,
  docs/src/roadmap.md, and docs/src/decisions/README.md updated for the
  prerequisite-blocked publickey authentication contract.
- no Rust source or Cargo metadata changed, so cargo fmt and cargo test were
  not required by the conditional validation gates.

## Validation

- static task/docs/source review: pass.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check: pass.

No Pi 5 hardware run, lab-controller API action, hardwareTestLock acquisition,
boot publication, service success, authentication success, session/channel or
shell work, live reachability claim, compatibility claim, broad expansion, or
phase transition was performed.

## Redaction Review

Pass. This contract retains only task ids, file paths, public SSH message
numbers and protocol literal names, fixed prerequisite/failure descriptions,
public field-order descriptions, validation commands, and classifications. It
retains no packet payload bytes, parsed usernames, peer-selected service or
method strings, authorized-key bytes, public-key blobs, signatures,
fingerprints, digests, session identifiers, ciphertext, plaintext, MAC/tag
material, keys, IV bytes, exchange hashes, shared secrets, peer raw
input/address, operator identity, key-derived identifiers, stable
transport/session identifiers, live hardware data, or boot artifacts.

## Acceptance

Accepted as a prerequisite-blocked contract. selected_next_task=null.
planningNeeded=true.
