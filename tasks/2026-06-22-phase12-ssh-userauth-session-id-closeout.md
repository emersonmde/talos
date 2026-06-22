# Phase 12.6 SSH userauth session-id closeout

Task id: phase12-ssh-userauth-session-id-closeout-20260622

Status: accepted.

Classification: phase12-ssh-userauth-session-id-closeout-accepted.

## Goal

Reconcile the publickey-authentication prerequisite contract, accepted
userauth session-identifier core, focused smoke evidence, validation outputs,
redaction boundaries, deferred work, and remaining blockers before any
authorized_keys parsing or publickey-authentication implementation is planned.

## Scope

- Reviewed the prerequisite-blocked publickey authentication contract and the
  accepted session-identifier core/smoke evidence.
- Recorded the accepted session-id frontier as prerequisite plumbing only.
- Kept all publickey parsing, key matching, signature verification,
  authentication responses, authentication success, sessions, shell
  attachment, live reachability, hardware, compatibility, broad expansion, and
  phase transition work out of scope.

## Non-goals

No authorized_keys parser, authorized-key byte parsing, key matching,
operator/user binding, publickey blob retention, signature verification,
SSH_MSG_USERAUTH_SUCCESS, SSH_MSG_USERAUTH_FAILURE, SSH_MSG_USERAUTH_PK_OK,
partial-success behavior, authentication response emission, service success,
authentication success, session/channel allocation, PTY/process/shell
attachment, live socket connection, hardware/lab action, boot publication,
OpenSSH/POSIX/Linux compatibility claim, broad expansion, phase transition, or
ssh-ready=true is accepted.

No retained evidence may include session-id bytes, exchange hashes,
authorized-key bytes, public-key blobs, signatures, fingerprints, digests,
packet payload bytes, ciphertext, plaintext, keys, IV bytes, shared secrets,
peer raw input/address, peer/user strings, operator identity, key-derived
identifiers, stable transport/session identifiers, live hardware data, or boot
artifacts.

## Reconciled Evidence

- Publickey-auth contract:
  tasks/2026-06-22-phase12-ssh-publickey-auth-contract.md accepted that the
  parser can recognize ssh-connection/publickey shape only as diagnostic
  pre-auth input. It blocked real authentication on session-identifier
  exposure and authorized_keys parser/key-match policy.
- Session-id core: tasks/2026-06-22-phase12-ssh-userauth-session-id-core.md
  accepted the bounded private session-id handle on SshRuntimeKexReady,
  zeroization with ready KEX state, and fixed-label fail-closed diagnostics.
- Session-id smoke:
  tasks/2026-06-22-phase12-ssh-userauth-session-id-smoke.md accepted focused
  local regression evidence for available-after-KEX, repeated private-handle
  access, unavailable/malformed/over-limit diagnostics, and false/zero
  readiness counters.
- Source frontier: src/ssh_runtime_crypto.rs and
  src/ssh_service_readiness.rs contain the accepted private session-id handle
  and redacted diagnostics only. They do not implement authorized_keys parsing,
  publickey matching, signature verification, response emission, auth success,
  sessions, channels, or shell attachment.

## Findings and disposition

- fixed: the publickey-authentication prerequisite chain is reconciled through
  publickey contract, session-id core, and session-id smoke evidence.
- fixed: the accepted session-id surface is explicitly prerequisite-only and
  does not authorize publickey parsing, key matching, signature verification,
  authentication responses, or authentication success.
- fixed: redaction boundaries are preserved across contract/core/smoke
  evidence; retained evidence is limited to task ids, file paths, fixed labels,
  public byte lengths, test names, validation commands, and classifications.
- fixed: readiness remains false/zero: service-success=false,
  authentication-success=false, session-count=0, channel-count=0,
  shell-attached=false, live-reachability=false, and ssh-ready=false.
- deferred: authorized_keys parser/key-match policy is the next smallest
  explicit prerequisite. The supervisor must plan it with concrete scope,
  gates, docs, and evidence before worker promotion.
- deferred: publickey verifier implementation, authentication response policy,
  account/user model, authentication success, session/channel allocation, shell
  attachment, live reachability, hardware proof, compatibility, broad
  expansion, and phase transition remain future explicit tasks.
- not-an-issue: no Rust source change is required for this closeout because it
  reconciles accepted evidence rather than adding protocol behavior.

## Accepted frontier

Talos now has the bounded userauth-facing session-identifier prerequisite
closed out. Runtime KEX can retain and expose the first exchange hash through a
private ready-KEX session-id handle for later userauth verification, and
diagnostics remain fixed-label/redacted.

No authorized-key parsing, publickey matching, signature verification,
authentication response emission, authentication success, service success,
session/channel allocation, shell attachment, live reachability, hardware
action, compatibility claim, broad expansion, or phase transition is accepted.
Readiness remains service-success=false, authentication-success=false,
session-count=0, channel-count=0, shell-attached=false,
live-reachability=false, and ssh-ready=false.

## Selected next task

selected_next_task=null.

planningNeeded=true. The next smallest prerequisite is an authorized_keys
parser/key-match policy task, but no supervisor-owned queued task exists with
explicit scope, non-goals, acceptance criteria, validation gates, docs, and
evidence. The worker must not promote publickey matching, signature
verification, authentication responses, authentication success, sessions,
shell attachment, live reachability, hardware, compatibility, broad expansion,
or a phase transition from this closeout.

## Validation

- static task/docs/source review: pass.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check: pass.

Conditional gates not run: cargo fmt --all -- --check and
cargo -Zjson-target-spec test --quiet were not required because this closeout
touched no Rust source or Cargo metadata.

No Pi 5 hardware run, lab-controller API action, hardwareTestLock acquisition,
boot publication, service success, authentication success, session/channel or
shell work, live reachability claim, compatibility claim, broad expansion, or
phase transition was performed.

## Redaction review

Pass. Retained closeout evidence contains only task ids, file paths, fixed
label names, public byte lengths, false/zero readiness counters, validation
commands, and classifications. It retains no session-id bytes, exchange
hashes, authorized-key bytes, public-key blobs, signatures, fingerprints,
digests, peer/user strings, ciphertext/plaintext, keys, IVs, shared secrets,
peer raw input/address, operator identity, stable transport/session
identifiers, live hardware data, or boot artifacts.

## Acceptance

Accepted as bounded prerequisite closeout. selected_next_task=null.
planningNeeded=true.
