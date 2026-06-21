# Phase 12 SSH Key Diagnostic Core

Task id: phase12-sshkeydiag-core-20260621

Status: accepted

Classification:
phase12-sshkeydiag-core-accepted

Evidence level: source/unit implementation, focused diagnostic command tests,
full host/QEMU-substitute no_std test coverage, docs build, and diff checks.
No key generation, secret material persistence, crypto/SSH dependency adoption,
SSH service readiness, hardware/lab action, live packet I/O, hardware
reachability, public ABI/POSIX/Linux compatibility claim, broad expansion, or
phase transition was performed.

## Goal

Implement the accepted fail-closed SSH key-management readiness classifier and
diagnostic surface selected by
phase12-ssh-key-management-readiness-contract-20260621.

## Scope Performed

- Added src/ssh_key_readiness.rs with an explicit
  SshKeyReadinessSnapshot over host-key metadata, authorized-key metadata,
  entropy diagnostic report, seed material state, persistence state, and
  exposure state.
- Added deterministic fail-closed labels:
  sshkeydiag-missing-host-key, sshkeydiag-missing-authorized-key,
  sshkeydiag-entropy-unready, sshkeydiag-seed-material-missing,
  sshkeydiag-seed-material-insufficient, sshkeydiag-persistence-unavailable,
  sshkeydiag-exposure-disabled, and sshkeydiag-not-ready.
- Added the internal diagnostic command sshkeydiag, which reports the
  all-missing default as not ready and does not consume secret material or
  ambient hardware/lab state.
- Added focused source/unit tests for all accepted deterministic controls and
  the diagnostic command output.
- Updated Phase 12 docs and roadmap with the accepted diagnostic frontier and
  selected next task.

## Findings

- fixed: the key-readiness surface is a classifier over explicit metadata, not
  a key generator, key parser, seed store, crypto dependency, SSH server, or
  hardware probe.
- fixed: the default diagnostic reports sshkeydiag-not-ready and individual
  fail-closed labels for missing host key, missing authorized key, entropy
  unready, missing seed material, persistence unavailable, and exposure
  disabled.
- fixed: deterministic-control entropy and untrusted local entropy both keep
  SSH key readiness false.
- fixed: missing and insufficient seed material are represented by distinct
  labels.
- fixed: persistence unavailable and exposure disabled independently keep the
  report not ready.
- fixed: host-key and authorized-key metadata alone are insufficient; the
  negative control remains not ready when entropy, persistence, or exposure
  prerequisites are unavailable.
- deferred: host-key generation/provisioning, authorized-key parsing/storage,
  seed-file format, secret zeroization, crypto/DRBG selection, service
  lifecycle, authentication policy, time policy, heap-pressure limits, and SSH
  server implementation remain future work.
- rejected: key generation, secret persistence, crypto/SSH dependency adoption,
  SSH service readiness, live packet I/O, hardware reachability, hardware/lab
  action, public ABI/POSIX/Linux compatibility, broad expansion, and phase
  transition remain outside this task.
- removed: no source behavior, dependency, hardware/lab helper, task evidence,
  or documentation path was removed.
- not-an-issue: adding an internal diagnostic command is within the accepted
  key-management readiness contract and provides the retained surface needed by
  the planned smoke task.

## Diagnostic Output

The default sshkeydiag command output is:

```text
diag: ok sshkeydiag
diag: sshkey-readiness sshkeydiag-not-ready
diag: sshkey-label sshkeydiag-missing-host-key
diag: sshkey-label sshkeydiag-missing-authorized-key
diag: sshkey-label sshkeydiag-entropy-unready
diag: sshkey-label sshkeydiag-seed-material-missing
diag: sshkey-label sshkeydiag-persistence-unavailable
diag: sshkey-label sshkeydiag-exposure-disabled
diag: sshkey-label sshkeydiag-not-ready
diag: ssh-ready false
```

## Evidence

- Contract predecessor:
  tasks/2026-06-21-phase12-ssh-key-management-readiness-contract.md.
- Source: src/ssh_key_readiness.rs, src/diagnostic_command.rs, and
  src/main.rs.
- Docs: docs/src/project/phase12-networking-ssh.md and docs/src/roadmap.md.

## Validation

- cargo fmt --all -- --check: pass after rustfmt.
- focused cargo -Zjson-target-spec test ssh_key_readiness --quiet: pass.
- focused cargo -Zjson-target-spec test sshkeydiag --quiet: pass.
- cargo -Zjson-target-spec test --quiet: pass.
- git diff --check: pass.
- mdbook build: pass.
- git diff --cached --check: pass.

## Acceptance Check

- Implementation stays within the accepted readiness contract and records
  findings with disposition: satisfied.
- Default classifier/diagnostic state fails closed and reports SSH not ready
  because required key/authorization/entropy/persistence/exposure prerequisites
  are missing or disabled: satisfied.
- Focused tests prove deterministic readiness labels and negative controls
  without using secret material, ambient hardware, lab state, or randomness:
  satisfied.
- selected_next_task=phase12-shell-sshkeydiag-smoke-20260621 if a retained
  smoke/evidence task is mechanically unblocked: satisfied.
- No key generation, crypto dependency adoption, SSH service readiness, live
  packet I/O, hardware reachability, public ABI/POSIX/Linux compatibility,
  broad expansion, or phase-transition claim is accepted: satisfied.

## Next Action

Promote phase12-shell-sshkeydiag-smoke-20260621 on the next worker wake if
dependencies remain satisfied. Do not generate keys, persist secrets, adopt
crypto or SSH dependencies, expose SSH, touch hardware/lab state, accept SSH
readiness, or promote stale link-ready discriminator tasks.
