# Phase 12.6 SSH owned transport banner core

Task id: phase12-ssh-owned-transport-banner-core-20260622

Status: accepted.

Classification: phase12-ssh-owned-transport-banner-core-accepted.

## Goal

Implement only the accepted Talos-owned SSH identification/banner behavior from
phase12-ssh-owned-transport-banner-contract-20260622 while keeping SSH service
readiness fail-closed.

## Scope

- Implement the exact fail-closed banner behavior accepted by the contract.
- Retain deterministic source/unit evidence scaled to the contract.
- Keep authentication, key exchange, encryption/MAC, channels, sessions, shell
  attachment, hardware/lab action, boot publication, and reachability
  unimplemented.

## Implementation

- Added the fixed local identification literal SSH-2.0-Talos_0.1 followed by
  CRLF.
- Added a bounded remote identification classifier over caller-provided bytes:
  one line only, LF required within 255 bytes including LF, optional CR before
  LF, required SSH-2.0- prefix, non-empty printable-ASCII software version, and
  no retention of peer identification text.
- Added fixed labels for the accepted modeled banner states:
  sshservicediag-identification-banner-modeled,
  sshservicediag-local-identification-literal,
  sshservicediag-remote-identification-valid,
  sshservicediag-remote-identification-invalid,
  sshservicediag-remote-identification-over-limit, and
  sshservicediag-transport-closed-before-kex.
- Kept ssh-ready false, listener-count 0, transport-enabled false,
  accepted-connection-count 0, session-count 0, channel-count 0,
  authentication-success false, shell-attached false, and
  reachability-accepted false.

Changed files:

- src/ssh_service_readiness.rs.
- docs/src/project/phase12-networking-ssh.md.
- docs/src/roadmap.md.
- tasks/2026-06-22-phase12-ssh-owned-transport-banner-core.md.

## Findings

- fixed: The service readiness model now has a Talos-owned local
  identification literal and remote identification classifier instead of a
  contract-only description.
- fixed: Unit coverage proves valid, invalid, EOF-before-line, and over-limit
  remote identification outcomes while keeping the pre-KEX close outcome true.
- fixed: Shape-modeled sshservicediag labels can now report the accepted local
  banner model without clearing transport-unaccepted or making ssh-ready true.
- not-an-issue: No task-owned host/QEMU-substitute socket smoke was required
  because this slice did not touch dispatcher, socket, or smoltcp bridge paths.

## Validation

- cargo -Zjson-target-spec test --quiet ssh_service_readiness: pass.
- cargo -Zjson-target-spec test --quiet sshservicediag: pass.
- task-owned host/QEMU-substitute banner smoke: not run; no dispatcher/socket
  substitute path was touched.
- cargo -Zjson-target-spec test --quiet: pass, 744 talos no_std tests passed.
- cargo fmt --all -- --check: pass.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check: pass.

## Redaction Review

Retained evidence contains only fixed labels, public fixture input classes,
the fixed Talos local banner literal, booleans, zero counters, task ids, paths,
and validation commands. It does not retain peer identification text from a
real peer, peer addresses, key bytes, authorized-key bytes, random bytes,
fingerprints, digests, signatures, operator identity, key-derived identifiers,
stable peer/session identifiers, or comparable identifiers.

## Acceptance

- Implementation matches the accepted banner contract and remains fail-closed
  beyond identification/banner behavior.
- Validation proves accepted banner behavior and failure modes without
  accepting authentication/session/shell/reachability.
- Diagnostics/evidence contain no secret/key/random bytes or stable
  identifiers.
- Findings are recorded with disposition.

No runtime crypto, key exchange, authentication success, channel/session
execution, PTY allocation, shell attachment, hardware/lab action, boot
publication, hardware reachability, OpenSSH/POSIX/Linux compatibility claim,
broad expansion, or phase transition is accepted.
