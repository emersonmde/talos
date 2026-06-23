# Phase 12.6 SSH OpenSSH compatibility discriminator contract

Task id: phase12-ssh-openssh-compat-discriminator-contract-20260623
Status: accepted
Owner: worker
Classification: phase12-ssh-openssh-compat-discriminator-contract-accepted

## Goal

Define the first bounded OpenSSH compatibility discriminator after accepted
local modeled SSH peer-output receipt, without implementing compatibility,
running an external OpenSSH client, accepting live reachability,
remote-receipt=true, compatibility=true, a phase transition, or ssh-ready=true.

## Reviewed Inputs

- tasks/2026-06-23-phase12-ssh-peer-output-receipt-contract.md.
- tasks/2026-06-23-phase12-ssh-peer-output-receipt-core.md.
- tasks/2026-06-23-phase12-ssh-peer-output-receipt-feature-smoke.md.
- tasks/2026-06-23-phase12-ssh-peer-output-receipt-closeout.md.
- tasks/2026-06-23-phase12-ssh-posix-eof-wait-closeout.md.
- tasks/2026-06-23-phase12-ssh-live-socket-delivery-closeout.md.
- src/ssh_service_readiness.rs.
- src/network.rs.
- src/userspace_socket_abi.rs.
- src/syscall.rs.
- docs/src/project/phase12-networking-ssh.md.
- docs/src/roadmap.md.

## Accepted Inputs

The next implementation/evidence task may consume only the accepted local
modeled SSH foundations:

- one descriptor-backed AF_INET/SOCK_STREAM port-22 listener and one accepted
  connected peer in Talos' in-kernel stream-socket model;
- accepted SSH socket delivery into and out of the service on that same
  modeled connection;
- accepted local authentication/session/channel, shell attachment,
  channel-data/stdio, channel-window, channel-lifecycle, POSIX EOF/wait, and
  peer-output receipt counters;
- accepted service output classes observed by the modeled peer:
  SSH_MSG_CHANNEL_DATA stdout, SSH_MSG_CHANNEL_EXTENDED_DATA stderr,
  SSH_MSG_CHANNEL_EOF, SSH_MSG_CHANNEL_REQUEST exit-status with request type
  exit-status and want_reply=false, and SSH_MSG_CHANNEL_CLOSE.

This contract does not prove a live TCP path, Pi 5 Ethernet reachability, an
external OpenSSH client, remote host receipt, POSIX/Linux compatibility, or
general SSH interoperability.

## Discriminator Contract

The first compatibility discriminator is an offline transcript-shaped
OpenSSH-compatible closeout sequence check. It compares Talos' sanitized public
local modeled output sequence against a fixed public expectation derived from
the SSH channel lifecycle shape OpenSSH clients expect after a shell command
completes.

The accepted expectation for one shell-attached session channel is:

1. Optional stdout channel data uses SSH_MSG_CHANNEL_DATA.
2. Optional stderr channel data uses SSH_MSG_CHANNEL_EXTENDED_DATA with the
   public stderr data-type code only.
3. SSH_MSG_CHANNEL_EOF is emitted after the final local output byte.
4. SSH_MSG_CHANNEL_REQUEST exit-status is emitted with request type
   exit-status, want_reply=false, and a public exit-status value/category.
5. SSH_MSG_CHANNEL_CLOSE is emitted only after EOF and exit-status have been
   emitted.
6. No channel data, extended data, exit-status, EOF, or close appears out of
   order, after close, or in unsupported duplicate form.

The future discriminator may report only a local/offline result label such as
openssh-compat-discriminator-local=true when the sanitized transcript shape
matches the expectation. It must keep live-reachability=false,
remote-receipt=false, compatibility=false, and ssh-ready=false.

The first implementation/evidence task must remain local/offline unless a later
supervisor-planned task explicitly owns live OpenSSH execution, hardware/lab
gates, redaction review, and inconclusive-run triage. The contract task does
not run that client.

## Failure Contract

All non-success paths must fail closed without accepting an OpenSSH
compatibility discriminator:

- missing socket-delivery, authentication/session/channel, shell,
  channel-data/stdio, channel-window, channel-lifecycle, POSIX EOF/wait, or
  peer-output receipt prerequisite;
- output receipt not observed by the accepted modeled peer before the
  discriminator runs;
- close before EOF or exit-status, exit-status after close, data after EOF or
  close, duplicate EOF, duplicate exit-status, duplicate close, unsupported
  request type, want_reply=true on exit-status, malformed message shape, or
  over-limit transcript length/count;
- any retained channel identifier, request payload bytes, command payload
  bytes, channel data bytes, key/session material, user name, fingerprint,
  signature, session identifier, live peer data, hardware data, boot artifact,
  or private user data.

Failure evidence must use fixed labels, public SSH message names or numbers,
public request type names, public status values/categories, public length/count
categories, readiness counters, validation commands, task ids, source/doc
paths, and classifications only.

## Readiness Counters

The future implementation/evidence task may add only a local/offline
discriminator counter/label. It may not turn any of these authoritative bits
true:

- live-reachability=false;
- remote-receipt=false;
- compatibility=false;
- ssh-ready=false.

The label must not be named compatibility=true. Compatibility remains false
until a later explicitly scoped OpenSSH or equivalent interoperability proof is
accepted.

## Findings

- fixed: selected the first compatibility discriminator as a local/offline
  transcript-shaped OpenSSH-compatible closeout sequence check rather than a
  live client run.
- fixed: defined exact accepted public observations: channel-data stdout,
  extended-data stderr, EOF, exit-status with want_reply=false, and close.
- fixed: preserved accepted local modeled prerequisites and rejected live
  reachability, remote receipt, compatibility, phase transition, and
  ssh-ready=true.
- fixed: defined fail-closed labels for missing prerequisites, missing peer
  observation, malformed or out-of-order transcript shape, duplicate lifecycle
  messages, unsupported request shape, over-limit evidence, and redaction
  violations.
- fixed: recorded that no implementation/evidence follow-up is present in the
  current task queue, so supervisor planning is required before a worker can
  promote the next task.
- not-an-issue: no Rust source implementation, external OpenSSH run, Pi 5
  hardware action, boot publication, or decision-log entry is required for
  this contract-only task.
- deferred: implementation/evidence task,
  phase12-ssh-openssh-compat-discriminator-core-20260623 queue creation, live
  OpenSSH execution, Pi 5 reachability, remote receipt, compatibility=true,
  PTY/SCP/SFTP, broad command expansion, phase transition, and ssh-ready=true.

## Validation

- static task/docs/source review: pass.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check: pass.

Conditional gates not run: cargo fmt --all -- --check and
cargo -Zjson-target-spec test --quiet were not required because this task
touched no Rust source or Cargo metadata.

No Pi 5 hardware run, lab-controller API action, hardwareTestLock acquisition,
boot publication, source implementation, external OpenSSH execution, live
reachability claim, remote-receipt=true claim, compatibility=true claim, broad
command expansion, phase transition, or ssh-ready=true was performed.

## Redaction Review

Pass. Durable evidence retains only task ids, source/doc paths, public socket
ABI names, public readiness bits, public SSH message names or numbers, public
request type names, public status values/categories, public count and length
categories, readiness counters, validation commands, fixed labels, and
classifications. It retains no private user data, channel identifiers, request
payload bytes, command payload bytes, channel data bytes, key/session material,
user names, fingerprints, signatures, session identifiers, live peer data,
hardware data, or boot artifacts.

## Result

Accepted as the first bounded OpenSSH compatibility discriminator contract.

selected_next_task=null.
planningNeeded=true.
planningReason=Supervisor must queue an explicit implementation/evidence task
such as phase12-ssh-openssh-compat-discriminator-core-20260623 before the
worker can promote further SSH compatibility-discriminator work.
