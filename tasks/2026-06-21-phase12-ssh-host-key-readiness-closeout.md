# Phase 12.5 SSH host-key readiness closeout

Task id: phase12-ssh-host-key-readiness-closeout-20260621

Status: accepted.

Classification: phase12-ssh-host-key-readiness-closeout-accepted.

## Goal

Close out the read-only VFS host-key metadata readiness slice after retained
sshkeydiag smoke evidence, without expanding into authorized-key storage,
persistence, SSH service behavior, transport, hardware reachability, or a phase
transition.

## Scope

- Reconciled the accepted host-key provisioning policy contract, metadata-only
  VFS implementation, and retained shell-visible diagnostic smoke evidence.
- Confirmed the current frontier: Talos can classify read-only VFS host-key
  metadata for /etc/talos/ssh/ssh_host_ed25519_key and clear only the host-key
  metadata prerequisite.
- Confirmed ssh-ready remains false because authorized-key metadata,
  persistence/exposure, SSH service behavior, live transport, and reachability
  remain unaccepted.
- Requested supervisor planning for the next Phase 12.5 prerequisite because no
  later queued Phase 12.5 task has complete objective dependencies and gates.

## Non-goals

- No source behavior changes.
- No authorized-key storage, writable persistence, SSH service, live transport,
  hardware reachability, public ABI/POSIX/Linux compatibility, broad expansion,
  stale link-ready discriminator promotion, or phase transition.
- No real private key, generated key, derived public key, digest, fingerprint,
  signature, or comparable stable secret identifier is retained in evidence.

## Findings

- fixed: docs now record the accepted host-key metadata frontier after the
  retained smoke evidence.
- fixed: closeout records planningNeeded=true because no explicit next
  Phase 12.5 prerequisite task exists with complete objective dependencies and
  gates.
- not-an-issue: the accepted host-key metadata slice uses labels, path names,
  public fixture state names, and byte-length buckets only; retained evidence
  contains no real private key, generated key, derived public key, digest,
  fingerprint, signature, or stable secret identifier.
- deferred: authorized-key metadata, persistence/exposure, SSH service
  behavior, live transport, hardware reachability, public
  ABI/POSIX/Linux compatibility, broad expansion, stale link-ready
  discriminator work, and phase transition remain future supervisor-planned
  work.

## Evidence Reviewed

- tasks/2026-06-21-phase12-ssh-host-key-provisioning-policy-contract.md
- tasks/2026-06-21-phase12-ssh-host-key-vfs-metadata-core.md
- tasks/2026-06-21-phase12-shell-ssh-host-keydiag-smoke.md
- tasks/evidence/2026-06-21-shell-ssh-host-keydiag-smoke/qemu-shell-ssh-host-keydiag-smoke.log
- docs/src/project/phase12-networking-ssh.md
- docs/src/roadmap.md

The reviewed policy selected operator-provisioned read-only VFS host-key
material at /etc/talos/ssh/ssh_host_ed25519_key. The accepted implementation
classifies only VFS metadata: missing, invalid/non-regular/zero-length/
oversized, insufficient length, and sufficient length. The retained smoke
transcript shows sufficient public-fixture metadata clearing only the host-key
prerequisite while ssh-ready remains false.

## Validation

- static task/docs/evidence review: pass.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass; existing large-search-index
  warning.
- git diff --cached --check: pass.

## Acceptance

- accepted: closeout reconciles the accepted host-key provisioning policy,
  metadata-only implementation, and retained sshkeydiag smoke evidence.
- accepted: findings are recorded with fixed, deferred, and not-an-issue
  dispositions.
- accepted: no next Phase 12.5 prerequisite task is mechanically unblocked;
  planningNeeded=true is required for supervisor planning.
- accepted: no authorized-key storage, writable persistence, SSH service, live
  transport, hardware reachability, public ABI/POSIX/Linux compatibility,
  broad expansion, stale link-ready discriminator, or phase-transition claim is
  accepted.
- accepted: no real private key, generated key, derived public key, digest,
  fingerprint, signature, or comparable stable secret identifier is retained in
  evidence.

selected_next_task=null.
