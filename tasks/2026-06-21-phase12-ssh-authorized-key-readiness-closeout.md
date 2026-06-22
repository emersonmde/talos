# Phase 12.5 SSH authorized-key readiness closeout

Task id: phase12-ssh-authorized-key-readiness-closeout-20260621

Status: accepted.

Classification: phase12-ssh-authorized-key-readiness-closeout-accepted.

## Goal

Reconcile the accepted authorized-key policy, metadata implementation, retained
smoke evidence, docs, deferred work, and current frontier.

## Scope

- Reconciled the accepted authorized-key source policy, metadata-only VFS
  implementation, and retained shell-visible diagnostic smoke evidence.
- Confirmed the current frontier: Talos can classify read-only VFS
  authorized-key metadata for /etc/talos/ssh/authorized_keys and clear only
  the authorized-key metadata prerequisite.
- Confirmed ssh-ready remains false because persistence/exposure, SSH service
  behavior, live transport, and reachability remain unaccepted.
- Requested supervisor planning for the next Phase 12.5 prerequisite because no
  later queued Phase 12.5 task has complete objective dependencies and gates.

## Non-goals

- No source behavior changes.
- No writable persistence, SSH service behavior, live transport, hardware
  reachability, public ABI/POSIX/Linux compatibility, broad expansion, stale
  link-ready discriminator promotion, or phase transition.
- No real authorized public key, operator identity, fingerprint, digest,
  signature, key-derived identifier, private key, generated key, or comparable
  stable identifier is retained in evidence.

## Findings

- fixed: docs now record the accepted authorized-key metadata frontier after
  retained smoke evidence.
- fixed: closeout records planningNeeded=true because no explicit next
  Phase 12.5 prerequisite task exists with complete objective dependencies and
  gates.
- not-an-issue: the accepted authorized-key metadata slice uses labels, path
  names, public fixture state names, and byte-length buckets only; retained
  evidence contains no real authorized public key, operator identity,
  fingerprint, digest, signature, key-derived identifier, private key,
  generated key, or comparable stable identifier.
- deferred: writable persistence, SSH service behavior, live transport,
  hardware reachability, public ABI/POSIX/Linux compatibility, broad expansion,
  stale link-ready discriminator work, and phase transition remain future
  supervisor-planned work.

## Evidence Reviewed

- tasks/2026-06-21-phase12-ssh-authorized-key-policy-contract.md
- tasks/2026-06-21-phase12-ssh-authorized-key-vfs-metadata-core.md
- tasks/2026-06-21-phase12-shell-ssh-authorized-keydiag-smoke.md
- tasks/evidence/2026-06-21-shell-ssh-authorized-keydiag-smoke/qemu-shell-ssh-authorized-keydiag-smoke.log
- docs/src/project/phase12-networking-ssh.md
- docs/src/roadmap.md

The reviewed policy selected operator-provisioned read-only VFS authorized-key
material at /etc/talos/ssh/authorized_keys. The accepted implementation
classifies only VFS metadata: missing, invalid/non-regular/unreadable/
zero-length/oversized, insufficient length, and sufficient length. The retained
smoke transcript shows sufficient public-fixture metadata clearing only the
authorized-key prerequisite while ssh-ready remains false.

## Validation

- static task/docs/evidence review: pass.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass; existing large-search-index
  warning.
- git diff --cached --check: pass.

## Acceptance

- accepted: closeout reconciles the accepted authorized-key policy,
  metadata-only implementation, and retained sshkeydiag smoke evidence.
- accepted: findings are recorded with fixed, deferred, and not-an-issue
  dispositions.
- accepted: no next Phase 12.5 prerequisite task is mechanically unblocked;
  planningNeeded=true is required for supervisor planning.
- accepted: no writable persistence, SSH service behavior, live transport,
  hardware reachability, public ABI/POSIX/Linux compatibility, broad expansion,
  stale link-ready discriminator, or phase-transition claim is accepted.
- accepted: no real authorized public key, operator identity, fingerprint,
  digest, signature, key-derived identifier, private key, generated key, or
  comparable stable identifier is retained in evidence.

selected_next_task=null.
