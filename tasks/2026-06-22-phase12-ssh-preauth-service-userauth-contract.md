# Phase 12.6 SSH pre-auth service/userauth contract

Task id: phase12-ssh-preauth-service-userauth-contract-20260622

Status: accepted.

Classification: phase12-ssh-preauth-service-userauth-contract-accepted.

## Goal

Define the smallest pre-authentication service/userauth boundary after the
accepted encrypted transport dispatch closeout, without implementing service
success, authentication success, sessions, shell attachment, live reachability,
hardware behavior, or public compatibility.

## Findings and Disposition

- fixed: selected the next real feature boundary as private parsing of
  caller-owned decrypted pre-authentication payloads already classified as
  service-request message 5 or userauth-request message 50.
- fixed: constrained service-request parsing to the SSH binary string that may
  equal the public literal ssh-userauth. Recognition is a routing diagnostic
  only; no SSH_MSG_SERVICE_ACCEPT response, service success, or readiness state
  is accepted.
- fixed: constrained userauth-request parsing to SSH binary string framing for
  user name, service name, and method name. The first implementation may compare
  the service name with ssh-connection and classify the method name as
  recognized-publickey or unsupported without retaining any string bytes.
- fixed: defined fail-closed behavior for malformed string lengths, trailing or
  missing fields, unsupported service names, userauth before the modeled service
  request prerequisite, unsupported methods, and every non-dispatch or crypto
  failure inherited from the encrypted transport dispatch frontier.
- fixed: restricted durable evidence to fixed labels, public message numbers,
  public string-length counters/bounds, false/zero readiness counters,
  validation commands, task ids, and classifications.
- deferred: actual SSH_MSG_SERVICE_ACCEPT emission, username/account lookup,
  public-key blob parsing, signature validation, authorized-key matching,
  partial/success/failure authentication responses, session/channel allocation,
  PTY/process/shell attachment, live sockets, OpenSSH/POSIX/Linux compatibility,
  hardware reachability, broad expansion, and phase transition.
- not-an-issue: no source change is required for this contract because the
  accepted dispatch core currently stops at fixed routing labels and does not
  parse SSH binary strings.

## Contract

The next implementation boundary is a private local pre-authentication parser
over caller-owned decrypted payload bytes that have already passed the accepted
encrypted transport dispatch classifier. It must consume only message 5
service-request and message 50 userauth-request payload shapes. It must not
retain packet payload bytes, parsed user names, service strings, method strings,
public-key blobs, signatures, ciphertext, plaintext, MAC/tag material, keys, IV
bytes, exchange hashes, shared secrets, peer text/address, operator identity,
key-derived identifiers, stable transport/session identifiers, live hardware
data, or boot artifacts.

The service-request path may parse one SSH binary string after message number 5
and compare it with the public literal ssh-userauth. A match may set only a
private modeled prerequisite and emit fixed routing diagnostics. It must not
emit SSH_MSG_SERVICE_ACCEPT, mark service-success=true, authenticate a user,
allocate a session, attach a shell, make ssh-ready true, claim live
reachability, or claim OpenSSH compatibility. Malformed string framing,
trailing bytes, missing bytes, and service names other than ssh-userauth fail
closed.

The userauth-request path may parse, in order, the SSH binary strings after
message number 50 for user name, service name, and method name. Durable evidence
must not retain the string contents. The parser may compare the service name
with the public literal ssh-connection and the method name with the public
literal publickey. A recognized publickey method is still a routing diagnostic
only: public-key blob parsing, boolean signature-present handling, signature
verification, authorized-key lookup, authentication success/failure packets,
account identity, sessions, and shell behavior remain unimplemented. Userauth
before the modeled service-request prerequisite, unsupported service names,
unsupported methods, malformed string framing, and extra/missing required bytes
must fail closed.

The first implementation should keep the boundary private to
ssh_service_readiness and should expose only fixed-label status plus small
public counters such as parsed string count and bounded payload length. It may
add focused unit tests or a task-owned smoke command, but it must not connect to
live sockets, emit protocol responses, or authenticate users.

Required fixed labels for the next implementation are:

- sshservicediag-preauth-service-request-modeled
- sshservicediag-preauth-service-userauth-recognized
- sshservicediag-preauth-service-unsupported
- sshservicediag-preauth-service-malformed
- sshservicediag-preauth-userauth-request-modeled
- sshservicediag-preauth-userauth-service-recognized
- sshservicediag-preauth-userauth-service-unsupported
- sshservicediag-preauth-userauth-method-publickey-modeled
- sshservicediag-preauth-userauth-method-unsupported
- sshservicediag-preauth-userauth-before-service
- sshservicediag-preauth-userauth-malformed
- sshservicediag-authentication-unimplemented
- sshservicediag-session-unimplemented
- sshservicediag-not-ready

## Accepted Frontier

The accepted frontier remains private local SSH pre-authentication modeling.
Talos may now plan a bounded core that parses caller-owned service-request and
userauth-request payload shapes after encrypted transport dispatch, reports
fixed diagnostics for recognized and fail-closed cases, and keeps
authentication-success=false, service-success=false, session-count=0,
channel-count=0, shell-attached=false, reachability-accepted=false, and
ssh-ready=false authoritative.

No service success, authentication success, authorized-key parsing/signature
validation, account/user model, session/channel success, PTY/process/shell
attachment, live socket connection, hardware/lab action, boot publication,
OpenSSH/POSIX/Linux compatibility, broad expansion, or phase transition is
accepted.

## Selected Next Task

The objective bounded implementation follow-up is
phase12-ssh-preauth-service-userauth-core-20260622. Supervisor planning is
required to enqueue that task with explicit scope, gates, docs, and evidence
before the worker may promote it.

## Evidence

- phase12-ssh-encrypted-transport-dispatch-closeout-20260622: accepted local
  pre-authentication encrypted transport dispatch frontier, selected this
  contract, and preserved ssh-ready=false.
- src/ssh_service_readiness.rs static review: dispatch currently identifies
  service-request message 5 and userauth-request message 50 by public message
  number only, then keeps authentication/session/shell/readiness false.
- docs/src/project/phase12-networking-ssh.md and docs/src/roadmap.md updated
  to record the accepted pre-authentication service/userauth contract frontier.

## Validation

- static task/docs/source review: pass.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check: pass.

No Rust source or Cargo metadata was touched, so cargo fmt and cargo test were
not required by this task's conditional gates. No Pi 5 hardware run,
lab-controller API action, hardwareTestLock acquisition, boot publication,
service success, authentication success, session/channel/shell work, live
reachability claim, compatibility claim, broad expansion, or phase transition
was performed.

## Redaction Review

Pass. This task retained only task ids, file paths, fixed labels, public SSH
message numbers, public literal names, public string-field order, small public
counters/bounds, validation command names, and classifications. It retained no
packet payload bytes, parsed user names, service strings from peers, method
strings from peers, public-key blobs, signatures, ciphertext, plaintext, keys,
IV bytes, tags, exchange hashes, shared secrets, peer raw input, peer address,
operator identity, key-derived identifiers, stable transport/session
identifiers, live hardware data, or boot artifacts.

## Result

Accepted. selected_next_task=phase12-ssh-preauth-service-userauth-core-20260622.
