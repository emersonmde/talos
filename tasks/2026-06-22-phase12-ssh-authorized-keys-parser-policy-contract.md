# Phase 12.6 SSH authorized_keys parser/key-match policy contract

Task id: phase12-ssh-authorized-keys-parser-policy-contract-20260622

Status: accepted.

Classification: phase12-ssh-authorized-keys-parser-policy-contract-accepted.

## Goal

Define the first bounded authorized_keys parser and publickey key-match policy
needed before SSH publickey signature verification, while keeping response
emission, authentication success, sessions, shell attachment, live
reachability, compatibility, broad expansion, and phase transition unaccepted.

## Scope

- Reviewed accepted authorized-key metadata policy/core evidence,
  publickey-authentication prerequisite evidence, and accepted userauth
  session-id prerequisite evidence.
- Selected the first parser/key-match policy for
  /etc/talos/ssh/authorized_keys over the accepted read-only VFS material.
- Defined accepted line handling, algorithm limits, malformed/unsupported
  behavior, key-match semantics, redaction boundaries, and user/account
  limitations for the next implementation slice.
- Kept this task to policy only. No Rust implementation, Cargo dependency
  adoption, signature verification, authentication response, authentication
  success, session/channel allocation, shell attachment, live reachability,
  hardware action, compatibility claim, broad expansion, or phase transition
  is accepted here.

## Non-goals

No Rust source implementation, Cargo dependency adoption, authorized-key byte
retention, public-key blob retention, fingerprint/digest retention, signature
verification, USERAUTH_PK_OK, SSH_MSG_USERAUTH_SUCCESS,
SSH_MSG_USERAUTH_FAILURE, partial-success behavior, authentication success,
account database, sessions/channels, PTY/process/shell attachment, live socket
connection, hardware/lab action, boot publication, OpenSSH/POSIX/Linux
compatibility claim, broad expansion, phase transition, or ssh-ready=true is
accepted.

No retained durable evidence may include authorized-key bytes, decoded public
key blobs, fingerprints, digests, signatures, user/operator identity,
key-derived identifiers, stable identifiers, peer strings, session-id bytes,
exchange hashes, hardware data, or boot artifacts.

## Reviewed Inputs

- tasks/2026-06-21-phase12-ssh-authorized-key-policy-contract.md
- tasks/2026-06-21-phase12-ssh-authorized-key-vfs-metadata-core.md
- tasks/2026-06-22-phase12-ssh-publickey-auth-contract.md
- tasks/2026-06-22-phase12-ssh-userauth-session-id-closeout.md
- tasks/2026-06-22-phase12-ssh-userauth-session-id-core.md
- tasks/2026-06-22-phase12-ssh-userauth-session-id-smoke.md
- src/ssh_key_readiness.rs
- src/ssh_service_readiness.rs
- src/ssh_runtime_crypto.rs
- docs/src/project/phase12-networking-ssh.md
- docs/src/roadmap.md
- docs/src/decisions/README.md

## Policy

The first authorized_keys parser/key-match implementation may read
/etc/talos/ssh/authorized_keys only through the accepted read-only VFS
boundary. It inherits the existing metadata guard: the file must be regular,
readable, non-empty, and at most 4096 bytes. Metadata-only diagnostics remain
valid and are not replaced by parser evidence.

The accepted first line format is the narrow OpenSSH-style public-key line:

- optional leading ASCII horizontal whitespace;
- key type literal ssh-ed25519;
- one or more ASCII whitespace bytes;
- base64 public-key blob text;
- optional ASCII whitespace and trailing comment text;
- LF or CRLF line ending, or end of file.

Blank lines and lines whose first non-whitespace byte is # are ignored.
Trailing comments are ignored and must not be retained. Key options before the
key type, quoted option strings, environment restrictions, command= options,
cert-authority, principals, revocation markers, SSH certificates, non-ed25519
key types, malformed base64, decoded blobs whose embedded SSH string type is
not ssh-ed25519, trailing non-comment binary material, and over-limit files are
unsupported for this slice and must fail closed.

The first key-match boundary is in-memory only: decode each accepted
ssh-ed25519 line into a public SSH publickey blob, verify the decoded blob has
the expected public type framing, and compare that decoded blob byte-for-byte
against the caller-owned publickey blob from a later SSH_MSG_USERAUTH_REQUEST.
The comparison may return only fixed labels, counts, public byte-length bounds,
and match/non-match state to the local diagnostic path. It must not retain,
print, hash, fingerprint, digest, derive from, or persist the authorized-key
bytes, decoded blob bytes, request publickey blob, comments, or user strings in
durable evidence.

User/account binding is deliberately not accepted. The first slice may state
only that one operator-provisioned key matched the request publickey blob. That
match is a prerequisite for future signature verification, not a user
authorization decision and not authentication success. User-name parsing,
account lookup, authorized principals, per-user files, options enforcement,
and final authorization remain deferred.

## Failure Labels

The next implementation may choose exact source enum names, but retained
diagnostics must stay in these fixed-label families:

- authorized-keys-missing-or-metadata-invalid;
- authorized-keys-empty-or-comment-only;
- authorized-keys-line-malformed;
- authorized-keys-line-unsupported;
- authorized-keys-algorithm-unsupported;
- authorized-keys-blob-malformed;
- authorized-keys-no-match;
- authorized-keys-match-prerequisite-only;
- authentication-unimplemented;
- not-ready.

These labels do not authorize USERAUTH_PK_OK, USERAUTH_FAILURE,
USERAUTH_SUCCESS, partial-success behavior, signature verification, account
authorization, sessions, shell attachment, live reachability, compatibility,
or ssh-ready=true.

## Findings and disposition

- fixed: the next publickey-authentication prerequisite now has an explicit
  authorized_keys parser/key-match policy instead of jumping from metadata
  presence to authentication behavior.
- fixed: the first accepted format is intentionally narrow:
  option-free ssh-ed25519 OpenSSH-style public-key lines with ignored blank,
  comment, and trailing-comment text.
- fixed: unsupported OpenSSH options, certificates, non-ed25519 algorithms,
  malformed base64/blobs, and invalid metadata fail closed rather than
  broadening account or authorization semantics.
- fixed: key matching is accepted only as an in-memory byte-for-byte blob
  comparison prerequisite. Durable evidence is fixed-label/count/length only
  and excludes keys, blobs, fingerprints, digests, comments, users, and stable
  identifiers.
- fixed: user/account binding is explicitly deferred; a key match does not
  imply account authorization, response emission, authentication success, or
  session readiness.
- deferred: parser/key-match source implementation, signature verification,
  response policy, account/user model, authentication success,
  session/channel allocation, shell attachment, live reachability, hardware
  proof, compatibility, broad expansion, and phase transition.
- not-an-issue: no Rust source change is required for this contract because it
  defines policy for the next implementation slice.

## Accepted frontier

Talos now has a bounded authorized_keys parser/key-match policy for the next
implementation task. The accepted frontier is still prerequisite-only:
/etc/talos/ssh/authorized_keys remains operator-provisioned read-only VFS
material, and a future in-memory ssh-ed25519 key match may clear only the
authorized-key key-match prerequisite.

No publickey signature verification, authentication response emission,
authentication success, account authorization, sessions/channels,
PTY/process/shell attachment, live socket reachability, hardware action,
OpenSSH/POSIX/Linux compatibility claim, broad expansion, or phase transition
is accepted. service-success=false, authentication-success=false,
session-count=0, channel-count=0, shell-attached=false,
live-reachability=false, and ssh-ready=false remain authoritative.

## Selected next task

selected_next_task=phase12-ssh-authorized-keys-parser-core-20260622.

The selected implementation task is objective because the accepted policy
defines the input file boundary, first line format, unsupported cases,
key-match semantics, redaction posture, user/account limitation, validation
expectations, and unchanged false/zero readiness frontier.

## Validation

- static task/docs/source review: pass.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check: pass.

Conditional gates not run: cargo fmt --all -- --check and
cargo -Zjson-target-spec test --quiet were not required because this contract
touched no Rust source or Cargo metadata.

No Pi 5 hardware run, lab-controller API action, hardwareTestLock acquisition,
boot publication, signature verification, authentication response,
authentication success, service success, session/channel or shell work, live
reachability claim, compatibility claim, broad expansion, or phase transition
was performed.

## Redaction review

Pass. Retained evidence contains only task ids, file paths, fixed labels,
public byte-length bounds, false/zero readiness counters, validation commands,
and classifications. It retains no authorized-key bytes, decoded public-key
blobs, request public-key blobs, fingerprints, digests, signatures,
user/operator identity, comments, peer strings, key-derived identifiers,
stable identifiers, session-id bytes, exchange hashes, hardware data, or boot
artifacts.

## Acceptance

Accepted as bounded authorized_keys parser/key-match policy contract.
selected_next_task=phase12-ssh-authorized-keys-parser-core-20260622.
