# Phase 12.5 shell SSH persistence/exposure diagnostic smoke

Task id: phase12-shell-ssh-persistence-exposure-diag-smoke-20260622

Status: accepted.

Classification: phase12-shell-ssh-persistence-exposure-diag-smoke-accepted.

## Goal

Retain host/QEMU-substitute sshkeydiag smoke evidence for the accepted
read-only VFS persistence and exposure metadata diagnostic path.

## Scope

- Add a task-owned retained smoke script that reaches sshkeydiag through the
  accepted diagnostic command surface.
- Exercise default disabled exposure, missing exposure marker, invalid exposure
  marker, and sufficient public-fixture persistence/exposure metadata states.
- Retain a transcript under task evidence.
- Keep the smoke at host/QEMU-substitute test level; no Pi 5 hardware,
  boot-archive publication, live transport, packet I/O, reachability, or SSH
  service behavior is part of this task.

## Non-goals

- No SSH service behavior, listener, authentication session, PTY/session
  plumbing, connection handling, live transport, packet I/O, hardware/lab
  action, boot publication, hardware reachability, public ABI/POSIX/Linux
  compatibility, stale link-ready discriminator promotion, broad expansion, or
  phase transition.
- No source behavior changes beyond the task-owned smoke harness and retained
  evidence.
- No real operator seed bytes, host private key bytes, authorized public key
  bytes, fingerprints, digests, signatures, generated key material, generated
  random byte streams, private CSPRNG state, operator identity,
  key-derived identifiers, or comparable stable identifiers in logs,
  diagnostics, docs, task records, or retained evidence.
- No writable persistence claim; read-only generated-root/initramfs material is
  classified only as a first persistence metadata boundary.

## Findings

- fixed: added scripts/qemu-shell-ssh-persistence-exposure-diag-smoke.sh as the
  retained host/QEMU-substitute smoke gate for read-only VFS persistence and
  exposure metadata through sshkeydiag.
- fixed: retained
  tasks/evidence/2026-06-22-shell-ssh-persistence-exposure-diag-smoke/qemu-shell-ssh-persistence-exposure-diag-smoke.log
  with default disabled exposure, missing exposure marker, invalid exposure
  marker, and sufficient public-fixture persistence/exposure summaries.
- not-an-issue: the retained smoke transcript records labels, public fixture
  state names, validation command filters, and test-count output only; it does
  not include real operator seed bytes, host private key bytes, authorized
  public key bytes, generated key material, generated random byte streams,
  private CSPRNG state, operator identity, key-derived identifiers, digests,
  fingerprints, signatures, or comparable stable identifiers.
- deferred: SSH service behavior, listener/session handling, live transport,
  packet I/O, hardware reachability, writable persistence, public
  ABI/POSIX/Linux compatibility, stale link-ready discriminator work, broad
  expansion, and phase transition remain future work.

## Smoke Evidence

Script:

    scripts/qemu-shell-ssh-persistence-exposure-diag-smoke.sh

Retained transcript:

    tasks/evidence/2026-06-22-shell-ssh-persistence-exposure-diag-smoke/qemu-shell-ssh-persistence-exposure-diag-smoke.log

The retained transcript records:

    qemu-shell-ssh-persistence-exposure-diag-smoke: boundary=internal diagnostic command sshkeydiag over read-only VFS persistence and exposure metadata classifiers
    qemu-shell-ssh-persistence-exposure-diag-smoke: default-disabled-exposure=sshkeydiag-persistence-unavailable,sshkeydiag-exposure-disabled,sshkeydiag-not-ready,ssh-ready=false
    qemu-shell-ssh-persistence-exposure-diag-smoke: missing-exposure-marker=sufficient-public-fixture-persistence-metadata,sshkeydiag-exposure-disabled,sshkeydiag-not-ready,ssh-ready=false
    qemu-shell-ssh-persistence-exposure-diag-smoke: invalid-exposure-marker=sshkeydiag-exposure-disabled,sshkeydiag-not-ready,ssh-ready=false
    qemu-shell-ssh-persistence-exposure-diag-smoke: sufficient-public-fixture=persistence-unavailable-cleared,exposure-disabled-cleared,sshkeydiag-not-ready,ssh-ready=false
    qemu-shell-ssh-persistence-exposure-diag-smoke: redaction=no-real-operator-seed,no-real-host-private-key,no-real-authorized-public-key,no-generated-key,no-generated-random-bytes,no-private-csprng-state,no-operator-identity,no-key-derived-identifier,no-digest,no-fingerprint,no-signature
    qemu-shell-ssh-persistence-exposure-diag-smoke: PASS classification=host-qemu-substitute-shell-ssh-persistence-exposure-diag-smoke-complete

The smoke gate invokes these focused filters:

- dispatcher_reports_ssh_key_readiness_fail_closed_without_secret_material
- dispatcher_reports_authorized_key_metadata_invalid_insufficient_and_sufficient_from_vfs
- dispatcher_reports_persistence_exposure_metadata_without_secret_material
- persistence_metadata_requires_all_accepted_material_sources
- exposure_marker_vfs_metadata_fails_closed_until_explicit_marker_is_valid
- persistence_and_exposure_metadata_clear_only_their_prerequisites

## Validation

- scripts/qemu-shell-ssh-persistence-exposure-diag-smoke.sh: pass; retained
  transcript recorded 736 passed in each focused host/QEMU-substitute cargo
  test filter and ended with
  host-qemu-substitute-shell-ssh-persistence-exposure-diag-smoke-complete.
- cargo -Zjson-target-spec test --quiet: pass; custom no_std harness reported
  736 passed.
- cargo fmt --all -- --check: pass.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass; existing large-search-index warning.
- git diff --cached --check: pass.

## Acceptance

- accepted: retained transcript covers default disabled exposure, missing
  exposure marker, invalid exposure marker, and sufficient public-fixture
  persistence/exposure metadata.
- accepted: sufficient metadata clears only sshkeydiag-persistence-unavailable
  and sshkeydiag-exposure-disabled; sshkeydiag-not-ready remains and ssh-ready
  remains false.
- accepted: retained evidence contains no secret/key/random bytes or stable
  secret/operator identifiers.
- accepted: task record lists findings with disposition.

selected_next_task=phase12-ssh-persistence-exposure-readiness-closeout-20260622.
