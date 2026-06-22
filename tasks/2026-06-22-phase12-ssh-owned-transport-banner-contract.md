# Phase 12.6 SSH owned transport banner contract

Task id: phase12-ssh-owned-transport-banner-contract-20260622

Status: accepted.

Classification: phase12-ssh-owned-transport-banner-contract-accepted.

## Goal

Define the smallest Talos-owned SSH identification/banner exchange after the
russh host build/API discriminator classified russh as reference-only for the
Talos runtime.

## Scope

- Use the accepted russh probe evidence to keep russh as reference/oracle
  material instead of a Talos runtime dependency.
- Define an owned SSH identification exchange over already accepted private
  socket/readiness surfaces.
- Specify the literal local banner, remote identification limits, fail-closed
  states, diagnostic labels, redaction rules, and forbidden compatibility
  claims.
- Select the smallest next implementation task that can prove banner behavior
  without authentication, channel/session execution, shell attachment, or
  hardware reachability.

## Non-goals

- No russh runtime dependency adoption, runtime crypto, key exchange,
  encryption/MAC, authentication, session/channel execution, PTY allocation,
  shell attachment, hardware/lab action, boot publication, reachability claim,
  OpenSSH/POSIX/Linux compatibility claim, broad expansion, or phase
  transition.
- No generated keys, private key bytes, authorized-key bytes, random byte
  dumps, fingerprints, digests, signatures, operator identity, key-derived
  identifiers, peer stable identifiers, or comparable identifiers in diagnostics
  or evidence.

## Reviewed Inputs

- tasks/2026-06-22-phase12-ssh-russh-host-build-probe.md.
- tasks/evidence/2026-06-22-ssh-russh-host-build-probe/api-notes.md.
- tasks/2026-06-21-phase12-network-socket-userspace-abi-contract.md.
- tasks/2026-06-21-phase12-network-socket-userspace-abi-core.md.
- tasks/2026-06-21-phase12-network-smoltcp-socket-bridge-core.md.
- tasks/2026-06-21-phase12-network-shell-sockdiag-smoltcp-tcp-closeout.md.
- tasks/2026-06-22-phase12-ssh-service-shape-contract.md.
- tasks/2026-06-22-phase12-ssh-service-readiness-diagnostic-core.md.
- docs/src/project/phase12-networking-ssh.md.
- docs/src/roadmap.md.
- src/network.rs.
- src/syscall.rs.

## Dependency Mapping

The accepted socket/readiness evidence is sufficient for a host/QEMU-substitute
identification/banner model only:

- Private AF_INET/SOCK_STREAM descriptors, bind/listen/connect/accept,
  send/recv, poll, poll_wait, and close are accepted through the Phase 12.4
  socket ABI and smoltcp socket bridge tasks.
- The shell-visible /bin/sockdiag smoltcp TCP closeout proves only
  host/QEMU-substitute execution through the private descriptor-backed socket
  bridge. It is not live packet I/O, Pi 5 hardware reachability, public socket
  ABI, or SSH readiness.
- The accepted SSH service diagnostic keeps dependency adoption, crypto backend,
  transport, authentication, session, shell attachment, and reachability
  unaccepted. A banner model may add identification-specific evidence, but it
  must not clear sshservicediag-transport-unaccepted or make ssh-ready true.

No missing socket prerequisite blocks the next banner implementation task,
provided that task stays host/QEMU-substitute and does not claim live listener,
packet I/O, hardware reachability, or SSH service readiness.

## Banner Contract

The next implementation may introduce a Talos-owned SSH identification exchange
record and deterministic helpers. The local identification string is the exact
ASCII literal:

    SSH-2.0-Talos_0.1\r\n

This string is a protocol-shape marker for a fail-closed Talos implementation.
It is not an OpenSSH compatibility claim and not an SSH service availability
claim.

The remote identification input rules are:

- The implementation reads one remote identification line only. Pre-banner
  comment lines are rejected for the first task to keep the model minimal.
- The maximum accepted input is 255 bytes including the terminating LF byte.
- The line must terminate with LF within the 255-byte limit. A CR immediately
  before LF is accepted and excluded from the semantic line.
- The semantic line must start with the literal prefix SSH-2.0-.
- At least one software-version byte must follow SSH-2.0-.
- Semantic bytes after the prefix must be printable ASCII 0x20 through 0x7e.
  NUL, other control bytes, non-ASCII bytes, empty software version, missing LF,
  and over-limit input are rejected.
- On accepted remote identification, the implementation may record only fixed
  labels and booleans such as remote-identification-present=true,
  remote-identification-valid=true, and remote-identification-limited=true. It
  must not retain the remote identification string, software version, comments,
  raw peer bytes, hash, digest, fingerprint, peer address, or any stable peer
  identifier.

After the local and remote identification lines are handled, the model must
close/fail before key exchange. The accepted outcome is
transport-closed-before-kex. Key exchange, encryption/MAC, authentication,
session/channel execution, PTY allocation, shell attachment, and reachability
remain unimplemented.

## Diagnostic Labels

The next implementation may add fixed diagnostic labels only:

- sshservicediag-identification-banner-modeled.
- sshservicediag-local-identification-literal.
- sshservicediag-remote-identification-valid.
- sshservicediag-remote-identification-invalid.
- sshservicediag-remote-identification-over-limit.
- sshservicediag-transport-closed-before-kex.

The existing fail-closed service labels remain authoritative:

- sshservicediag-not-ready remains present.
- sshservicediag-transport-unaccepted remains present.
- sshservicediag-crypto-backend-unaccepted remains present.
- sshservicediag-authentication-unimplemented remains present.
- sshservicediag-session-unimplemented remains present.
- ssh-ready remains false.

## Failure Modes

- Missing LF before the 255-byte limit returns the over-limit/missing-terminator
  failure path and closes before key exchange.
- Non-SSH-2.0 prefixes, empty software version, control bytes, non-ASCII bytes,
  and extra pre-banner lines return the invalid-identification failure path and
  close before key exchange.
- Socket EOF before a complete line returns the invalid-identification failure
  path and closes before key exchange.
- Any evidence or diagnostic that retains peer identification text, peer
  address, hashes/digests/fingerprints, key material, random bytes, operator
  identity, or stable peer/session identifiers violates this contract.
- Any wording that implies OpenSSH compatibility, accepted runtime SSH crypto,
  live SSH availability, successful authentication, attached shell, remote
  reachability, public ABI/POSIX/Linux compatibility, or a phase transition is
  rejected.

## Findings

- fixed: selected the reference-only branch because the accepted russh probe
  classified russh-host-build-probe-reference-only and selected this task.
- fixed: mapped the banner implementation dependency to accepted private
  descriptor-backed AF_INET/SOCK_STREAM and host-only smoltcp socket bridge
  evidence without claiming live packet I/O, hardware reachability, public
  socket ABI, or SSH readiness.
- fixed: defined the exact local identification literal,
  remote-identification parser limits, fail-closed states, diagnostic labels,
  and redaction rules for the next implementation.
- deferred: key exchange, runtime crypto backend, host-key loading, authorized
  key parsing, authentication, channel/session execution, PTY allocation, shell
  attachment, hardware reachability, live driver integration, OpenSSH
  compatibility claims, and public POSIX/Linux compatibility.
- not-an-issue: A host/QEMU-substitute banner implementation can be useful as
  the thinnest real SSH-shaped transport step while ssh-ready remains false and
  the service still closes before key exchange.
- removed: No dependency adoption, source runtime behavior, listener,
  authentication/session behavior, hardware artifact, boot publication, live
  packet I/O claim, reachability claim, compatibility claim, broad expansion, or
  phase-transition claim was introduced by this contract.

## Validation

- static task/docs/evidence review: pass. Reviewed the accepted russh probe,
  accepted private socket ABI and smoltcp bridge tasks, SSH service readiness
  diagnostic tasks, Phase 12 networking doc, roadmap, src/network.rs, and
  src/syscall.rs.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check: pass.

No Rust source behavior change, cargo test, Pi 5 hardware run,
hardwareTestLock acquisition, lab mutation, boot publication, generated-root
publication, live packet I/O, hardware reachability, runtime SSH crypto,
authentication/session work, OpenSSH/POSIX/Linux compatibility claim, broad
expansion, or phase transition was performed.

## Acceptance

- Contract names the exact banner/identification behavior, limits, failure
  modes, diagnostic labels, and forbidden compatibility claims.
- Contract maps dependencies to accepted private socket/readiness evidence and
  blocks no prerequisite for a host/QEMU-substitute banner core.
- Findings are recorded with disposition.
- selected_next_task=phase12-ssh-owned-transport-banner-core-20260622.
- No russh dependency adoption, runtime SSH crypto, key exchange,
  encryption/MAC, authentication/session/shell behavior, reachability,
  OpenSSH/POSIX/Linux compatibility, broad expansion, or phase transition is
  accepted.

selected_next_task=phase12-ssh-owned-transport-banner-core-20260622.
