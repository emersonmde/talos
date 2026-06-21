# Phase 12.5 SSH authorized-key policy contract

Task id: phase12-ssh-authorized-key-policy-contract-20260621

Status: accepted.

Classification: phase12-ssh-authorized-key-policy-contract-accepted.

## Goal

Select the first authorized-key metadata source after accepted host-key
metadata readiness, without parsing keys, storing operator identities, accepting
authentication behavior, or exposing SSH.

## Scope

- Reviewed the accepted host-key metadata closeout, sshkeydiag readiness
  contract/core, operator-seeded CSPRNG closeout, Phase 12 SSH architecture
  notes, roadmap, and ADR index.
- Selected operator-provisioned read-only VFS authorized-key material as the
  smallest reversible next prerequisite.
- Reserved /etc/talos/ssh/authorized_keys as the first authorized-key path.
- Defined metadata-only states for the next implementation slice: missing,
  invalid, insufficient, and metadata-present.
- Selected phase12-ssh-authorized-key-vfs-metadata-core-20260621 as the next
  bounded task.

## Non-goals

- No runtime source behavior change.
- No authorized-key parsing, user authentication, writable storage, identity
  binding, persistence/exposure, SSH service behavior, live transport, hardware
  reachability, public ABI/POSIX/Linux compatibility, stale link-ready
  discriminator work, broad expansion, or phase transition.
- No real authorized public key, operator identity, fingerprint, digest,
  signature, key-derived identifier, private key, generated key, or comparable
  stable identifier is retained in evidence.

## Findings

- fixed: the next authorized-key prerequisite now has a concrete source policy
  instead of an ambiguous storage/authentication task.
- fixed: the first source path is /etc/talos/ssh/authorized_keys, matching a
  conventional operator-provisioned authorized_keys file while staying inside
  the accepted read-only VFS/generated-root boundary.
- fixed: the next implementation may classify only VFS metadata. Missing path
  keeps sshkeydiag-missing-authorized-key; non-regular, unreadable,
  zero-length, or greater-than-4096-byte material is invalid; readable regular
  material length 1 through 63 is insufficient; readable regular material
  length 64 through 4096 is metadata-present and may clear only the
  authorized-key metadata prerequisite.
- deferred: authorized-key parsing, accepted key formats, user/account binding,
  authentication policy, writable key storage, persistence/exposure, SSH
  service behavior, live transport, hardware reachability, public
  ABI/POSIX/Linux compatibility, stale link-ready discriminator work, broad
  expansion, and phase transition.
- not-an-issue: using a metadata-only read-only VFS policy is enough for the
  next prerequisite because sshkeydiag still reports ssh-ready false until
  persistence/exposure, service, transport, and reachability are accepted
  separately.

## Policy

Talos will use operator-provisioned read-only VFS material as the first
authorized-key source policy. The reserved path is:

- /etc/talos/ssh/authorized_keys

The path is intended for a future OpenSSH authorized_keys-compatible public-key
list, but this contract accepts no parsing, cryptographic verification,
operator identity, user-account binding, authentication decision, or stable key
identifier.

The next bounded implementation may classify only metadata for the read-only
VFS object:

- missing path: preserve sshkeydiag-missing-authorized-key;
- invalid metadata: non-regular, unreadable, zero-length, or greater-than-4096
  bytes;
- insufficient metadata: readable regular file length 1 through 63 bytes;
- metadata-present: readable regular file length 64 through 4096 bytes.

Metadata-present may clear only the authorized-key metadata prerequisite.
ssh-ready remains false until persistence/exposure, SSH service behavior, live
transport, and reachability are accepted separately. Diagnostics, shell output,
serial logs, docs, task evidence, and public surfaces must not retain, print,
digest, fingerprint, derive from, compare, or otherwise expose authorized-key
bytes, operator identity, key-derived identifiers, or comparable stable
identifiers.

## Evidence Reviewed

- tasks/2026-06-21-phase12-ssh-host-key-readiness-closeout.md
- tasks/2026-06-21-phase12-ssh-host-key-provisioning-policy-contract.md
- tasks/2026-06-21-phase12-ssh-host-key-vfs-metadata-core.md
- tasks/2026-06-21-phase12-shell-ssh-host-keydiag-smoke.md
- tasks/2026-06-21-phase12-ssh-key-management-readiness-contract.md
- tasks/2026-06-21-phase12-sshkeydiag-core.md
- tasks/2026-06-21-phase12-operator-seeded-csprng-closeout.md
- docs/src/project/phase12-networking-ssh.md
- docs/src/roadmap.md
- docs/src/decisions/README.md
- src/ssh_key_readiness.rs
- src/diagnostic_command.rs

## Validation

- static source/task/docs/evidence review: pass.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass; existing large-search-index
  warning.
- git diff --cached --check: pass.

## Acceptance

- accepted: /etc/talos/ssh/authorized_keys is the selected first
  authorized-key source path.
- accepted: the selected path is the smallest reversible next prerequisite
  because it reuses the accepted read-only VFS/generated-root boundary and does
  not require parsing, authentication, writable storage, service behavior, live
  transport, or reachability.
- accepted: the next implementation may classify missing, invalid,
  insufficient, and metadata-present states using only VFS metadata.
- accepted: metadata-present may clear only the authorized-key metadata
  prerequisite; ssh-ready remains false until persistence/exposure, SSH service
  behavior, live transport, and reachability are accepted separately.
- accepted: findings are recorded with fixed, deferred, and not-an-issue
  dispositions.
- accepted: no real authorized public key, operator identity, fingerprint,
  digest, signature, key-derived identifier, private key, generated key, or
  comparable stable identifier is retained in evidence.

selected_next_task=phase12-ssh-authorized-key-vfs-metadata-core-20260621.
