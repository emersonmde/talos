# Phase 12.6 SSH pre-auth service/userauth smoke

Task id: phase12-shell-ssh-preauth-service-userauth-smoke-20260622

Status: accepted

Classification: phase12-shell-ssh-preauth-service-userauth-smoke-accepted.

## Goal

Retain focused smoke/regression evidence for the accepted private
pre-authentication service/userauth parser core without adding protocol
behavior or accepting service success, authentication success, sessions,
channels, shell attachment, live reachability, hardware behavior,
compatibility, broad expansion, or a phase transition.

## Scope

- Exercise the accepted service-request and userauth-request parser paths with
  focused local unit/regression evidence.
- Cover recognized ssh-userauth service-request and ssh-connection/publickey
  userauth after the modeled service prerequisite.
- Cover userauth before service, unsupported service, unsupported method,
  malformed/missing/trailing fields, inactive encrypted state, plaintext
  rejection, crypto failure, and unsupported dispatch paths.
- Retain only fixed labels, public message numbers/literals, field-order
  descriptions, small counters, validation commands, task ids, and
  classifications.

## Non-goals

- No new Rust protocol behavior, SSH_MSG_SERVICE_ACCEPT, service success,
  authentication success, authorized-key validation, account/user model, public
  key parsing, signature validation, session/channel success, PTY allocation,
  process or shell attachment, live socket connection, hardware/lab action,
  boot publication, OpenSSH/POSIX/Linux compatibility claim, broad expansion,
  phase transition, or ssh-ready=true.
- No retention of user names, peer-selected service or method strings, public
  key blobs, signatures, packet payload bytes, ciphertext, plaintext, MAC/tag
  material, keys, IV bytes, exchange hashes, shared secrets, peer raw
  input/address, operator identity, key-derived identifiers, stable
  session/transport identifiers, live hardware data, or boot artifacts.

## Findings

- fixed: focused local smoke evidence covers recognized service-request and
  recognized publickey userauth after the modeled service prerequisite.
- fixed: focused local smoke evidence covers userauth-before-service,
  unsupported service, unsupported method, malformed/trailing/missing fields,
  inactive encrypted state, plaintext rejection, crypto failure, and
  unsupported dispatch paths.
- fixed: readiness false/zero counters remain asserted by the focused parser
  tests: service-success=false, authentication-success=false,
  session-count=0, channel-count=0, shell-attached=false, and ssh-ready=false.
- not-an-issue: this smoke task adds no new protocol behavior; it records the
  accepted core's focused source/unit evidence as the retained smoke transcript.
- deferred: service accept, publickey authentication policy, authorized-key
  lookup, signature verification, user/account semantics, sessions/channels,
  shell attachment, live reachability, compatibility, and phase transition
  remain future work.

## Evidence

- static source/task/docs review: src/ssh_service_readiness.rs focused tests
  include preauth_service_userauth_recognizes_service_request_without_service_success,
  preauth_service_userauth_models_publickey_after_service_prerequisite,
  preauth_service_userauth_fails_closed_for_before_service_and_unsupported_shapes,
  and preauth_service_userauth_rejects_malformed_and_inherited_dispatch_failures.
- unit/QEMU-substitute: cargo -Zjson-target-spec test preauth_service_userauth
  --quiet passed with QEMU on PATH; retained output includes the four focused
  preauth_service_userauth tests and final test result: ok. 765 passed.
- fmt/lint: cargo fmt --all -- --check passed.
- unit/QEMU-substitute: cargo -Zjson-target-spec test --quiet passed with
  QEMU on PATH; retained tail reports test result: ok. 765 passed.
- diff check: git diff --check passed.
- docs: /home/node/.cargo/bin/mdbook build passed.
- pre-commit diff check: git diff --cached --check passed.

## Redaction Review

Pass. Retained smoke evidence contains only task ids, file paths, test names,
fixed labels, public SSH message numbers, public protocol literals, public
field-order descriptions, small public counters, validation commands, and
classifications. It retains no packet payload bytes, parsed user names,
peer-selected service/method strings, public key blobs, signatures,
ciphertext, plaintext, MAC/tag material, keys, IV bytes, exchange hashes,
shared secrets, peer raw input/address, operator identity, key-derived
identifiers, stable transport/session identifiers, live hardware data, or boot
artifacts.

## Acceptance

Accepted. selected_next_task=phase12-ssh-preauth-service-userauth-closeout-20260622.
