# Phase 12.6 SSH authorized_keys parser/key-match closeout

Task id: phase12-ssh-authorized-keys-parser-closeout-20260622

Status: accepted.

Classification: phase12-ssh-authorized-keys-parser-closeout-accepted.

## Goal

Close out the bounded authorized_keys parser/key-match prerequisite by
reconciling the accepted policy, implementation, smoke evidence, redaction
boundaries, and remaining publickey-authentication blockers.

## Scope

- Reconciled the accepted authorized_keys parser/key-match policy contract,
  source implementation, and focused smoke/regression evidence.
- Recorded the current false/zero readiness frontier for key-match evidence:
  key match is a prerequisite only and does not authorize a user, emit an
  authentication response, accept authentication success, allocate sessions or
  channels, attach a shell, prove live reachability, claim compatibility,
  broaden scope, transition phase, or set ssh-ready=true.
- Updated the Phase 12 SSH architecture notes and roadmap frontier.

## Non-goals

No Rust source implementation, Cargo dependency adoption, signature
verification, USERAUTH_PK_OK, SSH_MSG_USERAUTH_SUCCESS,
SSH_MSG_USERAUTH_FAILURE, partial-success behavior, authentication response
emission, authentication success, account database, sessions/channels,
PTY/process/shell attachment, live socket reachability, hardware/lab action,
boot publication, OpenSSH/POSIX/Linux compatibility claim, broad expansion,
phase transition, or ssh-ready=true is accepted.

No retained durable evidence may include authorized-key bytes, decoded
public-key blobs, request public-key blobs, fingerprints, digests, signatures,
comments, user/operator identity, key-derived identifiers, stable identifiers,
session-id bytes, exchange hashes, live hardware data, or boot artifacts.

## Reconciled evidence map

- policy: tasks/2026-06-22-phase12-ssh-authorized-keys-parser-policy-contract.md
  accepted the narrow option-free ssh-ed25519 authorized_keys policy, fixed
  fail-closed label families, redaction boundaries, and user/account deferral.
- core: tasks/2026-06-22-phase12-ssh-authorized-keys-parser-core.md accepted
  match_authorized_key_public_blob in src/ssh_key_readiness.rs, preserving the
  read-only VFS metadata guard and comparing decoded ssh-ed25519 publickey
  blobs with caller-owned request blobs in memory only.
- smoke: tasks/2026-06-22-phase12-ssh-authorized-keys-parser-smoke.md
  retained focused local source/unit regression evidence for match, non-match,
  missing/invalid/oversized metadata, blank/comment-only input, unsupported
  option or algorithm, malformed line shape, and malformed public-key blob.

## Findings and disposition

- fixed: the parser/key-match prerequisite now has a complete accepted chain:
  policy, source implementation, focused smoke evidence, docs, and redaction
  posture.
- fixed: the closeout explicitly preserves the prerequisite-only boundary:
  a matched authorized key may only unblock later signature verification.
- fixed: accepted evidence keeps authentication-success=false,
  service-success=false, session-count=0, channel-count=0,
  shell-attached=false, live-reachability=false, and ssh-ready=false.
- fixed: retained evidence excludes authorized-key bytes, decoded/request
  public-key blobs, fingerprints, digests, signatures, comments, user/operator
  identity, key-derived identifiers, stable identifiers, session-id bytes,
  exchange hashes, hardware data, and boot artifacts.
- deferred: publickey signature-verification contract and implementation,
  authentication response policy, authentication success, account/user
  semantics, sessions/channels, shell attachment, live reachability, hardware
  proof, compatibility, broad expansion, and phase transition.
- not-an-issue: this closeout touches no Rust source because the accepted
  source/unit evidence already exists in the core and smoke records.

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
boot publication, signature verification, authentication response,
authentication success, service success, session/channel or shell work, live
reachability claim, compatibility claim, broad expansion, or phase transition
was performed.

## Redaction review

Pass. Retained closeout evidence is limited to task ids, file paths, public
test names, fixed label families, public byte-length values, line counts,
false/zero readiness counters, validation commands, and classifications. It
retains no authorized-key bytes, decoded public-key blobs, request public-key
blobs, fingerprints, digests, signatures, comments, user/operator identity,
key-derived identifiers, stable identifiers, session-id bytes, exchange
hashes, live hardware data, or boot artifacts.

## Accepted frontier

Talos now has the bounded authorized_keys parser/key-match prerequisite closed
out. The accepted publickey-authentication prerequisites now include a private
userauth session-id handle and an in-memory authorized_keys key-match boundary.
Together they are only prerequisites for later publickey signature
verification.

No signature verification, authentication response emission, authentication
success, account authorization, sessions/channels, PTY/process/shell
attachment, live socket reachability, hardware action, OpenSSH/POSIX/Linux
compatibility claim, broad expansion, or phase transition is accepted.
service-success=false, authentication-success=false, session-count=0,
channel-count=0, shell-attached=false, live-reachability=false, and
ssh-ready=false remain authoritative.

## Selected next task

selected_next_task=phase12-ssh-publickey-verification-contract-20260622.

The selected next task is objective because session-id retention/redaction,
authorized_keys parser/key-match policy, parser implementation, smoke
evidence, and closeout are now accepted, while signature verification still
needs a bounded contract before any verifier implementation, authentication
response, authentication success, session/channel behavior, shell attachment,
reachability, compatibility, broad expansion, or phase transition.

## Acceptance

Accepted as bounded authorized_keys parser/key-match closeout.
selected_next_task=phase12-ssh-publickey-verification-contract-20260622.
