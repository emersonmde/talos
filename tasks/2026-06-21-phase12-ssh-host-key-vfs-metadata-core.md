# phase12-ssh-host-key-vfs-metadata-core-20260621

Status: accepted

## Goal

Implement the metadata-only host-key readiness boundary selected by
phase12-ssh-host-key-provisioning-policy-contract-20260621.

## Scope

- Add source/unit changes that classify operator-provisioned read-only VFS host-key material at /etc/talos/ssh/ssh_host_ed25519_key.
- Keep diagnostics at the metadata boundary: missing, invalid, insufficient, or sufficient state, safe byte lengths in source metadata, and readiness labels only.
- Integrate host-key metadata with sshkeydiag so sufficient metadata clears only the host-key prerequisite.
- Keep ssh-ready false until authorized-key, persistence/exposure, service, transport, and reachability prerequisites are accepted separately.

## Non-goals

- No real host-key generation, private-key parsing, public-key derivation, fingerprinting, signing, authorized-key storage, writable persistence, SSH service behavior, live transport, hardware reachability, public ABI/POSIX/Linux compatibility, broad expansion, stale link-ready discriminator promotion, or phase transition.
- No retained real private key, generated key, derived public key, digest, fingerprint, signature, random byte stream, RNG state, or comparable stable secret identifier.
- No cryptographic-validity acceptance beyond the metadata policy selected by the provisioning contract.

## Implementation

- Added HOST_KEY_PATH=/etc/talos/ssh/ssh_host_ed25519_key with metadata-only bounds of 64 through 4096 bytes.
- Added HostKeyMaterialMetadata and HostKeyMaterialState in src/ssh_key_readiness.rs.
- Added classify_host_key_material(ReadOnlyInitramfs), using VFS metadata only:
  - missing path -> sshkeydiag-missing-host-key remains present;
  - non-regular lookup, unreadable lookup error, zero length, or >4096 bytes -> sshkeydiag-host-key-invalid;
  - 1 through 63 bytes -> sshkeydiag-host-key-insufficient;
  - 64 through 4096 bytes -> host-key metadata prerequisite clears.
- Broadened the diagnostic command internal context from operator-seed-only metadata to read-only VFS metadata, while preserving the public helper name for existing tests/smoke scripts.
- sshkeydiag now combines host-key VFS metadata, operator seed metadata, and entropy metadata when called with read-only VFS metadata.

## Findings

- fixed: sshkeydiag previously had no way to distinguish invalid/insufficient host-key metadata from missing host-key metadata. Added explicit invalid and insufficient labels.
- fixed: the VFS-backed sshkeydiag path only incorporated operator seed metadata. It now also classifies the accepted host-key path by metadata.
- fixed: source/unit tests now cover missing, directory/non-regular, zero-length, oversized, insufficient, and sufficient host-key metadata using public fixtures only.
- not-an-issue: default diagnostic dispatch still reports sshkeydiag-missing-host-key and ssh-ready false, preserving the fail-closed baseline.
- deferred: retained shell-visible smoke evidence is left to phase12-shell-ssh-host-keydiag-smoke-20260621.

## Evidence

- static source/task/docs/evidence review:
  - src/ssh_key_readiness.rs adds metadata-only host-key VFS classification and fail-closed labels.
  - src/diagnostic_command.rs feeds read-only VFS host-key metadata into sshkeydiag without key parsing or byte logging.
  - No private key bytes, generated key material, public-key derivation, fingerprints, digests, signatures, or stable secret identifiers are retained.
- focused source/unit validation:
  - cargo -Zjson-target-spec test ssh_key --quiet: pass; custom harness reported 729 passed.
- full source/unit validation:
  - cargo -Zjson-target-spec test --quiet: pass; custom harness reported 729 passed.
- static docs/diff validation:
  - cargo fmt --all -- --check: pass.
  - git diff --check: pass.
  - /home/node/.cargo/bin/mdbook build: pass with existing large-search-index warning.
  - git diff --cached --check: pass.

## Acceptance

- Default classifier/diagnostic state fails closed with sshkeydiag-missing-host-key.
- Invalid and oversized metadata report sshkeydiag-host-key-invalid.
- Insufficient metadata reports sshkeydiag-host-key-insufficient.
- Sufficient metadata clears only the host-key metadata prerequisite; ssh-ready remains false because authorized-key, persistence/exposure, service, transport, and reachability prerequisites remain unaccepted.
- selected_next_task=phase12-shell-ssh-host-keydiag-smoke-20260621.
