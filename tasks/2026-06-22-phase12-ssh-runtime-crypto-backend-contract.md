# Phase 12.6 SSH runtime crypto backend contract

Task: phase12-ssh-runtime-crypto-backend-contract-20260622

Status: accepted

## Goal

Define the no_std runtime crypto backend and private API boundary needed before
the first actual SSH runtime key-exchange implementation.

## Scope

- Reviewed the accepted KEXINIT, CSPRNG, host-key private-material,
  listener/transport, service-readiness, persistence/exposure, and authorized-key
  prerequisite evidence.
- Selected exact dependency/API boundaries for X25519, SHA-256 exchange
  hashing and SSH KDF input expansion, the negotiated HMAC-SHA256 surface,
  and chacha20-poly1305@openssh.com encrypted-packet state.
- Recorded secret lifetime, zeroization, transcript/key-derivation redaction,
  and evidence policy before any runtime KEX implementation.

## Non-goals

- No Rust source implementation, Cargo dependency adoption, actual X25519,
  transcript hashing, SSH KDF, encryption/MAC, NEWKEYS, authentication/session
  behavior, shell attachment, hardware/lab action, boot publication, hardware
  reachability, public OpenSSH/POSIX/Linux compatibility claim, broad expansion,
  or phase transition is accepted.
- No private bytes, signatures, fingerprints, digests, random bytes, shared
  secrets, derived keys, packet plaintext/ciphertext, tags, operator identity,
  key-derived identifiers, or stable transport/session identifiers are retained
  in durable evidence.

## Accepted Contract

The first runtime SSH crypto backend remains Talos-owned orchestration over
small no_std RustCrypto crates and the already accepted host-key signing
boundary:

- X25519: add x25519-dalek 3.0.0-rc.1 only in a later implementation task,
  default-features=false, features=["zeroize"]. The runtime API must use
  EphemeralSecret generated from the accepted OperatorSeededCsprng through a
  private rand_core 0.10 CryptoRng adapter, derive one public key for the KEX
  packet path, consume the ephemeral secret exactly once for diffie_hellman,
  and zeroize the shared secret after key derivation.
- Exchange hash and SSH KDF: add sha2 0.11.0 only in a later implementation
  task, default-features=false, features=["zeroize"]. Talos owns the SSH
  binary transcript encoding and RFC 4253-style key expansion. The first hash
  algorithm is SHA-256 only, bound to curve25519-sha256 and ssh-ed25519.
- Negotiated HMAC surface: add hmac 0.13.0 only in a later implementation
  task, default-features=false, features=["zeroize"]. The fixed
  hmac-sha2-256 name remains modeled for negotiation and future non-AEAD packet
  MAC work; the first chacha20-poly1305@openssh.com encrypted-packet path uses
  AEAD/Poly1305 tagging and must not also emit a standalone HMAC.
- Encrypted packet state: add ssh-cipher 0.3.0-rc.10 only in a later
  implementation task, default-features=false,
  features=["chacha20poly1305", "zeroize"]. Talos must call only the
  chacha20-poly1305@openssh.com path and reject AES, TDES, getrandom, ambient
  RNG, and default broad cipher features for the first runtime slice.
- Host-key signing: consume the accepted in-memory ssh-ed25519 signing handle
  for the exchange hash only. The runtime KEX task may pass caller-owned hash
  bytes into the existing signer and emit a signature into the caller-owned KEX
  packet path, but durable diagnostics must retain only fixed labels and
  lengths.

The first implementation boundary is a private runtime module, tentatively
ssh_runtime_crypto, that exposes only fixed-label readiness and a one-shot KEX
result shape to the SSH transport layer:

- inputs: accepted local/remote identification strings, accepted local/remote
  KEXINIT packets, remote curve25519 public key bytes, accepted host-key handle,
  and a ready OperatorSeededCsprng;
- outputs: local curve25519 public key bytes, caller-owned host-key public blob
  for packet emission, caller-owned host-key signature bytes for packet
  emission, and private send/receive encrypted-packet states;
- fixed labels: crypto-backend-ready, kex-peer-public-key-invalid,
  kex-csprng-not-ready, kex-host-key-not-ready, kex-transcript-invalid,
  kex-key-derivation-failed, and encrypted-packet-state-ready;
- rejected public surface: public API, syscall ABI, POSIX/Linux compatibility,
  authentication/session success, shell channel execution, live reachability,
  stable session IDs, and durable transcript/key material.

The first runtime KEX implementation must keep ssh-ready false until later
authentication/session/shell and reachability tasks explicitly accept readiness.

## Dependency Feasibility Evidence

- cargo info x25519-dalek@3.0.0-rc.1: rust-version 1.85; features show
  default=[precomputed-tables, zeroize], getrandom optional, static/reusable
  secrets optional. Source inspection shows #![no_std], EphemeralSecret,
  random_from_rng<R: CryptoRng>, consuming diffie_hellman, and zeroize-on-drop
  support behind the zeroize feature.
- cargo info ssh-cipher@0.3.0-rc.10 and source inspection: crate is #![no_std],
  has a dedicated chacha20poly1305 feature for chacha20-poly1305@openssh.com,
  documents that the OpenSSH construction differs from RFC8439 variants, and
  exposes zeroize support without requiring AES/TDES/getrandom features.
- cargo info hmac@0.13.0 and sha2@0.11.0 plus source inspection: both are
  #![no_std]; hmac exposes the generic Hmac API and sha2 exposes Sha256 with a
  zeroize feature for hash state.
- Current Cargo.lock/source review: existing ssh-key/ed25519 dependencies
  already use rand_core 0.10-compatible crates, and the accepted CSPRNG uses
  chacha20::rand_core 0.10, so the implementation can introduce a private
  adapter instead of ambient randomness.

## Findings

- fixed: selected a no_std dependency boundary for the accepted algorithm set:
  x25519-dalek 3.0.0-rc.1, sha2 0.11.0, hmac 0.13.0, and ssh-cipher
  0.3.0-rc.10 with only the stated feature sets.
- fixed: selected ssh-cipher rather than chacha20poly1305 for the first packet
  cipher because ssh-cipher explicitly implements the
  chacha20-poly1305@openssh.com construction; the RFC8439 AEAD crate is not
  selected for runtime SSH packet crypto.
- fixed: constrained the X25519 randomness boundary to the accepted
  OperatorSeededCsprng through a private rand_core 0.10 CryptoRng adapter and
  rejected getrandom/ambient host RNG features.
- fixed: split SHA-256 exchange hash/KDF duties from the negotiated
  hmac-sha2-256 name; the first AEAD encrypted-packet path must not add a
  standalone HMAC over chacha20-poly1305@openssh.com packets.
- fixed: defined redaction and zeroization rules for ephemeral private keys,
  shared secrets, exchange hashes, derived keys, signatures, packet plaintext,
  packet ciphertext, tags, and stable identifiers.
- deferred: actual Cargo.toml edits, runtime module implementation,
  cross-crate feature-tree proof after adoption, KEX packet emission, NEWKEYS,
  encrypted packet I/O, authentication/session behavior, shell attachment,
  hardware reachability, compatibility, broad expansion, and phase transition.
- not-an-issue: hardwareTestLock is unlocked/restored; this contract is
  static/docs/source review only and makes no Pi 5 hardware claim.

## Redaction Review

Pass. This task retained only crate names/versions/features, source paths,
algorithm names, fixed labels, validation commands, and evidence
classifications. It retained no private bytes, signatures, fingerprints,
digests, random bytes, shared secrets, derived keys, packet plaintext,
packet ciphertext, tags, operator identity, key-derived identifiers, stable
session identifiers, peer addresses, or peer-supplied transport text.

## Validation

- static task/docs/source review: pass.
- cargo info x25519-dalek@3.0.0-rc.1: pass; dependency metadata captured.
- cargo info chacha20poly1305@0.11.0-rc.3: pass; rejected for runtime packet
  crypto because it is the RFC8439 AEAD family rather than the OpenSSH packet
  construction.
- cargo info ssh-cipher@0.3.0-rc.10: pass; dependency metadata captured.
- cargo info hmac@0.13.0: pass; dependency metadata captured.
- cargo info sha2@0.11.0: pass; dependency metadata captured.
- local registry source inspection for x25519-dalek, ssh-cipher, hmac, and
  sha2: pass.
- cargo fmt --all -- --check: not run; no Rust source touched.
- cargo -Zjson-target-spec test --quiet: not run; no Rust source or Cargo
  metadata touched.
- jq empty on task-owned JSON evidence: not run; no JSON evidence created.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass with existing large search-index
  warning.
- git diff --cached --check: pass.
- post-commit git status --short --branch: clean/ahead origin/main.

## Outcome

Accepted as a static contract. The no_std runtime crypto backend/API boundary
is now explicit enough for supervisor planning of a bounded runtime KEX
implementation task, but no worker-owned queued implementation task exists in
durable state. Supervisor planning is required to queue the next task; the
worker did not create a new taskQueue item.
