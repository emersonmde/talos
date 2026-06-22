# Phase 12.6 SSH userauth session-id smoke

Task id: phase12-ssh-userauth-session-id-smoke-20260622

Status: accepted.

Classification: phase12-ssh-userauth-session-id-smoke-accepted.

## Goal

Retain focused smoke/regression evidence for the accepted private SSH
userauth session-identifier prerequisite without adding publickey
authentication behavior or accepting service success, authentication success,
sessions, channels, shell attachment, live reachability, hardware behavior,
compatibility, broad expansion, or a phase transition.

## Scope

- Exercise the accepted runtime/userauth session-id paths with focused local
  unit/regression evidence.
- Cover session-id availability only after accepted runtime KEX readiness.
- Cover fail-closed unavailable, malformed, and over-limit session-id
  diagnostics before or outside the accepted ready-KEX state.
- Retain only fixed labels, byte lengths, false/zero readiness counters,
  validation commands, task ids, file paths, and classifications.

## Non-goals

- No new Rust protocol behavior, publickey parsing or matching, authorized_key
  lookup, signature verification, SSH_MSG_USERAUTH_SUCCESS,
  SSH_MSG_USERAUTH_FAILURE, USERAUTH_PK_OK, authentication response emission,
  service success, authentication success, account/user model,
  session/channel success, PTY allocation, process or shell attachment, live
  socket connection, hardware/lab action, boot publication,
  OpenSSH/POSIX/Linux compatibility claim, broad expansion, phase transition,
  or ssh-ready=true.
- No retention of session-id bytes, exchange hashes, public-key blobs,
  signatures, packet payload bytes, ciphertext, plaintext, MAC/tag material,
  keys, IV bytes, shared secrets, peer raw input/address, peer/user strings,
  operator identity, key-derived identifiers, stable transport/session
  identifiers, live hardware data, or boot artifacts.

## Findings and disposition

- fixed: focused local smoke evidence covers session-id availability only
  after accepted runtime KEX readiness through the private ready-KEX handle.
- fixed: focused local smoke evidence covers repeated available handle access
  without exposing session-id bytes in retained evidence.
- fixed: focused local smoke evidence covers fail-closed unavailable,
  malformed, and over-limit session-id diagnostics with fixed labels and byte
  lengths only.
- fixed: readiness false/zero counters remain asserted by the focused
  session-id diagnostics: service-success=false, authentication-success=false,
  session-count=0, channel-count=0, shell-attached=false, and ssh-ready=false.
- not-an-issue: this smoke task adds no new protocol behavior; it records the
  accepted core's focused source/unit evidence as the retained smoke
  transcript.
- deferred: authorized_keys parsing/key-match policy, publickey signature
  verification, authentication response policy, authentication success,
  sessions/channels, shell attachment, live reachability, compatibility,
  hardware proof, and phase transition remain future explicit work.

## Evidence

- static source/task/docs review: src/ssh_runtime_crypto.rs focused tests
  include userauth_session_identifier_handle_is_available_only_on_ready_kex,
  which obtains the private handle only from SshRuntimeKexReady and checks
  repeated access/lifetime behavior without retained byte evidence.
- static source/task/docs review: src/ssh_service_readiness.rs focused tests
  include userauth_session_identifier_is_available_only_after_runtime_kex and
  userauth_session_identifier_fails_closed_when_unavailable_or_malformed,
  which cover available-after-KEX, unavailable, malformed, over-limit, and
  false/zero readiness cases.
- unit/QEMU-substitute: cargo -Zjson-target-spec test session_id --quiet
  passed with QEMU on PATH; the harness completed 768 no_std tests including
  the focused runtime/service session-id cases.
- diff check: git diff --check passed.
- docs: /home/node/.cargo/bin/mdbook build passed.
- pre-commit diff check: git diff --cached --check passed.

Conditional gates not run: cargo fmt --all -- --check and full
cargo -Zjson-target-spec test --quiet were not required because this smoke
task touched no Rust source or Cargo metadata after the accepted core evidence.

## Redaction review

Pass. Retained smoke evidence contains only task ids, file paths, test names,
fixed label names, public byte lengths, false/zero readiness counters,
validation commands, and classifications. It retains no session-id bytes,
exchange hashes, public-key blobs, signatures, authorized-key bytes,
fingerprints, digests, peer/user strings, ciphertext/plaintext, keys, IVs,
shared secrets, peer raw input/address, operator identity, stable
transport/session identifiers, live hardware data, or boot artifacts.

## Accepted frontier

Talos has local smoke/regression evidence for the bounded userauth-facing
session-id prerequisite after accepted runtime KEX. Authentication-success=false,
service-success=false, session-count=0, channel-count=0, shell-attached=false,
live-reachability=false, and ssh-ready=false remain authoritative.

selected_next_task=phase12-ssh-userauth-session-id-closeout-20260622.
