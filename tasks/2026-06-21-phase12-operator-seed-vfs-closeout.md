# Phase 12.5 operator seed VFS closeout

Task id: phase12-operator-seed-vfs-closeout-20260621

Status: accepted.

Classification: phase12-operator-seed-vfs-closeout-accepted

## Goal

Close out the read-only VFS/initramfs operator seed diagnostic slice by
reconciling the accepted contract, source/unit implementation, retained
shell-visible smoke evidence, docs, deferred work, and rejected claims.

## Scope

- Reviewed the accepted operator seed VFS contract, core implementation task,
  retained shell-visible smoke task, Phase 12.5 docs, and roadmap entries.
- Reconciled the accepted diagnostic frontier: operator seed metadata is visible
  to entropydiag and sshkeydiag, but only as missing, invalid, insufficient, or
  sufficient length state plus byte length.
- Confirmed no existing queued Phase 12.5 prerequisite task is mechanically
  unblocked after this closeout.

## Findings

- fixed: closeout docs now record that the operator seed slice is complete at a
  diagnostic metadata boundary and requires supervisor planning for the next
  Phase 12.5 prerequisite.
- not-an-issue: the accepted smoke transcript is redacted; it contains no seed
  bytes, seed digest, seed fingerprint, secret material, cryptographic-strength
  true, or ssh-ready true claim.
- not-an-issue: the accepted implementation uses deterministic public fixtures
  only to exercise VFS file length metadata in source/unit and smoke evidence.
- deferred: cryptographic entropy, random-byte generation,
  CSPRNG/conditioning, crypto dependency evaluation/adoption, host-key
  generation or provisioning, authorized-key storage, writable seed
  persistence, SSH service behavior, live transport, hardware reachability,
  public ABI/POSIX/Linux compatibility, broad expansion, and phase transition
  remain future work.
- deferred: stale link-ready discriminator tasks remain blocked by missing
  selected discriminator and selected_next_task evidence from the earlier
  source-contract task; this closeout does not unblock or promote them.

## Reconciled Evidence

Accepted predecessor tasks:

- tasks/2026-06-21-phase12-operator-seed-vfs-contract.md:
  phase12-operator-seed-vfs-contract-accepted.
- tasks/2026-06-21-phase12-operator-seed-vfs-core.md:
  phase12-operator-seed-vfs-core-accepted.
- tasks/2026-06-21-phase12-shell-operator-seed-diag-smoke.md:
  phase12-shell-operator-seed-diag-smoke-accepted.

Retained smoke transcript:

    tasks/evidence/2026-06-21-shell-operator-seed-diag-smoke/qemu-shell-operator-seed-diag-smoke.log

Accepted frontier:

- /etc/talos/operator-seed.bin is an optional read-only VFS/initramfs file of
  opaque raw bytes.
- Diagnostics may expose only metadata: missing, invalid, insufficient, or
  sufficient state plus byte length.
- The sufficient threshold is 32 bytes and the first diagnostic-read limit is
  4096 bytes.
- entropydiag can clear only entropydiag-operator-seed-required when sufficient
  seed metadata is present; cryptographic-strength remains false.
- sshkeydiag can clear only the seed-material label when sufficient seed
  metadata is present; ssh-ready remains false because host-key,
  authorized-key, persistence/exposure, crypto/SSH service, and reachability
  prerequisites remain unaccepted.

## Validation

- static task/docs/evidence review: pass.
- jq empty on task-owned JSON evidence: not applicable; no JSON evidence was
  created for this closeout.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check: pass.

## Acceptance

- accepted: the operator seed material slice is closed at the read-only
  diagnostic metadata boundary.
- accepted: docs now reconcile accepted contract, core implementation, retained
  smoke evidence, deferred work, and rejected claims.
- accepted: no cryptographic-strength, random-byte generation,
  CSPRNG/conditioning, host-key generation or provisioning, authorized-key
  storage, writable seed persistence, crypto/SSH dependency adoption, SSH
  service behavior, live transport, hardware/lab action, hardware reachability,
  public ABI/POSIX/Linux compatibility, broad expansion, stale link-ready
  discriminator promotion, or phase transition is accepted.
- accepted: no explicit queued Phase 12.5 prerequisite task is mechanically
  unblocked after this closeout.

selected_next_task=null.

planningNeeded=true: supervisor planning is required to define the next bounded
Phase 12.5 prerequisite before crypto dependency evaluation, host-key
provisioning policy, writable persistence, SSH service work, live transport,
hardware reachability, stale link-ready discriminator work, broad expansion, or
phase transition.
