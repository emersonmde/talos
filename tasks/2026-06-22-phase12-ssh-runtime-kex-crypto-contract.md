# Phase 12.6 SSH runtime KEX/crypto contract

Task id: phase12-ssh-runtime-kex-crypto-contract-20260622
Status: accepted
Owner: worker
Classification: phase12-ssh-runtime-kex-crypto-contract-planning-needed.

## Goal

Define the first actual runtime SSH key-exchange/crypto implementation slice,
or block with a concrete missing prerequisite before implementation.

## Scope

- Reviewed accepted KEXINIT/algorithm negotiation, operator-seeded CSPRNG,
  host-key metadata, authorized-key metadata, persistence/exposure metadata,
  listener/transport, and SSH implementation strategy evidence.
- Reconciled the accepted modeled algorithm policy against the requirements for
  actual X25519, host-key signing, encryption/MAC, and NEWKEYS.
- Kept the outcome limited to contract/planning state. No code, dependency
  adoption, runtime crypto, key exchange, host-key signing, authentication,
  session/channel behavior, shell attachment, hardware action, reachability, or
  compatibility claim is accepted here.

## Non-goals

- No code implementation, Cargo dependency adoption, actual key exchange,
  encryption/MAC enablement, NEWKEYS, host-key signing, authentication/session
  success, shell attachment, hardware/lab action, boot publication, hardware
  reachability, OpenSSH/POSIX/Linux compatibility claim, broad expansion, or
  phase transition.
- No random bytes, private host-key bytes, authorized-key bytes, shared
  secrets, signatures, fingerprints, digests, peer addresses, peer
  identification text, operator identity, key-derived identifiers, or stable
  transport/session identifiers retained.

## Reviewed Inputs

- tasks/2026-06-22-phase12-ssh-kexinit-negotiation-closeout.md
- tasks/2026-06-22-phase12-ssh-kexinit-negotiation-core.md
- tasks/2026-06-22-phase12-ssh-kexinit-negotiation-contract.md
- tasks/2026-06-21-phase12-operator-seeded-csprng-closeout.md
- tasks/2026-06-21-phase12-ssh-host-key-readiness-closeout.md
- tasks/2026-06-21-phase12-ssh-authorized-key-readiness-closeout.md
- tasks/2026-06-22-phase12-ssh-persistence-exposure-readiness-closeout.md
- tasks/2026-06-22-phase12-ssh-listener-transport-closeout.md
- tasks/2026-06-22-phase12-ssh-implementation-strategy-adr.md
- tasks/2026-06-22-phase12-ssh-implementation-dependency-feasibility-contract.md
- src/csprng.rs
- src/ssh_key_readiness.rs
- src/ssh_service_readiness.rs
- Cargo.toml
- docs/src/project/phase12-networking-ssh.md
- docs/src/roadmap.md

## Contract Outcome

The first actual runtime KEX/crypto implementation is not mechanically
objective from the accepted frontier.

The accepted modeled KEXINIT policy names a reversible algorithm set:
curve25519-sha256, ssh-ed25519, chacha20-poly1305@openssh.com,
hmac-sha2-256, compression none, and empty language lists. That policy is
sufficient for local negotiation labels, but not sufficient for runtime crypto
implementation because the accepted host-key and authorized-key work classifies
only read-only VFS metadata. It does not parse private host-key material,
validate an Ed25519 host key format, bind a public host identity, perform
host-key signing, parse authorized-key material, or define secret-bearing
diagnostic/evidence handling for those operations.

The accepted CSPRNG boundary is usable only as a prerequisite: it can fill
private buffers from sufficient operator seed material and redacts generated
bytes from evidence. A runtime crypto implementation still needs a separately
accepted boundary for ephemeral X25519 private scalar generation, server
ephemeral public-key exposure, shared-secret handling, transcript hashing,
derived-key material lifetime, packet encryption/MAC state, and zeroization.
Those are intentionally not accepted by the modeled KEXINIT slice.

The currently accepted dependencies do not yet provide an objective no_std
runtime crypto stack selection for this slice. Cargo has accepted chacha20 and
zeroize support for the CSPRNG boundary, and smoltcp for socket substrate, but
no accepted X25519, Ed25519 signing, ChaCha20-Poly1305 AEAD, SHA-256/HMAC, SSH
binary-packet encryption, or key-derivation dependency/API boundary exists for
Talos runtime.

## Required Supervisor Planning

planningNeeded=true.

The next implementation cannot be selected by the worker from the current
queue. Supervisor planning should split the missing prerequisites before any
runtime KEX implementation:

- host-key private material format and parsing/signing contract for
  /etc/talos/ssh/ssh_host_ed25519_key, including no secret-byte evidence,
  public identity/fingerprint redaction policy, failure labels, and no_std
  dependency constraints;
- runtime crypto backend/dependency contract for X25519, Ed25519 signing,
  SHA-256/HMAC, ChaCha20-Poly1305, SSH key derivation, key lifetime, and
  zeroization;
- only after those contracts are accepted, a bounded runtime KEX implementation
  task may consume the accepted KEXINIT transcript and close before
  authentication/session/shell behavior unless separately planned.

## Findings

- fixed: reconciled the accepted KEXINIT policy with actual runtime crypto
  prerequisites instead of treating modeled algorithm labels as implementation
  readiness.
- fixed: recorded that accepted host-key and authorized-key readiness remain
  metadata-only and are insufficient for host-key parsing/signing or
  authentication.
- fixed: recorded that the accepted CSPRNG is a prerequisite for private random
  buffers but does not by itself accept ephemeral KEX secret handling,
  transcript hashing, derived keys, packet encryption, or MAC state.
- deferred: no_std/runtime dependency selection for X25519, Ed25519 signing,
  SHA-256/HMAC, ChaCha20-Poly1305, SSH KDF, packet encryption/MAC, and
  zeroization boundaries.
- deferred: actual key exchange, NEWKEYS, authentication/session behavior,
  shell attachment, hardware reachability, OpenSSH/POSIX/Linux compatibility,
  broad expansion, and phase transition.
- not-an-issue: docs/src/decisions/README.md is unchanged because this task
  selects no crypto dependency or hard-to-reverse algorithm policy; the prior
  modeled KEXINIT policy remains reversible local diagnostic policy only.

## Redaction Review

This contract retains task ids, paths, fixed public Talos policy names, fixed
missing-prerequisite descriptions, validation command names, and planning
requirements only. It retains no generated random bytes, private host-key
bytes, authorized-key bytes, shared secrets, signatures, fingerprints,
digests, peer addresses, peer identification text, operator identity,
key-derived identifiers, or stable transport/session identifiers.

## Validation

- static task/docs/evidence review: pass.
- cargo fmt --all -- --check: not run; no Rust source touched.
- cargo -Zjson-target-spec test --quiet: not run; no Rust source or Cargo
  metadata touched.
- jq empty on task-owned JSON evidence: not applicable; no task-owned JSON
  evidence created.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass with existing large search-index
  warning.
- git diff --cached --check: pass.

Validation levels: static inspection, docs build, and diff checks. No code
implementation, dependency adoption, QEMU/substitute run, Pi 5 hardware run,
lab-controller API action, hardwareTestLock acquisition, boot publication,
runtime SSH crypto, authentication/session work, shell attachment,
OpenSSH/POSIX/Linux compatibility, broad expansion, or phase transition was
performed.

## Acceptance

Accepted as a planning-needed contract. The task blocks runtime KEX/crypto
implementation on concrete missing prerequisites: private host-key
parsing/signing and no_std runtime crypto dependency/API boundaries. ssh-ready
remains false. No actual key exchange, encryption/MAC, NEWKEYS, host-key
signing, authentication/session success, shell attachment, hardware
reachability, public compatibility, broad expansion, or phase transition is
accepted.

selected_next_task=null.
