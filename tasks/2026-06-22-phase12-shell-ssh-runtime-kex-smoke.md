# Phase 12.6 shell SSH runtime KEX smoke

Task id: phase12-shell-ssh-runtime-kex-smoke-20260622

Status: accepted.

Classification: phase12-shell-ssh-runtime-kex-smoke-accepted.

## Goal

Retain host/QEMU-substitute smoke evidence that the accepted local
sshservicediag runtime KEX integration exercises the real runtime KEX core,
reports fixed success and fail-closed labels, and still keeps ssh-ready false.

## Scope

- Added scripts/qemu-shell-ssh-runtime-kex-smoke.sh as the task-owned retained
  smoke gate.
- Added a focused service-readiness regression test for runtime KEX
  fail-closed labels without retaining secret evidence.
- Retained transcript evidence under
  tasks/evidence/2026-06-22-shell-ssh-runtime-kex-smoke/.
- Covered the accepted local listener/transport, remote identification,
  KEXINIT negotiation, host-key private-material readiness,
  OperatorSeededCsprng readiness, runtime KEX success label, and private
  encrypted-packet-state-ready label.
- Covered deterministic failure labels for CSPRNG-not-ready,
  host-key-not-ready, invalid peer public key, invalid host-key prerequisite
  state, disabled/prerequisite-missing service state, and transcript-invalid
  focused runtime crypto state.

## Non-goals

No NEWKEYS activation, encrypted packet I/O, authentication/session success,
shell attachment, live hardware/lab action, boot publication, live transport
reachability claim, OpenSSH/POSIX/Linux compatibility claim, broad expansion,
or phase transition is accepted. Retained evidence must not contain private
keys, random bytes, shared secrets, exchange hashes, derived keys, signature
bytes, public-key blobs, packet plaintext/ciphertext, tags, peer raw input,
operator identity, key-derived identifiers, or stable session identifiers.

## Findings

- fixed: retained smoke evidence now records the accepted real runtime KEX
  path through sshservicediag with crypto-backend-ready and
  encrypted-packet-state-ready labels while ssh-ready remains false.
- fixed: service-level regression coverage now records CSPRNG-not-ready,
  host-key-not-ready, invalid peer-public-key, and invalid host-key
  prerequisite fail-closed paths without exposing secret material.
- fixed: retained focused runtime crypto evidence covers transcript-invalid
  and missing-prerequisite labels from the private runtime crypto module.
- not-an-issue: the retained transcript contains public fixed labels, test
  names, validation command output, and non-goal labels only.
- deferred: NEWKEYS activation, encrypted packet I/O, user authentication,
  authorized-key parsing, session/channel allocation, PTY behavior, shell
  attachment, live reachability, OpenSSH/POSIX/Linux compatibility, hardware
  proof, broad expansion, and phase transition remain future work.

## Smoke Evidence

Script:

    scripts/qemu-shell-ssh-runtime-kex-smoke.sh

Retained transcript:

    tasks/evidence/2026-06-22-shell-ssh-runtime-kex-smoke/qemu-shell-ssh-runtime-kex-smoke.log

The transcript records:

    qemu-shell-ssh-runtime-kex-smoke: boundary=internal sshservicediag runtime KEX integration plus focused runtime crypto tests
    qemu-shell-ssh-runtime-kex-smoke: success=real-curve25519-sha256,accepted-host-key-signing-handle,operator-seeded-csprng,sshservicediag-crypto-backend-ready,sshservicediag-encrypted-packet-state-ready,ssh-ready=false
    qemu-shell-ssh-runtime-kex-smoke: fail-closed=sshservicediag-kex-csprng-not-ready,sshservicediag-kex-host-key-not-ready,sshservicediag-kex-peer-public-key-invalid,sshservicediag-kex-transcript-invalid,sshservicediag-prerequisites-missing,sshservicediag-crypto-backend-unaccepted,ssh-ready=false
    qemu-shell-ssh-runtime-kex-smoke: PASS classification=host-qemu-substitute-shell-ssh-runtime-kex-smoke-complete

The smoke gate invokes these focused filters:

- runtime_kex_integration_marks_crypto_ready_without_ssh_readiness
- runtime_kex_integration_reports_fail_closed_labels_without_secret_evidence
- runtime_kex_success_uses_real_crypto_and_private_packet_state_handles
- runtime_kex_fail_closed_labels_cover_missing_prerequisites
- host_key_private_material_maps_to_fail_closed_states
- host_key_private_material_clears_only_host_key_prerequisite
- exposure_disabled_state_fails_closed_without_service_caps
- exposure_enabled_with_missing_metadata_stays_prerequisites_missing

The current no_std custom test harness reported 756 tests passing during each
retained focused filter run.

## Redaction Review

Static redaction review: pass. The script and retained transcript contain only
task ids, public fixed labels, public algorithm names, non-goal labels, test
filter names, and validation command output. They do not retain private keys,
random bytes, shared secrets, exchange hashes, derived keys, signature bytes,
public-key blobs, packet plaintext/ciphertext, tags, peer raw input, operator
identity, key-derived identifiers, stable session identifiers, or live
transport identifiers.

## Validation

- scripts/qemu-shell-ssh-runtime-kex-smoke.sh: pass.
- cargo fmt --all -- --check: pass.
- cargo -Zjson-target-spec test --quiet: pass.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass with existing large search-index
  warning.
- git diff --cached --check: pass.

Validation levels: task-owned host/QEMU-substitute smoke transcript, focused
service/runtime crypto regression tests, full no_std QEMU/substitute test
suite, fmt check, docs build, and diff checks. No Pi 5 hardware run,
lab-controller API action, hardwareTestLock acquisition, boot publication,
NEWKEYS activation, encrypted packet I/O, authentication/session work, shell
attachment, live reachability claim, compatibility claim, broad expansion, or
phase transition was performed.

## Acceptance

Accepted. The retained host/QEMU-substitute smoke evidence covers the accepted
local runtime KEX success path through the real runtime crypto core and fixed
fail-closed labels while ssh-ready remains false. NEWKEYS activation,
encrypted packet I/O, authentication/session/shell behavior, live reachability,
public compatibility, hardware proof, broad expansion, and phase transition
remain unaccepted.

selected_next_task=phase12-ssh-runtime-kex-closeout-20260622.
