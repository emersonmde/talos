# Phase 12.5 operator seed VFS core

Task id: phase12-operator-seed-vfs-core-20260621

Status: accepted.

Classification: phase12-operator-seed-vfs-core-accepted

## Goal

Implement the source/unit operator seed material metadata path selected by the
accepted read-only VFS/initramfs contract, without accepting cryptographic
strength, writable persistence, host-key generation, an SSH service, hardware
reachability, or a phase transition.

## Scope

- Added the accepted operator seed path constant:
  /etc/talos/operator-seed.bin.
- Added metadata-only classification for missing, invalid, insufficient, and
  sufficient seed material.
- Removed the operator seed fingerprint field from the entropy observation
  shape so seed diagnostics retain only byte length metadata.
- Mapped VFS seed metadata into entropy and SSH key-readiness snapshots.
- Added focused source/unit tests for default missing seed material,
  insufficient and sufficient regular files, invalid directory and oversized
  objects, and SSH readiness label mapping.

## Findings

- fixed: Operator seed diagnostics now use the accepted
  /etc/talos/operator-seed.bin VFS path, 32-byte sufficient threshold, and
  4096-byte maximum diagnostic size.
- fixed: Seed metadata is length-only; the previous fingerprint field was
  removed because the accepted contract rejects stable digests, fingerprints,
  or cross-boot comparable secret identifiers.
- fixed: Missing seed material remains fail-closed and preserves the accepted
  default entropydiag and sshkeydiag behavior.
- fixed: Invalid or insufficient seed metadata does not clear SSH readiness;
  sshkeydiag maps invalid or insufficient material to
  sshkeydiag-seed-material-insufficient.
- fixed: Sufficient seed metadata clears only the seed-material prerequisite;
  cryptographic-strength and ssh-ready remain false.
- not-an-issue: Focused tests use deterministic public all-zero fixture bytes
  only as source/unit VFS lengths; retained diagnostics and task evidence expose
  no seed bytes, digest, fingerprint, or derived random material.
- deferred: Shell-visible retained transcripts for seed metadata cases,
  dedicated invalid-seed diagnostic labels, CSPRNG/conditioning, host-key
  generation/provisioning, authorized-key storage, writable persistence,
  crypto/SSH dependency adoption, SSH service behavior, live transport,
  hardware reachability, public ABI/POSIX/Linux compatibility, and broad
  expansion.

## Evidence

- Source changes:
  - src/entropy.rs adds OperatorSeedMaterialMetadata, the
    /etc/talos/operator-seed.bin path constant, length thresholds, VFS metadata
    classification, and entropy snapshot construction without reading or
    exposing seed bytes.
  - src/ssh_key_readiness.rs maps missing, invalid, insufficient, and
    sufficient seed metadata into existing fail-closed SSH key-readiness labels.
- Focused source/unit tests:
  - cargo -Zjson-target-spec test operator_seed --quiet: pass, 717 no_std tests
    executed by the project runner.
- Full no_std suite:
  - cargo -Zjson-target-spec test --quiet: pass, 717 tests.
- Redaction/static review:
  - no seed bytes are printed by diagnostics;
  - no seed digest or fingerprint is retained;
  - committed fixture bytes are deterministic public length fixtures only.

## Validation

- cargo fmt --all -- --check: pass.
- cargo -Zjson-target-spec test operator_seed --quiet: pass, 717 no_std tests.
- cargo -Zjson-target-spec test --quiet: pass, 717 tests.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check: pass.

## Rejected Claims

No random-byte generation, CSPRNG/conditioning, cryptographic-strength
acceptance, host-key generation or provisioning, authorized-key storage,
writable seed persistence, crypto/SSH dependency adoption, SSH service
behavior, live packet I/O, hardware/lab action, hardware reachability, public
ABI/POSIX/Linux compatibility, broad expansion, stale link-ready discriminator
promotion, or phase transition is accepted.

## Next Action

selected_next_task=phase12-shell-operator-seed-diag-smoke-20260621.

Promote phase12-shell-operator-seed-diag-smoke-20260621 on the next worker wake
if dependencies remain satisfied. The smoke task should retain shell-visible
entropydiag and sshkeydiag output for the accepted seed metadata cases without
printing seed bytes or accepting cryptographic-strength or SSH readiness.
