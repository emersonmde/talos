# Phase 12.6 SSH listener/transport closeout

Task id: phase12-ssh-listener-transport-closeout-20260622

Status: accepted.

Classification: phase12-ssh-listener-transport-closeout-accepted.

## Goal

Close out the bounded local SSH listener/transport slice before runtime
crypto, authentication, session/channel, shell attachment, hardware
reachability, compatibility, or phase-transition work.

## Scope

- Reconciled the listener/transport contract, implementation core, retained
  smoke evidence, docs, validation, redaction posture, and deferred work.
- Recorded findings with disposition.
- Checked whether a next queued task is mechanically unblocked.

## Non-goals

- No runtime SSH crypto, key exchange, encryption/MAC, authentication success,
  session/channel execution, PTY allocation, shell attachment, hardware/lab
  action, boot publication, hardware reachability, OpenSSH/POSIX/Linux
  compatibility claim, broad network exposure, broad expansion, or phase
  transition.
- No secret/key/random bytes or stable peer/session identifiers retained in
  closeout evidence.

## Reconciliation

The accepted listener/transport slice now covers:

- phase12-ssh-listener-transport-contract-20260622: selected one local modeled
  Talos SSH service endpoint over accepted private descriptor-backed sockets,
  send/recv/readiness, host-only smoltcp bridge evidence, fail-closed
  sshservicediag, and the owned banner core.
- phase12-ssh-listener-transport-core-20260622: implemented the local modeled
  pre-KEX exchange with bind/listen, local connect/accept, Talos local
  identification send, one bounded remote-identification classification, and
  close-before-KEX.
- phase12-shell-ssh-listener-transport-smoke-20260622: retained
  host/QEMU-substitute smoke evidence for disabled/prerequisite-missing states
  plus source-level local transport-modeled evidence.

The accepted diagnostic frontier is still fail-closed. Disabled and
prerequisite-missing states retain zero listener/connection counters and
ssh-ready=false. The shape-modeled local endpoint may report
local-listener-modeled, local-transport-modeled, identification-banner-modeled,
remote-identification valid/invalid/over-limit labels, listener-count=1,
transport-enabled=true, accepted-connection-count=1, and
transport-closed-before-kex only for the bounded local modeled exchange.
authentication-success=false, shell-attached=false, reachability-accepted=false,
and ssh-ready=false remain authoritative.

The shell-visible VFS diagnostic path remains prerequisites-missing until a
later accepted crypto/service slice supplies sufficient prerequisites through
the diagnostic context. The local modeled listener/transport evidence is not a
hardware reachability, public socket ABI, OpenSSH compatibility, authentication,
session, shell, or SSH-ready claim.

## Findings

- fixed: reconciled the accepted contract, core implementation, retained smoke
  transcript, Phase 12 docs, roadmap frontier, validation, and redaction
  posture.
- fixed: recorded the current frontier as local host/QEMU-substitute
  listener/transport modeling only, with ssh-ready=false and no external
  reachability or compatibility claim.
- fixed: selected no next implementation task because there is no queued or
  ready task after this closeout with explicit objective dependencies,
  acceptance criteria, validation gates, docs, evidence, and scope.
- not-an-issue: listener-count=1, transport-enabled=true, and
  accepted-connection-count=1 are accepted only as local modeled diagnostic
  counters for the pre-KEX exchange and do not make ssh-ready true.
- deferred: runtime SSH crypto, key exchange, encryption/MAC, host-key loading,
  authorized-key parsing, authentication, session/channel execution,
  PTY/shell attachment, hardware reachability, live driver integration,
  OpenSSH compatibility claims, public POSIX/Linux compatibility, and phase
  transition.
- removed: no new source behavior, dependency adoption, hardware artifact, boot
  publication, live packet I/O claim, reachability claim, compatibility claim,
  broad expansion, or phase-transition claim was introduced by this closeout.

## Evidence

- contract task record:
  tasks/2026-06-22-phase12-ssh-listener-transport-contract.md.
- implementation task record:
  tasks/2026-06-22-phase12-ssh-listener-transport-core.md.
- smoke task record:
  tasks/2026-06-22-phase12-shell-ssh-listener-transport-smoke.md.
- retained smoke transcript:
  tasks/evidence/2026-06-22-ssh-listener-transport-smoke/qemu-shell-ssh-listener-transport-smoke.log.

## Redaction Review

Closeout evidence references only task records, paths, fixed labels, counters,
booleans, and validation command names. It does not retain peer identification
text, peer addresses, key bytes, fingerprints, random bytes, operator identity,
key-derived identifiers, or stable transport/session identifiers.

## Validation

- static task/docs/evidence review: pass. Reviewed listener/transport contract,
  core, smoke task record, retained smoke transcript, Phase 12 networking/SSH
  project doc, roadmap frontier, and task queue state.
- jq empty on task-owned JSON evidence: not applicable; no task-owned JSON
  evidence was created.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check: pass.

Validation levels: static inspection, docs build, and diff checks. No Rust
source behavior change, cargo test, QEMU/substitute run, Pi 5 hardware run,
lab-controller API action, hardwareTestLock acquisition, boot publication,
generated-root publication, live packet I/O, hardware reachability, runtime SSH
crypto, authentication/session work, shell attachment,
OpenSSH/POSIX/Linux compatibility claim, broad expansion, or phase transition
was performed.

## Acceptance

Accepted. The bounded listener/transport slice is closed at static
task/docs/evidence review level with the accepted local modeled pre-KEX
exchange, retained host/QEMU-substitute smoke evidence, and fail-closed
diagnostic posture reconciled. No crypto, authentication/session success, shell
attachment, hardware reachability, public compatibility, broad expansion, or
phase transition is accepted.

selected_next_task=null.

planningNeeded=true because no queued or ready task exists after this closeout.
Supervisor planning is required before runtime SSH crypto, key exchange,
authentication/session/shell attachment, hardware reachability,
OpenSSH/POSIX/Linux compatibility, broad expansion, or phase transition.
