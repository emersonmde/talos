# Phase 12.5 operator seed VFS contract

Task id: phase12-operator-seed-vfs-contract-20260621

Status: accepted.

Classification: phase12-operator-seed-vfs-contract-accepted

## Goal

Define the smallest read-only VFS/initramfs operator seed material contract
that lets entropy and SSH key-readiness diagnostics distinguish missing,
insufficient, and provisioned seed material without accepting cryptographic
strength, writable persistence, host-key generation, an SSH service, hardware
reachability, or a phase transition.

## Scope

- Reviewed the accepted Phase 8 read-only initramfs/VFS model, Phase 10
  generated-root image policy, accepted entropydiag classifier, accepted
  sshkeydiag classifier, retained sshkeydiag smoke evidence, and Phase 12.5
  architecture docs.
- Defined the read-only seed path as /etc/talos/operator-seed.bin.
- Defined metadata-only diagnostic handling for missing, invalid, insufficient,
  and sufficient-length seed material.
- Recorded redaction and no-secret-logging requirements before any source/unit
  integration.
- Selected phase12-operator-seed-vfs-core-20260621 as the next bounded
  implementation task.

## Contract

The operator seed file is an optional immutable regular file in the accepted
read-only initramfs/generated-root VFS model:

- path: /etc/talos/operator-seed.bin;
- kind: regular file only;
- format: opaque raw bytes, not parsed as text, JSON, DER, PEM, or key
  material;
- minimum sufficient length: 32 bytes;
- maximum diagnostic-read length for the first implementation: 4096 bytes;
- accepted metadata exposed to diagnostics: present/missing/invalid state,
  exact byte length, and length bucket missing, invalid, insufficient, or
  sufficient;
- rejected metadata for logs, diagnostics, task records, and shell output:
  seed bytes, partial seed bytes, derived random bytes, actual seed digest,
  actual seed fingerprint, key derivation output, or any stable identifier that
  could help compare operator secrets across boots.

The file is operator-provisioned input carried through the read-only root. It
is not writable seed persistence, generated randomness, a CSPRNG state file, a
host key, an authorized key, or proof that the boot has cryptographic entropy.

## Diagnostic Mapping

- missing: lookup of /etc/talos/operator-seed.bin returns ENOENT. entropydiag
  continues to report entropydiag-operator-seed-required and ssh-ready false;
  sshkeydiag reports sshkeydiag-seed-material-missing and remains not ready.
- invalid: the path resolves to a directory, unsupported object, unreadable
  object, malformed VFS state, or a regular file larger than the fixed
  diagnostic-read limit. Diagnostics must not read or print any seed bytes;
  the implementation may classify this as insufficient until a later contract
  adds a dedicated invalid label.
- insufficient: the path resolves to a regular file of length 1 through 31
  bytes. entropydiag may report that operator seed material was provisioned
  while cryptographic-strength remains false and ssh-ready remains false;
  sshkeydiag reports sshkeydiag-seed-material-insufficient.
- sufficient metadata: the path resolves to a regular file of length 32 through
  4096 bytes. entropydiag may clear only the operator-seed-required indication
  and still reports cryptographic-strength false and ssh-ready false.
  sshkeydiag may clear only the seed-material-missing/insufficient labels; it
  remains not ready unless later tasks separately accept host-key metadata,
  authorized-key metadata, cryptographic entropy, persistence/exposure policy,
  crypto dependency behavior, and service readiness.

The first implementation may use deterministic non-secret fixture files for
unit tests, including all-zero or ASCII fixture bytes, as long as the retained
logs and diagnostics show only path, length, and labels. Real operator secrets
must not be checked into the repository, printed to serial, retained in task
evidence, or summarized by stable digest/fingerprint.

## Findings

- fixed: Operator seed material now has a concrete VFS path and raw byte-file
  shape that the next task can implement without choosing policy mid-task.
- fixed: Missing, invalid, insufficient, and sufficient-length cases have
  deterministic diagnostic dispositions.
- fixed: The sufficient-length case explicitly clears only the seed-material
  prerequisite; cryptographic-strength and SSH-readiness remain false.
- fixed: Redaction rules reject raw bytes, partial bytes, real digests, real
  fingerprints, derived bytes, and cross-boot comparable secret identifiers in
  logs or task evidence.
- deferred: Dedicated invalid-seed diagnostic labels, CSPRNG/conditioning,
  host-key generation or provisioning, authorized-key storage, writable seed
  persistence, crypto/SSH dependency adoption, service lifecycle, live packet
  transport, Pi 5 hardware proof, and exposure controls.
- not-an-issue: No runtime code, QEMU/substitute run, Pi 5 run, boot archive
  publication, or hardware lock is required for this contract-only task.

## Evidence

- static source/docs/evidence review:
  - src/entropy.rs
  - src/ssh_key_readiness.rs
  - src/initramfs.rs
  - docs/src/project/phase8-readonly-initramfs-vfs-contract.md
  - docs/src/project/phase10-generated-userland-image-contract.md
  - tasks/2026-06-21-phase12-entropy-source-contract.md
  - tasks/2026-06-21-phase12-entropydiag-core.md
  - tasks/2026-06-21-phase12-ssh-key-management-readiness-contract.md
  - tasks/2026-06-21-phase12-sshkeydiag-core.md
  - tasks/2026-06-21-phase12-shell-sshkeydiag-smoke.md
- validation:
  - static source/docs/evidence review: pass.
  - git diff --check: pass.
  - /home/node/.cargo/bin/mdbook build: pass.
  - git diff --cached --check: pass.

## Rejected Claims

- No random-byte generation, CSPRNG/conditioning, cryptographic-strength
  acceptance, host-key generation or provisioning, authorized-key storage,
  writable seed persistence, crypto/SSH dependency adoption, SSH service
  behavior, live packet I/O, hardware/lab action, hardware reachability, public
  ABI/POSIX/Linux compatibility, broad expansion, stale link-ready
  discriminator promotion, or phase transition is accepted.

## Next Action

selected_next_task=phase12-operator-seed-vfs-core-20260621.

Promote phase12-operator-seed-vfs-core-20260621 on the next worker wake if
dependencies remain satisfied. The implementation must follow this path,
length, redaction, and rejected-claim contract exactly.
