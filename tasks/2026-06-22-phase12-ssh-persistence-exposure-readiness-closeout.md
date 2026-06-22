# Phase 12.5 SSH persistence/exposure readiness closeout

Task id: phase12-ssh-persistence-exposure-readiness-closeout-20260622

Status: accepted.

Classification: phase12-ssh-persistence-exposure-readiness-closeout-planning-needed.

## Goal

Close out the metadata-only SSH persistence/exposure readiness slice after the
accepted policy contract, VFS metadata implementation, and retained
host/QEMU-substitute sshkeydiag smoke evidence.

## Scope

- Reconcile accepted persistence/exposure metadata work, deferred work,
  validation evidence, redaction posture, and current risks.
- Keep the accepted frontier limited to read-only VFS/initramfs metadata for
  generated-root SSH material and explicit operator exposure opt-in metadata.
- Record whether a mechanically unblocked next Phase 12.5/12.6 task exists.
- Update the Phase 12 SSH documentation and roadmap frontier.

## Non-goals

- No source behavior changes.
- No SSH service behavior, listener, authentication session, PTY/session
  plumbing, connection handling, live transport, packet I/O, hardware/lab
  action, boot publication, hardware reachability, public ABI/POSIX/Linux
  compatibility, stale link-ready discriminator promotion, broad expansion, or
  phase transition.
- No writable persistence claim; read-only generated-root/initramfs material is
  classified only as a first persistence metadata boundary.
- No real operator seed bytes, host private key bytes, authorized public key
  bytes, fingerprints, digests, signatures, generated key material, generated
  random byte streams, private CSPRNG state, operator identity,
  key-derived identifiers, or comparable stable identifiers in docs, task
  records, diagnostics, logs, or retained evidence.

## Findings

- fixed: the persistence/exposure slice now has a complete metadata-only chain:
  policy, source/unit implementation, and retained host/QEMU-substitute smoke
  evidence.
- fixed: docs now state the accepted frontier explicitly: sufficient
  generated-root metadata can clear only sshkeydiag-persistence-unavailable and
  a valid /etc/talos/ssh/exposure-enabled marker can clear only
  sshkeydiag-exposure-disabled.
- fixed: the closeout records planningNeeded=true because no later queued
  Phase 12.5/12.6 prerequisite task has complete objective dependencies and
  gates after this slice.
- deferred: SSH service behavior, listener/session/authentication plumbing,
  live transport, packet I/O, hardware reachability, writable persistence,
  durable key-store semantics, public ABI/POSIX/Linux compatibility, stale
  link-ready discriminator work, broad expansion, and phase transition.
- not-an-issue: retained metadata and smoke evidence are sufficient to close
  this diagnostic prerequisite because sshkeydiag-not-ready remains present and
  ssh-ready remains false until service behavior, live transport, and
  reachability are accepted separately.

## Evidence Reviewed

- tasks/2026-06-22-phase12-ssh-persistence-exposure-policy-contract.md
- tasks/2026-06-22-phase12-ssh-persistence-exposure-vfs-core.md
- tasks/2026-06-22-phase12-shell-ssh-persistence-exposure-diag-smoke.md
- tasks/evidence/2026-06-22-shell-ssh-persistence-exposure-diag-smoke/qemu-shell-ssh-persistence-exposure-diag-smoke.log
- src/ssh_key_readiness.rs
- src/diagnostic_command.rs
- docs/src/project/phase12-networking-ssh.md
- docs/src/roadmap.md

## Closeout

The accepted persistence/exposure readiness slice covers:

- a policy selecting read-only generated-root/initramfs metadata for
  /etc/talos/operator-seed.bin,
  /etc/talos/ssh/ssh_host_ed25519_key, and
  /etc/talos/ssh/authorized_keys as the first persistence metadata boundary;
- a policy selecting /etc/talos/ssh/exposure-enabled as the explicit operator
  exposure opt-in marker;
- VFS metadata classifiers that fail closed for missing or invalid material;
- retained host/QEMU-substitute smoke evidence covering default disabled
  exposure, missing exposure marker, invalid exposure marker, and sufficient
  public-fixture persistence/exposure metadata.

The accepted frontier remains metadata-only and diagnostic. Sufficient
persistence/exposure metadata may clear only
sshkeydiag-persistence-unavailable and sshkeydiag-exposure-disabled.
sshkeydiag-not-ready remains present and ssh-ready remains false because SSH
service behavior, live transport, and reachability remain unaccepted.

selected_next_task=null.

planningNeeded=true because no later queued Phase 12.5/12.6 prerequisite task
has complete objective dependencies and gates. Supervisor planning is required
before crypto dependency evaluation/adoption, SSH server implementation or
porting, service lifecycle, listener/session/authentication plumbing, live
transport, hardware reachability, public ABI/POSIX/Linux compatibility, stale
link-ready discriminator work, broad expansion, or phase transition.

## Redaction Review

The closeout retains labels, path names, public fixture state names, validation
commands, task references, and test-count summaries only. It does not retain
real operator seed bytes, host private key bytes, authorized public key bytes,
fingerprints, digests, signatures, generated key material, generated random
byte streams, private CSPRNG state, operator identity, key-derived identifiers,
or comparable stable identifiers.

## Validation

- static task/docs/evidence review: pass.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check: pass.

## Acceptance

- accepted: closeout reconciles accepted work, deferred work, docs, validation,
  and risks for the persistence/exposure metadata slice.
- accepted: findings are recorded with fixed, deferred, and not-an-issue
  dispositions.
- accepted: selected_next_task=null and planningNeeded=true are recorded because
  no mechanically unblocked next Phase 12.5/12.6 task exists.
- accepted: no SSH service behavior, live transport, hardware reachability,
  public ABI/POSIX/Linux compatibility, broad expansion, stale link-ready
  discriminator, or phase-transition claim is accepted.
- accepted: no secret/key/random bytes or stable secret/operator identifiers are
  retained.
