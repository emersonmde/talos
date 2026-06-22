# Phase 12.6 SSH authorized_keys parser/key-match smoke

Task id: phase12-ssh-authorized-keys-parser-smoke-20260622

Status: accepted.

Classification: phase12-ssh-authorized-keys-parser-smoke-accepted.

## Goal

Retain focused smoke/regression evidence for the accepted authorized_keys
parser/key-match boundary without adding signature verification,
authentication response emission, authentication success, sessions, shell
attachment, live reachability, hardware behavior, compatibility, broad
expansion, or a phase transition.

## Scope

- Exercise the accepted parser/key-match boundary through focused local
  source/unit regression evidence.
- Cover accepted key match, fail-closed non-match, missing/invalid/oversized
  metadata, blank/comment-only input, unsupported option or algorithm,
  malformed line shape, and malformed public-key blob.
- Retain only fixed labels, file paths, test names, public byte-length values,
  line counts, false/zero readiness counters, validation commands, task ids,
  and classifications.

## Non-goals

- No new Rust protocol behavior, signature verification, USERAUTH_PK_OK,
  SSH_MSG_USERAUTH_SUCCESS, SSH_MSG_USERAUTH_FAILURE, partial-success
  behavior, authentication response emission, authentication success,
  account/user model, sessions/channels, PTY/process/shell attachment, live
  socket connection, hardware/lab action, boot publication,
  OpenSSH/POSIX/Linux compatibility claim, broad expansion, phase transition,
  or ssh-ready=true.
- No retention of authorized-key bytes, public-key blobs, fingerprints,
  digests, signatures, comments, user/operator identity, key-derived
  identifiers, stable identifiers, session-id bytes, live hardware data, or
  boot artifacts.

## Findings and disposition

- fixed: focused local smoke evidence covers accepted ssh-ed25519 key-match
  through the accepted /etc/talos/ssh/authorized_keys read-only VFS boundary.
- fixed: focused local smoke evidence covers fail-closed non-match,
  missing/invalid/oversized metadata, blank/comment-only input, unsupported
  option or algorithm, malformed line shape, and malformed public-key blob.
- fixed: readiness false/zero counters remain asserted by the focused
  parser/key-match tests: authentication-success=false,
  match-prerequisite-only is true only for the match prerequisite,
  matched-public-key length is omitted for failures, and ssh-ready=false.
- not-an-issue: this smoke task adds no new protocol behavior; it records the
  accepted core's focused source/unit evidence as the retained smoke
  transcript.
- deferred: publickey signature verification, authentication response policy,
  USERAUTH_PK_OK, USERAUTH_SUCCESS/FAILURE, account/user semantics,
  sessions/channels, shell attachment, live reachability, compatibility,
  hardware proof, and phase transition remain future explicit work.

## Evidence

- static source/task/docs review: src/ssh_key_readiness.rs focused tests
  include authorized_key_parser_matches_only_as_prerequisite,
  authorized_key_parser_reports_no_match_without_readiness,
  authorized_key_parser_ignores_blank_and_comment_only_files_but_fails_closed,
  authorized_key_parser_rejects_missing_invalid_and_oversized_metadata, and
  authorized_key_parser_rejects_unsupported_and_malformed_lines.
- unit/QEMU-substitute: cargo -Zjson-target-spec test authorized_key --quiet
  passed with QEMU on PATH; the harness completed 773 no_std tests including
  the focused authorized_key parser/key-match cases.
- diff check: git diff --check passed.
- docs: /home/node/.cargo/bin/mdbook build passed with the existing large
  search-index warning.
- pre-commit diff check: git diff --cached --check passed.

Conditional gates not run: cargo fmt --all -- --check and full
cargo -Zjson-target-spec test --quiet were not required because this smoke
task touched no Rust source or Cargo metadata after the accepted core
evidence.

## Redaction review

Pass. Retained smoke evidence contains only task ids, file paths, public test
names, fixed label names, public byte lengths, line counts, false/zero
readiness counters, validation commands, and classifications. It retains no
authorized-key bytes, decoded public-key blobs, request public-key blobs,
fingerprints, digests, signatures, comments, user/operator identity,
key-derived identifiers, stable identifiers, session-id bytes, live hardware
data, or boot artifacts.

## Accepted frontier

Talos has local smoke/regression evidence for the bounded authorized_keys
parser/key-match prerequisite. Key match remains only a prerequisite for later
publickey signature verification; authentication-success=false,
service-success=false, session-count=0, channel-count=0, shell-attached=false,
live-reachability=false, and ssh-ready=false remain authoritative.

selected_next_task=phase12-ssh-authorized-keys-parser-closeout-20260622.
