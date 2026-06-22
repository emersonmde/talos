# Phase 12.6 SSH pre-auth service/userauth core

Task id: phase12-ssh-preauth-service-userauth-core-20260622

Status: accepted

Classification: phase12-ssh-preauth-service-userauth-core-accepted.

## Goal

Implement the private pre-authentication service-request/userauth-request parser
core selected by the accepted contract, without accepting service success,
authentication success, sessions, channels, shell attachment, live reachability,
hardware behavior, compatibility, broad expansion, or a phase transition.

## Scope

- Parse only caller-owned decrypted payloads that pass the accepted encrypted
  transport dispatch classifier.
- Parse SSH_MSG_SERVICE_REQUEST (5) as one SSH binary string and recognize only
  the public literal ssh-userauth.
- Parse SSH_MSG_USERAUTH_REQUEST (50) with SSH binary string framing for user
  name, service, and method, and recognize only ssh-connection/publickey shape
  after the modeled service-request prerequisite.
- Retain only fixed labels, public message numbers/literals, small public
  bounds, a small parsed-field counter, and false/zero readiness state.

## Non-goals

- No SSH_MSG_SERVICE_ACCEPT, service success, authentication success,
  authorized-key validation, account/user model, public-key signature
  validation, session/channel success, PTY allocation, process or shell
  attachment, live socket connection, hardware/lab action, boot publication,
  OpenSSH/POSIX/Linux compatibility claim, broad expansion, phase transition,
  or ssh-ready=true.
- No retention of user names, peer-selected service or method strings, public
  key blobs, signatures, packet payload bytes, peer text/address, operator
  identity, key-derived identifiers, stable session/transport identifiers,
  secret material, live hardware data, or boot artifacts.

## Findings

- fixed: added SshPreauthServiceUserauthInput, result/report types, and
  classify_ssh_preauth_service_userauth in src/ssh_service_readiness.rs.
- fixed: service-request parsing recognizes ssh-userauth as a diagnostic-only
  modeled prerequisite and keeps service-success=false.
- fixed: userauth-request parsing recognizes ssh-connection/publickey shape only
  after the modeled service prerequisite and keeps authentication-success=false.
- fixed: userauth-before-service, unsupported service, unsupported method,
  malformed strings, trailing/missing fields, inactive encrypted state,
  post-NEWKEYS plaintext, crypto failure, and unsupported dispatch message
  paths fail closed with fixed labels.
- not-an-issue: the implementation compares public protocol literals in memory
  to produce fixed diagnostics, but does not retain caller payload slices or
  peer-provided strings in durable evidence.
- deferred: publickey authentication policy, authorized-key lookup, signature
  verification, user/account semantics, service accept, session/channel/shell
  work, live reachability, and compatibility remain future tasks.

## Evidence

- static source/task/docs review: src/ssh_service_readiness.rs now exposes a
  private pre-auth parser over accepted encrypted dispatch results only.
- unit/QEMU-substitute: cargo -Zjson-target-spec test preauth_service_userauth
  --quiet passed.
- unit/QEMU-substitute: cargo -Zjson-target-spec test --quiet passed with
  QEMU on PATH; retained tail reports test result: ok. 765 passed.
- fmt/lint: cargo fmt --all -- --check passed after rustfmt.
- diff check: git diff --check passed.
- docs: /home/node/.cargo/bin/mdbook build passed.
- pre-commit diff check: git diff --cached --check passed.

## Redaction Review

Pass. Retained evidence contains only task ids, file paths, fixed labels, public
SSH message numbers, public protocol literals, public field order, small public
bounds/counters, validation commands, and classifications. It retains no packet
payload bytes, parsed user names, peer-selected service/method strings, public
key blobs, signatures, ciphertext, plaintext, MAC/tag material, keys, IV bytes,
exchange hashes, shared secrets, peer raw input/address, operator identity,
key-derived identifiers, stable transport/session identifiers, live hardware
data, or boot artifacts.

## Acceptance

Accepted. selected_next_task=phase12-shell-ssh-preauth-service-userauth-smoke-20260622.
