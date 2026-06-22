# Phase 12.6 SSH listener/transport contract

Task id: phase12-ssh-listener-transport-contract-20260622

Status: accepted.

Classification: phase12-ssh-listener-transport-contract-accepted.

## Goal

Define the smallest local SSH listener/transport behavior that can build on the
accepted private socket/readiness surfaces and Talos-owned banner model while
keeping SSH readiness fail-closed.

## Scope

- Reconcile accepted private socket bind/listen/connect/accept/send/recv/poll,
  host-only smoltcp socket bridge, sshservicediag, and owned banner evidence.
- Specify exact service lifecycle transitions, local endpoint boundary,
  listener and connection counters, transport-enabled criteria, banner
  ordering, close-before-KEX outcome, diagnostics, redaction, and failure modes.
- Select the next implementation task only if the accepted substrate is enough
  for a bounded local modeled transport slice.

## Non-goals

- No runtime russh adoption, runtime SSH crypto, key exchange, encryption/MAC,
  authentication success, session/channel execution, PTY allocation, shell
  attachment, hardware/lab action, boot publication, hardware reachability,
  OpenSSH/POSIX/Linux compatibility claim, broad network exposure, writable
  persistence, stale link-ready discriminator work, broad expansion, or phase
  transition.
- No peer identification text, peer addresses, keys, fingerprints, random
  bytes, operator identity, key-derived identifiers, or stable
  transport/session identifiers retained in diagnostics or evidence.

## Reviewed Inputs

- tasks/2026-06-20-phase12-network-socket-bind-listen-core.md.
- tasks/2026-06-20-phase12-network-socket-connect-accept-core.md.
- tasks/2026-06-20-phase12-network-socket-send-recv-core.md.
- tasks/2026-06-21-phase12-network-socket-readiness-poll-core.md.
- tasks/2026-06-21-phase12-network-smoltcp-socket-bridge-core.md.
- tasks/2026-06-22-phase12-ssh-service-readiness-diagnostic-core.md.
- tasks/2026-06-22-phase12-ssh-owned-transport-banner-contract.md.
- tasks/2026-06-22-phase12-ssh-owned-transport-banner-core.md.
- tasks/2026-06-22-phase12-ssh-dependency-path-closeout.md.
- docs/src/project/phase12-networking-ssh.md.
- docs/src/roadmap.md.
- src/network.rs.
- src/syscall.rs.
- src/ssh_service_readiness.rs.
- src/diagnostic_command.rs.

## Dependency Mapping

The accepted substrate is sufficient for a host/QEMU-substitute local
listener/transport model only:

- Private descriptor-backed AF_INET/SOCK_STREAM socket, bind, listen, connect,
  accept, send, recv, and poll/readiness behavior is accepted as local
  process-owned state.
- The host-only smoltcp socket bridge may record established/payload
  observations behind the same private descriptor path, but it is not live
  packet I/O, hardware reachability, public socket ABI, or SSH readiness.
- sshservicediag already exposes fail-closed lifecycle labels and caps with
  listener-count=0, transport-enabled=false, accepted-connection-count=0, and
  ssh-ready=false.
- The owned banner core already fixes SSH-2.0-Talos_0.1 CRLF, one remote
  identification line, the 255-byte limit, redacted valid/invalid/over-limit
  classifications, and transport-closed-before-kex.

No missing substrate blocks the next implementation if it stays local,
host/QEMU-substitute, descriptor-backed, redacted, and fail-closed before key
exchange.

## Listener And Transport Contract

The next implementation may add a crate-internal Talos SSH service transport
model that composes the accepted service-readiness report and banner classifier
with one local descriptor-backed listener/connection path.

The only local endpoint selected by this contract is a fixed modeled Talos SSH
service endpoint. The implementation may use the private AF_INET/SOCK_STREAM
socket table, but must treat the endpoint as local-only evidence. It must not
claim live device reachability, external network exposure, public socket ABI
compatibility, or OpenSSH compatibility.

Lifecycle and caps:

- Disabled or prerequisite-missing service states keep listener-count=0,
  transport-enabled=false, accepted-connection-count=0, and ssh-ready=false.
- The shape-modeled state may expose listener-count=1 only for the bounded local
  modeled endpoint.
- transport-enabled may become true only for that local modeled endpoint after
  accepted key/CSPRNG/persistence/exposure prerequisites are shape-modeled and a
  descriptor-backed listener/accepted connection path is available.
- accepted-connection-count may report one modeled accepted connection for the
  single exchange under test. session-count and channel-count remain 0.
- authentication-success, shell-attached, reachability-accepted, and ssh-ready
  remain false.

Exchange ordering:

- The local listener is created through the accepted private bind/listen path.
- A local client connection is accepted through the accepted connect/accept
  path.
- Talos sends the exact local identification literal SSH-2.0-Talos_0.1 CRLF.
- Talos consumes and classifies exactly one remote identification line using the
  accepted banner-core rules.
- Talos closes the modeled transport before key exchange. The accepted terminal
  outcome remains transport-closed-before-kex.

Diagnostics may add only fixed labels, counters, and booleans:

- sshservicediag-local-listener-modeled.
- sshservicediag-local-transport-modeled.
- sshservicediag-identification-banner-modeled.
- sshservicediag-local-identification-literal.
- sshservicediag-remote-identification-valid.
- sshservicediag-remote-identification-invalid.
- sshservicediag-remote-identification-over-limit.
- sshservicediag-transport-closed-before-kex.
- listener-count=0 or listener-count=1.
- transport-enabled=false or transport-enabled=true.
- accepted-connection-count=0 or accepted-connection-count=1.

The fail-closed labels remain authoritative:

- sshservicediag-not-ready remains present.
- sshservicediag-crypto-backend-unaccepted remains present.
- sshservicediag-authentication-unimplemented remains present.
- sshservicediag-session-unimplemented remains present.
- ssh-ready remains false.

The implementation may replace sshservicediag-transport-unaccepted with a
transport-specific label only for the local modeled pre-KEX path if the source
and tests prove all exchange ordering and redaction requirements above. This
does not accept runtime SSH crypto, authentication, session, shell,
reachability, or compatibility.

## Failure Modes

- Disabled or missing-prerequisite states do not create a modeled listener or
  connection and keep transport-enabled=false.
- Socket creation, bind, listen, connect, accept, send, recv, or poll failures
  must be all-or-nothing and reported with fixed failure labels or existing
  deterministic error vocabulary; no partial service-ready state may be
  retained.
- Invalid, over-limit, or EOF-before-complete remote identification input closes
  before key exchange and records only fixed classification labels.
- Any diagnostic or evidence retaining peer identification text, peer address,
  raw peer bytes, hashes, digests, fingerprints, key material, random bytes,
  operator identity, or stable peer/session identifiers violates this contract.
- Any wording that implies OpenSSH compatibility, accepted runtime SSH crypto,
  successful authentication, attached shell, external reachability, public
  ABI/POSIX/Linux compatibility, broad network exposure, or a phase transition
  is rejected.

## Findings

- fixed: mapped listener/transport implementation dependencies to accepted
  private descriptor-backed sockets, local readiness, host-only smoltcp bridge,
  fail-closed sshservicediag, and owned banner evidence.
- fixed: specified the exact local endpoint boundary, listener/transport caps,
  counter transitions, banner exchange ordering, close-before-KEX terminal
  outcome, diagnostics, redaction posture, and failure modes.
- fixed: selected the next implementation task because the accepted substrate is
  enough for local host/QEMU-substitute modeled transport without accepting
  reachability or SSH readiness.
- deferred: runtime SSH crypto, key exchange, encryption/MAC, host-key loading,
  authorized-key parsing, authentication, session/channel execution, PTY/shell
  attachment, hardware reachability, live driver integration, OpenSSH
  compatibility claims, public POSIX/Linux compatibility, and phase transition.
- not-an-issue: listener-count=1 and transport-enabled=true can be useful
  diagnostic facts for the local pre-KEX modeled endpoint while ssh-ready
  remains false and all higher SSH capabilities stay fail-closed.
- removed: No source behavior, dependency adoption, hardware artifact, boot
  publication, live packet I/O claim, reachability claim, compatibility claim,
  broad expansion, or phase-transition claim was introduced by this contract.

## Validation

- static source/task/docs/evidence review: pass. Reviewed accepted private
  socket/readiness tasks, smoltcp bridge, SSH service diagnostic, owned banner
  contract/core, closeout, Phase 12 networking doc, roadmap, and source anchors.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check: pass.

No Rust source behavior change, cargo test, Pi 5 hardware run,
hardwareTestLock acquisition, lab mutation, boot publication, generated-root
publication, live packet I/O, hardware reachability, runtime SSH crypto,
authentication/session work, shell attachment, OpenSSH/POSIX/Linux
compatibility claim, broad expansion, or phase transition was performed.

## Acceptance

- Contract fixes exact listener/transport behavior for one local Talos SSH
  service endpoint using only accepted private socket/readiness surfaces.
- Contract preserves fixed local identification literal SSH-2.0-Talos_0.1 CRLF,
  remote identification limits, fail-closed pre-KEX close, and redaction rules.
- Contract specifies diagnostics that may raise listener-count from zero and
  transport-enabled only for the bounded local modeled service while keeping
  ssh-ready false.
- Findings are recorded with disposition.
- selected_next_task=phase12-ssh-listener-transport-core-20260622.
- No crypto, authentication/session success, shell attachment, hardware
  reachability, public OpenSSH/POSIX/Linux compatibility, broad expansion, or
  phase transition is accepted.

selected_next_task=phase12-ssh-listener-transport-core-20260622.
