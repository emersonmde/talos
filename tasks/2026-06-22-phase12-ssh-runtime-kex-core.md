# Phase 12.6 SSH runtime KEX core

Task id: phase12-ssh-runtime-kex-core-20260622

Status: accepted.

Classification: phase12-ssh-runtime-kex-core-accepted.

## Goal

Implement the bounded local runtime KEX core selected by the accepted no_std
runtime crypto backend contract while keeping ssh-ready false and rejecting
NEWKEYS, encrypted packet I/O, authentication/session/shell behavior,
hardware reachability, compatibility, broad expansion, and phase transition.

## Scope

- Adopted only the accepted no-default runtime crypto dependency boundary:
  x25519-dalek 3.0.0-rc.1 with zeroize, sha2 0.11.0 with zeroize,
  hmac 0.13.0 with zeroize, and ssh-cipher 0.3.0-rc.10 with
  chacha20poly1305 plus zeroize.
- Added a private ssh_runtime_crypto module that performs one
  curve25519-sha256 exchange using EphemeralSecret from the accepted
  OperatorSeededCsprng through a private rand_core 0.10 CryptoRng adapter.
- Computes the SSH binary transcript hash with SHA-256, signs the exchange
  hash through the accepted in-memory ssh-ed25519 host-key handle, expands
  RFC4253-style key material, and constructs private
  chacha20-poly1305@openssh.com packet-state handles.
- Integrated runtime KEX result classification into sshservicediag after the
  accepted listener/transport and KEXINIT path with fixed labels and no byte
  retention.

## Non-goals

- No NEWKEYS activation, encrypted packet I/O, user authentication,
  authorized-key parsing, session/channel allocation, PTY behavior, shell
  attachment, live hardware/lab action, boot publication, reachability claim,
  OpenSSH/POSIX/Linux compatibility claim, public syscall/API surface, broad
  expansion, or phase transition.
- No durable retention of private keys, random bytes, shared secrets, exchange
  hashes, derived keys, signatures, public-key blobs, packet plaintext,
  packet ciphertext, tags, peer raw input, operator identity, key-derived
  identifiers, or stable session identifiers.

## Findings

- fixed: runtime KEX now uses real x25519-dalek EphemeralSecret and rejects
  non-contributory peer public keys instead of accepting a modeled exchange.
- fixed: X25519 randomness is sourced only from the accepted
  OperatorSeededCsprng via a private rand_core 0.10 CryptoRng adapter; the
  Cargo graph has no getrandom package.
- fixed: transcript hashing and KDF expansion are Talos-owned SHA-256 code;
  KEX input validation fails closed before byte material is retained in
  diagnostics.
- fixed: host-key signing goes through the accepted in-memory ssh-ed25519
  handle and returns caller-owned packet material only.
- fixed: packet-crypto readiness constructs private ssh-cipher
  chacha20-poly1305@openssh.com state handles with sequence number zero but
  does not activate NEWKEYS or perform encrypted packet I/O.
- fixed: sshservicediag now distinguishes crypto-backend-ready,
  kex-csprng-not-ready, kex-host-key-not-ready,
  kex-peer-public-key-invalid, kex-transcript-invalid,
  kex-key-derivation-failed, encrypted-packet-state-not-ready, and
  encrypted-packet-state-ready.
- deferred: NEWKEYS activation, encrypted packet I/O, authentication,
  authorized-key parsing, sessions/channels, PTY/shell attachment, hardware
  reachability, compatibility, broad expansion, and phase transition.
- not-an-issue: hmac 0.13.0 is adopted for the negotiated hmac-sha2-256
  surface, but the first accepted AEAD packet path does not emit a standalone
  HMAC over chacha20-poly1305@openssh.com packets.

## Evidence

- Source implementation:
  - Cargo.toml and Cargo.lock add the accepted no-default runtime crypto
    dependencies and features.
  - src/ssh_runtime_crypto.rs owns the private one-shot X25519, transcript
    hash, SSH KDF, host-key signing consumption, and packet-state construction.
  - src/ssh_service_readiness.rs composes runtime KEX result labels after the
    accepted local listener/transport and KEXINIT path while keeping ssh-ready
    false.
  - src/ssh_key_readiness.rs exposes caller-owned public-key/signature blobs
    for the runtime KEX packet path without diagnostic byte retention.
- Source/unit evidence:
  - runtime_kex_success_uses_real_crypto_and_private_packet_state_handles
    covers the success path with real X25519, host-key signing, and private
    chacha20-poly1305@openssh.com packet-state handles.
  - runtime_kex_fail_closed_labels_cover_missing_prerequisites covers missing
    CSPRNG, missing host key, invalid peer public key, transcript invalid, and
    KDF-failure label classification.
  - runtime_kex_integration_marks_crypto_ready_without_ssh_readiness proves
    sshservicediag reports crypto-backend-ready and
    encrypted-packet-state-ready while authentication/session/shell remain
    unimplemented and ssh-ready remains false.
- Dependency feature evidence:
  - cargo -Zjson-target-spec tree -e features -p x25519-dalek -p ssh-cipher
    -p hmac -p sha2: pass; reviewed accepted crate feature graph.
  - cargo -Zjson-target-spec tree -e features -i getrandom: pass; package ID
    did not match any package, proving ambient getrandom is absent.
  - cargo -Zjson-target-spec tree -e features -i aes: pass; nothing to print.
  - cargo -Zjson-target-spec tree -e features -i des: pass; nothing to print.

## Redaction Review

Pass. Retained task/docs evidence contains only task ids, file paths, public
crate names and versions, feature names, public algorithm names, fixed labels,
test names, validation command names, and validation classifications. Source
tests use public fixtures only. Durable evidence does not retain private keys,
random bytes, shared secrets, exchange hashes, derived keys, signature bytes,
public-key blobs, packet plaintext/ciphertext, tags, peer raw input, operator
identity, key-derived identifiers, or stable transport/session identifiers.

## Validation

- cargo fmt --all -- --check: pass.
- cargo -Zjson-target-spec test --quiet ssh_runtime_crypto: pass
  (QEMU/substitute no_std runner; 755 tests passed).
- cargo -Zjson-target-spec test --quiet: pass
  (QEMU/substitute no_std runner; 755 tests passed).
- cargo -Zjson-target-spec tree -e features -p x25519-dalek -p ssh-cipher
  -p hmac -p sha2: pass; accepted feature graph reviewed.
- cargo -Zjson-target-spec tree -e features -i getrandom: pass; no getrandom
  package in the graph.
- cargo -Zjson-target-spec tree -e features -i aes: pass; no enabled AES
  dependency path.
- cargo -Zjson-target-spec tree -e features -i des: pass; no enabled DES/TDES
  dependency path.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass with existing large search-index
  warning.
- git diff --cached --check: pass.

Validation levels: fmt/lint/typecheck, no_std QEMU/substitute unit tests,
static task/source/docs review, dependency feature inspection, docs build, and
diff checks. No Pi 5 hardware run, lab-controller API action,
hardwareTestLock acquisition, boot publication, generated-root publication,
live packet I/O, hardware reachability, NEWKEYS activation,
authentication/session work, shell attachment, OpenSSH/POSIX/Linux
compatibility claim, broad expansion, or phase transition was performed.

## Acceptance

Accepted. Talos now has a bounded local runtime KEX core that uses the real
accepted crypto primitives and host-key signing handle to produce caller-owned
packet material plus private encrypted-packet state handles. ssh-ready remains
false. The selected next bounded task is
phase12-shell-ssh-runtime-kex-smoke-20260622.
