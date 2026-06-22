# Phase 12.6 SSH host-key private-material contract

Task id: phase12-ssh-host-key-private-material-contract-20260622
Status: accepted
Owner: worker
Classification: phase12-ssh-host-key-private-material-contract-accepted.

## Goal

Define the first runtime host-key private-material parsing and signing
boundary for /etc/talos/ssh/ssh_host_ed25519_key, after the accepted runtime
KEX/crypto contract blocked on metadata-only key readiness.

## Scope

- Reviewed accepted host-key metadata readiness, modeled KEXINIT algorithm
  policy, operator-seeded CSPRNG, persistence/exposure readiness, service
  readiness, listener/transport, and runtime KEX/crypto blocker evidence.
- Selected the exact accepted private host-key file format, dependency feature
  boundary, parse/classification labels, signing API boundary, zeroization, and
  durable-evidence redaction rules for the next implementation slice.
- Kept this task to contract work only. No code implementation, Cargo
  dependency adoption, actual private-key parsing, signing, runtime KEX,
  encryption/MAC, NEWKEYS, authentication/session behavior, shell attachment,
  hardware action, reachability, compatibility claim, broad expansion, or phase
  transition is accepted here.

## Non-goals

- No code implementation, Cargo dependency adoption, actual private-key
  parsing, signing, KEX, encryption/MAC, NEWKEYS, authentication/session
  behavior, shell attachment, hardware/lab action, boot publication, hardware
  reachability, OpenSSH/POSIX/Linux compatibility claim, broad expansion, or
  phase transition.
- No private host-key bytes, authorized-key bytes, signatures, fingerprints,
  digests, peer identifiers, operator identity, random bytes, shared secrets,
  key-derived identifiers, or stable transport/session identifiers retained.

## Reviewed Inputs

- tasks/2026-06-22-phase12-ssh-runtime-kex-crypto-contract.md
- tasks/2026-06-22-phase12-ssh-kexinit-negotiation-closeout.md
- tasks/2026-06-21-phase12-ssh-host-key-readiness-closeout.md
- tasks/2026-06-21-phase12-ssh-authorized-key-readiness-closeout.md
- tasks/2026-06-21-phase12-operator-seeded-csprng-closeout.md
- tasks/2026-06-22-phase12-ssh-persistence-exposure-readiness-closeout.md
- tasks/2026-06-22-phase12-ssh-service-readiness-closeout.md
- tasks/2026-06-22-phase12-ssh-listener-transport-closeout.md
- tasks/2026-06-22-phase12-ssh-implementation-strategy-adr.md
- tasks/2026-06-22-phase12-ssh-implementation-dependency-feasibility-contract.md
- src/ssh_key_readiness.rs
- src/ssh_service_readiness.rs
- Cargo.toml
- docs/src/project/phase12-networking-ssh.md
- docs/src/roadmap.md
- docs/src/decisions/README.md
- local registry metadata/source:
  - ssh-key 0.7.0-rc.10 Cargo.toml, README.md, src/private.rs,
    src/public/ed25519.rs, and src/signature.rs
  - ed25519-dalek 3.0.0-rc.0 Cargo.toml
  - base64ct 1.8.3 Cargo.toml
  - zeroize 1.8.1 Cargo.toml

## Contract

Talos selects operator-provisioned unencrypted OpenSSH Ed25519 private-key
material at /etc/talos/ssh/ssh_host_ed25519_key as the first runtime host-key
private-material format.

The accepted input format for the next implementation is:

- a regular readable VFS file at the existing host-key path;
- existing metadata cap: 64 through 4096 bytes remains the only
  metadata-present size range before parsing;
- PEM text beginning with BEGIN OPENSSH PRIVATE KEY and containing an
  openssh-key-v1 envelope;
- ciphername none, kdfname none, empty kdfoptions, and exactly one key;
- algorithm ssh-ed25519 only;
- public-key blob in the envelope must be internally consistent with the
  Ed25519 private material;
- comment text, if present, is ignored and not retained.

The rejected input formats are:

- encrypted OpenSSH private keys, passphrases, bcrypt-pbkdf, AES, and
  chacha20poly1305 private-key decryption;
- RSA, ECDSA, DSA, FIDO/security-key, certificate, PPK, PKCS#1, PKCS#8, SEC1,
  legacy SSH private-key, multi-key, generated-key, or embedded-source-key
  formats;
- malformed PEM/base64/openssh-key-v1 data, mismatched public/private
  material, non-regular files, unreadable files, zero-length files, files below
  the accepted metadata minimum, and files above the accepted metadata cap.

The selected dependency boundary for the follow-up implementation is
ssh-key 0.7.0-rc.10 with default-features=false and exactly features
[alloc, ed25519], plus the existing zeroize dependency. This boundary may use
ssh-key parsing and Ed25519 signing support and its transitive ed25519-dalek
support, but it must not enable ssh-key default features, std, encryption,
crypto broad feature, RSA/ECDSA/DSA/P-curve features, ppk, serde, getrandom,
private-key generation, host OS randomness, or encrypted-key support. If Cargo
feature resolution proves this exact feature set impossible in the core task,
the core task must block rather than widen the feature set.

The signing API boundary for the follow-up implementation is:

- parse the read-only VFS bytes into a Talos-owned host-key handle whose
  secret-bearing buffers are zeroized on drop;
- expose only fail-closed classification labels and an in-memory signing
  operation for the SSH exchange hash bytes supplied by a later runtime KEX
  task;
- produce an ephemeral ssh-ed25519 signature for the caller-owned packet path;
- allow the live packet path to send the SSH public-key blob and signature when
  a later runtime KEX task accepts that behavior, but never retain public-key
  blobs, signatures, fingerprints, digests, private bytes, comments, key
  identifiers, or comparable stable key-derived values in diagnostics, docs,
  task records, serial logs, or retained evidence.

The follow-up implementation may add only the selected dependency boundary,
classification/parsing/signing readiness code, and focused public-fixture tests
or retained host/QEMU-substitute evidence. It must keep ssh-ready false until
later runtime KEX, authentication/session, shell, live reachability, and
compatibility tasks explicitly accept their own slices.

## Failure Labels

- Missing file keeps sshkeydiag-missing-host-key.
- Non-regular, unreadable, zero-length, above-cap, malformed, unsupported,
  encrypted, multi-key, wrong-algorithm, or internally inconsistent material
  reports sshkeydiag-host-key-invalid.
- Regular readable material below the existing metadata minimum reports
  sshkeydiag-host-key-insufficient.
- Sufficient unencrypted OpenSSH ssh-ed25519 private material clears only the
  host-key private-material prerequisite for later runtime KEX planning; it
  does not make ssh-ready true.

## Findings

- fixed: metadata-only host-key readiness is no longer treated as parsed
  private-key or signing readiness.
- fixed: selected one host-key private-material input format: unencrypted
  OpenSSH ssh-ed25519 private-key material at the existing read-only VFS path.
- fixed: selected a narrow dependency feature boundary for the next core task:
  ssh-key 0.7.0-rc.10 default-features=false with alloc and ed25519 only, plus
  existing zeroize behavior.
- fixed: rejected encrypted keys, generated keys, ambient host randomness,
  broad ssh-key default features, broad crypto feature selection, and non-Ed25519
  key formats for the first implementation slice.
- fixed: durable evidence redaction forbids retaining private bytes,
  signatures, fingerprints, digests, comments, public-key blobs, operator
  identity, key-derived identifiers, or stable session/transport identifiers.
- deferred: actual parsing/signing implementation, Cargo dependency adoption,
  runtime KEX consumption, encryption/MAC, NEWKEYS, authentication/session
  behavior, shell attachment, live hardware reachability, compatibility claim,
  broad expansion, and phase transition.
- not-an-issue: docs/src/decisions/README.md is updated because selecting a
  host-key private-material format and dependency boundary constrains future
  SSH server identity handling.

## Redaction Review

This contract retains only task ids, paths, public crate names and versions,
fixed public SSH algorithm and format names, dependency feature names,
classification labels, validation command names, and boundary decisions. It
retains no private host-key bytes, authorized-key bytes, signatures,
fingerprints, digests, peer identifiers, operator identity, generated random
bytes, shared secrets, comments, public-key blobs, key-derived identifiers, or
stable transport/session identifiers.

## Validation

- static task/docs/source review: pass.
- cargo fmt --all -- --check: not run; no Rust source touched.
- cargo -Zjson-target-spec test --quiet: not run; no Rust source or Cargo
  metadata touched.
- jq empty on task-owned JSON evidence: not applicable; no task-owned JSON
  evidence created.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass with existing large search-index
  warning.
- git diff --cached --check: pass.

Validation levels: static inspection, local dependency metadata/source review,
docs build, and diff checks. No code implementation, dependency adoption,
QEMU/substitute run, Pi 5 hardware run, lab-controller API action,
hardwareTestLock acquisition, boot publication, runtime SSH KEX,
authentication/session work, shell attachment, OpenSSH/POSIX/Linux
compatibility, broad expansion, or phase transition was performed.

## Acceptance

Accepted. The first host-key private-material parsing/signing boundary is
operator-provisioned unencrypted OpenSSH ssh-ed25519 private-key material at
/etc/talos/ssh/ssh_host_ed25519_key, parsed and signed only by the selected
narrow ssh-key dependency feature boundary in a later implementation task.
ssh-ready remains false. No actual private-key parsing, signing, runtime KEX,
encryption/MAC, NEWKEYS, authentication/session success, shell attachment,
hardware reachability, public compatibility, broad expansion, or phase
transition is accepted.

selected_next_task=phase12-ssh-host-key-private-material-core-20260622.
