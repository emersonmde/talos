# phase12-ssh-authorized-key-vfs-metadata-core-20260621

Status: accepted

## Goal

Implement the metadata-only authorized-key readiness boundary selected by
phase12-ssh-authorized-key-policy-contract-20260621.

## Scope

- Add source/unit changes that classify operator-provisioned read-only VFS
  authorized-key material at /etc/talos/ssh/authorized_keys.
- Keep diagnostics at the metadata boundary: missing, invalid, insufficient,
  or sufficient state, safe byte lengths in source metadata, and readiness
  labels only.
- Integrate authorized-key metadata with sshkeydiag so sufficient metadata
  clears only the authorized-key prerequisite.
- Keep ssh-ready false until persistence/exposure, service, transport, and
  reachability prerequisites are accepted separately.
- Preserve accepted host-key, operator seed, CSPRNG, and fail-closed default
  diagnostics.

## Non-goals

- No authorized-key parsing, key validation, fingerprinting, user/account
  model, authentication, host-key parsing/generation, writable persistence,
  SSH service behavior, live transport, hardware reachability, public
  ABI/POSIX/Linux compatibility, broad expansion, stale link-ready
  discriminator promotion, or phase transition.
- No real authorized public key, operator identity, fingerprint, digest,
  signature, key-derived identifier, private key, generated key, or comparable
  stable identifier retained in source comments, tests, docs, logs, or
  evidence.

## Implementation

- Added AUTHORIZED_KEY_PATH=/etc/talos/ssh/authorized_keys with metadata-only
  bounds of 64 through 4096 bytes.
- Added AuthorizedKeyMaterialMetadata and AuthorizedKeyMaterialState in
  src/ssh_key_readiness.rs.
- Added classify_authorized_key_material(ReadOnlyInitramfs), using VFS
  metadata only:
  - missing path -> sshkeydiag-missing-authorized-key remains present;
  - non-regular lookup, unreadable lookup error, zero length, or >4096 bytes
    -> sshkeydiag-authorized-key-invalid;
  - 1 through 63 bytes -> sshkeydiag-authorized-key-insufficient;
  - 64 through 4096 bytes -> authorized-key metadata prerequisite clears.
- Updated sshkeydiag to combine authorized-key metadata with accepted host-key,
  operator seed, and entropy metadata when called with read-only VFS metadata.

## Findings

- fixed: sshkeydiag previously had no way to distinguish invalid/insufficient
  authorized-key metadata from missing authorized-key metadata. Added explicit
  invalid and insufficient labels.
- fixed: the VFS-backed sshkeydiag path now classifies the accepted
  authorized-key path by metadata without parsing or retaining key bytes.
- fixed: source/unit tests cover missing, directory/non-regular, zero-length,
  oversized, insufficient, and sufficient authorized-key metadata using public
  fixture bytes only.
- fixed: dispatcher tests now prove sufficient authorized-key metadata clears
  only the authorized-key prerequisite while ssh-ready remains false.
- not-an-issue: default diagnostic dispatch still reports
  sshkeydiag-missing-authorized-key and ssh-ready false, preserving the
  fail-closed baseline.
- deferred: retained shell-visible smoke evidence is left to
  phase12-shell-ssh-authorized-keydiag-smoke-20260621.

## Evidence

- static source/task/docs/evidence review:
  - src/ssh_key_readiness.rs adds metadata-only authorized-key VFS
    classification and fail-closed labels.
  - src/diagnostic_command.rs feeds read-only VFS authorized-key metadata into
    sshkeydiag without key parsing or byte logging.
  - No real authorized public key, operator identity, fingerprint, digest,
    signature, key-derived identifier, private key, generated key, or
    comparable stable identifier is retained.
- focused source/unit validation:
  - cargo -Zjson-target-spec test sshkeydiag: pass; custom harness reported
    732 passed.
- full source/unit validation:
  - cargo -Zjson-target-spec test --quiet: pass; custom harness reported 732
    passed.
- static docs/diff validation:
  - cargo fmt --all -- --check: pass.
  - git diff --check: pass.
  - /home/node/.cargo/bin/mdbook build: pass.
  - git diff --cached --check: pass.

## Acceptance

- Missing path preserves sshkeydiag-missing-authorized-key.
- Invalid and oversized metadata report sshkeydiag-authorized-key-invalid.
- Insufficient metadata reports sshkeydiag-authorized-key-insufficient.
- Sufficient metadata clears only the authorized-key metadata prerequisite;
  ssh-ready remains false because persistence/exposure, service, transport,
  and reachability prerequisites remain unaccepted.
- Existing host-key, seed/CSPRNG, entropy, persistence, and exposure
  diagnostics remain compatible.
- selected_next_task=phase12-shell-ssh-authorized-keydiag-smoke-20260621.
