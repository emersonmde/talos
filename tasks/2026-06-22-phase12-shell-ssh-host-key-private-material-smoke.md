# Phase 12.6 shell host-key private-material smoke

Task id: phase12-shell-ssh-host-key-private-material-smoke-20260622

Status: accepted.

Classification: phase12-shell-ssh-host-key-private-material-smoke-accepted.

## Goal

Retain host/QEMU-substitute diagnostic evidence for the accepted host-key
private-material parsing/signing readiness core without retaining secrets.

## Scope

- Added a task-owned retained smoke script that exercises the accepted
  sshkeydiag host-key private-material path and focused loader/signing tests.
- Exercised missing, invalid, unsupported/encrypted, insufficient, and
  sufficient public-fixture host-key material states.
- Retained a redacted transcript under task evidence.
- Kept the smoke at host/QEMU-substitute level; no Pi 5 hardware, boot-archive
  publication, live SSH transport, runtime key exchange, authentication, or
  shell behavior is part of this task.

## Non-goals

- No source behavior changes.
- No real private keys, private bytes, signature bytes, fingerprints, digests,
  random bytes, shared secrets, operator identity, stable identifiers,
  runtime KEX, encryption/MAC, NEWKEYS, authentication/session success, shell
  attachment, live transport, hardware/lab action, boot publication,
  reachability claim, compatibility claim, broad expansion, or phase
  transition.

## Findings

- fixed: added
  scripts/qemu-shell-ssh-host-key-private-material-smoke.sh as the retained
  host/QEMU-substitute smoke gate for the accepted private-material
  classifier and signing-handle boundary.
- fixed: retained
  tasks/evidence/2026-06-22-shell-ssh-host-key-private-material-smoke/qemu-shell-ssh-host-key-private-material-smoke.log
  with contract-required public-fixture states and fail-closed labels.
- not-an-issue: the retained smoke transcript records task ids, fixed labels,
  public fixture state names, test filter names, and validation command output
  only; it does not include private bytes, signature bytes, fingerprints,
  digests, random bytes, shared secrets, operator identity, stable identifiers,
  or key-derived identifiers.
- deferred: runtime KEX consumption, encryption/MAC, NEWKEYS, authentication,
  session/channel behavior, shell attachment, hardware reachability,
  compatibility claims, broad expansion, and phase transition remain future
  work.

## Smoke Evidence

Script:

    scripts/qemu-shell-ssh-host-key-private-material-smoke.sh

Retained transcript:

    tasks/evidence/2026-06-22-shell-ssh-host-key-private-material-smoke/qemu-shell-ssh-host-key-private-material-smoke.log

The retained transcript records:

    qemu-shell-ssh-host-key-private-material-smoke: boundary=internal diagnostic command sshkeydiag plus focused loader/signing tests over read-only VFS host-key private-material classifier
    qemu-shell-ssh-host-key-private-material-smoke: missing-host-key=sshkeydiag-missing-host-key,sshkeydiag-missing-authorized-key,sshkeydiag-entropy-unready,ssh-ready=false
    qemu-shell-ssh-host-key-private-material-smoke: invalid-host-key=non-regular-empty-oversized-malformed-encrypted-unsupported,sshkeydiag-host-key-invalid,ssh-ready=false
    qemu-shell-ssh-host-key-private-material-smoke: insufficient-host-key=sshkeydiag-host-key-insufficient,ssh-ready=false
    qemu-shell-ssh-host-key-private-material-smoke: sufficient-public-fixture=host-key-private-material-prerequisite-cleared,sshkeydiag-missing-authorized-key,sshkeydiag-entropy-unready,sshkeydiag-persistence-unavailable,sshkeydiag-exposure-disabled,ssh-ready=false
    qemu-shell-ssh-host-key-private-material-smoke: signing-api=public-fixture-loads-and-signs-caller-owned-exchange-hash,in-memory-ephemeral-signature-only
    qemu-shell-ssh-host-key-private-material-smoke: redaction=no-real-private-key,no-private-bytes,no-signature-bytes,no-fingerprint,no-digest,no-random-bytes,no-shared-secret,no-operator-identity,no-stable-identifier,no-key-derived-identifier
    qemu-shell-ssh-host-key-private-material-smoke: PASS classification=host-qemu-substitute-shell-ssh-host-key-private-material-smoke-complete

The smoke gate invokes these focused filters:

- dispatcher_reports_host_key_metadata_invalid_insufficient_and_sufficient_from_vfs
- host_key_private_material_maps_to_fail_closed_states
- host_key_private_material_clears_only_host_key_prerequisite
- host_key_private_material_loads_and_signs_public_fixture
- host_key_private_material_loader_rejects_nonaccepted_inputs

The current no_std custom test harness reported all 752 tests passing during
each retained focused filter run.

## Redaction Review

Static redaction review: pass. The script and retained transcript contain no
real private key material, private bytes, signature bytes, fingerprints,
digests, random bytes, shared secrets, operator identity, stable identifiers,
key-derived identifiers, live transport identifiers, peer identifiers, or
session identifiers. The transcript retains only public fixture state names,
fixed labels, test names, command names, and the fact that an ephemeral
in-memory signature shape was exercised by focused unit evidence.

## Validation

- scripts/qemu-shell-ssh-host-key-private-material-smoke.sh: pass.
- cargo fmt --all -- --check: pass.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass with existing large search-index
  warning.
- git diff --cached --check: pass.

Validation levels: no_std QEMU/substitute focused tests, static redaction
review, fmt check, docs build, and diff checks. No Pi 5 hardware run,
lab-controller API action, hardwareTestLock acquisition, boot publication,
runtime SSH KEX, encryption/MAC, NEWKEYS, authentication/session work, shell
attachment, reachability claim, compatibility claim, broad expansion, or phase
transition was performed.

## Acceptance

- accepted: retained evidence covers missing, invalid, unsupported/encrypted,
  insufficient, and sufficient public-fixture host-key private-material
  states.
- accepted: sufficient public-fixture private material clears only the
  host-key prerequisite; ssh-ready remains false because authorized-key,
  entropy, persistence, exposure, service, transport, KEX, authentication,
  session, shell, and reachability prerequisites remain unaccepted.
- accepted: focused retained evidence covers the bounded signing API as
  caller-owned exchange-hash input and an ephemeral in-memory ssh-ed25519
  signature object only.
- accepted: retained transcript contains no private bytes, signature bytes,
  fingerprints, digests, random bytes, shared secrets, operator identity,
  stable identifiers, or key-derived identifiers.

selected_next_task=phase12-ssh-host-key-private-material-closeout-20260622.
