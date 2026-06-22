# Phase 12.6 SSH session channel-open contract

Task id: phase12-ssh-session-channel-open-contract-20260622

Status: accepted.

Classification: phase12-ssh-session-channel-open-contract-accepted.

## Goal

Define the smallest post-authentication session channel-open contract after
the accepted local publickey USERAUTH_SUCCESS account-policy frontier, while
keeping shell attachment, PTY/process ownership, exec/subsystem requests, live
reachability, hardware proof, OpenSSH/POSIX/Linux compatibility, broad
expansion, phase transition, and ssh-ready=true unaccepted.

## Scope

- Reviewed the accepted publickey USERAUTH_SUCCESS account-policy closeout,
  SSH service readiness source, encrypted transport dispatch shape, diagnostic
  counters, local shell/TTY/process boundaries, Phase 12 project docs,
  roadmap, and decision log.
- Defined the first modeled post-auth session/channel boundary for one
  SSH_MSG_CHANNEL_OPEN request whose channel type is the public SSH string
  session.
- Defined the readiness counter transition for the next implementation:
  authentication-success remains true from the accepted prerequisite, and a
  successful modeled session channel-open may report session-count=1 and
  channel-count=1.
- Preserved shell-attached=false, live-reachability=false, and
  ssh-ready=false as authoritative after the modeled channel-open result.

## Non-goals

No Rust source implementation, packet serialization, actual encrypted packet
I/O, multiple sessions, multiple channels, channel window flow control,
channel data, PTY allocation, shell attachment, process ownership, exec or
subsystem requests, environment requests, signal handling, login session
semantics, filesystem-backed shell behavior, live socket reachability,
hardware/lab action, boot publication, OpenSSH/POSIX/Linux compatibility
claim, broad expansion, phase transition, or ssh-ready=true is accepted.

Durable evidence must not retain request payload bytes, recipient or sender
channel identifiers, window sizes, packet sizes, user names, operator
identity, key material, key-derived identifiers, stable identifiers,
session-id bytes, signatures, hardware data, or boot artifacts.

## Reviewed Inputs

- tasks/2026-06-22-phase12-ssh-publickey-auth-success-account-closeout.md.
- tasks/2026-06-22-phase12-ssh-publickey-auth-success-account-policy-contract.md.
- tasks/2026-06-22-phase12-ssh-publickey-auth-success-account-core.md.
- tasks/2026-06-22-phase12-ssh-publickey-auth-success-account-smoke.md.
- src/ssh_service_readiness.rs.
- src/diagnostic_command.rs.
- src/local_command_loop.rs.
- src/tty.rs.
- src/process_install.rs.
- src/scheduler.rs.
- docs/src/project/phase12-networking-ssh.md.
- docs/src/roadmap.md.
- docs/src/decisions/README.md.

## Contract

The next implementation may add a local modeled session channel-open
classifier with exactly one accepted success path:

- the accepted local publickey USERAUTH_SUCCESS account-policy result is
  present for the same modeled SSH service flow;
- encrypted transport dispatch has reached the post-authentication channel
  request slice;
- the decoded message number is SSH_MSG_CHANNEL_OPEN;
- the channel type is the public SSH string session;
- no prior modeled session/channel is open in this slice;
- the request field shape is within accepted bounded lengths and counts; and
- session/channel policy is enabled.

The successful modeled result may classify SSH_MSG_CHANNEL_OPEN_CONFIRMATION
and may move only these counters:

- authentication-success remains true from the accepted account-success
  prerequisite;
- session-count may become 1;
- channel-count may become 1;
- shell-attached remains false;
- live-reachability remains false; and
- ssh-ready remains false.

The next implementation must classify SSH_MSG_CHANNEL_OPEN_FAILURE for all
fail-closed paths in this slice, including missing authentication success,
wrong message number, unsupported channel type, malformed channel-open packet,
policy disabled, existing modeled session/channel already open, unsupported
or over-limit field shape, redaction-sensitive paths, and every other
non-success path.

The accepted session channel is a protocol bookkeeping handle only. It is not
a PTY, TTY, process owner, scheduler task, login session, shell instance,
environment, current working directory, socket reachability proof, POSIX file
descriptor, OpenSSH compatibility claim, or public ABI. Channel data,
window-adjust behavior, channel close/eof, shell/pty/exec/subsystem request
handling, process launch, filesystem-backed command execution, and live
packet I/O remain separate future contracts.

The next implementation may choose exact enum/source names, but retained
diagnostics and task evidence must stay in these fixed-label families:

- session-channel-open-prerequisite-only;
- session-channel-open-session-type;
- session-channel-open-failure-authentication-missing;
- session-channel-open-failure-unsupported-message;
- session-channel-open-failure-unsupported-channel-type;
- session-channel-open-failure-request-malformed;
- session-channel-open-failure-policy-disabled;
- session-channel-open-failure-existing-channel;
- session-channel-open-failure-redaction-sensitive;
- authentication-success-local-only;
- session-open-local-only;
- channel-open-local-only;
- shell-unattached;
- not-ready.

Retained evidence may expose only fixed labels, public SSH message names or
numbers, public field-count/length categories, readiness counters, validation
commands, task ids, source/doc paths, and classifications.

## Findings and Disposition

- fixed: defined the smallest post-authentication boundary as one modeled
  SSH_MSG_CHANNEL_OPEN request for channel type session instead of jumping to
  shell, PTY, exec, subsystem, or live socket behavior.
- fixed: tied the success path to the accepted local publickey
  USERAUTH_SUCCESS account-policy prerequisite.
- fixed: defined the only counter transition allowed in the next source slice:
  session-count=1 and channel-count=1 after modeled channel-open success,
  while shell-attached=false, live-reachability=false, and ssh-ready=false
  remain authoritative.
- fixed: required unsupported, malformed, missing-authentication, disabled,
  duplicate-channel, over-limit, and redaction-sensitive paths to fail closed
  with channel-open failure labels.
- fixed: separated the protocol bookkeeping handle from local shell, TTY,
  process, descriptor, filesystem, and scheduler ownership.
- fixed: durable evidence redaction excludes channel identifiers, request
  payload bytes, window/packet sizes, user/operator identity, key material,
  session-id bytes, stable identifiers, hardware data, and boot artifacts.
- deferred: Rust source implementation, retained smoke evidence, closeout,
  channel data, EOF/close/window flow control, PTY/process/shell attachment,
  exec/subsystem requests, live reachability, hardware proof,
  OpenSSH/POSIX/Linux compatibility, broad expansion, phase transition, and
  ssh-ready=true.
- not-an-issue: no Rust source change is required for this contract because it
  defines the next implementation boundary only.

## Validation

- static task/docs/source review: pass.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check: pass.

Conditional gates not run: cargo fmt --all -- --check and
cargo -Zjson-target-spec test --quiet were not required because this contract
touched no Rust source or Cargo metadata.

No Pi 5 hardware run, lab-controller API action, hardwareTestLock acquisition,
boot publication, Rust source implementation, shell attachment, PTY/process
ownership, live reachability claim, compatibility claim, broad expansion, or
phase transition was performed.

## Redaction Review

Pass. Retained evidence contains only task ids, source/doc paths, public SSH
message names, fixed session/channel label families, public field-count and
length categories, readiness counters, validation commands, and
classifications. It retains no request payload bytes, channel identifiers,
window sizes, packet sizes, user names, operator identity, key material,
key-derived identifiers, stable identifiers, session-id bytes, signatures,
hardware data, or boot artifacts.

## Accepted Frontier

Talos now has a bounded post-authentication session channel-open contract. The
next implementation may model SSH_MSG_CHANNEL_OPEN_CONFIRMATION only for one
authenticated SSH_MSG_CHANNEL_OPEN request whose channel type is session and
whose bounded field shape passes policy. That result may report
session-count=1 and channel-count=1, with authentication-success still coming
from the accepted account-success prerequisite.

This accepts only the contract. No source implementation, shell attachment,
PTY/process ownership, exec/subsystem behavior, live reachability, hardware
proof, OpenSSH/POSIX/Linux compatibility, broad expansion, phase transition,
or ssh-ready=true is accepted.

selected_next_task=phase12-ssh-session-channel-open-core-20260622.
