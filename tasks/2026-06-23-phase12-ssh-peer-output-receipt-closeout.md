# Phase 12.6 SSH peer-output receipt closeout

Task id: phase12-ssh-peer-output-receipt-closeout-20260623
Status: accepted
Owner: worker
Classification: phase12-ssh-peer-output-receipt-closeout-accepted

## Goal

Reconcile the accepted local modeled SSH peer-output receipt contract, source
implementation, feature-smoke evidence, validation, redaction posture, and
deferred scope without accepting live reachability, remote-receipt=true,
OpenSSH/POSIX/Linux compatibility, broad command expansion, a phase transition,
or ssh-ready=true.

## Reviewed Inputs

- tasks/2026-06-23-phase12-ssh-peer-output-receipt-contract.md.
- tasks/2026-06-23-phase12-ssh-peer-output-receipt-core.md.
- tasks/2026-06-23-phase12-ssh-peer-output-receipt-feature-smoke.md.
- tasks/2026-06-23-phase12-ssh-posix-eof-wait-closeout.md.
- tasks/2026-06-23-phase12-ssh-live-socket-delivery-closeout.md.
- src/ssh_service_readiness.rs.
- src/network.rs.
- src/userspace_socket_abi.rs.
- src/syscall.rs.
- docs/src/project/phase12-networking-ssh.md.
- docs/src/roadmap.md.

## Reconciled Frontier

The accepted frontier is local modeled SSH peer-output receipt only:

- one descriptor-backed AF_INET/SOCK_STREAM port-22 listener and one accepted
  local connected peer in Talos' in-kernel stream-socket model;
- one accepted service-side SSH connection on that modeled stream socket, with
  local socket-delivery, authentication/session/channel, shell attachment,
  channel-data/stdio, channel-window, channel-lifecycle, and POSIX EOF/wait
  prerequisites satisfied by the same modeled flow;
- service-side generation of accepted SSH output classes only: channel-data
  stdout/stderr, SSH_MSG_CHANNEL_EOF, SSH_MSG_CHANNEL_REQUEST exit-status with
  request type exit-status and want_reply=false, and SSH_MSG_CHANNEL_CLOSE;
- service-side send through the accepted stream-socket path and peer-side
  observation through the same connected peer's descriptor-backed receive path
  using fixed labels, public message/request names, public status values, and
  public length/count categories;
- fixed fail-closed controls for missing listener/connection or SSH
  prerequisites, missing socket-delivery/POSIX EOF-wait or channel-window
  prerequisite, would-block peer receive, output backpressure, closed peer,
  malformed output, zero-length output, over-limit output, lifecycle violation,
  and redaction-sensitive input.

peer-output-receipt-local=true is accepted only for that local modeled success
path. socket-delivery-local=true, posix-eof-wait-local=true, and the existing
local authentication/session/channel, shell, channel-data, channel-window, and
channel-lifecycle counters remain local modeled prerequisites, not live network
or OpenSSH evidence. live-reachability=false, remote-receipt=false,
compatibility=false, and ssh-ready=false remain authoritative.

## Deferred Scope

The closeout does not accept Pi 5 SSH reachability, external OpenSSH execution,
live remote receipt, OpenSSH/POSIX/Linux compatibility, PTY/SCP/SFTP, multiple
sessions, multiple children, blocking wait, scheduler-owned process lifetime,
boot publication, broad command expansion, a phase transition, or
ssh-ready=true.

The next bounded follow-up may define an OpenSSH compatibility discriminator,
but it must remain a contract unless a later implementation/evidence task owns
the required execution and redaction gates. This closeout does not run a live
client and does not claim compatibility.

## Findings

- fixed: reconciled the peer-output receipt contract, source behavior, focused
  feature-smoke coverage, full regression evidence, docs, and readiness
  counters into one accepted frontier statement.
- fixed: recorded that accepted peer-output receipt is same-connection local
  modeled stream-socket evidence only, not live reachability, remote receipt,
  OpenSSH/POSIX/Linux compatibility, or ssh-ready=true.
- fixed: preserved the accepted success output classes and ordering:
  channel-data stdout/stderr, channel EOF, exit-status, and close.
- fixed: preserved fail-closed controls for missing prerequisites,
  would-block peer receive, output backpressure, closed peer, malformed or
  over-limit output, lifecycle violations, and redaction-sensitive input.
- fixed: updated Phase 12 docs and roadmap with the closeout frontier and the
  selected OpenSSH compatibility discriminator contract follow-up.
- not-an-issue: no Rust source, Cargo metadata, Pi 5 hardware run,
  lab-controller action, boot publication, external OpenSSH execution, or
  decision-log entry is required for this reconciliation-only task.
- deferred: live reachability, remote receipt, OpenSSH/POSIX/Linux
  compatibility, PTY/SCP/SFTP, multiple sessions/children, broad command
  expansion, phase transition, and ssh-ready=true.

## Validation

- static task/docs/source review: pass.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check: pass.

Conditional gates not run: cargo fmt --all -- --check and
cargo -Zjson-target-spec test --quiet were not required because this task
touched no Rust source or Cargo metadata.

No Pi 5 hardware run, lab-controller API action, hardwareTestLock acquisition,
boot publication, external OpenSSH execution, live reachability claim,
remote-receipt=true claim, compatibility claim, broad command expansion, phase
transition, or ssh-ready=true was performed.

## Redaction Review

Pass. Durable evidence retains only task ids, source/doc paths, public socket
ABI names, public readiness bits, public SSH message names/numbers, public
request type names, public status values, public count/length categories,
readiness counters, validation commands, fixed labels, and classifications. It
retains no private user data, channel identifiers, request payload bytes,
command payload bytes, channel data bytes, key/session material, live peer
data, hardware data, or boot artifacts.

## Result

Accepted as the bounded local modeled SSH peer-output receipt closeout.

selected_next_task=phase12-ssh-openssh-compat-discriminator-contract-20260623.
