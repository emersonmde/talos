# Phase 12.6 SSH host-key private-material closeout

Task id: phase12-ssh-host-key-private-material-closeout-20260622
Status: accepted
Owner: worker
Classification: phase12-ssh-host-key-private-material-closeout-accepted.

## Goal

Reconcile the accepted host-key private-material contract, implementation,
retained smoke evidence, docs, redaction posture, and remaining runtime KEX
prerequisites before selecting the next bounded crypto-backend contract.

## Scope

- Reviewed the accepted host-key private-material contract, core
  implementation, retained host/QEMU-substitute smoke transcript, Phase 12
  docs, roadmap frontier, and ADR entry.
- Confirmed that Talos now has bounded parsing/signing readiness for
  operator-provisioned unencrypted OpenSSH ssh-ed25519 private material at the
  existing read-only VFS host-key path.
- Confirmed that this readiness only unblocks later runtime KEX planning. It
  does not accept authentication/session behavior, shell attachment, live
  reachability, compatibility, broad expansion, or phase transition.

## Non-goals

- No new code implementation, dependency adoption, runtime KEX,
  encryption/MAC, NEWKEYS, authentication/session behavior, shell attachment,
  hardware/lab action, boot publication, hardware reachability,
  OpenSSH/POSIX/Linux compatibility claim, broad expansion, or phase
  transition.
- No private bytes, signatures, fingerprints, digests, random bytes, shared
  secrets, operator identity, stable identifiers, key-derived identifiers,
  public-key blobs, or comments retained.

## Reviewed Inputs

- tasks/2026-06-22-phase12-ssh-host-key-private-material-contract.md
- tasks/2026-06-22-phase12-ssh-host-key-private-material-core.md
- tasks/2026-06-22-phase12-shell-ssh-host-key-private-material-smoke.md
- tasks/evidence/2026-06-22-shell-ssh-host-key-private-material-smoke/qemu-shell-ssh-host-key-private-material-smoke.log
- docs/src/project/phase12-networking-ssh.md
- docs/src/roadmap.md
- docs/src/decisions/README.md
- Cargo.toml and Cargo.lock
- src/ssh_key_readiness.rs
- src/diagnostic_command.rs
- src/main.rs

## Findings

- fixed: the closeout records that metadata-only host-key readiness has been
  superseded for runtime KEX planning by accepted private-material parsing and
  in-memory Ed25519 signing readiness.
- fixed: accepted public-fixture evidence covers missing, invalid,
  unsupported/encrypted, insufficient, and sufficient host-key private-material
  states, and sufficient material clears only the host-key private-material
  prerequisite.
- fixed: the accepted implementation keeps ssh-ready false because runtime KEX,
  encryption/MAC, NEWKEYS, authentication/session, shell, live reachability,
  and compatibility remain unaccepted.
- fixed: the retained evidence and task records keep private bytes, signatures,
  fingerprints, digests, public-key blobs, comments, operator identity, random
  bytes, shared secrets, key-derived identifiers, and stable
  transport/session identifiers out of durable evidence.
- fixed: docs now state the private-material closeout frontier and identify
  the next bounded runtime crypto backend contract.
- deferred: actual runtime KEX, X25519 exchange, transcript hashing, SSH KDF,
  packet encryption/MAC state, NEWKEYS, authentication/session behavior, shell
  attachment, hardware reachability, OpenSSH/POSIX/Linux compatibility, broad
  expansion, and phase transition.
- not-an-issue: no new ADR is needed for this closeout because the expensive
  host-key format and dependency-boundary decision is already recorded in
  docs/src/decisions/README.md.

## Reconciliation

The accepted contract selected one private host-key format and dependency
boundary: unencrypted OpenSSH ssh-ed25519 private material in the existing
read-only VFS path, parsed through ssh-key 0.7.0-rc.10 with
default-features=false and alloc plus ed25519.

The accepted core implemented that boundary and added an in-memory signing
handle for later caller-owned exchange-hash bytes. The handle can produce an
ephemeral ssh-ed25519 Signature object, but no durable diagnostic or evidence
path retains private material, signature bytes, public-key blobs,
fingerprints, digests, comments, or stable key-derived identifiers.

The retained smoke evidence proves the fail-closed states and the sufficient
public-fixture path at host/QEMU-substitute level. It does not claim live SSH
transport, actual key exchange, packet encryption/MAC, NEWKEYS,
authentication/session success, shell attachment, hardware reachability, or
OpenSSH compatibility.

This is sufficient only for runtime KEX planning. The remaining blocker from
phase12-ssh-runtime-kex-crypto-contract-20260622 is now narrowed to the
runtime crypto backend/API boundary for X25519, SHA-256/HMAC,
ChaCha20-Poly1305, SSH KDF, derived key lifetime, zeroization, encrypted
packet state, and integration with the accepted host-key signing handle.

## Redaction Review

Pass. This closeout retains only task ids, public file paths, public crate
names and versions, public SSH algorithm/format names, fixed labels,
validation commands, and evidence classifications. It retains no private
host-key bytes, real operator key material, signatures, fingerprints, digests,
public-key blobs, comments, random bytes, shared secrets, peer identifiers,
operator identity, key-derived identifiers, or stable transport/session
identifiers.

## Validation

- static task/docs/evidence review: pass.
- cargo fmt --all -- --check: not run; no Rust source touched.
- cargo -Zjson-target-spec test --quiet: not run; no Rust source or Cargo
  metadata touched.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass with existing large search-index
  warning.
- git diff --cached --check: pass.

Validation levels: static task/docs/evidence review, docs build, and diff
checks. No source behavior change, QEMU/substitute test rerun, Pi 5 hardware
run, lab-controller API action, hardwareTestLock acquisition, boot
publication, runtime SSH KEX, encryption/MAC, NEWKEYS,
authentication/session work, shell attachment, OpenSSH/POSIX/Linux
compatibility, broad expansion, or phase transition was performed.

## Acceptance

Accepted. Host-key private-material parsing/signing readiness is closed out as
sufficient only for later runtime KEX planning. ssh-ready remains false. The
selected next bounded task is
phase12-ssh-runtime-crypto-backend-contract-20260622.

selected_next_task=phase12-ssh-runtime-crypto-backend-contract-20260622.
