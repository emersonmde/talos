# Phase 12.6 SSH POSIX EOF/wait closeout

Task id: phase12-ssh-posix-eof-wait-closeout-20260623
Status: accepted
Owner: worker
Classification: phase12-ssh-posix-eof-wait-closeout-accepted

## Goal

Reconcile the accepted local modeled SSH POSIX EOF/wait contract, source
implementation, and feature-smoke evidence without accepting live reachability,
remote receipt, OpenSSH/POSIX/Linux compatibility, broad command expansion, a
phase transition, or ssh-ready=true.

## Reviewed Inputs

- tasks/2026-06-23-phase12-ssh-posix-eof-wait-contract.md.
- tasks/2026-06-23-phase12-ssh-posix-eof-wait-core.md.
- tasks/2026-06-23-phase12-ssh-posix-eof-wait-feature-smoke.md.
- tasks/2026-06-23-phase12-ssh-live-socket-delivery-closeout.md.
- tasks/2026-06-23-phase12-ssh-channel-lifecycle-exit-status-closeout.md.
- src/ssh_service_readiness.rs.
- src/local_command_loop.rs.
- docs/src/project/phase12-networking-ssh.md.
- docs/src/roadmap.md.

## Reconciled Frontier

The accepted frontier is local modeled SSH-attached POSIX EOF/wait integration
only:

- one accepted local in-kernel stream socket delivery path into the modeled SSH
  service;
- one authenticated local modeled session channel with shell attachment,
  channel-data/stdio, channel-window, and channel-lifecycle prerequisites;
- one inbound SSH_MSG_CHANNEL_EOF recorded as stdin EOF for one local modeled
  attached process/session;
- one completed LocalCommandProcessLifecycleRecord wait/status observation
  consumed as the process completion source;
- one local exit-status request with request type exit-status and
  want_reply=false;
- one local stdout EOF, one local close, and full closure only after peer close;
- fixed fail-closed controls for missing prerequisites, missing or consumed
  wait status, invalid ordering, duplicate lifecycle events, output
  backpressure, closed peer, and redaction-sensitive input.

posix-eof-wait-local=true is accepted only for the local modeled success path.
socket-delivery-local=true and the accepted local authentication/session/channel
shell/channel-data/window/lifecycle counters remain local modeled prerequisites,
not live network evidence. live-reachability=false, remote-receipt=false,
compatibility=false, and ssh-ready=false remain authoritative.

## Deferred Scope

The closeout does not accept Pi 5 SSH reachability, remote receipt of channel
EOF/stdout/exit-status/close, OpenSSH/POSIX/Linux compatibility, multiple
sessions, multiple children, blocking wait, scheduler-owned process lifetime,
PTY behavior, SCP/SFTP, boot publication, broad command expansion, a phase
transition, or ssh-ready=true.

No fake/kernel-backed shell command expansion is accepted as progress. Future
feature work must continue through descriptor-backed process, filesystem, and
userspace layers rather than command shims.

## Findings

- fixed: reconciled contract, implementation, and feature-smoke evidence into
  one accepted frontier statement.
- fixed: recorded that the accepted EOF/wait behavior is local modeled only and
  does not prove live reachability, remote receipt, OpenSSH/POSIX/Linux
  compatibility, or ssh-ready=true.
- fixed: preserved the false readiness counters and the fail-closed negative
  controls from the accepted core and smoke tasks.
- fixed: updated Phase 12 docs and roadmap with the closeout frontier and
  planning-needed handoff.
- not-an-issue: no Rust source, Cargo metadata, Pi 5 hardware run,
  lab-controller action, boot publication, or mdbook design decision entry is
  required for this reconciliation-only task.
- deferred: the next bounded SSH/POSIX feature slice requires supervisor
  planning because no explicit queued follow-up task exists after this
  closeout.

## Validation

- static task/docs/source review: pass.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check: pass.

Conditional gates not run: cargo fmt --all -- --check and
cargo -Zjson-target-spec test --quiet were not required because this task
touched no Rust source or Cargo metadata.

No Pi 5 hardware run, lab-controller API action, hardwareTestLock acquisition,
boot publication, live reachability claim, remote receipt claim,
OpenSSH/POSIX/Linux compatibility claim, broad command expansion, phase
transition, or ssh-ready=true was performed.

## Redaction Review

Pass. Durable evidence retains only task ids, source/doc paths, public SSH
message names and numbers, public request type names, public status values,
public length categories, boolean readiness counters, fixed labels, validation
commands, and classifications. It retains no private user data, channel
identifiers, request payload bytes, command payload bytes, channel data bytes,
key/session material, live peer data, hardware data, or boot artifacts.

## Result

Accepted as the bounded local modeled SSH POSIX EOF/wait closeout.

selected_next_task=null.
planningNeeded=true: supervisor planning is required before any further
bounded SSH/POSIX slice, live reachability, remote receipt, compatibility
discriminator, broad command expansion, phase transition, or ssh-ready=true.
