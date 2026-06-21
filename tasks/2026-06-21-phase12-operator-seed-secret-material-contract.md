# Phase 12.5 operator seed secret-material contract

Task id: phase12-operator-seed-secret-material-contract-20260621

Status: accepted.

Classification: phase12-operator-seed-secret-material-contract-accepted

## Goal

Define the narrow secret-material read boundary for
/etc/talos/operator-seed.bin so a later CSPRNG task can consume operator seed
bytes internally without exposing those bytes through diagnostics, logs, task
evidence, shell output, or comparable identifiers.

## Scope

- Reviewed the accepted operator seed VFS contract, source/unit implementation,
  retained diagnostic smoke, closeout, Phase 12.5 architecture docs, and
  read-only initramfs/generated-root model.
- Defined the only future internal consumer allowed to read operator seed
  bytes: a Talos-owned CSPRNG seed-conditioning/internal RNG component.
- Defined lifetime, copy, redaction, deterministic fixture, evidence, and
  failure-mode rules for that future secret-material consumer.
- Selected phase12-csprng-dependency-selection-contract-20260621 as the next
  bounded Phase 12.5 prerequisite task.

## Contract

/etc/talos/operator-seed.bin remains an optional immutable regular file in the
accepted read-only VFS/generated-root model:

- path: /etc/talos/operator-seed.bin;
- format: opaque raw bytes;
- minimum sufficient secret-material length: 32 bytes;
- maximum secret-material read length for the first CSPRNG slice: 4096 bytes;
- accepted diagnostics and task evidence: path, missing/invalid/insufficient/
  sufficient state, exact byte length, length bucket, redaction labels, and
  fail-closed readiness labels only;
- rejected diagnostics and task evidence: seed bytes, partial seed bytes,
  generated random bytes, CSPRNG internal state, actual digest, actual
  fingerprint, key-derivation output, or any stable identifier that can compare
  real operator secrets across boots.

Only a future Talos-owned CSPRNG seed-conditioning/internal RNG component may
read seed bytes. That component may read at most 4096 bytes from the accepted
read-only VFS path, use the bytes only as seed input to the selected CSPRNG/DRBG
strategy, and return only bounded readiness/error state plus metadata already
accepted by this contract. It must not expose a byte stream, seed digest,
fingerprint, or cross-boot comparable identifier to entropydiag, sshkeydiag,
shell output, serial logs, task evidence, or public ABI surfaces.

entropydiag, sshkeydiag, shell diagnostics, and task evidence remain
metadata-only. Sufficient seed material can satisfy only the operator seed
material prerequisite for a later CSPRNG task. It does not by itself accept
cryptographic strength, SSH readiness, host-key generation/provisioning,
authorized-key storage, writable persistence, SSH service behavior, live
transport, hardware reachability, public ABI/POSIX/Linux compatibility, broad
expansion, stale link-ready discriminator work, or a phase transition.

## Failure And Lifetime Rules

- missing path: fail closed; no byte read attempt; preserve existing missing
  seed diagnostics.
- invalid object, unreadable object, malformed VFS state, zero-length file, or
  oversized file: fail closed; do not condition the CSPRNG; diagnostics may
  expose only invalid/insufficient metadata.
- insufficient length 1 through 31 bytes: fail closed for cryptographic
  readiness; diagnostics may expose only insufficient metadata and byte length.
- sufficient length 32 through 4096 bytes: a future CSPRNG component may copy
  the bytes into a bounded stack or owned buffer only for seed conditioning,
  then clear or drop that buffer before returning.
- repeated reads: the first implementation should read once during CSPRNG
  initialization and retain only CSPRNG state/readiness metadata afterward.
  Re-reading, reseeding policy, and writable persistence require later tasks.
- errors: return not-ready/error state without preserving or printing secret
  bytes.

## Fixture And Evidence Policy

Source/unit tests may use deterministic public fixture bytes, including zeros
or ASCII patterns, to prove length, fail-closed behavior, CSPRNG API shape, and
redaction. Retained evidence may name those fixtures only as public fixtures
and may record lengths and labels. Real operator seed bytes must never be
checked into the repository, printed to serial, retained in transcripts, hashed
for evidence, fingerprinted, summarized, or stored as comparable identifiers.

## Findings

- fixed: secret-material access is now narrower than the accepted diagnostic
  metadata path; only a future CSPRNG seed-conditioning/internal RNG component
  may read seed bytes.
- fixed: minimum length, maximum read length, missing/invalid/insufficient
  failure modes, lifetime/copy expectations, deterministic public-fixture
  policy, and no-real-secret evidence policy are recorded.
- fixed: entropydiag, sshkeydiag, shell output, serial logs, and task evidence
  remain metadata-only and redacted.
- deferred: selecting the exact no_std CSPRNG/DRBG dependency strategy,
  implementing random-byte generation, accepting cryptographic strength,
  host-key generation/provisioning, authorized-key storage, writable seed
  persistence, SSH service behavior, live transport, hardware reachability, and
  phase transition work.
- not-an-issue: no runtime implementation, Cargo dependency edit, unit test,
  QEMU/substitute run, Pi 5 run, boot archive publication, or hardware lock is
  required for this contract-only task.

## Evidence

- static source/docs/evidence review:
  - src/entropy.rs
  - src/ssh_key_readiness.rs
  - src/initramfs.rs
  - scripts/qemu-shell-operator-seed-diag-smoke.sh
  - tasks/evidence/2026-06-21-shell-operator-seed-diag-smoke/qemu-shell-operator-seed-diag-smoke.log
  - tasks/2026-06-21-phase12-operator-seed-vfs-contract.md
  - tasks/2026-06-21-phase12-operator-seed-vfs-core.md
  - tasks/2026-06-21-phase12-shell-operator-seed-diag-smoke.md
  - tasks/2026-06-21-phase12-operator-seed-vfs-closeout.md
  - docs/src/project/phase12-networking-ssh.md
  - docs/src/roadmap.md
  - docs/src/decisions/README.md
- static review notes:
  - current diagnostics classify only VFS metadata and do not read or print seed
    bytes;
  - the retained smoke transcript records missing, insufficient, and sufficient
    metadata labels and redaction claims only;
  - generated-root/initramfs seed material remains read-only input, not writable
    persistence or a CSPRNG state file.

## Validation

- static source/docs/evidence review: pass.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check: pass.

## Acceptance

- accepted: /etc/talos/operator-seed.bin has a secret-material contract tied to
  the read-only VFS/generated-root model and narrower than the accepted
  metadata diagnostic path.
- accepted: only a future Talos-owned CSPRNG seed-conditioning/internal RNG
  component may read seed bytes, and only for bounded seed input.
- accepted: entropydiag, sshkeydiag, shell diagnostics, serial logs, task
  evidence, and public surfaces remain metadata-only.
- accepted: missing, invalid, insufficient, and oversized inputs fail closed;
  sufficient length is 32 through 4096 bytes; deterministic public fixtures are
  allowed for tests; real secret evidence is forbidden.
- accepted: no crypto dependency, random-byte generation,
  cryptographic-strength, host-key generation or provisioning, authorized-key
  storage, writable persistence, SSH service, live transport, hardware
  reachability, stale link-ready discriminator, broad expansion, or phase
  transition is accepted.

selected_next_task=phase12-csprng-dependency-selection-contract-20260621.

Promote phase12-csprng-dependency-selection-contract-20260621 on the next
worker wake if dependencies remain satisfied. That task may select an exact
no_std CSPRNG/DRBG dependency strategy and API boundary, but must still avoid
runtime random-byte generation until a later implementation task.
