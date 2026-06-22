# Phase 12.6 SSH publickey signature-verification closeout

Task id: phase12-ssh-publickey-verification-closeout-20260622

Status: accepted.

Classification: phase12-ssh-publickey-verification-closeout-accepted.

## Goal

Close out the prerequisite-only publickey verifier frontier by reconciling the
accepted contract, implementation, retained smoke evidence, validation,
redaction posture, deferred work, and the next bounded authentication step.

## Scope

- Reconciled the accepted publickey verification contract, verifier core, and
  retained source/unit smoke evidence.
- Updated project and roadmap docs to describe the accepted verifier frontier
  and unchanged false/zero SSH readiness boundary.
- Recorded deferred work for authentication response policy, account binding,
  authentication success, sessions/channels, shell attachment, live
  reachability, compatibility, hardware proof, broad expansion, and phase
  transition.
- Selected exactly one bounded follow-up task id for supervisor planning.

## Non-goals

No Rust source implementation, authentication response emission,
USERAUTH_PK_OK, SSH_MSG_USERAUTH_FAILURE, SSH_MSG_USERAUTH_SUCCESS,
partial-success behavior, authentication success, account/user authorization,
account database, sessions/channels, PTY/process/shell attachment, live socket
reachability, hardware/lab action, boot publication, OpenSSH/POSIX/Linux
compatibility claim, broad expansion, phase transition, or ssh-ready=true is
accepted.

Durable evidence must not retain session-id bytes, authorized-key bytes,
request/decoded public-key blobs, signature bytes, signed-data bytes,
fingerprints, digests, user names, comments, user/operator identity,
key-derived identifiers, stable identifiers, hardware data, or boot artifacts.

## Reviewed Inputs

- tasks/2026-06-22-phase12-ssh-publickey-verification-contract.md
- tasks/2026-06-22-phase12-ssh-publickey-verification-core.md
- tasks/2026-06-22-phase12-ssh-publickey-verification-smoke.md
- tasks/2026-06-22-phase12-ssh-authorized-keys-parser-closeout.md
- tasks/2026-06-22-phase12-ssh-userauth-session-id-closeout.md
- src/ssh_service_readiness.rs
- src/ssh_key_readiness.rs
- src/ssh_runtime_crypto.rs
- docs/src/project/phase12-networking-ssh.md
- docs/src/roadmap.md

## Findings and Disposition

- fixed: reconciled the verifier contract, source implementation, and smoke
  transcript into one accepted prerequisite-only frontier.
- fixed: project and roadmap docs now state that Talos has only local
  prerequisite-only ssh-ed25519 publickey signature verification over modeled
  decrypted USERAUTH_REQUEST/publickey payloads.
- fixed: the closeout explicitly preserves the false/zero readiness boundary:
  service-success=false, authentication-success=false, session-count=0,
  channel-count=0, shell-attached=false, live-reachability=false, and
  ssh-ready=false remain authoritative.
- fixed: selected the next bounded authentication step as a response-policy
  contract rather than source behavior. The supervisor must materialize an
  explicit queued task before any worker promotion.
- deferred: authentication response policy implementation, USERAUTH_PK_OK,
  USERAUTH_FAILURE, USERAUTH_SUCCESS, account binding, authentication success,
  sessions/channels, shell attachment, live reachability, hardware proof,
  compatibility, broad expansion, and phase transition.
- not-an-issue: no decision-log update is required because this closeout
  changes no accepted architecture policy beyond documenting the reconciled
  frontier and next planning need.

## Reconciled Evidence

- Contract:
  phase12-ssh-publickey-verification-contract-20260622 defines the
  ssh-ed25519-only verifier prerequisites, RFC 4252 signed-data shape,
  fixed-label failure families, lifetime/redaction rules, and
  prerequisite-only success boundary.
- Implementation:
  phase12-ssh-publickey-verification-core-20260622 implements the local
  prerequisite-only verifier in src/ssh_service_readiness.rs. It requires
  signature-present=true, an available private SshUserauthSessionIdentifier
  handle, a same-request authorized_keys prerequisite-only key match, an
  ssh-ed25519 request key, and an ssh-ed25519 signature. The temporary
  signed-data buffer is zeroized before return.
- Smoke:
  phase12-shell-ssh-publickey-verification-smoke-20260622 retains source/unit
  evidence for prerequisite-only success, signature rejected, malformed
  signature, unsupported algorithm, malformed key blob, authorized-key
  missing/no-match, missing session-id, signature-present=false/not-requested,
  and malformed signed-data paths.
- Boundary:
  successful verifier classification remains only a cryptographic prerequisite
  and cannot emit authentication responses, authenticate an account, allocate
  sessions/channels, attach a shell, claim live reachability or compatibility,
  advance a phase, or flip ssh-ready.

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

Talos now has a reconciled prerequisite-only ssh-ed25519 publickey
signature-verification frontier. The verifier operates only on a modeled
decrypted SSH_MSG_USERAUTH_REQUEST/publickey payload after the accepted private
session-id and authorized_keys key-match prerequisites.

This accepts only the cryptographic prerequisite. No authentication response
emission, USERAUTH_PK_OK, SSH_MSG_USERAUTH_FAILURE,
SSH_MSG_USERAUTH_SUCCESS, partial-success behavior, account binding,
authentication success, sessions/channels, shell attachment, live reachability,
hardware proof, OpenSSH/POSIX/Linux compatibility claim, broad expansion,
phase transition, or ssh-ready=true is accepted.

## Selected Next Task

selected_next_task=phase12-ssh-publickey-auth-response-policy-contract-20260622.

The selected follow-up is objective because the verifier frontier is closed
but response semantics remain explicitly deferred. The next task must be a
contract-only authentication response policy slice unless the supervisor
chooses a stricter pause. It must not implement responses, account binding,
authentication success, sessions/channels, shell attachment, live reachability,
hardware proof, compatibility, broad expansion, phase transition, or
ssh-ready=true without an explicit accepted contract.

Supervisor planning is required to materialize the follow-up task in the
durable queue; this closeout does not create or promote a new task.

## Acceptance

Accepted as bounded publickey signature-verification closeout.
selected_next_task=phase12-ssh-publickey-auth-response-policy-contract-20260622.
