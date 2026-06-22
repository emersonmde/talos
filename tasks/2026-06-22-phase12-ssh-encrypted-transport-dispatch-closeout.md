# Phase 12.6 SSH encrypted transport dispatch closeout

Task id: phase12-ssh-encrypted-transport-dispatch-closeout-20260622

Status: accepted.

Classification: phase12-ssh-encrypted-transport-dispatch-closeout-accepted.

## Goal

Close out the encrypted transport dispatch mini-frontier by reconciling the
accepted contract, core implementation, smoke evidence, redaction policy,
deferred work, and the next pre-authentication planning boundary.

## Findings and Disposition

- fixed: reconciled the accepted dispatch contract, core implementation, and
  retained smoke/regression evidence into one local pre-authentication
  encrypted transport dispatch frontier.
- fixed: confirmed the accepted classifier may inspect only the first public SSH
  message-number byte from caller-owned decrypted payload bytes after the
  accepted encrypted-packet state is active.
- fixed: confirmed service-request message 5 and userauth-request message 50
  are routing diagnostics only. They do not accept service success,
  authentication success, sessions/channels, shell attachment, live
  reachability, compatibility, hardware behavior, or ssh-ready=true.
- fixed: confirmed empty payloads, inactive encrypted state, unsupported
  message numbers, post-NEWKEYS plaintext rejection, and packet crypto failure
  fail closed with fixed labels.
- fixed: reconciled redaction policy across the contract, core, and smoke tasks:
  durable evidence remains limited to fixed labels, public message numbers,
  small false/zero counters, validation commands, task ids, and
  classifications.
- deferred: service acceptance, userauth parsing/validation, authorized-key
  signature validation, account/user model, decrypted receive-loop integration,
  sessions/channels, PTY/process/shell attachment, live sockets, hardware
  reachability, public OpenSSH/POSIX/Linux compatibility, broad expansion, and
  phase transition.
- not-an-issue: no source behavior change is required for this closeout because
  the accepted core and retained smoke evidence already satisfy the local
  dispatch frontier.

## Reconciled Evidence

- phase12-ssh-encrypted-transport-dispatch-contract-20260622 accepted the
  post-NEWKEYS encrypted transport dispatch contract and fixed redaction rules.
- phase12-ssh-encrypted-transport-dispatch-core-20260622 accepted
  `classify_ssh_encrypted_transport_dispatch` and
  `SshEncryptedTransportDispatchReport` in `src/ssh_service_readiness.rs`.
- phase12-shell-ssh-encrypted-transport-dispatch-smoke-20260622 retained
  focused local smoke/regression evidence with
  `cargo -Zjson-target-spec test encrypted_transport_dispatch --quiet`, run
  with the documented QEMU PATH.
- Full local no_std regression evidence for the smoke task passed with
  `cargo -Zjson-target-spec test --quiet`, 761 tests.

## Accepted Frontier

The accepted frontier is local pre-authentication encrypted transport dispatch
classification only. The classifier may report fixed diagnostics for public SSH
message numbers 5 and 50 only under active encrypted-packet state and remains
not-ready on every path. ssh-ready remains false.

This closeout does not accept authentication success, service success,
authorized-key parsing or signature validation, account/user policy,
session/channel success, PTY allocation, shell attachment, process launch, live
socket connection, hardware/lab action, boot publication, live reachability,
OpenSSH/POSIX/Linux compatibility, broad expansion, or phase transition.

## Deferred Work

- define the pre-authentication service/userauth contract without accepting
  service success or authentication success.
- integrate decrypted packet receive-loop ownership after the service/userauth
  contract makes the retained inputs and labels explicit.
- later, plan authorized-key/signature validation, account/user policy,
  session/channel allocation, PTY/process/shell attachment, and live transport
  as separate bounded tasks with their own gates.

## Validation

- static task/docs/source review: pass.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check: pass.

No Rust source or Cargo metadata was touched, so cargo fmt and cargo test were
not required by this task's conditional gates.

## Redaction Review

Pass. Durable evidence retains only task ids, file paths, fixed labels, public
SSH message numbers 5 and 50, small false/zero readiness counters, validation
commands, and classifications. It retains no service names, user names, auth
method strings, public keys, signatures, packet payload bytes, ciphertext,
plaintext, MAC/tag material, keys, IV bytes, exchange hashes, shared secrets,
peer text/address, operator identity, key-derived identifiers, stable
transport/session identifiers, live hardware data, or boot artifacts.

## Result

Accepted.
selected_next_task=phase12-ssh-preauth-service-userauth-contract-20260622.
