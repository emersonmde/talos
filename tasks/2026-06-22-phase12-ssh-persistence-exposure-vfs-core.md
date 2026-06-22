# phase12-ssh-persistence-exposure-vfs-core-20260622

Status: accepted

Classification: phase12-ssh-persistence-exposure-vfs-core-accepted

## Goal

Implement the metadata-only read-only VFS persistence/exposure classification
selected by phase12-ssh-persistence-exposure-policy-contract-20260622.

## Scope

- Wire sshkeydiag's read-only VFS metadata path so sufficient generated-root
  operator seed, host-key, and authorized-key metadata can clear only the
  persistence-unavailable label.
- Add read-only VFS metadata classification for
  /etc/talos/ssh/exposure-enabled so missing or invalid metadata keeps
  exposure disabled and a valid explicit marker clears only the
  exposure-disabled label.
- Add deterministic source/unit coverage for persistence and exposure
  combinations.
- Keep the boundary metadata-only and fail-closed.

## Non-goals

- No SSH service behavior, listener, authentication session, PTY/session
  plumbing, connection handling, live transport, packet I/O, hardware/lab
  action, boot publication, hardware reachability, public ABI/POSIX/Linux
  compatibility, stale link-ready discriminator work, broad expansion, or
  phase transition.
- No writable persistence claim; read-only generated-root/initramfs material
  is classified only as a first persistence metadata boundary.
- No parsing or validation of real private-key or authorized-key formats.
- No real operator seed bytes, host private-key bytes, authorized public-key
  bytes, fingerprints, digests, signatures, generated key material, generated
  random byte streams, private CSPRNG state, operator identity, key-derived
  identifiers, or comparable stable identifiers retained in source comments,
  tests, docs, logs, or evidence.

## Implementation

- Added EXPOSURE_MARKER_PATH=/etc/talos/ssh/exposure-enabled and a
  metadata-only maximum of 4096 bytes in src/ssh_key_readiness.rs.
- Added classify_persistence_metadata(...), which reports metadata-present
  only when accepted operator seed, host-key, and authorized-key metadata are
  all sufficient.
- Added classify_exposure_marker(ReadOnlyInitramfs), which treats missing,
  lookup errors, non-regular metadata, and oversized marker metadata as
  disabled, and treats a regular readable 0 through 4096 byte marker as
  explicitly enabled without reading or retaining contents.
- Updated src/diagnostic_command.rs so the read-only VFS sshkeydiag path feeds
  persistence and exposure metadata into SshKeyReadinessSnapshot.
- Added source/unit tests for all-material persistence, missing/invalid/enabled
  exposure marker metadata, and shell-visible sshkeydiag label effects.

## Findings

- fixed: the VFS-backed sshkeydiag path previously left persistence unavailable
  even when seed, host-key, and authorized-key metadata were all sufficient.
  Persistence now clears only under the accepted all-three metadata condition.
- fixed: the VFS-backed sshkeydiag path previously had no source-backed
  exposure marker. It now classifies /etc/talos/ssh/exposure-enabled
  metadata and keeps exposure disabled unless the marker is valid.
- fixed: focused tests now prove missing/invalid exposure marker metadata keeps
  sshkeydiag-exposure-disabled, while a valid marker clears only that label.
- fixed: focused tests now prove sufficient public-fixture seed, host-key, and
  authorized-key metadata clears sshkeydiag-persistence-unavailable without
  clearing sshkeydiag-not-ready or making ssh-ready true.
- not-an-issue: the default fail-closed diagnostic output remains unchanged.
- deferred: retained shell-visible smoke evidence is left to
  phase12-shell-ssh-persistence-exposure-diag-smoke-20260622.
- deferred: SSH service behavior, live transport, reachability, writable
  persistence, public ABI/POSIX/Linux compatibility, stale link-ready
  discriminator work, broad expansion, and phase transition remain outside this
  task.

## Evidence

- static source/task/docs/evidence review:
  - src/ssh_key_readiness.rs owns the metadata-only persistence and exposure
    classifiers.
  - src/diagnostic_command.rs wires those classifiers into sshkeydiag only for
    explicit read-only VFS metadata contexts.
  - No secret/key/random bytes, private CSPRNG state, operator identity,
    key-derived identifiers, fingerprints, digests, signatures, or comparable
    stable identifiers are retained.

## Validation

- cargo -Zjson-target-spec test ssh_key_readiness --quiet: pass; custom harness
  reported 736 passed after one fixture-index correction.
- cargo -Zjson-target-spec test diagnostic_command --quiet: pass; custom
  harness reported 736 passed.
- cargo -Zjson-target-spec test --quiet: pass; custom harness reported 736
  passed.
- cargo fmt --all -- --check: pass.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass with existing large-search-index
  warning.
- git diff --cached --check: pass.

## Acceptance

- Missing or invalid exposure marker metadata keeps
  sshkeydiag-exposure-disabled.
- Sufficient public-fixture operator seed, host-key, authorized-key, and
  explicit exposure metadata remove only sshkeydiag-persistence-unavailable
  and sshkeydiag-exposure-disabled.
- sshkeydiag-not-ready remains present and ssh-ready remains false in all new
  cases.
- Default fail-closed diagnostic output remains unchanged.
- No secret/key/random bytes or stable secret/operator identifiers are logged
  or retained.
- selected_next_task=phase12-shell-ssh-persistence-exposure-diag-smoke-20260622.
