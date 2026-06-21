# Phase 12.5 shell host-key diagnostic smoke

Task id: phase12-shell-ssh-host-keydiag-smoke-20260621

Status: accepted.

Classification: phase12-shell-ssh-host-keydiag-smoke-accepted.

## Goal

Retain host/QEMU-substitute shell/internal smoke evidence for the accepted
read-only VFS host-key metadata diagnostic path.

## Scope

- Added a task-owned retained smoke script that reaches sshkeydiag through the
  accepted diagnostic command surface.
- Exercised missing, invalid, insufficient, and sufficient public-fixture
  host-key metadata states.
- Retained a transcript under task evidence.
- Kept the smoke at host/QEMU-substitute test level; no Pi 5 hardware,
  boot-archive publication, live transport, or SSH service behavior is part of
  this task.

## Non-goals

- No source behavior changes.
- No real private key, generated key, derived public key, digest, fingerprint,
  signature, host-key generation, authorized-key storage, writable persistence,
  SSH service behavior, live transport, hardware/lab action, hardware
  reachability, public ABI/POSIX/Linux compatibility, stale link-ready
  discriminator promotion, broad expansion, or phase transition.

## Findings

- fixed: added scripts/qemu-shell-ssh-host-keydiag-smoke.sh as the retained
  host/QEMU-substitute smoke gate for read-only VFS host-key metadata through
  sshkeydiag.
- fixed: retained
  tasks/evidence/2026-06-21-shell-ssh-host-keydiag-smoke/qemu-shell-ssh-host-keydiag-smoke.log
  with missing, invalid, insufficient, and sufficient public-fixture case
  summaries.
- not-an-issue: the retained smoke transcript records labels, public fixture
  state names, and validation command output only; it does not include real
  private-key bytes, generated keys, derived public keys, digests,
  fingerprints, signatures, or comparable stable secret identifiers.
- deferred: authorized-key storage, writable persistence, SSH service behavior,
  live transport, hardware reachability, public ABI/POSIX/Linux compatibility,
  stale link-ready discriminator work, broad expansion, and phase transition
  remain future work.

## Smoke Evidence

Script:

    scripts/qemu-shell-ssh-host-keydiag-smoke.sh

Retained transcript:

    tasks/evidence/2026-06-21-shell-ssh-host-keydiag-smoke/qemu-shell-ssh-host-keydiag-smoke.log

The retained transcript records:

    qemu-shell-ssh-host-keydiag-smoke: boundary=internal diagnostic command sshkeydiag over read-only VFS host-key metadata classifier
    qemu-shell-ssh-host-keydiag-smoke: missing-host-key=sshkeydiag-missing-host-key,sshkeydiag-missing-authorized-key,sshkeydiag-entropy-unready,sshkeydiag-seed-material-missing,ssh-ready=false
    qemu-shell-ssh-host-keydiag-smoke: invalid-host-key=sshkeydiag-host-key-invalid,ssh-ready=false
    qemu-shell-ssh-host-keydiag-smoke: insufficient-host-key=sshkeydiag-host-key-insufficient,ssh-ready=false
    qemu-shell-ssh-host-keydiag-smoke: sufficient-public-fixture=host-key-prerequisite-cleared,sshkeydiag-missing-authorized-key,sshkeydiag-entropy-unready,ssh-ready=false
    qemu-shell-ssh-host-keydiag-smoke: redaction=no-real-private-key,no-generated-key,no-derived-public-key,no-digest,no-fingerprint,no-signature,no-stable-secret-identifier
    qemu-shell-ssh-host-keydiag-smoke: PASS classification=host-qemu-substitute-shell-ssh-host-keydiag-smoke-complete

The smoke gate invokes these focused filters:

- dispatcher_reports_operator_seed_missing_from_vfs_without_secret_material
- dispatcher_reports_host_key_metadata_invalid_insufficient_and_sufficient_from_vfs
- host_key_vfs_metadata_maps_to_fail_closed_states_without_reading_key_bytes
- host_key_vfs_metadata_clears_only_host_key_prerequisite

The current no_std custom test harness reported all 729 tests passing during
the retained transcript.

## Validation

- scripts/qemu-shell-ssh-host-keydiag-smoke.sh: pass.
- static redaction review: pass; retained smoke transcript contains no real
  private-key bytes, generated keys, derived public keys, digests,
  fingerprints, signatures, host-key generation output, or comparable stable
  secret identifiers.
- cargo fmt --all -- --check: pass.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass; existing warning that the search
  index is very large.
- git diff --cached --check: pass.

## Acceptance

- accepted: retained evidence exercises the accepted host-key metadata
  diagnostic path through sshkeydiag.
- accepted: missing host-key metadata fails closed.
- accepted: sufficient public-fixture metadata clears only the host-key label;
  ssh-ready remains false because authorized-key, entropy, persistence,
  exposure, service, transport, and reachability prerequisites remain
  unaccepted.
- accepted: evidence preserves accepted CSPRNG/cryptographic-strength metadata
  boundaries by making no cryptographic-strength or SSH service claim.
- accepted: retained transcript contains no real private key, generated key,
  derived public key, digest, fingerprint, signature, or comparable stable
  secret identifier.

selected_next_task=phase12-ssh-host-key-readiness-closeout-20260621.
