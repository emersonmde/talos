# Phase 12.6 SSH host-key private-material core

Task id: phase12-ssh-host-key-private-material-core-20260622
Status: accepted
Owner: worker
Classification: phase12-ssh-host-key-private-material-core-accepted.

## Goal

Implement the accepted bounded host-key private-material parsing/signing
readiness core selected by the host-key private-material contract.

## Scope

- Implemented the accepted operator-provisioned unencrypted OpenSSH
  ssh-ed25519 private-material boundary for
  /etc/talos/ssh/ssh_host_ed25519_key.
- Added the accepted ssh-key 0.7.0-rc.10 dependency with
  default-features=false and alloc plus ed25519, plus the direct no-default
  signature trait crate needed to call ssh-key's raw signing API.
- Kept diagnostics fail-closed and local: sshkeydiag now treats malformed,
  encrypted, unsupported, non-regular, empty, oversized, and insufficient
  host-key material as invalid/insufficient/missing labels, and only accepted
  public-fixture OpenSSH Ed25519 material clears the host-key prerequisite.
- Added an in-memory signing handle for later runtime KEX callers; the handle
  can sign caller-supplied exchange-hash bytes and returns an ephemeral
  ssh-ed25519 Signature object without retaining signature bytes in durable
  evidence.
- Initialized the existing bump allocator in the no_std test harness so the
  allocating ssh-key parser can run in QEMU tests.

## Non-goals

- No runtime KEX, encryption/MAC, NEWKEYS, authentication/session behavior,
  shell attachment, live transport reachability, hardware/lab action, boot
  publication, OpenSSH/POSIX/Linux compatibility claim, broad expansion, or
  phase transition.
- No real private key material, real signatures, fingerprints, digests, peer
  identifiers, operator identity, random bytes, shared secrets, key-derived
  identifiers, or stable transport/session identifiers retained in task
  evidence.

## Findings

- fixed: metadata-only host-key readiness no longer clears the host-key
  prerequisite; sufficient host-key material must parse as unencrypted
  OpenSSH ssh-ed25519 private material.
- fixed: malformed sufficient-length material, encrypted OpenSSH material,
  non-regular files, empty files, oversized files, and below-minimum files are
  deterministic fail-closed host-key states.
- fixed: diagnostics and persistence/exposure tests now use a public fixture
  OpenSSH Ed25519 private key when the scenario needs the host-key
  prerequisite to clear.
- fixed: the in-memory host-key handle exposes only byte length, fixed
  algorithm label, and an ephemeral ssh-key Signature wrapper for callers; it
  does not expose private bytes through diagnostics or retained evidence.
- fixed: no_std QEMU tests now initialize the existing global bump allocator
  before tests run, which is required by the allocating ssh-key parser.
- not-an-issue: the direct signature dependency is trait glue for ssh-key's
  public raw-signing API, not a new crypto backend, algorithm surface, or host
  randomness source.
- deferred: runtime KEX consumption, public-key packet emission, NEWKEYS,
  encryption/MAC, authentication/session behavior, shell attachment, hardware
  reachability, compatibility, broad expansion, and phase transition.

## Evidence

- Source implementation:
  - Cargo.toml and Cargo.lock add ssh-key 0.7.0-rc.10 with
    default-features=false and alloc plus ed25519.
  - Cargo.toml and Cargo.lock add signature 3.0.0 with default-features=false
    only for the signing trait needed by ssh-key's raw Signature API.
  - src/ssh_key_readiness.rs parses host-key material after metadata bounds,
    rejects encrypted/non-Ed25519/malformed material, and adds the in-memory
    signing handle.
  - src/diagnostic_command.rs updates public test fixtures so authorized-key
    and persistence/exposure diagnostics still isolate their own labels.
  - src/main.rs initializes the existing bump allocator for no_std tests.
- Source/unit evidence:
  - host_key_private_material_maps_to_fail_closed_states covers missing,
    non-regular, empty, oversized, insufficient, malformed, encrypted, and
    sufficient public-fixture states.
  - host_key_private_material_clears_only_host_key_prerequisite proves the
    accepted host-key material clears only the host-key label and keeps
    ssh-ready false.
  - host_key_private_material_loads_and_signs_public_fixture proves the
    public fixture loads and can produce an ephemeral ssh-ed25519 signature
    shape for a caller-supplied exchange-hash byte string.
  - host_key_private_material_loader_rejects_nonaccepted_inputs covers
    loader-level fail-closed labels.
- Dependency feature evidence:
  - cargo -Zjson-target-spec tree -e features -i aes reports nothing to print;
    AES is present only as optional lockfile metadata from ssh-cipher, not as an
    enabled dependency path.
  - cargo -Zjson-target-spec tree -e features -p ssh-key shows ssh-key's
    enabled boundary as alloc, ed25519, rand_core required by ed25519, zeroize,
    ssh-encoding base64/digest/pem/zeroize, and ssh-cipher default+zeroize
    without AES, encrypted-key, getrandom, RSA/ECDSA/DSA/P-curve, ppk, serde,
    or broad crypto features.

## Redaction Review

This task record retains only task ids, file paths, public crate names and
versions, fixed public SSH format/algorithm names, dependency feature names,
test names, validation command names, and fixed diagnostic labels. It does not
retain private host-key bytes, real operator key material, signatures,
fingerprints, digests, public-key blobs, comments, peer identifiers, operator
identity, random bytes, shared secrets, key-derived identifiers, or stable
transport/session identifiers. Source tests use public fixture key material
only; no real operator/private deployment key material is present.

## Validation

- cargo fmt --all -- --check: pass.
- cargo -Zjson-target-spec test --quiet: pass with QEMU 9.2.0 on PATH; 752
  no_std QEMU tests passed.
- cargo -Zjson-target-spec tree -e features -i aes: pass; no enabled AES
  dependency path.
- cargo -Zjson-target-spec tree -e features -p ssh-key: pass; reviewed enabled
  ssh-key feature/dependency boundary.
- task-owned focused tests: pass as part of the full cargo test gate.
- jq empty on task-owned JSON evidence: not applicable; no task-owned JSON
  evidence created.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass with existing large search-index
  warning.
- git diff --cached --check: pass.

Validation levels: fmt/lint check, no_std QEMU/substitute unit tests, static
task/docs/source review, docs build, and diff checks. No Pi 5 hardware run,
lab-controller API action, hardwareTestLock acquisition, boot publication,
runtime SSH KEX, authentication/session work, shell attachment,
OpenSSH/POSIX/Linux compatibility, broad expansion, or phase transition was
performed.

## Acceptance

Accepted. Talos now has bounded host-key private-material parsing/signing
readiness for operator-provisioned unencrypted OpenSSH ssh-ed25519 private
material at /etc/talos/ssh/ssh_host_ed25519_key. ssh-ready remains false. The
selected next bounded task is
phase12-shell-ssh-host-key-private-material-smoke-20260622.
