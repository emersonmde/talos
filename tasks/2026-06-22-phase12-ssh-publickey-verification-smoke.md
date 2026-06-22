# Phase 12.6 SSH publickey signature-verification smoke

Task id: phase12-shell-ssh-publickey-verification-smoke-20260622

Status: accepted.

Classification: phase12-shell-ssh-publickey-verification-smoke-accepted.

## Goal

Record retained source/unit smoke evidence for the prerequisite-only
publickey signature verifier accepted by
phase12-ssh-publickey-verification-core-20260622, without broadening into
authentication responses, account authorization, sessions, shell attachment,
live reachability, compatibility, or ssh-ready=true.

## Scope

- Reviewed the accepted verifier implementation and unit coverage in
  src/ssh_service_readiness.rs.
- Re-ran verifier-focused source/unit coverage and the full no_std test suite
  under the configured QEMU runner.
- Retained only fixed labels, public byte-length fields, false/zero readiness
  counters, validation commands, task ids, paths, and classifications.
- Confirmed successful signature verification remains prerequisite-only and
  cannot emit authentication responses or flip readiness.

## Non-goals

No new verifier contract, broad refactor, authentication response emission,
USERAUTH_PK_OK, USERAUTH_FAILURE, USERAUTH_SUCCESS, authentication success,
account/user authorization, sessions/channels, shell attachment, live
reachability, hardware/lab action, OpenSSH/POSIX/Linux compatibility claim,
broad expansion, phase transition, or ssh-ready=true is accepted.

Durable evidence must not retain session-id bytes, authorized-key bytes,
request/decoded public-key blobs, signature bytes, signed-data bytes,
fingerprints, digests, user names, comments, user/operator identity,
key-derived identifiers, stable identifiers, hardware data, or boot artifacts.

## Reviewed Inputs

- tasks/2026-06-22-phase12-ssh-publickey-verification-contract.md
- tasks/2026-06-22-phase12-ssh-publickey-verification-core.md
- tasks/2026-06-22-phase12-ssh-authorized-keys-parser-closeout.md
- tasks/2026-06-22-phase12-ssh-userauth-session-id-closeout.md
- src/ssh_service_readiness.rs
- src/ssh_key_readiness.rs
- src/ssh_runtime_crypto.rs
- docs/src/project/phase12-networking-ssh.md
- docs/src/roadmap.md

## Findings and Disposition

- not-an-issue: the accepted core tests already cover prerequisite-only
  success, signature rejected, malformed signature, unsupported algorithm,
  key-blob malformed, authorized-key missing/no-match, missing session-id,
  signature-present=false, and malformed signed-data paths.
- not-an-issue: the verifier's success report retains only fixed labels,
  public byte lengths, and false/zero service/auth/session/channel/shell/
  reachability/readiness counters.
- fixed: retained this smoke/regression transcript as durable evidence so the
  verifier core can be closed out without reusing implementation evidence
  alone.
- deferred: authentication responses, account binding, authentication success,
  sessions/channels, shell attachment, live reachability, hardware proof,
  compatibility, broad expansion, phase transition, and ssh-ready=true.

## Smoke Evidence

The retained source/unit smoke set in src/ssh_service_readiness.rs covers:

- prerequisite-only success:
  publickey_verification_accepts_valid_signature_as_prerequisite_only.
- signature rejected:
  publickey_verification_rejects_bad_signature_without_authentication_success.
- malformed signature:
  publickey_verification_fails_closed_for_malformed_signature.
- unsupported algorithm and key-blob malformed:
  publickey_verification_fails_closed_for_unsupported_algorithm_and_key_blob.
- authorized-key missing/no-match and missing session-id:
  publickey_verification_requires_authorized_key_match_and_session_identifier.
- signature-present=false/not-requested and malformed signed-data:
  publickey_verification_fails_closed_without_signature_or_well_formed_signed_data.

The success case asserts the fixed prerequisite-only label plus
authentication-unimplemented, session-unimplemented, and not-ready labels. It
also asserts service-success=false, authentication-success=false,
session-count=0, channel-count=0, shell-attached=false,
live-reachability=false, and ssh-ready=false.

The fail-closed cases assert the fixed result/label families for signature
rejected, signature malformed, unsupported algorithm, malformed key blob,
authorized-key missing/no-match, missing session-id, signature not present, and
malformed signed data, while keeping authentication success and ssh-ready false.

## Validation

- static source/task/docs review: pass.
- cargo fmt --all -- --check: pass.
- cargo -Zjson-target-spec test publickey_verification --quiet: pass under the
  configured QEMU runner; the custom no_std runner executed 779 tests and the
  publickey_verification cases above passed.
- cargo -Zjson-target-spec test --quiet: pass under the configured QEMU
  runner; 779 no_std tests passed.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass with existing large search-index
  warning.
- git diff --cached --check: pass.

No Pi 5 hardware run, lab-controller API action, hardwareTestLock acquisition,
boot publication, authentication response, authentication success, service
success, session/channel or shell work, live reachability claim, compatibility
claim, broad expansion, or phase transition was performed.

## Redaction Review

Pass. Retained evidence contains only task ids, source/doc paths, fixed labels,
public byte-length field names, false/zero readiness counters, validation
commands, test names, and classifications. It retains no session-id bytes,
authorized-key bytes, request/decoded public-key blobs, signature bytes,
signed-data bytes, fingerprints, digests, user names, comments, user/operator
identity, key-derived identifiers, stable identifiers, hardware data, or boot
artifacts.

## Accepted Frontier

Talos has retained smoke/regression evidence for prerequisite-only
ssh-ed25519 publickey signature verification over the modeled decrypted
SSH_MSG_USERAUTH_REQUEST/publickey payload. The verifier remains a prerequisite
only: no authentication response emission, USERAUTH_PK_OK, USERAUTH_FAILURE,
USERAUTH_SUCCESS, account binding, authentication success, sessions/channels,
shell attachment, live reachability, compatibility, broad expansion, phase
transition, or ssh-ready=true is accepted.

## Selected Next Task

selected_next_task=phase12-ssh-publickey-verification-closeout-20260622.

The selected closeout task is objective because the verifier contract, core,
and retained smoke evidence now exist and need reconciliation before any later
authentication-response or account-binding planning.

## Acceptance

Accepted as bounded publickey signature-verification retained smoke evidence.
selected_next_task=phase12-ssh-publickey-verification-closeout-20260622.
