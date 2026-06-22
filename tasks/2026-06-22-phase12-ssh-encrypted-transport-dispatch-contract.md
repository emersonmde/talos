# Phase 12.6 SSH encrypted transport dispatch contract

Task id: phase12-ssh-encrypted-transport-dispatch-contract-20260622

Status: accepted.

Classification: phase12-ssh-encrypted-transport-dispatch-contract-accepted.

## Goal

Define the smallest post-NEWKEYS encrypted packet dispatch boundary after the
accepted local NEWKEYS/encrypted-packet diagnostic, without implementing user
authentication, session/channel success, shell attachment, live reachability, or
public compatibility.

## Findings and Disposition

- fixed: reconciled the accepted NEWKEYS closeout into a narrow next boundary:
  decrypted post-NEWKEYS packet payloads may be classified by their first SSH
  message number only after both NEWKEYS directions are active.
- fixed: defined the first dispatch state as pre-authentication only. It may
  recognize service-request and userauth-request message classes for routing
  diagnostics, but every path remains not-ready and authentication success
  remains false.
- fixed: defined fail-closed handling for malformed empty payloads,
  unsupported message numbers, pre-NEWKEYS dispatch attempts, post-NEWKEYS
  plaintext I/O attempts, and packet crypto failures.
- fixed: restricted durable evidence to fixed labels, public message numbers,
  small counters, public bounds, validation commands, task ids, and
  classifications.
- deferred: actual decrypted-packet receive integration, userauth service
  acceptance, authorized-key signature validation, channel/session success,
  shell attachment, live sockets, OpenSSH/POSIX/Linux compatibility, hardware
  reachability, broad expansion, and phase transition.
- not-an-issue: no source change is required for this contract because the
  accepted packet crypto core currently exposes only a local diagnostic, not a
  retained decrypted inbound payload API.

## Contract

The next implementation boundary is a private local dispatch classifier over a
caller-owned decrypted SSH binary packet payload. The classifier may inspect
only the first payload byte as the SSH message number. It must not retain the
payload, ciphertext, plaintext, MAC/tag material, keys, IV bytes, exchange hash,
shared secret, signature, peer text/address, operator identity, key-derived
identifier, stable transport/session identifier, or live hardware data.

The initial state is pre-authentication. The accepted public message-number
classes for the first implementation are:

- SSH_MSG_SERVICE_REQUEST, message number 5: may be classified as a
  service-request dispatch only.
- SSH_MSG_USERAUTH_REQUEST, message number 50: may be classified as a
  userauth-request dispatch only.

Both classes remain fail-closed diagnostics. They do not accept service success,
user authentication, authorized-key parsing/signature validation, session
allocation, channel success, shell attachment, ssh-ready=true, OpenSSH
compatibility, live reachability, or hardware behavior.

Malformed dispatch includes an empty decrypted payload or a packet whose
encrypted-packet state is not active. Unsupported dispatch includes every
message number outside the two accepted pre-authentication classes. Plaintext
I/O after either NEWKEYS direction has activated remains a crypto failure, not a
fallback dispatch path. Any packet crypto failure remains authoritative and must
prevent dispatch.

The fixed labels for the next implementation should be:

- sshservicediag-encrypted-transport-dispatch-modeled
- sshservicediag-encrypted-transport-preauth-state
- sshservicediag-encrypted-transport-service-request
- sshservicediag-encrypted-transport-userauth-request
- sshservicediag-encrypted-transport-message-unsupported
- sshservicediag-encrypted-transport-packet-malformed
- sshservicediag-encrypted-transport-plaintext-rejected
- sshservicediag-authentication-unimplemented
- sshservicediag-session-unimplemented
- sshservicediag-not-ready

## Accepted Frontier

The accepted frontier remains private local SSH transport modeling only. Talos
may now plan a bounded encrypted-transport dispatch core that consumes
caller-owned decrypted payload bytes after the accepted NEWKEYS/encrypted packet
state is active and reports fixed pre-authentication dispatch labels. The
frontier does not accept authentication success, session/channel success, shell
attachment, live socket reachability, public OpenSSH/POSIX/Linux compatibility,
hardware/lab action, boot publication, broad expansion, or phase transition.
ssh-ready remains false.

## Selected Next Task

The objective bounded implementation follow-up is
phase12-ssh-encrypted-transport-dispatch-core-20260622. Supervisor planning is
required to enqueue that task with explicit scope, gates, docs, and evidence
before the worker may promote it.

## Validation

- static task/docs/source review: pass.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass with existing large search-index
  warning.
- git diff --cached --check: pass.

No Rust source or Cargo metadata was touched, so cargo fmt and cargo test were
not required by this task's conditional gates.

## Redaction Review

Pass. This contract retains only task ids, file paths, fixed labels, public SSH
message numbers, public bounds, public state names, validation commands, and
classifications. It retains no packet payload bytes, ciphertext, plaintext,
keys, IV bytes, tags, exchange hashes, shared secrets, signatures, peer raw
input, peer address/text, operator identity, key-derived identifiers, stable
transport/session identifiers, live hardware data, or boot artifacts.

## Non-Goals Preserved

No authentication/session/shell success, authorized-key parsing/signature
validation, live socket connection, hardware/lab action, boot publication, live
reachability claim, OpenSSH/POSIX/Linux compatibility claim, ssh-ready=true
claim, broad expansion, or phase transition is accepted.

## Result

Accepted. selected_next_task=phase12-ssh-encrypted-transport-dispatch-core-20260622.
