# Phase 12.6 SSH runtime KEX closeout

Task id: phase12-ssh-runtime-kex-closeout-20260622

Status: accepted.

Classification: phase12-ssh-runtime-kex-closeout-accepted.

## Goal

Reconcile the accepted runtime KEX contract, core implementation, dependency
feature evidence, retained smoke transcript, and redaction review before any
NEWKEYS or encrypted packet I/O task is promoted.

## Scope

- Reviewed the accepted runtime crypto backend contract, runtime KEX core, and
  retained runtime KEX smoke task records.
- Confirmed the accepted frontier is local modeled runtime KEX readiness through
  sshservicediag with private encrypted-packet state handles only.
- Updated the Phase 12 architecture notes and roadmap to reflect the closeout
  frontier and the next bounded task selection.

## Non-goals

No source behavior change, NEWKEYS activation, encrypted packet I/O,
authentication/session success, shell attachment, live hardware/lab action,
boot publication, live transport reachability claim,
OpenSSH/POSIX/Linux compatibility claim, broad expansion, or phase transition
is accepted. This closeout does not retain private keys, random bytes, shared
secrets, exchange hashes, derived keys, signature bytes, public-key blobs,
packet plaintext/ciphertext, tags, peer raw input, operator identity,
key-derived identifiers, or stable session identifiers.

## Findings

- fixed: reconciled the runtime crypto backend contract with the implemented
  dependency boundary. The accepted Cargo/runtime surface remains limited to
  x25519-dalek 3.0.0-rc.1, sha2 0.11.0, hmac 0.13.0, and
  ssh-cipher 0.3.0-rc.10 with the accepted no-default feature sets.
- fixed: reconciled the runtime KEX core evidence with the smoke transcript.
  The success path uses real curve25519-sha256, the accepted
  OperatorSeededCsprng, accepted host-key signing handle, SHA-256 transcript
  hashing/KDF, and private chacha20-poly1305@openssh.com packet-state handles.
- fixed: recorded that the accepted frontier remains pre-NEWKEYS and local.
  ssh-ready remains false, and authentication/session/shell and live
  reachability remain unaccepted.
- fixed: selected exactly one bounded follow-up:
  phase12-ssh-newkeys-packet-crypto-contract-20260622.
- not-an-issue: hmac-sha2-256 remains part of the modeled algorithm policy and
  dependency surface, but the accepted chacha20-poly1305@openssh.com packet
  path must not emit a standalone HMAC.
- deferred: NEWKEYS activation, encrypted packet I/O, authentication,
  authorized-key parsing, sessions/channels, PTY/shell attachment, live
  reachability, OpenSSH/POSIX/Linux compatibility, hardware proof, broad
  expansion, and phase transition.

## Evidence

- phase12-ssh-runtime-crypto-backend-contract-20260622: accepted static
  no_std crypto backend/API contract and secret/redaction policy.
- phase12-ssh-runtime-kex-core-20260622: accepted source implementation,
  dependency feature evidence, runtime KEX source/unit evidence, and redaction
  review.
- phase12-shell-ssh-runtime-kex-smoke-20260622: accepted retained
  host/QEMU-substitute smoke transcript covering real runtime KEX success,
  crypto-backend-ready, encrypted-packet-state-ready, and deterministic
  fail-closed labels while ssh-ready remains false.

## Redaction Review

Pass. This closeout retained only task ids, file paths, crate names and
versions, public algorithm names, fixed diagnostic labels, validation command
names, and classifications. It retained no private keys, random bytes, shared
secrets, exchange hashes, derived keys, signatures, public-key blobs, packet
plaintext/ciphertext, tags, peer raw input, operator identity,
key-derived identifiers, stable session identifiers, live peer addresses, or
hardware data.

## Validation

- static task/docs/source review: pass.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check: pass.

Validation levels: static task/source/docs review, docs build, and diff
checks. No Rust source was touched, so cargo fmt and cargo test were not
required by this task's gates. No Pi 5 hardware run, lab-controller API
action, hardwareTestLock acquisition, boot publication, NEWKEYS activation,
encrypted packet I/O, authentication/session work, shell attachment, live
reachability claim, compatibility claim, broad expansion, or phase transition
was performed.

## Acceptance

Accepted. Runtime KEX is closed out at the local modeled pre-NEWKEYS frontier:
the accepted service diagnostic path can reach real runtime KEX success and
private encrypted-packet-state readiness while ssh-ready remains false. The
selected next bounded task is
phase12-ssh-newkeys-packet-crypto-contract-20260622.
