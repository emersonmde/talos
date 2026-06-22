# Phase 12.6 SSH service shape contract

Task id: phase12-ssh-service-shape-contract-20260622

Status: accepted.

Classification: phase12-ssh-service-shape-contract-accepted.

## Goal

Define the smallest fail-closed SSH service readiness and lifecycle shape that
can be implemented as a diagnostic boundary before dependency adoption, a
listener, live transport, authentication success, session execution, or shell
attachment.

## Scope

- Use phase12-ssh-implementation-strategy-adr-20260622 and
  phase12-ssh-implementation-dependency-feasibility-contract-20260622 as the
  authority for the selected russh source/reference boundary.
- Define service readiness inputs from accepted CSPRNG, host-key metadata,
  authorized-key metadata, persistence/exposure metadata, and explicit operator
  exposure opt-in.
- Define initial lifecycle states, authentication/session/PTTY placeholders,
  failure modes, and diagnostic output that prove readiness modeling without
  opening a listener.
- Select one implementation task that can add only the fail-closed
  service-shape/readiness model.

## Non-goals

- No Cargo dependency changes, dependency adoption, russh runtime integration,
  source implementation, SSH listener, socket binding, TCP accept path, packet
  movement, handshake, real authentication, channel/session execution, PTY
  allocation, shell attachment, live transport, hardware/lab action, boot
  publication, reachability claim, or public ABI/POSIX/Linux compatibility
  claim.
- No generated host keys, authorized-key parsing/validation, signatures,
  fingerprints, digests, generated random byte streams, private CSPRNG state,
  operator identity, key-derived identifiers, or comparable stable identifiers
  in diagnostics or evidence.
- No writable persistence, stale link-ready discriminator promotion, broad
  expansion, or phase transition.

## Reviewed Inputs

- tasks/2026-06-22-phase12-ssh-implementation-strategy-adr.md
- tasks/2026-06-22-phase12-ssh-implementation-dependency-feasibility-contract.md
- tasks/2026-06-22-phase12-ssh-persistence-exposure-readiness-closeout.md
- docs/src/project/phase12-networking-ssh.md
- docs/src/roadmap.md
- docs/src/decisions/README.md
- src/ssh_key_readiness.rs
- src/diagnostic_command.rs

## Contract

Talos accepts only a service-shape diagnostic contract. The future
implementation may model SSH service readiness from already accepted metadata
and readiness inputs, but it must remain fail-closed and must not make
ssh-ready true.

The accepted service readiness inputs are:

- dependency strategy: OpenSSH remains the compatibility target and russh 0.61.2
  remains source/reference material only; no runtime dependency is adopted;
- entropy/CSPRNG: accepted operator-seeded CSPRNG readiness may be represented
  as an input, but runtime SSH crypto strength and a russh crypto backend remain
  unaccepted;
- host-key material: only metadata sufficiency for
  /etc/talos/ssh/ssh_host_ed25519_key may be consumed; key bytes, public-key
  derivation, fingerprints, signatures, and parsing remain unaccepted;
- authorized-key material: only metadata sufficiency for
  /etc/talos/ssh/authorized_keys may be consumed; authorized-key parsing,
  operator identity binding, and key matching remain unaccepted;
- persistence/exposure: only read-only generated-root/initramfs metadata and the
  /etc/talos/ssh/exposure-enabled marker may be consumed; writable persistence
  and durable key-store semantics remain unaccepted;
- service implementation: missing by design in this contract;
- live transport and reachability: missing by design in this contract.

The accepted lifecycle model is:

- disabled: exposure marker missing or invalid, so no service may start;
- prerequisites-missing: one or more metadata/readiness inputs are absent or
  insufficient;
- shape-modeled: all accepted metadata prerequisites and exposure opt-in are
  present, but no dependency adoption, listener, transport, authentication,
  session, PTTY, shell attachment, or reachability exists;
- dependency-unaccepted: a runtime russh dependency, fork, or port has not been
  accepted;
- crypto-backend-unaccepted: no Talos runtime SSH crypto backend has been
  accepted;
- transport-unaccepted: no TCP bind/accept, packet I/O, or reachability has
  been accepted;
- authentication-unimplemented: no host-key load, authorized-key parse, user
  account model, or authentication decision path exists;
- session-unimplemented: no channel/session execution, PTTY allocation, shell
  attachment, or process launch over SSH exists.

The diagnostic surface selected for the next implementation task is a
fail-closed service readiness diagnostic, preferably sshservicediag, separate
from the existing key-material diagnostic unless a narrower implementation note
justifies reusing the existing command dispatcher. The diagnostic may print
only fixed labels and booleans. It must not print, hash, compare, derive,
retain, or otherwise expose secret/key/random bytes, fingerprints, signatures,
digests, private CSPRNG state, operator identity, key-derived identifiers, or
stable transport/session identifiers.

The selected initial labels are:

- sshservicediag-not-ready;
- sshservicediag-dependency-unaccepted;
- sshservicediag-crypto-backend-unaccepted;
- sshservicediag-transport-unaccepted;
- sshservicediag-authentication-unimplemented;
- sshservicediag-session-unimplemented;
- sshservicediag-exposure-disabled when the accepted exposure marker is absent
  or invalid;
- sshservicediag-prerequisites-missing when accepted metadata/readiness inputs
  are absent or insufficient;
- sshservicediag-shape-modeled when accepted metadata/readiness inputs and
  exposure opt-in are present, while live service, transport, and reachability
  remain unaccepted.

The selected diagnostic caps are fail-closed placeholders, not runtime service
policy:

- listener-count=0;
- transport-enabled=false;
- accepted-connection-count=0;
- session-count=0;
- channel-count=0;
- authentication-success=false;
- shell-attached=false;
- reachability-accepted=false.

Future runtime buffer, window, algorithm, rekey, authentication-attempt,
session, and channel limits must be accepted by later implementation tasks.
They must not be inherited from russh defaults by this contract.

## Failure Modes

- Missing, invalid, or insufficient accepted metadata keeps the diagnostic in
  prerequisites-missing or exposure-disabled state.
- Sufficient accepted metadata and exposure opt-in may clear only the
  prerequisite/exposure failure labels in the service diagnostic. It must still
  report not-ready because dependency adoption, crypto backend, listener,
  transport, authentication, session, shell attachment, and reachability are
  unaccepted.
- Any attempt to expose secret/key/random bytes, fingerprints, digests,
  signatures, operator identity, key-derived identifiers, private CSPRNG state,
  or stable transport/session identifiers is a contract violation.
- Any diagnostic wording that implies live SSH availability, successful
  authentication, an attached shell, remote reachability, OpenSSH/POSIX/Linux
  compatibility, or accepted runtime crypto is rejected.

## Findings

- fixed: split service readiness modeling from key-material metadata readiness
  so sshkeydiag can stay focused on prerequisites while the next implementation
  models service lifecycle failure modes.
- fixed: named a separate fail-closed diagnostic surface and labels that keep
  ssh-ready false even when all accepted metadata prerequisites are present.
- fixed: preserved the russh boundary as source/reference only; no dependency
  adoption, default feature inheritance, host tokio listener, runtime crypto
  backend, or service behavior is accepted.
- fixed: defined lifecycle states for disabled, prerequisites-missing,
  shape-modeled, dependency-unaccepted, crypto-backend-unaccepted,
  transport-unaccepted, authentication-unimplemented, and
  session-unimplemented.
- deferred: runtime russh dependency adoption or fork/port, host-key parsing,
  authorized-key parsing, user/account policy, real authentication, packet
  processing, listener/transport integration, PTTY/session/shell attachment,
  reachability proof, and runtime buffer/window/algorithm limits.
- not-an-issue: selecting phase12-ssh-service-readiness-diagnostic-core-20260622
  as the next task does not make ssh-ready true. It is limited to implementing
  this fail-closed diagnostic readiness model.
- not-an-issue: retained evidence contains only public labels, paths, task ids,
  and contract decisions; it contains no secret/key/random bytes or stable
  secret/operator identifiers.

## Validation

- static source/task/docs/evidence review: pass.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check: pass.

## Acceptance

- Contract records service readiness inputs, lifecycle states, failure modes,
  diagnostics, and explicit rejected live-service claims.
- Findings are recorded with disposition.
- ssh-ready remains false until service behavior, live transport, and
  reachability are separately accepted.
- No listener, transport, authentication success, shell session, reachability,
  ABI/POSIX/Linux compatibility, or secret/key/random/operator identifier
  retention is accepted.

selected_next_task=phase12-ssh-service-readiness-diagnostic-core-20260622.
