# Phase 12.6 SSH publickey authentication success account closeout

Task id: phase12-ssh-publickey-auth-success-account-closeout-20260622

Status: accepted.

Classification: phase12-ssh-publickey-auth-success-account-closeout-accepted.

## Goal

Close out the publickey USERAUTH_SUCCESS single-account policy frontier by
reconciling the accepted contract, implementation, retained smoke evidence,
docs, validation, redaction posture, deferred work, and next bounded SSH step.

## Scope

- Reconciled the accepted account-policy contract, source implementation, and
  retained smoke evidence.
- Updated the Phase 12 project notes and roadmap to state the accepted local
  modeled USERAUTH_SUCCESS/account-policy frontier.
- Kept sessions, channels, shell attachment, live reachability, hardware proof,
  OpenSSH/POSIX/Linux compatibility, broad expansion, phase transition, and
  ssh-ready=true explicitly unaccepted.
- Selected the next bounded contract task only after the accepted frontier was
  reconciled.

## Non-goals

No Rust source implementation, session/channel allocation, PTY/process/shell
attachment, live sockets, hardware/lab action, boot publication,
OpenSSH/POSIX/Linux compatibility claim, broad expansion, phase transition, or
ssh-ready=true is accepted by this closeout.

## Reviewed Inputs

- tasks/2026-06-22-phase12-ssh-publickey-auth-success-account-policy-contract.md.
- tasks/2026-06-22-phase12-ssh-publickey-auth-success-account-core.md.
- tasks/2026-06-22-phase12-ssh-publickey-auth-success-account-smoke.md.
- tasks/evidence/2026-06-22-ssh-publickey-auth-success-account-smoke/qemu-shell-ssh-publickey-auth-success-account-smoke.log.
- src/ssh_service_readiness.rs account-success classifier and focused tests.
- docs/src/project/phase12-networking-ssh.md.
- docs/src/roadmap.md.

## Findings and Disposition

- fixed: reconciled the contract/core/smoke chain into a single accepted
  frontier: local modeled publickey USERAUTH_SUCCESS is accepted only for the
  reserved Talos account with accepted encrypted userauth shape, private
  session-id handle, same-request authorized_keys key-match prerequisite,
  prerequisite-only verifier success, enabled account policy, and signed-valid
  ssh-ed25519 request.
- fixed: documented that account mismatch, disabled or missing policy,
  missing service/session/key-match/verifier prerequisites, unsigned probes
  outside the PK_OK slice, invalid or malformed signatures, unauthorized keys,
  malformed requests, unsupported algorithms, and redaction-sensitive paths
  fail closed with USERAUTH_FAILURE and fixed labels.
- fixed: preserved the readiness boundary: authentication-success=true is local
  and modeled only, while service-success=false, session-count=0,
  channel-count=0, shell-attached=false, live-reachability=false, and
  ssh-ready=false remain authoritative.
- not-an-issue: the source implementation already keeps request material,
  session-id bytes, key material, signatures, fingerprints, digests, peer
  strings, hardware data, and boot artifacts out of durable evidence; only
  fixed labels, public message numbers, public length/count fields, paths,
  validation commands, task ids, and classifications are retained.
- deferred: session/channel allocation, channel-open request handling,
  PTY/process/shell attachment, exec/subsystem requests, live sockets,
  hardware proof, OpenSSH/POSIX/Linux compatibility, broad account model, phase
  transition, and ssh-ready=true.

## Reconciled Evidence

- Contract evidence:
  tasks/2026-06-22-phase12-ssh-publickey-auth-success-account-policy-contract.md.
- Source/unit evidence:
  tasks/2026-06-22-phase12-ssh-publickey-auth-success-account-core.md.
- Retained smoke evidence:
  tasks/2026-06-22-phase12-ssh-publickey-auth-success-account-smoke.md and
  tasks/evidence/2026-06-22-ssh-publickey-auth-success-account-smoke/qemu-shell-ssh-publickey-auth-success-account-smoke.log.
- Static source review:
  src/ssh_service_readiness.rs confirms authentication_success() follows only
  userauth_success(), while service_success(), session_count(),
  channel_count(), shell_attached(), reachability_accepted(), and ssh_ready()
  remain false/zero for this report type.

## Validation

- static task/docs/source review: pass.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check: pass.

No Rust source or Cargo metadata was touched, so cargo fmt and cargo test gates
are not required for this closeout.

No Pi 5 hardware run, lab-controller API action, hardwareTestLock acquisition,
boot publication, service/session/channel/shell work, live reachability claim,
compatibility claim, broad expansion, or phase transition was performed.

## Redaction Review

Pass. The closeout retains only task ids, source/doc/script/evidence paths,
fixed labels, public SSH message numbers, public byte-length/count field names,
validation commands, test names, readiness booleans/counters, and
classifications. It retains no session-id bytes, authorized-key bytes, request
or decoded public-key blobs, signature bytes, signed-data bytes, fingerprints,
digests, private user/operator identity, peer strings, key-derived identifiers,
stable identifiers, hardware data, or boot artifacts.

## Acceptance

Accepted as the closeout for the bounded local modeled publickey
USERAUTH_SUCCESS single-account policy frontier.

selected_next_task=phase12-ssh-session-channel-open-contract-20260622.
