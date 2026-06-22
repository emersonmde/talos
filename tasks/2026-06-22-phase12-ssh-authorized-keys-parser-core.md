# Phase 12.6 SSH authorized_keys parser/key-match core

Task id: phase12-ssh-authorized-keys-parser-core-20260622

Status: accepted.

Classification: phase12-ssh-authorized-keys-parser-core-accepted.

## Goal

Implement the bounded authorized_keys parser/key-match boundary accepted by
phase12-ssh-authorized-keys-parser-policy-contract-20260622, while keeping
signature verification, authentication responses, authentication success,
sessions, shell attachment, live reachability, compatibility, broad expansion,
phase transition, and ssh-ready=true unaccepted.

## Scope

- Added source behavior in src/ssh_key_readiness.rs for the accepted
  /etc/talos/ssh/authorized_keys read-only VFS boundary.
- Preserved the existing metadata guard: the file must be regular, readable,
  non-empty, and no larger than the accepted public byte bound before parsing.
- Implemented the first option-free ssh-ed25519 line parser with blank/comment
  skipping, trailing-comment discard, unsupported/malformed fail-closed labels,
  and in-memory-only publickey blob comparison.
- Added focused unit coverage for match, non-match, comment/blank-only,
  missing/invalid/oversized metadata, unsupported option, unsupported
  algorithm, malformed line, and malformed blob paths.
- Kept parser tests redacted by generating authorized-key fixture bytes at
  runtime from an existing public fixture and by mutating request blobs only in
  memory.

## Non-goals

No signature verification, USERAUTH_PK_OK, SSH_MSG_USERAUTH_SUCCESS,
SSH_MSG_USERAUTH_FAILURE, partial-success behavior, authentication success,
account database, sessions/channels, PTY/process/shell attachment, live socket
reachability, hardware/lab action, OpenSSH/POSIX/Linux compatibility claim,
broad expansion, phase transition, or ssh-ready=true is accepted.

No durable evidence retains authorized-key bytes, decoded public-key blobs,
request public-key blobs, fingerprints, digests, signatures, comments,
user/operator identity, key-derived identifiers, stable identifiers, hardware
data, boot artifacts, session-id bytes, or exchange hashes.

## Implementation

The new match surface is match_authorized_key_public_blob. It first applies
the accepted AuthorizedKeyMaterialMetadata boundary, then parses only accepted
option-free ssh-ed25519 lines. Each accepted line is decoded into an SSH
publickey blob and compared with the caller-owned request blob in memory.

The retained report exposes only fixed labels, line counts, public byte-length
values, match/non-match state, authentication-success=false, and ssh-ready=false.
The existing metadata-only sshkeydiag path is unchanged and remains valid as a
separate readiness diagnostic.

## Findings and disposition

- fixed: authorized_keys material can now be parsed through the accepted
  read-only VFS boundary instead of stopping at metadata presence.
- fixed: key matching is limited to in-memory ssh-ed25519 publickey blob
  comparison and reports only a prerequisite match label.
- fixed: blank and comment-only files fail closed with a fixed empty/comment
  label rather than clearing readiness.
- fixed: missing, invalid, and oversized VFS metadata fail closed before key
  bytes are interpreted.
- fixed: key options, unsupported algorithms, malformed lines, and malformed
  blobs fail closed with distinct fixed-label families.
- fixed: focused tests prove match/non-match and fail-closed paths while
  authentication-success and ssh-ready remain false.
- fixed: source fixtures avoid committed authorized-key lines and request
  publickey blobs; generated test material is in-memory only.
- deferred: signature verification, authentication response emission,
  authentication success, account/user binding, sessions/channels, shell
  attachment, live reachability, hardware proof, compatibility, broad
  expansion, and phase transition.
- not-an-issue: the existing sshkeydiag metadata output remains metadata-only;
  this task adds a private prerequisite parser/match API rather than promoting
  user-visible readiness.

## Evidence

- source/unit evidence: src/ssh_key_readiness.rs implements
  match_authorized_key_public_blob with focused authorized_key tests.
- cargo fmt --all -- --check: pass.
- cargo -Zjson-target-spec test authorized_key --quiet: pass.
- cargo -Zjson-target-spec test --quiet: initial environment run failed
  because qemu-system-aarch64 was not on PATH; rerun with the documented QEMU
  path passed, 773 tests.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass with existing large search-index
  warning.
- git diff --cached --check: pass.

## Redaction review

Pass. Retained evidence is limited to fixed labels, source paths, task ids,
public byte-length values, line counts, validation commands, and
classifications. Parser tests generate authorized-key fixture material at
runtime and mutate request blobs only in memory. Durable evidence retains no
authorized-key bytes, decoded public-key blobs, request public-key blobs,
fingerprints, digests, signatures, comments, user/operator identity,
key-derived identifiers, stable identifiers, hardware data, boot artifacts,
session-id bytes, or exchange hashes.

## Accepted frontier

Talos now has the bounded authorized_keys parser/key-match prerequisite for
publickey authentication. A matched key clears only the key-match prerequisite
for later signature verification. It does not authorize an account, emit an
authentication response, accept authentication success, allocate sessions or
channels, attach a shell, prove live reachability, claim compatibility,
broaden scope, transition phase, or set ssh-ready=true.

service-success=false, authentication-success=false, session-count=0,
channel-count=0, shell-attached=false, live-reachability=false, and
ssh-ready=false remain authoritative.

## Selected next task

selected_next_task=phase12-ssh-authorized-keys-parser-smoke-20260622.

The selected smoke task is objective because the parser/key-match source and
focused unit coverage now exist, but a separate smoke/regression record is
required before parser closeout and later publickey signature-verification
planning.

## Acceptance

Accepted as bounded authorized_keys parser/key-match core.
selected_next_task=phase12-ssh-authorized-keys-parser-smoke-20260622.
