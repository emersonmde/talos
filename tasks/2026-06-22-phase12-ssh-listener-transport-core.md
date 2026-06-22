# Phase 12.6 SSH listener/transport core

Task id: phase12-ssh-listener-transport-core-20260622

Status: accepted.

Classification: phase12-ssh-listener-transport-core-accepted.

## Goal

Implement the accepted bounded local SSH listener/transport contract over
Talos-owned private socket/readiness and banner-classification surfaces while
keeping ssh-ready false.

## Scope

- Implement one local modeled SSH service endpoint in the SSH service readiness
  model using the accepted descriptor-backed AF_INET/SOCK_STREAM socket table.
- Cover disabled, prerequisite-missing, valid modeled remote identification,
  invalid modeled remote identification, and over-limit modeled remote
  identification states with source/unit tests.
- Keep diagnostics and retained evidence limited to fixed labels, counters,
  booleans, task ids, paths, and validation commands.

## Non-goals

- No runtime russh adoption, runtime SSH crypto, key exchange, encryption/MAC,
  authentication success, session/channel execution, PTY allocation, shell
  attachment, hardware/lab action, boot publication, hardware reachability,
  OpenSSH/POSIX/Linux compatibility claim, broad network exposure, writable
  persistence, broad socket expansion, stale link-ready discriminator work,
  broad expansion, or phase transition.
- No peer identification text, peer addresses, keys, fingerprints, random
  bytes, operator identity, key-derived identifiers, or stable
  transport/session identifiers retained in diagnostics or evidence.

## Implementation

- Added fixed sshservicediag labels for local-listener-modeled and
  local-transport-modeled.
- Extended SshServiceReadinessReport with listener-count,
  transport-enabled, accepted-connection-count, and redacted
  remote-identification classification state.
- Added a crate-internal local transport probe that:
  - creates one private socket listener through accepted bind/listen;
  - creates one local client and accepted connection through accepted
    connect/accept/readiness;
  - sends the fixed Talos local identification literal;
  - receives and classifies exactly one bounded remote identification line;
  - closes the modeled transport before key exchange.
- Kept disabled and prerequisite-missing states fail-closed with zero
  listener/connection counters and transport-enabled=false.
- Updated sshservicediag transcript expectations for sufficient public-fixture
  metadata to expose only fixed labels/counters and ssh-ready=false.
- Updated Phase 12 project docs and roadmap with the accepted local modeled
  transport frontier.

## Findings

- fixed: the shape-modeled service state now composes accepted private
  socket/readiness and owned banner behavior into one local pre-KEX exchange.
- fixed: diagnostics can report local listener and transport availability with
  fixed labels/counters while ssh-ready remains false.
- fixed: invalid and over-limit remote identification classifications close
  before KEX and retain only fixed classification labels.
- fixed: disabled and prerequisite-missing states do not create listener or
  connection claims.
- deferred: runtime SSH crypto, key exchange, encryption/MAC, host-key loading,
  authorized-key parsing, authentication, session/channel execution, PTY/shell
  attachment, hardware reachability, live driver integration, OpenSSH
  compatibility claims, public POSIX/Linux compatibility, and phase transition.
- removed: no runtime russh dependency, hardware artifact, boot publication,
  live packet I/O claim, reachability claim, compatibility claim, broad
  expansion, or phase-transition claim was introduced.
- not-an-issue: listener-count=1, transport-enabled=true, and
  accepted-connection-count=1 are local modeled diagnostic facts only; they do
  not make ssh-ready true or accept external reachability.

## Validation

- cargo fmt --all -- --check: pass.
- cargo -Zjson-target-spec test --quiet ssh_service_readiness: pass.
- cargo -Zjson-target-spec test --quiet sshservicediag: pass.
- cargo -Zjson-target-spec test --quiet socket: pass.
- cargo -Zjson-target-spec test --quiet: pass.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check: pass.

Validation levels: fmt/lint/typecheck, unit tests, docs build, and diff
checks. No QEMU/substitute run, Pi 5 hardware run, lab-controller API action,
hardwareTestLock acquisition, boot publication, generated-root publication,
live packet I/O, hardware reachability, runtime SSH crypto,
authentication/session work, shell attachment, OpenSSH/POSIX/Linux
compatibility claim, broad expansion, or phase transition was performed.

## Redaction Review

Retained evidence contains fixed labels, counters, booleans, task ids, file
paths, and validation command names only. It does not retain peer
identification text, peer addresses, key bytes, fingerprints, random bytes,
operator identity, key-derived identifiers, or stable transport/session
identifiers.

## Acceptance

- Implementation matches the accepted listener/transport contract and keeps
  ssh-ready false.
- A bounded local service endpoint can expose the fixed local banner,
  consume/classify one remote identification line, and close before KEX using
  accepted private socket/readiness behavior.
- Diagnostics expose only fixed labels, counters, booleans, and redacted
  classifications.
- Source/unit tests cover valid, invalid, over-limit, disabled, and
  prerequisite-missing states.
- No crypto, authentication/session success, shell attachment, hardware
  reachability, public OpenSSH/POSIX/Linux compatibility, broad expansion, or
  phase transition is accepted.

selected_next_task=phase12-shell-ssh-listener-transport-smoke-20260622.
