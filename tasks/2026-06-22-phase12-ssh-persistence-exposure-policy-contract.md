# Phase 12.5 SSH persistence/exposure policy contract

Task id: phase12-ssh-persistence-exposure-policy-contract-20260622

Status: accepted.

Classification: phase12-ssh-persistence-exposure-policy-contract-accepted.

## Goal

Define the smallest explicit persistence and operator-exposure metadata
prerequisite after accepted operator-seeded CSPRNG, host-key metadata, and
authorized-key metadata readiness, without accepting writable persistence, SSH
service behavior, live transport, or reachability.

## Scope

- Reviewed the accepted operator seed, operator-seeded CSPRNG, host-key
  metadata, authorized-key metadata, and sshkeydiag evidence chain.
- Selected a metadata-only persistence boundary for read-only generated-root
  SSH material.
- Selected an explicit operator exposure opt-in marker at
  /etc/talos/ssh/exposure-enabled.
- Defined exact persistence/exposure states, labels, paths, redaction rules,
  and the next implementation task.
- Selected phase12-ssh-persistence-exposure-vfs-core-20260622 as the next
  bounded task.

## Non-goals

- No runtime source behavior change.
- No SSH service behavior, listener, authentication session, PTY/session
  plumbing, connection handling, live transport, packet I/O, hardware/lab
  action, boot publication, hardware reachability, public ABI/POSIX/Linux
  compatibility, stale link-ready discriminator work, broad expansion, or
  phase transition.
- No writable persistence claim; read-only generated-root/initramfs material
  may be classified only as the first persistence metadata boundary.
- No real operator seed bytes, host private-key bytes, authorized public-key
  bytes, fingerprints, digests, signatures, generated key material, generated
  random byte streams, private CSPRNG state, operator identity, key-derived
  identifiers, or comparable stable identifiers are retained in evidence.

## Findings

- fixed: the persistence prerequisite now has an explicit first-slice policy:
  sufficient metadata for all accepted generated-root SSH material paths may
  clear only sshkeydiag-persistence-unavailable.
- fixed: the operator exposure prerequisite now has an explicit opt-in marker:
  /etc/talos/ssh/exposure-enabled.
- fixed: missing, invalid, or insufficient generated-root material keeps
  persistence unavailable; missing, invalid, non-regular, unreadable, or
  oversized exposure marker metadata keeps exposure disabled.
- deferred: writable key/seed persistence, durable deployment policy,
  SSH service behavior, listener/session/authentication plumbing, live
  transport, hardware reachability, public ABI/POSIX/Linux compatibility,
  stale link-ready discriminator work, broad expansion, and phase transition.
- not-an-issue: a metadata-only generated-root boundary is enough for the next
  diagnostic prerequisite because sshkeydiag remains not-ready and ssh-ready
  remains false until service behavior, live transport, and reachability are
  accepted separately.

## Policy

Talos will treat read-only generated-root/initramfs SSH material as the first
persistence metadata boundary. The persistence prerequisite is metadata-present
only when all accepted prerequisite files are present as sufficient metadata:

- /etc/talos/operator-seed.bin: regular readable file length 32 through 4096
  bytes under the accepted operator-seed contract;
- /etc/talos/ssh/ssh_host_ed25519_key: regular readable file length 64 through
  4096 bytes under the accepted host-key metadata contract;
- /etc/talos/ssh/authorized_keys: regular readable file length 64 through 4096
  bytes under the accepted authorized-key metadata contract.

If any of those paths are missing, invalid, unreadable, non-regular,
zero-length where invalid for that contract, insufficient, or oversized, the
persistence state remains unavailable and sshkeydiag must retain
sshkeydiag-persistence-unavailable. Metadata-present persistence may remove
only sshkeydiag-persistence-unavailable. It does not accept writable
persistence, durable key-store semantics, deployment policy, service behavior,
transport, or reachability.

Talos will require an explicit operator exposure opt-in marker before
diagnostics may clear the exposure prerequisite:

- /etc/talos/ssh/exposure-enabled

The marker is metadata-only. Missing path keeps exposure disabled.
Non-regular, unreadable, malformed VFS state, or greater-than-4096-byte marker
metadata also keeps exposure disabled. A regular readable marker of 0 through
4096 bytes is explicitly enabled; contents are ignored and must not be logged
or retained. Explicitly enabled exposure may remove only
sshkeydiag-exposure-disabled.

Even when persistence metadata is present and exposure is explicitly enabled,
sshkeydiag-not-ready remains present and ssh-ready remains false until SSH
service behavior, live transport, and reachability are accepted separately.
Diagnostics, shell output, serial logs, docs, task evidence, and public
surfaces must not retain, print, digest, fingerprint, derive from, compare, or
otherwise expose secret/key/random bytes, private CSPRNG state, operator
identity, key-derived identifiers, or comparable stable identifiers.

## Evidence Reviewed

- tasks/2026-06-21-phase12-operator-seeded-csprng-closeout.md
- tasks/2026-06-21-phase12-ssh-host-key-readiness-closeout.md
- tasks/2026-06-21-phase12-ssh-authorized-key-readiness-closeout.md
- tasks/2026-06-21-phase12-shell-ssh-authorized-keydiag-smoke.md
- docs/src/project/phase12-networking-ssh.md
- docs/src/roadmap.md
- src/ssh_key_readiness.rs
- src/diagnostic_command.rs

## Validation

- static source/task/docs/evidence review: pass.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check: pass.

## Acceptance

- accepted: generated-root metadata for /etc/talos/operator-seed.bin,
  /etc/talos/ssh/ssh_host_ed25519_key, and
  /etc/talos/ssh/authorized_keys is the selected first persistence metadata
  policy.
- accepted: /etc/talos/ssh/exposure-enabled is the selected explicit operator
  exposure opt-in marker.
- accepted: sufficient persistence/exposure metadata may remove only
  sshkeydiag-persistence-unavailable and sshkeydiag-exposure-disabled.
- accepted: sshkeydiag-not-ready remains present and ssh-ready remains false
  until SSH service behavior, live transport, and reachability are accepted
  separately.
- accepted: findings are recorded with fixed, deferred, and not-an-issue
  dispositions.
- accepted: no secret/key/random bytes, private CSPRNG state, operator
  identity, key-derived identifiers, or comparable stable identifiers are
  retained in evidence.

selected_next_task=phase12-ssh-persistence-exposure-vfs-core-20260622.
