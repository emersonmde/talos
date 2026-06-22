# Phase 12.6 SSH listener/transport smoke

Task id: phase12-shell-ssh-listener-transport-smoke-20260622
Status: accepted
Owner: worker
Classification: phase12-shell-ssh-listener-transport-smoke-accepted.

## Goal

Retain host/QEMU-substitute smoke evidence that the accepted SSH
listener/transport diagnostic surface reports fail-closed disabled and
prerequisite-missing states, plus the local modeled listener/transport state,
without accepting crypto, authentication, sessions, shell attachment, hardware
reachability, public compatibility, broad expansion, or a phase transition.

## Scope

- Added task-owned smoke harness
  scripts/qemu-shell-ssh-listener-transport-smoke.sh.
- Retained transcript evidence under
  tasks/evidence/2026-06-22-ssh-listener-transport-smoke/.
- Corrected test expectations exposed by the smoke run:
  - fixed-size source label arrays now match MAX_SSH_SERVICE_READINESS_LABELS.
  - disabled diagnostic transcript follows classifier label order.
  - VFS-only sshservicediag fixture remains prerequisites-missing because
    cryptographic entropy is still unaccepted in that path.

## Findings

- fixed: The initial retained smoke harness could print PASS after a cargo
  failure because set -e was ineffective inside the shell if compound. The
  harness now returns from run_filter and exits the redirected subshell on
  failed filters, so failures are retained and stop the smoke.
- fixed: Two listener/transport source test expected arrays drifted from the
  classifier's fixed 16-label buffer after the core task added labels.
- fixed: Disabled sshservicediag expected labels were ordered differently from
  the classifier. The test now matches the classifier order:
  transport-unaccepted before dependency and crypto labels.
- fixed: The persistence/exposure diagnostic test expected shape-modeled local
  transport from VFS metadata alone. That path still lacks accepted
  cryptographic entropy, so it correctly stays prerequisites-missing with zero
  listener/transport counters.
- not-an-issue: The accepted local transport-modeled state is retained through
  the source/QEMU-substitute classifier tests. The shell-visible diagnostic
  path stays fail-closed until a later accepted crypto/service slice provides
  prerequisites through a diagnostic context.
- deferred: Runtime SSH crypto, key exchange, authentication/session success,
  PTY allocation, shell attachment, hardware reachability, and
  OpenSSH/POSIX/Linux compatibility remain separate future work.

## Evidence

- host/QEMU-substitute smoke transcript:
  tasks/evidence/2026-06-22-ssh-listener-transport-smoke/qemu-shell-ssh-listener-transport-smoke.log
- The transcript records:
  - disabled state: sshservicediag-exposure-disabled,
    sshservicediag-prerequisites-missing,
    sshservicediag-transport-unaccepted, zero listener/connection counters,
    and ssh-ready=false.
  - prerequisite-missing state: sshservicediag-prerequisites-missing,
    sshservicediag-transport-unaccepted, zero listener/connection counters,
    and ssh-ready=false.
  - local transport-modeled source state: sshservicediag-local-listener-modeled,
    sshservicediag-local-transport-modeled,
    sshservicediag-identification-banner-modeled,
    sshservicediag-remote-identification-valid,
    sshservicediag-transport-closed-before-kex, listener-count=1,
    transport-enabled=true, accepted-connection-count=1,
    authentication-success=false, shell-attached=false,
    reachability-accepted=false, and ssh-ready=false.
  - remote classifications: valid, invalid, over-limit.

## Redaction Review

Retained evidence contains fixed labels, counters, booleans, test names,
non-goal labels, and validation output only. It does not retain peer
identification text, peer addresses, key bytes, fingerprints, random bytes,
operator identity, key-derived identifiers, or stable transport/session
identifiers.

## Validation

- task-owned host/QEMU-substitute smoke script: pass.
- cargo -Zjson-target-spec test --quiet ssh_service_readiness: pass.
- cargo -Zjson-target-spec test --quiet sshservicediag: pass.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check: pass.

## Acceptance

Accepted. The retained smoke evidence covers the listener/transport modeled
state and fail-closed disabled/prerequisite states while keeping ssh-ready
false. No runtime SSH crypto, authentication/session success, shell attachment,
hardware reachability, public compatibility, broad expansion, or phase
transition is accepted.

selected_next_task=phase12-ssh-listener-transport-closeout-20260622.
