# Phase 12.6 SSH session shell attachment contract

Task id: phase12-ssh-session-shell-attachment-contract-20260623
Status: accepted
Owner: worker
Classification: phase12-ssh-session-shell-attachment-contract-accepted

## Goal

Define the smallest feature-led SSH shell attachment contract after the
accepted shell-request classifier, without implementing CHANNEL_SUCCESS or
shell attachment in this task.

## Reviewed Inputs

- tasks/2026-06-22-phase12-ssh-session-shell-request-core.md.
- tasks/2026-06-22-phase12-ssh-session-channel-open-closeout.md.
- tasks/2026-06-22-phase12-ssh-publickey-auth-success-account-closeout.md.
- tasks/2026-06-03-phase10-runtime-console0-stdin-closeout.md.
- tasks/2026-06-04-phase10-scheduler-backed-stdin-wait-closeout.md.
- tasks/2026-06-04-phase10-terminal-ctrl-d-eof-closeout.md.
- tasks/2026-06-03-phase10-userspace-stdio-triad-closeout.md.
- tasks/2026-06-05-phase10-async-process-control-frontier-closeout.md.
- docs/src/architecture/console.md.
- docs/src/architecture/tty-stdio.md.
- docs/src/architecture/lower-el-userspace.md.
- src/ssh_service_readiness.rs.
- docs/src/project/phase12-networking-ssh.md.
- docs/src/roadmap.md.

## Sufficiency Decision

The accepted local foundations are sufficient for a bounded local modeled
CHANNEL_SUCCESS implementation, but only if the implementation is framed as
one authenticated SSH session owning one shell attachment to the already
accepted local execution and stdio surfaces.

The sufficient foundations are:

- local modeled USERAUTH_SUCCESS and one local modeled session channel;
- recognized shell request with failure/no-attachment behavior from the prior
  source slice;
- VFS-backed process launch, lifecycle/status, waitpid, laststatus, and
  background accounting records;
- inherited fd0/fd1/fd2 descriptor records, runtime-console0/local-input
  userspace stdin, scheduler-owned stdin wait/wakeup, first-byte Ctrl-D EOF,
  and userspace stdout/stderr write surfaces.

The implementation remains local and modeled. It must not claim live encrypted
socket reachability, OpenSSH/POSIX/Linux compatibility, Pi 5 hardware
reachability, phase transition, or ssh-ready=true.

## Contract

The next source task may introduce CHANNEL_SUCCESS and shell-attached=true only
for this exact success path:

- accepted local modeled publickey USERAUTH_SUCCESS is present;
- exactly one local modeled session channel is open;
- exactly one recognized SSH_MSG_CHANNEL_REQUEST request type shell is present;
- no prior shell attachment, close, EOF, lifecycle violation, or redaction
  sensitive state exists;
- shell attachment policy is enabled;
- the attachment owns one SSH session shell record tied to the accepted local
  execution surface, including process/session owner identity, inherited
  fd0/fd1/fd2 descriptor identity, runtime-console0/local-input stdin
  readiness semantics, stdout/stderr output identities, and lifecycle/close
  accounting;
- any retained diagnostics use only fixed labels, public SSH names/numbers,
  public count/length categories, counters, paths, commands, task ids, and
  classifications.

The channel-data boundary is deliberately narrow. The implementation may model
the attachment handoff between SSH channel input/output and the accepted local
stdio identities, but it must keep live encrypted packet I/O, socket delivery,
OpenSSH interoperability, channel window management, and hardware reachability
deferred. A later live-reachability task must prove those layers separately.

## Failure Contract

All non-success paths must stay failure/no-attachment:

- missing authentication success;
- missing open session channel;
- missing or duplicate shell request;
- duplicate shell attachment;
- disabled policy;
- unsupported request families such as exec, subsystem, pty-req, env, signal,
  or x11;
- malformed, over-limit, trailing-data, or redaction-sensitive inputs;
- missing local execution/stdio ownership record;
- invalid lifecycle order, EOF/close-before-attach, or already-closed channel.

For want-reply=true, failure paths return or record SSH_MSG_CHANNEL_FAILURE.
For want-reply=false, failure paths record only fixed no-reply
failure/no-attachment labels. No failure path may claim that a shell started.

## Readiness Counters

On the accepted local modeled success path the next source task may report:

- authentication-success=true;
- session-count=1;
- channel-count=1;
- shell-request-count=1;
- shell-attached=true.

live-reachability=false and ssh-ready=false remain authoritative until a later
task proves live encrypted channel I/O and reachability. The first
CHANNEL_SUCCESS implementation must not use ssh-ready=true as a shortcut for
local modeled attachment.

## Findings

- fixed: reconciled the accepted SSH authentication, channel-open, and
  shell-request frontier with accepted local execution, descriptor, stdio, and
  scheduler-backed stdin foundations.
- fixed: defined the minimal CHANNEL_SUCCESS eligibility contract for one
  authenticated session shell attachment.
- fixed: kept fake/kernel-backed remote shell command expansion rejected as
  progress; the shell attachment must consume accepted local
  TTY/process/descriptor/VFS/userspace surfaces.
- fixed: defined failure/no-attachment behavior for missing prerequisites,
  duplicate requests or attachments, unsupported request families, malformed
  or redaction-sensitive inputs, missing local execution ownership, and
  lifecycle violations.
- fixed: preserved live-reachability=false and ssh-ready=false after local
  modeled CHANNEL_SUCCESS.
- deferred: source implementation, live channel data delivery, EOF/close/window
  flow control, exec/subsystem/pty-req/env/signal/x11 expansion, live sockets,
  hardware proof, OpenSSH/POSIX/Linux compatibility, broad expansion, phase
  transition, and ssh-ready=true.
- not-an-issue: no Pi 5 hardware run is required because this task accepts a
  local contract only and does not publish a boot artifact.

## Validation

- static task/docs/source review: pass.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.

Conditional gates not run: cargo fmt --all -- --check and
cargo -Zjson-target-spec test --quiet were not required because this task
touched no Rust source or Cargo metadata.

No Pi 5 hardware run, lab-controller API action, hardwareTestLock acquisition,
boot publication, source implementation, live reachability claim,
compatibility claim, broad expansion, phase transition, or ssh-ready=true was
performed.

## Redaction Review

Pass. Durable evidence retains only task ids, source/doc paths, fixed labels,
public SSH message/request names or numbers, public count/length categories,
validation commands, readiness counters, and classifications. It retains no
private user data, channel identifiers, request payload bytes, key/session
material, hardware data, or boot artifacts.

## Result

Accepted as the bounded local modeled SSH shell attachment contract.

selected_next_task=phase12-ssh-session-shell-attachment-core-20260623.
