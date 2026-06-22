# Phase 12.6 SSH service readiness diagnostic core

Task id: phase12-ssh-service-readiness-diagnostic-core-20260622

Status: accepted.

Classification: phase12-ssh-service-readiness-diagnostic-core-accepted.

## Goal

Implement the smallest fail-closed SSH service readiness diagnostic model from
the accepted service-shape contract without adopting an SSH dependency, opening
a listener, accepting live transport, authenticating a user, attaching a shell,
or proving reachability.

## Scope

- Add only service readiness/lifecycle state modeling and diagnostic output.
- Combine accepted key-management, CSPRNG/entropy, persistence, and exposure
  readiness metadata without exposing secret/key/random bytes or stable
  identifiers.
- Retain source/unit and host/QEMU-substitute smoke evidence for disabled,
  prerequisites-missing, not-implemented, and shape-modeled/no-transport states.
- Keep ssh-ready false.

## Non-goals

- No SSH listener, socket bind/accept, TCP packet movement, handshake,
  encryption/MAC packet processing, authentication success, channel/session
  execution, PTY allocation, shell attachment, live transport, hardware/lab
  action, boot publication, reachability claim, or public ABI/POSIX/Linux
  compatibility claim.
- No generated host keys, authorized-key parsing/validation, signatures,
  fingerprints, digests, generated random byte streams, private CSPRNG state,
  operator identity, key-derived identifiers, or comparable stable identifiers
  in diagnostics, logs, or evidence.
- No writable persistence, broad service framework, stale link-ready
  discriminator promotion, broad expansion, or phase transition.

## Implementation

- Added src/ssh_service_readiness.rs, a fixed-label service readiness
  classifier over the existing SshKeyReadinessReport.
- Added sshservicediag to the internal diagnostic command dispatcher and
  command list.
- sshservicediag reports only fixed labels, lifecycle state, zero counters,
  false caps, and ssh-ready false:
  - sshservicediag-not-ready
  - sshservicediag-exposure-disabled
  - sshservicediag-prerequisites-missing
  - sshservicediag-shape-modeled
  - sshservicediag-dependency-unaccepted
  - sshservicediag-crypto-backend-unaccepted
  - sshservicediag-transport-unaccepted
  - sshservicediag-authentication-unimplemented
  - sshservicediag-session-unimplemented
  - listener-count=0
  - transport-enabled=false
  - accepted-connection-count=0
  - session-count=0
  - channel-count=0
  - authentication-success=false
  - shell-attached=false
  - reachability-accepted=false

## Findings

- fixed: implemented the accepted service-shape diagnostic as a separate
  sshservicediag surface so sshkeydiag remains focused on prerequisite
  metadata.
- fixed: modeled disabled, prerequisites-missing, and shape-modeled lifecycle
  states while always retaining fail-closed not-ready service labels.
- fixed: added dispatcher/source tests for missing/disabled, exposure-enabled
  prerequisites-missing, and fully prerequisite-satisfied/no-transport states.
- fixed: retained task-owned host/QEMU-substitute smoke evidence for the new
  diagnostic command.
- not-an-issue: the diagnostic composes existing key-readiness labels instead
  of reading key/seed material or creating a service framework.
- deferred: russh dependency adoption or Talos porting, runtime SSH crypto
  backend, listener/transport integration, host-key loading/parsing,
  authorized-key parsing, user/account policy, real authentication, packet
  processing, PTTY/session/shell attachment, reachability proof, and runtime
  buffer/window/algorithm limits.

## Evidence

- Source/unit evidence:
  - cargo -Zjson-target-spec test --quiet ssh_service_readiness: pass.
  - cargo -Zjson-target-spec test --quiet sshservicediag: pass.
  - cargo -Zjson-target-spec test --quiet: pass, 740 talos no_std tests.
- Retained host/QEMU-substitute diagnostic smoke:
  - scripts/qemu-shell-ssh-service-diag-smoke.sh: pass.
  - tasks/evidence/2026-06-22-ssh-service-readiness-diagnostic-core/qemu-shell-ssh-service-diag-smoke.log.
- Static review:
  - no generated host keys, authorized-key bytes, fingerprints, signatures,
    digests, generated random byte streams, private CSPRNG state, operator
    identity, key-derived identifiers, stable transport/session identifiers, or
    comparable stable identifiers are retained.

## Validation

- cargo fmt --all -- --check: pass.
- cargo -Zjson-target-spec test --quiet ssh_service_readiness: pass.
- cargo -Zjson-target-spec test --quiet sshservicediag: pass.
- scripts/qemu-shell-ssh-service-diag-smoke.sh: pass.
- cargo -Zjson-target-spec test --quiet: pass, 740 tests.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check: pass.

## Acceptance

- Implementation matches the accepted service-shape contract and remains
  fail-closed.
- Source/unit tests and retained host/QEMU-substitute smoke cover
  missing/disabled, not-implemented, and fully prerequisite-satisfied but
  no-transport states.
- Diagnostics do not expose secret/key/random bytes, fingerprints, digests,
  signatures, private CSPRNG state, operator identity, key-derived identifiers,
  or comparable stable identifiers.
- No listener, live transport, authentication/session success, shell
  attachment, hardware reachability, public ABI/POSIX/Linux compatibility,
  broad expansion, or phase transition is accepted.

selected_next_task=phase12-ssh-service-readiness-closeout-20260622.
