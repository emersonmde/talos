# Phase 12.6 SSH encrypted transport dispatch smoke

Task id: phase12-shell-ssh-encrypted-transport-dispatch-smoke-20260622

Status: accepted.

Classification: phase12-shell-ssh-encrypted-transport-dispatch-smoke-accepted.

## Goal

Retain focused local smoke/regression evidence for the encrypted transport
dispatch classifier, without expanding into authentication, sessions, shell
attachment, live reachability, public compatibility, hardware behavior, or a
phase transition.

## Findings and Disposition

- fixed: retained focused smoke/regression evidence through the existing
  `encrypted_transport_dispatch_*` no_std test coverage in
  `src/ssh_service_readiness.rs`.
- fixed: the retained evidence covers active encrypted-packet state for public
  SSH message numbers 5 and 50, reporting service-request and userauth-request
  routing diagnostics only when the accepted encrypted-packet state is active.
- fixed: the retained evidence covers fail-closed labels for empty payload,
  unsupported message number, inactive encrypted-packet state, post-NEWKEYS
  plaintext rejection, and packet crypto failure.
- fixed: evidence redaction remains limited to fixed labels, public message
  numbers, small false/zero counters, validation commands, task ids, and
  classifications.
- not-an-issue: no new source behavior was needed; the accepted core already
  included the focused smoke/regression coverage required by this task.
- removed: no dead code was found in this bounded surface.
- deferred: decrypted receive-loop integration, service acceptance,
  userauth/authentication success, authorized-key signature validation,
  session/channel allocation, PTY/process/shell attachment, live sockets,
  hardware reachability, public compatibility, broad expansion, and phase
  transition.

## Smoke Evidence

- `cargo -Zjson-target-spec test encrypted_transport_dispatch --quiet`: pass
  with the documented QEMU PATH. The no_std harness ran the full 761-test suite
  and included the focused encrypted transport dispatch regressions.
- `encrypted_transport_dispatch_routes_preauth_message_numbers_only_when_active`
  covers service-request message 5 and userauth-request message 50 under active
  encrypted-packet state.
- `encrypted_transport_dispatch_fails_closed_without_retaining_payload_material`
  covers empty payload, unsupported message number, inactive encrypted-packet
  state, post-NEWKEYS plaintext rejection, and packet crypto failure.

## Validation

- static source/task/docs review: pass.
- cargo fmt --all -- --check: pass.
- cargo -Zjson-target-spec test --quiet: pass with the documented QEMU PATH,
  761 no_std tests.
- targeted encrypted transport dispatch smoke/regression command: pass with
  `cargo -Zjson-target-spec test encrypted_transport_dispatch --quiet`.
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

The accepted frontier remains local pre-authentication encrypted-transport
dispatch classification only. ssh-ready remains false.
Authentication/session/shell success, service success, authorized-key signature
validation, live reachability, OpenSSH/POSIX/Linux compatibility, hardware/lab
action, boot publication, broad expansion, and phase transition remain
unaccepted.

## Result

Accepted.
selected_next_task=phase12-ssh-encrypted-transport-dispatch-closeout-20260622.
