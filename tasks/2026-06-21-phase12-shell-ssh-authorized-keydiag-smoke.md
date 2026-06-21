# Phase 12.5 shell authorized-key diagnostic smoke

Task id: phase12-shell-ssh-authorized-keydiag-smoke-20260621

Status: accepted.

Classification: phase12-shell-ssh-authorized-keydiag-smoke-accepted.

## Goal

Retain host/QEMU-substitute shell/internal smoke evidence for the accepted
read-only VFS authorized-key metadata diagnostic path.

## Scope

- Added a task-owned retained smoke script that reaches sshkeydiag through the
  accepted diagnostic command surface.
- Exercised missing, invalid, insufficient, and sufficient public-fixture
  authorized-key metadata states.
- Retained a transcript under task evidence.
- Kept the smoke at host/QEMU-substitute test level; no Pi 5 hardware,
  boot-archive publication, live transport, or SSH service behavior is part of
  this task.

## Non-goals

- No source behavior changes beyond task-owned smoke harness and evidence.
- No authorized-key parsing, public-key validation, authentication, real
  operator key retention, operator identity retention, key-derived identifier,
  host-key parsing/generation, writable persistence, SSH service behavior,
  live transport, hardware/lab action, hardware reachability, public
  ABI/POSIX/Linux compatibility, stale link-ready discriminator promotion,
  broad expansion, or phase transition.

## Findings

- fixed: added scripts/qemu-shell-ssh-authorized-keydiag-smoke.sh as the
  retained host/QEMU-substitute smoke gate for read-only VFS authorized-key
  metadata through sshkeydiag.
- fixed: retained
  tasks/evidence/2026-06-21-shell-ssh-authorized-keydiag-smoke/qemu-shell-ssh-authorized-keydiag-smoke.log
  with missing, invalid, insufficient, and sufficient public-fixture case
  summaries.
- not-an-issue: the retained smoke transcript records labels, public fixture
  state names, and validation command output only; it does not include real
  authorized public keys, operator identities, fingerprints, digests,
  signatures, key-derived identifiers, private keys, generated keys, or
  comparable stable identifiers.
- deferred: authorized-key parsing, user authentication, writable persistence,
  SSH service behavior, live transport, hardware reachability, public
  ABI/POSIX/Linux compatibility, stale link-ready discriminator work, broad
  expansion, and phase transition remain future work.

## Smoke Evidence

Script:

    scripts/qemu-shell-ssh-authorized-keydiag-smoke.sh

Retained transcript:

    tasks/evidence/2026-06-21-shell-ssh-authorized-keydiag-smoke/qemu-shell-ssh-authorized-keydiag-smoke.log

The retained transcript records:

    qemu-shell-ssh-authorized-keydiag-smoke: boundary=internal diagnostic command sshkeydiag over read-only VFS authorized-key metadata classifier
    qemu-shell-ssh-authorized-keydiag-smoke: missing-authorized-key=sshkeydiag-missing-authorized-key,sshkeydiag-entropy-unready,ssh-ready=false
    qemu-shell-ssh-authorized-keydiag-smoke: invalid-authorized-key=sshkeydiag-authorized-key-invalid,ssh-ready=false
    qemu-shell-ssh-authorized-keydiag-smoke: insufficient-authorized-key=sshkeydiag-authorized-key-insufficient,ssh-ready=false
    qemu-shell-ssh-authorized-keydiag-smoke: sufficient-public-fixture=authorized-key-prerequisite-cleared,sshkeydiag-entropy-unready,sshkeydiag-persistence-unavailable,sshkeydiag-exposure-disabled,ssh-ready=false
    qemu-shell-ssh-authorized-keydiag-smoke: redaction=no-real-authorized-public-key,no-operator-identity,no-key-derived-identifier,no-digest,no-fingerprint,no-signature,no-private-key,no-generated-key
    qemu-shell-ssh-authorized-keydiag-smoke: PASS classification=host-qemu-substitute-shell-ssh-authorized-keydiag-smoke-complete

The smoke gate invokes these focused filters:

- dispatcher_reports_authorized_key_metadata_invalid_insufficient_and_sufficient_from_vfs
- authorized_key_vfs_metadata_maps_to_fail_closed_states_without_reading_key_bytes
- authorized_key_vfs_metadata_clears_only_authorized_key_prerequisite

The current no_std custom test harness reported all 732 tests passing during
the retained transcript.

## Validation

- scripts/qemu-shell-ssh-authorized-keydiag-smoke.sh: pass.
- static redaction review: pass; retained smoke transcript contains no real
  authorized public key, operator identity, fingerprint, digest, signature,
  key-derived identifier, private key, generated key, or comparable stable
  identifier.
- cargo fmt --all -- --check: pass.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass; existing warning that the search
  index is very large.
- git diff --cached --check: pass.

## Acceptance

- accepted: retained evidence exercises the accepted authorized-key metadata
  diagnostic path through sshkeydiag.
- accepted: missing authorized-key metadata fails closed.
- accepted: invalid and insufficient authorized-key metadata report explicit
  sshkeydiag authorized-key labels.
- accepted: sufficient public-fixture metadata clears only the authorized-key
  prerequisite; ssh-ready remains false because persistence, exposure, service,
  transport, and reachability prerequisites remain unaccepted.
- accepted: retained transcript contains no real authorized public key,
  operator identity, fingerprint, digest, signature, key-derived identifier,
  private key, generated key, or comparable stable identifier.

selected_next_task=phase12-ssh-authorized-key-readiness-closeout-20260621.
