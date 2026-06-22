# Phase 12.6 SSH encrypted transport dispatch core

Task id: phase12-ssh-encrypted-transport-dispatch-core-20260622

Status: accepted.

Classification: phase12-ssh-encrypted-transport-dispatch-core-accepted.

## Goal

Implement the smallest local post-NEWKEYS encrypted transport dispatch
classifier, without accepting authentication, sessions, shell attachment, live
reachability, public compatibility, hardware behavior, or a phase transition.

## Findings and Disposition

- fixed: added a no_std-compatible local dispatch boundary in
  `src/ssh_service_readiness.rs` for caller-owned decrypted payloads after the
  accepted encrypted-packet state is active.
- fixed: classified only the first public SSH message number byte. Message 5
  reports `sshservicediag-encrypted-transport-service-request`; message 50
  reports `sshservicediag-encrypted-transport-userauth-request`.
- fixed: preserved the pre-authentication frontier. Positive dispatch reports
  also retain `sshservicediag-authentication-unimplemented`,
  `sshservicediag-session-unimplemented`, and `sshservicediag-not-ready`;
  authentication-success=false, session-count=0, channel-count=0,
  shell-attached=false, and ssh-ready=false remain authoritative.
- fixed: empty payload, inactive encrypted-packet state, unsupported message
  number, post-NEWKEYS plaintext attempts, and packet crypto failure all fail
  closed with fixed labels.
- fixed: focused unit evidence covers service-request, userauth-request,
  malformed, unsupported, inactive, plaintext-rejected, and crypto-failure
  cases without retaining payload, packet, key, peer, operator, session,
  hardware, or boot material.
- removed: no dead code was found in this bounded surface.
- deferred: decrypted receive-loop integration, service acceptance,
  userauth/authentication success, authorized-key signature validation,
  session/channel allocation, PTY/process/shell attachment, live sockets,
  hardware reachability, public compatibility, broad expansion, and phase
  transition.

## Source Evidence

- `src/ssh_service_readiness.rs` now defines
  `classify_ssh_encrypted_transport_dispatch` and the retained
  `SshEncryptedTransportDispatchReport`.
- Retained report data is limited to fixed labels, the public first-byte
  message number when dispatch has reached an active encrypted-packet state,
  the active-state boolean, and false/zero readiness counters.
- The classifier checks packet crypto failure and post-NEWKEYS plaintext
  rejection before inspecting any decrypted payload byte. Inactive encrypted
  packet state and empty payloads do not retain a message number.

## Validation

- cargo fmt --all: pass.
- cargo -Zjson-target-spec test --quiet: initial run failed before tests because
  QEMU was not on PATH; rerun with the documented QEMU PATH passed with 761
  no_std tests.
- cargo fmt --all -- --check: pass.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass with existing large search-index
  warning.
- git diff --cached --check: pass.

## Redaction Review

Pass. Durable evidence retains only task ids, file paths, fixed labels, public
SSH message numbers 5 and 50, small false/zero readiness counters, validation
commands, and classifications. It retains no service names, user names, auth
method strings, public keys, signatures, packet payload bytes, ciphertext,
plaintext, MAC/tag material, keys, IV bytes, exchange hashes, shared secrets,
peer text/address, operator identity, key-derived identifiers, stable
transport/session identifiers, live hardware data, or boot artifacts.

## Accepted Frontier

The accepted frontier is local pre-authentication encrypted-transport dispatch
classification only. ssh-ready remains false. Authentication/session/shell
success, service success, authorized-key signature validation, live
reachability, OpenSSH/POSIX/Linux compatibility, hardware/lab action, boot
publication, broad expansion, and phase transition remain unaccepted.

## Result

Accepted.
selected_next_task=phase12-shell-ssh-encrypted-transport-dispatch-smoke-20260622.
