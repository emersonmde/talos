# Phase 12.6 SSH userauth session-id core

Task id: phase12-ssh-userauth-session-id-core-20260622

Status: accepted.

Classification: phase12-ssh-userauth-session-id-core-accepted.

## Goal

Expose the bounded SSH session identifier produced by accepted runtime KEX to
userauth-facing source as a private handle, and keep diagnostics fail-closed and
redacted when the prerequisite is unavailable.

## Scope

- Retained the first runtime KEX exchange hash as the userauth session
  identifier inside SshRuntimeKexReady.
- Added SshUserauthSessionIdentifier as a private fixed-size handle over the
  retained session id.
- Zeroized the retained session identifier when SshRuntimeKexReady is dropped.
- Added redacted userauth-facing classifier/report labels for available,
  unavailable, malformed, and over-limit session-id states.
- Added focused source/unit coverage for available-after-KEX, unavailable,
  malformed, over-limit, repeated access/lifetime, and no diagnostic byte leak
  behavior.

## Non-goals

No publickey parsing or key matching, signature verification, authentication
response emission, SSH_MSG_USERAUTH_SUCCESS, USERAUTH_FAILURE, PK_OK,
service-success acceptance, authentication success, session/channel allocation,
shell attachment, live reachability, hardware/lab action, OpenSSH/POSIX/Linux
compatibility claim, broad expansion, phase transition, or ssh-ready=true is
accepted.

## Findings and disposition

- fixed: runtime KEX now retains the accepted first exchange hash as a private
  session identifier for later userauth signature verification.
- fixed: the session identifier is exposed only through a fixed-size private
  handle and is zeroized with the ready KEX state.
- fixed: userauth-facing diagnostics report fixed labels and optional byte
  lengths only; they do not expose session-id bytes, exchange hashes, keys,
  signatures, payloads, peer/user strings, or stable identifiers.
- fixed: fail-closed unavailable, malformed, and over-limit session-id states
  are represented with fixed labels.
- deferred: authorized_keys parsing/key-match policy, publickey signature
  verification, authentication response policy, authentication success,
  sessions/channels, shell attachment, live reachability, compatibility, and
  hardware proof remain future explicit tasks.
- not-an-issue: storing the session identifier inside SshRuntimeKexReady is
  consistent with SSH semantics for the first exchange hash and keeps the value
  bounded to the accepted KEX state lifetime.

## Evidence

- Source/unit evidence:
  - src/ssh_runtime_crypto.rs retains userauth_session_identifier inside
    SshRuntimeKexReady, exposes SshUserauthSessionIdentifier, and zeroizes the
    retained bytes on drop.
  - src/ssh_service_readiness.rs exposes fixed labels:
    sshservicediag-userauth-session-identifier-available,
    sshservicediag-userauth-session-identifier-unavailable,
    sshservicediag-userauth-session-identifier-malformed, and
    sshservicediag-userauth-session-identifier-over-limit.
  - Unit tests cover available-after-KEX, unavailable, malformed, over-limit,
    repeated handle access, and false/zero readiness.
- Validation:
  - cargo fmt --all -- --check: pass.
  - cargo -Zjson-target-spec test session_id --quiet: pass; 768 no_std tests
    passed, including the new runtime/service session-id cases.
  - cargo -Zjson-target-spec test --quiet: pass.
  - git diff --check: pass.
  - /home/node/.cargo/bin/mdbook build: pass.
  - git diff --cached --check: pass.

## Redaction review

Pass. Retained durable evidence is limited to task ids, file paths, fixed label
names, small public byte lengths, validation commands, and classifications. No
session-id bytes, exchange hashes, public-key blobs, signatures, authorized-key
bytes, fingerprints, digests, peer/user strings, ciphertext/plaintext, keys,
IVs, shared secrets, operator identity, stable transport/session identifiers,
live hardware data, or boot artifacts are retained.

## Accepted frontier

Talos now has a bounded userauth-facing session-id prerequisite after accepted
runtime KEX. Authentication-success=false, service-success=false,
session-count=0, channel-count=0, shell-attached=false, live-reachability=false,
and ssh-ready=false remain authoritative.

selected_next_task=phase12-ssh-userauth-session-id-smoke-20260622.
