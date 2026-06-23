# Phase 12.6 SSH POSIX EOF/wait contract

Task id: phase12-ssh-posix-eof-wait-contract-20260623
Status: accepted
Owner: worker
Classification: phase12-ssh-posix-eof-wait-contract-accepted

## Goal

Define the bounded contract for connecting the accepted local modeled SSH shell
path to the existing one-record POSIX process wait/exit foundation without
accepting live reachability, remote receipt, OpenSSH/POSIX/Linux
compatibility, broad command expansion, a phase transition, or ssh-ready=true.

## Reviewed Inputs

- tasks/2026-06-23-phase12-ssh-live-socket-delivery-closeout.md.
- tasks/2026-06-23-phase12-ssh-live-socket-delivery-contract.md.
- tasks/2026-06-23-phase12-ssh-channel-lifecycle-exit-status-closeout.md.
- tasks/2026-06-03-phase10-minimal-waitpid-lifecycle-observation-core.md.
- tasks/2026-06-03-phase10-process-lifecycle-status-record-core.md.
- src/ssh_service_readiness.rs.
- src/local_command_loop.rs.
- docs/src/project/phase12-networking-ssh.md.
- docs/src/roadmap.md.

## Accepted Inputs

The next source task may consume only already accepted local-modeled
foundations:

- the SSH pipeline is local modeled only through socket-delivery-local=true,
  authentication-success=true, one session channel, shell-attached=true,
  channel-data-stdio-local=true, channel-window-management=true, and
  channel-lifecycle-local=true;
- the channel lifecycle layer already models inbound SSH_MSG_CHANNEL_EOF,
  local SSH_MSG_CHANNEL_REQUEST exit-status emission with want_reply=false,
  and local SSH_MSG_CHANNEL_CLOSE send/receive ordering;
- the POSIX process side already records one completed local shell child
  lifecycle/status record, exposes a consuming waitpid-style observation, and
  preserves a non-consuming last-process/status observation.

This contract does not turn the local command loop into a network-visible
OpenSSH session. It only defines the one local modeled SSH-attached
process/session ordering that a later source task may implement.

## Contract

The next source task may introduce one local modeled SSH POSIX EOF/wait report
for exactly one authenticated, shell-attached session channel backed by one
local process/session lifecycle record.

The accepted success ordering is:

1. The accepted local socket-delivery path establishes the modeled SSH
   connection and channel prerequisites.
2. A single local process/session is attached to that accepted channel with
   fd0/fd1/fd2 ownership inherited from the accepted shell/process model.
3. Inbound SSH_MSG_CHANNEL_EOF on the accepted channel is treated as stdin EOF
   for that attached local process/session. It may not fabricate command
   input, broad shell commands, PATH lookup, or kernel-backed command output.
4. The local process/session reaches the existing completed lifecycle/status
   state using the accepted one-record wait/exit foundation.
5. The SSH channel emits exactly one local exit-status request with request
   type exit-status, want_reply=false, and the public u32 status from the
   completed lifecycle/status record.
6. After exit-status, the local side may emit SSH_MSG_CHANNEL_EOF for stdout
   exhaustion, then SSH_MSG_CHANNEL_CLOSE.
7. The channel is fully closed only after the local close has been emitted and
   the peer close has been received; later stdin/stdout/window/lifecycle
   operations fail closed.

The contract accepts only local modeled EOF/wait integration. It does not prove
that a remote peer received EOF, stdout, exit-status, close, or any socket
bytes.

## Failure Contract

All non-success paths must fail closed without accepting EOF/wait integration:

- missing local modeled socket delivery, authentication, session channel, shell
  attachment, stdio ownership, or channel lifecycle prerequisite;
- no attached local process/session, no completed lifecycle/status record, or a
  wait record that was already consumed;
- inbound EOF before shell attachment or after both channel close directions;
- duplicate inbound EOF, duplicate exit-status emission, duplicate local EOF,
  or duplicate local close;
- exit-status attempted before process completion, after peer close when the
  local lifecycle forbids output, or with a redaction-sensitive lifecycle
  source;
- malformed, unsupported, over-limit, lifecycle-invalid, or
  redaction-sensitive SSH lifecycle input;
- send/output backpressure, closed peer, hangup/error, or missing accepted
  connection when output is queued to the local socket model.

Failure evidence may retain only task ids, source/doc paths, public SSH message
names/numbers, public request type names, public status values, public length
categories, fixed labels, readiness counters, validation commands, and
classifications. It must not retain channel identifiers, request payload bytes,
command payload bytes, channel data bytes, key/session material, private user
data, live peer data, hardware data, or boot artifacts.

## Readiness Counters

The next source task may add or update only local modeled counters needed for
this slice:

- posix-eof-wait-local=true or equivalent on the accepted local EOF/wait
  success path;
- existing local authentication/session/channel/shell/socket-delivery,
  channel-data, channel-window, and channel-lifecycle counters may remain true
  only when their accepted prerequisites are satisfied by the same modeled
  path;
- live-reachability=false;
- remote-receipt=false;
- compatibility=false;
- ssh-ready=false.

The implementation must not use ssh-ready=true as a shortcut for local
EOF/wait integration.

## Findings

- fixed: named the accepted local modeled SSH path extended by this contract.
- fixed: named the existing POSIX process wait/exit foundation as the only
  process lifecycle input the next source task may consume.
- fixed: defined EOF, process completion, exit-status, channel EOF, and channel
  close ordering for exactly one local modeled attached process/session.
- fixed: rejected broad command expansion and new fake/kernel-backed commands
  as progress.
- fixed: preserved hard false live-reachability, remote-receipt,
  compatibility, and ssh-ready counters.
- not-an-issue: no Rust source implementation or Pi 5 hardware run is required
  for this contract-only task.
- deferred: source implementation, feature smoke evidence, Pi 5 reachability,
  remote receipt, OpenSSH/POSIX/Linux compatibility, multiple sessions,
  multiple children, blocking wait, scheduler-owned process lifetime, broad
  shell expansion, phase transition, and ssh-ready=true.

## Deferred Scope

The next task is limited to source behavior for this contract. It must not add
new shell commands as progress, claim Linux/POSIX compatibility, publish a boot
archive, acquire the hardware lock, or infer live SSH readiness from local
modeled EOF/wait behavior.

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
receipt claim, OpenSSH/POSIX/Linux compatibility claim, broad command
expansion, phase transition, or ssh-ready=true was performed.

## Redaction Review

Pass. Durable evidence retains only task ids, source/doc paths, public SSH
message names and numbers, public request type names, public status values,
public length categories, fixed labels, readiness counters, validation
commands, and classifications. It retains no private user data, channel
identifiers, request payload bytes, command payload bytes, channel data bytes,
key/session material, live peer data, hardware data, or boot artifacts.

## Result

Accepted as the bounded local modeled SSH POSIX EOF/wait contract.

selected_next_task=phase12-ssh-posix-eof-wait-core-20260623.
