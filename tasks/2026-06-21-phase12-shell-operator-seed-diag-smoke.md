# Phase 12.5 operator seed diagnostic smoke

Task id: phase12-shell-operator-seed-diag-smoke-20260621

Status: accepted.

Classification: phase12-shell-operator-seed-diag-smoke-accepted

## Goal

Retain host/QEMU-substitute shell-visible diagnostic evidence that the accepted
read-only VFS operator seed metadata path is observable through entropydiag and
sshkeydiag without exposing seed bytes or accepting cryptographic strength, SSH
readiness, key generation, writable persistence, service behavior, hardware
reachability, or a phase transition.

## Scope

- Added a metadata-aware diagnostic dispatch helper for explicit read-only
  initramfs operator seed material.
- Added focused diagnostic-command tests for missing, insufficient, and
  sufficient operator seed metadata.
- Added a task-owned retained smoke script and transcript.
- Kept the default diagnostic command path fail-closed for the existing Phase 8
  fixture.

## Findings

- fixed: diagnostic command dispatch can now format entropydiag and sshkeydiag
  output from an explicit read-only initramfs seed metadata context.
- fixed: missing seed material preserves entropydiag-operator-seed-required and
  sshkeydiag-seed-material-missing.
- fixed: insufficient seed material remains not ready and reports
  sshkeydiag-seed-material-insufficient.
- fixed: sufficient seed metadata clears only the seed-material label;
  cryptographic-strength and ssh-ready remain false.
- not-an-issue: smoke fixtures use deterministic public all-zero byte arrays
  only to establish VFS file length; diagnostics and transcripts do not print
  seed bytes, digest, fingerprint, or derived material.
- deferred: random-byte generation, CSPRNG/conditioning, host-key generation or
  provisioning, authorized-key storage, writable seed persistence, crypto/SSH
  dependency adoption, SSH service behavior, live transport, hardware
  reachability, public ABI/POSIX/Linux compatibility, broad expansion, and
  phase transition remain future work.

## Smoke Evidence

Script:

    scripts/qemu-shell-operator-seed-diag-smoke.sh

Retained transcript:

    tasks/evidence/2026-06-21-shell-operator-seed-diag-smoke/qemu-shell-operator-seed-diag-smoke.log

The retained transcript records:

    qemu-shell-operator-seed-diag-smoke: boundary=diagnostic command dispatch over explicit read-only initramfs operator seed metadata
    qemu-shell-operator-seed-diag-smoke: missing-seed=entropydiag-fail-closed-no-input,entropydiag-operator-seed-required,sshkeydiag-seed-material-missing,cryptographic-strength=false,ssh-ready=false
    qemu-shell-operator-seed-diag-smoke: insufficient-seed=entropydiag-untrusted-local-mix,sshkeydiag-seed-material-insufficient,cryptographic-strength=false,ssh-ready=false
    qemu-shell-operator-seed-diag-smoke: sufficient-seed=entropydiag-untrusted-local-mix,no-seed-material-label,cryptographic-strength=false,ssh-ready=false
    qemu-shell-operator-seed-diag-smoke: redaction=no-seed-bytes,no-seed-digest,no-seed-fingerprint,no-secret-material
    qemu-shell-operator-seed-diag-smoke: PASS classification=host-qemu-substitute-shell-operator-seed-diag-smoke-complete

The smoke gate runs:

- dispatcher_reports_operator_seed_missing_from_vfs_without_secret_material
- dispatcher_reports_operator_seed_insufficient_from_vfs_without_secret_material
- dispatcher_reports_operator_seed_sufficient_from_vfs_without_secret_material

The underlying no_std test harness reported all tests passing in the retained
transcript.

## Validation

- scripts/qemu-shell-operator-seed-diag-smoke.sh: pass.
- cargo fmt --all -- --check: pass after rustfmt applied one wrapping fix.
- cargo -Zjson-target-spec test --quiet: pass, 720 no_std tests.
- static redaction review: pass; retained smoke transcript and diagnostic
  output contain no seed bytes, seed digest, seed fingerprint, secret material,
  cryptographic-strength true, or ssh-ready true claims.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass; existing warning that the search
  index is very large.
- git diff --cached --check: pending before commit.

## Acceptance

- accepted: retained smoke evidence exercises entropydiag and sshkeydiag output
  for missing, insufficient, and sufficient operator seed metadata through the
  explicit read-only initramfs diagnostic context.
- accepted: missing, insufficient, and sufficient metadata cases keep
  cryptographic-strength false and ssh-ready false.
- accepted: no seed bytes, digest, fingerprint, derived material, random-byte
  generation, CSPRNG/conditioning, host-key generation or provisioning,
  authorized-key storage, writable seed persistence, crypto/SSH dependency
  adoption, SSH service behavior, live packet I/O, hardware/lab action,
  hardware reachability, public ABI/POSIX/Linux compatibility, broad expansion,
  stale link-ready discriminator promotion, or phase transition is accepted.

selected_next_task=phase12-operator-seed-vfs-closeout-20260621.
