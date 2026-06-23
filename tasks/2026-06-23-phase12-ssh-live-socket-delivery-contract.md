# Phase 12.6 SSH live socket-delivery contract

Task id: phase12-ssh-live-socket-delivery-contract-20260623
Status: accepted
Owner: worker
Classification: phase12-ssh-live-socket-delivery-contract-accepted

## Goal

Define the smallest source contract that connects the accepted SSH
transport/auth/session/channel pipeline to the accepted in-kernel stream socket
boundary without accepting Pi 5 reachability, remote receipt, OpenSSH
compatibility, a phase transition, or ssh-ready=true.

## Reviewed Inputs

- tasks/2026-06-23-phase12-ssh-channel-lifecycle-exit-status-closeout.md.
- tasks/2026-06-23-phase12-ssh-channel-lifecycle-exit-status-core.md.
- tasks/2026-06-23-phase12-ssh-channel-window-accounting-closeout.md.
- tasks/2026-06-23-phase12-ssh-channel-data-stdio-closeout.md.
- src/ssh_service_readiness.rs.
- src/ssh_runtime_crypto.rs.
- src/network.rs.
- src/userspace_socket_abi.rs.
- src/syscall.rs.
- docs/src/project/phase12-networking-ssh.md.
- docs/src/roadmap.md.

## Sufficiency Decision

The accepted local foundations are sufficient for a bounded source task that
models SSH input and output through Talos' in-kernel stream socket descriptors.
The implementation can be objective because the needed socket ownership,
descriptor, readiness, send, recv, accept, close, and poll/wait surfaces already
exist in the local socket model, and the accepted SSH pipeline already has fixed
local reports for identification, KEX, encrypted dispatch, authentication,
session channel-open, shell attachment, channel-data/stdio, channel-window
accounting, and channel lifecycle.

This contract deliberately stays below live hardware reachability. The next
source task may accept only local in-kernel socket-delivery behavior: bytes
queued through one modeled stream socket connection may enter the accepted SSH
pipeline, and accepted SSH output may be queued back to the peer socket under
the same local model. That evidence does not prove that a Pi 5 Ethernet path,
remote host, OpenSSH client, TCP/IP packet device, or external peer received
anything.

## Source Ownership

The next source task should keep ownership explicit:

- src/network.rs owns stream socket lifecycle, listener state, accepted
  connection descriptors, recv queues, send readiness/backpressure, close,
  hangup, and readiness bits.
- src/syscall.rs and src/userspace_socket_abi.rs own the private Talos socket
  ABI shape for socket, bind, listen, accept, send, recv, poll, poll_wait, and
  close.
- src/ssh_service_readiness.rs owns the SSH service pipeline contract,
  redacted diagnostics, and any local socket-delivery classifier/report added
  by the implementation.
- src/ssh_runtime_crypto.rs owns runtime KEX, NEWKEYS, encrypted packet state,
  sequence accounting, and crypto failure labels.

No implementation should put SSH payload interpretation into the generic socket
table, and no socket code should claim SSH readiness. The socket layer supplies
bounded byte transport and readiness only; the SSH service layer owns protocol
classification and fail-closed labels.

## Contract

The next source task may introduce a local socket-delivery report only for this
success path:

- one Talos SSH listener is modeled on an in-kernel AF_INET/SOCK_STREAM socket
  endpoint for port 22;
- a connected peer is accepted through the socket table, producing exactly one
  accepted SSH connection descriptor owned by the SSH service;
- listener READ readiness is observed only for a pending connected peer;
- accepted connection READ readiness is observed only for queued peer bytes or
  peer hangup;
- accepted connection WRITE readiness is observed only when the peer receive
  queue has capacity;
- recv_peek reads a bounded byte slice from the accepted descriptor into a
  temporary input buffer, and recv_commit consumes bytes only after the SSH
  service has classified the same bounded slice;
- the classified input is dispatched through the already accepted SSH pipeline
  in order: identification, KEX/NEWKEYS/encrypted dispatch,
  service/userauth/authentication success, channel-open, shell attachment,
  channel-data/stdio, channel-window accounting, and channel lifecycle;
- accepted SSH output is constructed only by existing local report surfaces or
  a new bounded local socket-delivery output surface, then sent with
  send_ready/send on the accepted connection descriptor;
- send backpressure maps to a fixed fail-closed socket-delivery label and must
  not mutate SSH readiness counters as if remote receipt happened;
- close, peer hangup, malformed input, over-limit input, lifecycle-invalid
  input, missing listener, missing accepted connection, missing SSH
  prerequisite, and crypto failure all propagate to fixed fail-closed labels;
- poll and poll_wait evidence may observe only local socket readiness bits:
  READ, WRITE, HANGUP, and ERROR.

On the accepted local success path the next source task may report a new
local socket-delivery counter/label equivalent to
socket-delivery-local=true. The report may also preserve the existing accepted
counters for authentication-success, session-count=1, channel-count=1,
shell-attached=true, channel-data-stdio-local=true,
channel-window-management=true, and channel-lifecycle-local=true when those
preconditions are satisfied by the same modeled path.

live-reachability=false, remote-receipt=false, compatibility=false,
phase-transition=false, and ssh-ready=false remain authoritative.

## Failure Contract

All non-success paths must fail closed without claiming socket delivery:

- no SSH listener descriptor, wrong owner, wrong socket domain/type/protocol, or
  endpoint collision;
- accept before a connected peer is pending;
- recv on a missing, unaccepted, closed, wrong-owner, or non-readable
  descriptor;
- zero-length, malformed, trailing, over-limit, redaction-sensitive, or
  lifecycle-invalid SSH input;
- send before accepted SSH output exists;
- send when the peer receive queue has no remaining capacity;
- poll or poll_wait with invalid entries, unsupported events, invalid timeout,
  missing descriptor, or wrong owner;
- close before required lifecycle evidence or after peer hangup/error.

Failure evidence must use fixed labels, public readiness bits, public message
names/numbers, public count and length categories, source paths, commands, task
ids, and classifications only. Durable evidence must not retain channel
identifiers, request payload bytes, command payload bytes, channel data bytes,
key/session material, private user data, live peer data, hardware data, or boot
artifacts.

## Readiness Counters

The next source task may add or update only local modeled readiness counters:

- socket-delivery-local=true on the accepted local socket-delivery path;
- existing local SSH counters may remain true only when their accepted
  prerequisites are satisfied by the same modeled flow;
- live-reachability=false;
- remote-receipt=false;
- compatibility=false;
- ssh-ready=false.

The implementation must not use ssh-ready=true as a shortcut for local socket
delivery. A later hardware/reachability or compatibility task must prove those
layers directly.

## Findings

- fixed: named concrete source owners for socket accept/recv/send/poll/close,
  SSH input dispatch, output construction, crypto state, and redacted
  diagnostics.
- fixed: defined the accepted local socket-delivery flow from listener accept
  through recv/commit, SSH dispatch, output send, backpressure, poll/wait, and
  lifecycle close/error propagation.
- fixed: kept socket-delivery evidence local to in-kernel modeled stream
  sockets and explicitly rejected hardware reachability, remote receipt,
  OpenSSH/POSIX/Linux compatibility, phase transition, and ssh-ready=true.
- fixed: defined fail-closed behavior for missing listener/connection,
  wrong-owner descriptors, would-block/backpressure, closed peer, malformed or
  over-limit input, lifecycle violations, poll/wait errors, and
  redaction-sensitive inputs.
- not-an-issue: no Rust implementation or Pi 5 hardware run is required for
  this contract-only task.
- deferred: source implementation, feature smoke evidence, Pi 5 reachability,
  remote receipt, OpenSSH compatibility, POSIX process wait/exit integration,
  boot publication, phase transition, and ssh-ready=true.

## Validation

- static task/docs/source review: pass.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check: pass.

Conditional gates not run: cargo fmt --all -- --check and
cargo -Zjson-target-spec test --quiet were not required because this task
touched no Rust source or Cargo metadata.

No Pi 5 hardware run, lab-controller API action, hardwareTestLock acquisition,
boot publication, source implementation, live reachability claim, remote
receipt claim, compatibility claim, broad expansion, phase transition, or
ssh-ready=true was performed.

## Redaction Review

Pass. Durable evidence retains only task ids, source/doc paths, public socket
ABI names, public readiness bits, public SSH message names or numbers, public
count and length categories, validation commands, readiness counters, fixed
labels, and classifications. It retains no private user data, channel
identifiers, request payload bytes, command payload bytes, channel data bytes,
key/session material, live peer data, hardware data, or boot artifacts.

## Result

Accepted as the bounded local modeled SSH live socket-delivery contract.

selected_next_task=phase12-ssh-live-socket-delivery-core-20260623.
