# Phase 12.6 SSH pre-auth service/userauth closeout

Task id: phase12-ssh-preauth-service-userauth-closeout-20260622

Status: accepted

Classification: phase12-ssh-preauth-service-userauth-closeout-accepted.

## Goal

Reconcile the accepted pre-authentication service/userauth contract, core, and
smoke evidence without accepting service success, authentication success,
sessions, channels, shell attachment, live reachability, hardware behavior,
compatibility, broad expansion, or a phase transition.

## Scope

- Reconcile accepted contract, implementation, smoke/regression evidence,
  validation outputs, redaction posture, and deferred work.
- Confirm the accepted frontier is local pre-authentication parser modeling
  only.
- Select exactly one bounded next authentication task only if the accepted
  evidence makes that next boundary objective.

## Non-goals

- No new parser behavior, SSH_MSG_SERVICE_ACCEPT emission, authentication
  response emission, account/user model, authorized-key parsing, signature
  validation, session/channel allocation, PTY/process/shell attachment, live
  socket connection, hardware/lab action, boot publication, reachability
  claim, compatibility claim, broad expansion, phase transition, or
  ssh-ready=true.
- No retention of packet payload bytes, parsed usernames, peer service/method
  strings, public-key blobs, signatures, ciphertext, plaintext, keys, IVs,
  exchange hashes, shared secrets, peer addresses, operator identity, stable
  transport/session identifiers, live hardware data, or boot artifacts.

## Findings

- fixed: reconciled the contract/core/smoke chain. The accepted boundary is a
  private parser over caller-owned decrypted service-request and userauth
  payloads after encrypted transport dispatch, with only fixed diagnostic
  labels and false/zero readiness state retained.
- fixed: confirmed recognized ssh-userauth service-request remains a modeled
  prerequisite only; it does not send SSH_MSG_SERVICE_ACCEPT or mark
  service-success=true.
- fixed: confirmed recognized ssh-connection/publickey userauth remains a
  diagnostic-only shape after the modeled service prerequisite; it does not
  parse public-key blobs, inspect signatures, validate authorized keys, or mark
  authentication-success=true.
- fixed: reconciled fail-closed coverage for userauth-before-service,
  unsupported service/method, malformed/missing/trailing fields, inactive
  encrypted state, post-NEWKEYS plaintext rejection, crypto failure, and
  unsupported dispatch paths.
- fixed: selected the next objective bounded task as
  phase12-ssh-publickey-auth-contract-20260622, because the parser now exposes
  the first publickey-method boundary while all actual authentication semantics
  remain unaccepted.
- deferred: service accept responses, authentication response policy,
  authorized-key source and parsing, public-key blob/signature handling,
  account/user semantics, signature verification, authentication success,
  sessions/channels, shell attachment, live reachability, OpenSSH/POSIX/Linux
  compatibility, hardware proof, broad expansion, and phase transition.
- not-an-issue: no Rust source change is required for this closeout; the
  accepted core and smoke evidence already exercise the bounded parser frontier.

## Reconciled Evidence Map

- phase12-ssh-preauth-service-userauth-contract-20260622: accepted the private
  service/userauth parser contract, fixed labels, fail-closed cases, redaction
  policy, and selected the bounded core implementation.
- phase12-ssh-preauth-service-userauth-core-20260622: accepted
  src/ssh_service_readiness.rs parser/report types and unit coverage for
  recognized service-request, recognized publickey shape after the modeled
  service prerequisite, and fail-closed inherited dispatch paths.
- phase12-shell-ssh-preauth-service-userauth-smoke-20260622: retained focused
  local smoke/regression evidence from the preauth_service_userauth test
  filter and full test pass, with service-success=false,
  authentication-success=false, session-count=0, channel-count=0,
  shell-attached=false, and ssh-ready=false.
- docs/src/project/phase12-networking-ssh.md and docs/src/roadmap.md now
  record this closeout frontier and the selected next bounded contract task.

## Accepted Frontier

The accepted frontier is local pre-authentication service/userauth parser
modeling only. Service-request message 5 may recognize ssh-userauth as a
modeled prerequisite, and userauth-request message 50 may recognize
ssh-connection/publickey shape after that prerequisite. All positive paths are
diagnostic-only. service-success=false, authentication-success=false,
session-count=0, channel-count=0, shell-attached=false, live reachability=false,
and ssh-ready=false remain authoritative.

No SSH service accept response, authentication success/failure response,
authorized-key parsing, signature validation, account/user model,
session/channel allocation, PTY/process/shell attachment, live socket behavior,
hardware/lab action, boot publication, OpenSSH/POSIX/Linux compatibility,
broad expansion, or phase transition is accepted.

## Selected Next Task

selected_next_task=phase12-ssh-publickey-auth-contract-20260622.

The next task must remain contract-only unless supervisor planning explicitly
assigns implementation scope. It should define the smallest publickey
authentication boundary after the accepted service/userauth parser, including
signature-present policy, authorized-key prerequisites, fail-closed response
labels, and evidence redaction rules, without accepting authentication success
or session/shell behavior.

## Evidence

- static task/docs/source review: pass; accepted task records and
  src/ssh_service_readiness.rs show only private parser modeling and false/zero
  readiness state.
- docs update: docs/src/project/phase12-networking-ssh.md and
  docs/src/roadmap.md updated for the closeout frontier and next bounded task.
- no Rust source or Cargo metadata changed, so cargo fmt and cargo test were
  not required by the conditional validation gates.

## Validation

- static task/docs/source review: pass.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check: pass.

No Pi 5 hardware run, lab-controller API action, hardwareTestLock acquisition,
boot publication, service success, authentication success, session/channel or
shell work, live reachability claim, compatibility claim, broad expansion, or
phase transition was performed.

## Redaction Review

Pass. This closeout retained only task ids, file paths, test names, fixed
labels, public SSH message numbers and protocol literal names, public field
order descriptions, small public counters/bounds, validation commands, and
classifications. It retained no packet payload bytes, parsed usernames,
peer-selected service or method strings, public-key blobs, signatures,
ciphertext, plaintext, MAC/tag material, keys, IV bytes, exchange hashes,
shared secrets, peer raw input/address, operator identity, key-derived
identifiers, stable transport/session identifiers, live hardware data, or boot
artifacts.

## Acceptance

Accepted. selected_next_task=phase12-ssh-publickey-auth-contract-20260622.
