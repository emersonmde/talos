# Phase 12.6 SSH POSIX EOF/wait core

Task id: phase12-ssh-posix-eof-wait-core-20260623
Status: accepted
Owner: worker
Classification: phase12-ssh-posix-eof-wait-core-accepted

## Goal

Implement the accepted bounded POSIX EOF/wait integration for the local modeled
SSH shell path without accepting live reachability, remote receipt,
OpenSSH/POSIX/Linux compatibility, broad command expansion, a phase
transition, or ssh-ready=true.

## Reviewed Inputs

- tasks/2026-06-23-phase12-ssh-posix-eof-wait-contract.md.
- tasks/2026-06-23-phase12-ssh-live-socket-delivery-closeout.md.
- tasks/2026-06-23-phase12-ssh-channel-lifecycle-exit-status-closeout.md.
- tasks/2026-06-03-phase10-minimal-waitpid-lifecycle-observation-core.md.
- tasks/2026-06-03-phase10-process-lifecycle-status-record-core.md.
- src/ssh_service_readiness.rs.
- src/local_command_loop.rs.
- docs/src/project/phase12-networking-ssh.md.
- docs/src/roadmap.md.

## Source Changes

- src/local_command_loop.rs exposes a bounded
  LocalCommandProcessLifecycleRecord::completed_wait_exit_status_u32 helper
  for consumers that need the accepted one-record completed wait/status value
  without retaining process payload or command bytes.
- src/ssh_service_readiness.rs adds local modeled outbound channel EOF state,
  a local outbound EOF classifier, and a bounded SshPosixEofWaitReport path
  that composes accepted socket/auth/session/shell/stdio/lifecycle
  prerequisites with one completed wait/status record.
- src/ssh_service_readiness.rs retains focused no_std unit coverage for the
  success path and fail-closed controls.

## Accepted Behavior

The accepted source path is local modeled only:

1. The existing accepted socket-delivery/authentication/session/channel/shell,
   channel-data/stdio, channel-window, and channel-lifecycle prerequisites
   must already be true for the same modeled path.
2. Inbound SSH_MSG_CHANNEL_EOF is recorded as stdin EOF for the one attached
   local process/session.
3. Exactly one completed LocalCommandProcessLifecycleRecord wait/status value
   is consumed as the process completion source.
4. Talos emits one local exit-status request with request type exit-status,
   want_reply=false, and the public u32 status from the completed record.
5. Talos emits local stdout EOF, emits local close, and records full closure
   only after peer close is received.

The accepted success counter is posix-eof-wait-local=true only for this local
modeled sequence. live-reachability=false, remote-receipt=false,
compatibility=false, and ssh-ready=false remain authoritative.

## Failure Contract

The implementation fails closed without accepting EOF/wait integration for:

- missing socket-delivery, authentication, session channel, shell attachment,
  local process/session ownership, local stdio ownership, or open lifecycle
  prerequisites;
- missing process status, already-consumed wait status, or a lifecycle/status
  record that is not an exited/reaped public u32 status;
- duplicate EOF/status/close or invalid channel lifecycle ordering;
- output backpressure or closed peer before output;
- malformed lifecycle input or redaction-sensitive input.

Failure evidence retains only fixed labels, public SSH message numbers, public
status values, boolean counters, validation commands, task ids, and source/doc
paths. It retains no private user data, channel identifiers, request payload
bytes, command payload bytes, channel data bytes, key/session material, live
peer data, hardware data, or boot artifacts.

## Findings

- fixed: added a narrow completed wait/status adapter on the existing
  LocalCommandProcessLifecycleRecord instead of inventing a separate process
  completion source.
- fixed: added outbound local channel EOF state so the accepted ordering can
  distinguish inbound stdin EOF from local stdout EOF.
- fixed: implemented the local modeled SSH POSIX EOF/wait success path with
  exit-status, stdout EOF, local close, and peer-close full closure.
- fixed: retained fail-closed controls for missing prerequisites, missing or
  consumed wait records, lifecycle violations, output backpressure, closed
  peer, and redaction-sensitive input.
- fixed: preserved live-reachability=false, remote-receipt=false,
  compatibility=false, and ssh-ready=false.
- not-an-issue: no Pi 5 hardware run, boot archive publication, remote receipt
  proof, OpenSSH compatibility discriminator, broad command expansion, or
  phase transition is required for this local modeled core task.
- deferred: feature smoke closeout, Pi 5 reachability, remote receipt,
  OpenSSH/POSIX/Linux compatibility, multiple sessions, multiple children,
  blocking wait, scheduler-owned process lifetime, broad shell expansion,
  phase transition, and ssh-ready=true.

## Validation

- cargo fmt --all -- --check: pass.
- cargo -Zjson-target-spec test ssh_posix_eof_wait --quiet: pass; qemu
  no_std harness ran 815 tests because the harness does not narrow execution by
  the substring filter.
- cargo -Zjson-target-spec test --quiet: pass.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check: pass.

No Pi 5 hardware run, lab-controller API action, hardwareTestLock acquisition,
boot publication, live reachability claim, remote receipt claim,
OpenSSH/POSIX/Linux compatibility claim, broad command expansion, phase
transition, or ssh-ready=true was performed.

## Redaction Review

Pass. Durable evidence retains only task ids, source/doc paths, public SSH
message names and numbers, public request type names, public status values,
public length categories, boolean readiness counters, fixed labels,
validation commands, and classifications. It retains no private user data,
channel identifiers, request payload bytes, command payload bytes, channel data
bytes, key/session material, live peer data, hardware data, or boot artifacts.

## Result

Accepted as the bounded local modeled SSH POSIX EOF/wait core implementation.

selected_next_task=phase12-ssh-posix-eof-wait-feature-smoke-20260623.
