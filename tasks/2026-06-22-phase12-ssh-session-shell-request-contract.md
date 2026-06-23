# Phase 12.6 SSH session shell-request contract

Task id: phase12-ssh-session-shell-request-contract-20260622

Status: accepted.

Classification: phase12-ssh-session-shell-request-contract-accepted.

## Goal

Define the smallest post-channel-open SSH shell-request contract after the
accepted local modeled session channel bookkeeping frontier, while keeping PTY
allocation, process ownership, shell execution, channel data, live reachability,
hardware proof, OpenSSH/POSIX/Linux compatibility, broad expansion, phase
transition, and ssh-ready=true unaccepted.

## Scope

- Reviewed the accepted session channel-open closeout and local shell, TTY,
  scheduler, process, descriptor, and filesystem boundaries.
- Defined the first modeled SSH_MSG_CHANNEL_REQUEST boundary for request type
  shell on the one accepted session channel bookkeeping handle.
- Preserved the rule that recognizing a shell request is not enough to accept
  shell attachment. Until PTY/process/shell ownership is separately accepted,
  the modeled response remains failure/no-attachment behavior.
- Defined the only readiness counters available to the next implementation:
  authentication-success=true from the accepted prerequisite,
  session-count=1, channel-count=1, shell-request-count may become 1, and
  shell-attached=false, live-reachability=false, and ssh-ready=false remain
  authoritative.
- Selected the next bounded implementation task.

## Non-goals

No Rust source implementation, packet serialization, live encrypted packet I/O,
PTY allocation, TTY session ownership, process ownership, shell process launch,
descriptor handoff, channel data handling, EOF/close/window flow control, exec,
subsystem, pty-req, env, signal, or x11 forwarding requests, filesystem-backed
remote command execution, live socket reachability, hardware/lab action, boot
publication, OpenSSH/POSIX/Linux compatibility claim, broad expansion, phase
transition, or ssh-ready=true is accepted.

Durable evidence must not retain request payload bytes, channel identifiers,
window sizes, packet sizes, user/operator identity, key material,
key-derived identifiers, stable identifiers, session-id bytes, signatures,
hardware data, or boot artifacts.

## Reviewed Inputs

- tasks/2026-06-22-phase12-ssh-session-channel-open-closeout.md.
- tasks/2026-06-22-phase12-ssh-session-channel-open-contract.md.
- tasks/2026-06-22-phase12-ssh-session-channel-open-core.md.
- tasks/2026-06-22-phase12-ssh-session-channel-open-smoke.md.
- src/ssh_service_readiness.rs.
- src/local_command_loop.rs.
- src/tty.rs.
- src/process_install.rs.
- src/scheduler.rs.
- src/fs.rs.
- docs/src/project/phase12-networking-ssh.md.
- docs/src/roadmap.md.
- docs/src/decisions/README.md.

## Contract

The next implementation may add a local modeled shell-request classifier with
exactly one recognized path:

- the accepted local publickey USERAUTH_SUCCESS account-policy result is
  present for the same modeled SSH service flow;
- exactly one local modeled session channel is already open from the accepted
  SSH_MSG_CHANNEL_OPEN session-channel frontier;
- no shell is already attached in this slice;
- shell-request policy is enabled;
- the path is not redaction-sensitive;
- the decoded message number is SSH_MSG_CHANNEL_REQUEST;
- the request type is the public SSH string shell;
- the request has the exact public four-field shell shape:
  message number, recipient channel, request type, and want-reply boolean; and
- no request-specific payload bytes follow the want-reply boolean.

The recognized shell request is prerequisite-only. It may record only that the
public request label was recognized on an already-open local session channel.
It must not model shell attachment, a PTY, a TTY owner, a process owner, a
login session, scheduler ownership, descriptor ownership, current working
directory, environment, filesystem-backed command execution, remote channel
data, live packet I/O, or OpenSSH compatibility.

Because no PTY/process/shell attachment is accepted by this contract, the next
implementation must not classify SSH_MSG_CHANNEL_SUCCESS as accepted behavior.
If want-reply is true, the modeled response must be
SSH_MSG_CHANNEL_FAILURE. If want-reply is false, the modeled classifier may
record the fixed failure/no-reply boundary but must not claim that a shell
started. In both cases shell-attached remains false and ssh-ready remains false.

The next implementation must fail closed for missing authentication success,
missing open session channel, disabled policy, duplicate shell request,
redaction-sensitive paths, wrong message number, unsupported channel request
type, malformed packet, over-limit field shape, trailing request-specific data,
and every other non-recognized path.

The successful recognized-but-unattached path may move only these counters:

- authentication-success remains true from the accepted account-success
  prerequisite;
- session-count remains 1;
- channel-count remains 1;
- shell-request-count may become 1;
- shell-attached remains false;
- live-reachability remains false; and
- ssh-ready remains false.

The next implementation may choose exact enum/source names, but retained
diagnostics and task evidence must stay in these fixed-label families:

- session-shell-request-prerequisite-only;
- session-shell-request-shell-type;
- session-shell-request-want-reply;
- session-shell-request-no-reply;
- session-shell-request-failure-shell-unattached;
- session-shell-request-failure-authentication-missing;
- session-shell-request-failure-channel-missing;
- session-shell-request-failure-policy-disabled;
- session-shell-request-failure-duplicate;
- session-shell-request-failure-unsupported-message;
- session-shell-request-failure-unsupported-request-type;
- session-shell-request-failure-request-malformed;
- session-shell-request-failure-redaction-sensitive;
- authentication-success-local-only;
- session-open-local-only;
- channel-open-local-only;
- shell-unattached;
- not-ready.

Retained evidence may expose only fixed labels, public SSH message names or
numbers, public request type names, public want-reply true/false categories,
public field-count/length categories, readiness counters, validation commands,
task ids, source/doc paths, and classifications.

## Findings and Disposition

- fixed: defined the smallest post-channel-open boundary as recognizing one
  SSH_MSG_CHANNEL_REQUEST whose request type is shell on the already accepted
  local session channel bookkeeping handle.
- fixed: separated shell-request recognition from shell attachment. The
  contract requires failure/no-attachment behavior until PTY/process/shell
  ownership is accepted separately.
- fixed: defined want-reply behavior without accepting CHANNEL_SUCCESS:
  want-reply=true produces CHANNEL_FAILURE, and want-reply=false may record
  no-reply failure/no-attachment classification only.
- fixed: preserved shell-attached=false, live-reachability=false, and
  ssh-ready=false after the modeled shell-request result.
- fixed: required malformed, unsupported, missing-prerequisite, disabled,
  duplicate, over-limit, trailing-data, and redaction-sensitive paths to fail
  closed with fixed labels.
- fixed: durable evidence redaction excludes request payload bytes, channel
  identifiers, window and packet sizes, user/operator identity, key material,
  session-id bytes, stable identifiers, hardware data, and boot artifacts.
- deferred: Rust source implementation, retained smoke evidence, closeout,
  PTY/process/shell attachment, descriptor handoff, channel data,
  EOF/close/window flow control, exec/subsystem/pty/env/signal handling, live
  reachability, hardware proof, OpenSSH/POSIX/Linux compatibility, broad
  expansion, phase transition, and ssh-ready=true.
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
message and request names, fixed shell-request label families, public
want-reply categories, public field-count and length categories, readiness
counters, validation commands, and classifications. It retains no request
payload bytes, channel identifiers, window sizes, packet sizes, user/operator
identity, key material, key-derived identifiers, stable identifiers, session-id
bytes, signatures, hardware data, or boot artifacts.

## Accepted Frontier

Talos now has a bounded post-channel-open shell-request contract. The next
implementation may recognize one SSH_MSG_CHANNEL_REQUEST request type shell on
the existing local modeled session channel, but it must keep the request
unattached: want-reply=true returns CHANNEL_FAILURE and want-reply=false records
only no-reply failure/no-attachment classification. That result may report
shell-request-count=1 while authentication-success=true, session-count=1, and
channel-count=1 come from the accepted prerequisites.

This accepts only the contract. No source implementation, CHANNEL_SUCCESS,
shell attachment, PTY/process ownership, descriptor handoff, channel data,
exec/subsystem/pty/env/signal behavior, live reachability, hardware proof,
OpenSSH/POSIX/Linux compatibility, broad expansion, phase transition, or
ssh-ready=true is accepted.

selected_next_task=phase12-ssh-session-shell-request-core-20260622.
