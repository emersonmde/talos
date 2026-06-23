# Phase 12.6 SSH channel-data stdio contract

Task id: phase12-ssh-channel-data-stdio-contract-20260623
Status: accepted
Owner: worker
Classification: phase12-ssh-channel-data-stdio-contract-accepted

## Goal

Define the smallest feature-led channel-data/stdio bridge contract after the
accepted local modeled shell attachment closeout, without implementing source
behavior in this task.

## Reviewed Inputs

- tasks/2026-06-23-phase12-ssh-session-shell-attachment-contract.md.
- tasks/2026-06-23-phase12-ssh-session-shell-attachment-core.md.
- tasks/2026-06-23-phase12-ssh-session-shell-attachment-feature-smoke.md.
- tasks/2026-06-23-phase12-ssh-session-shell-attachment-closeout.md.
- tasks/2026-06-22-phase12-ssh-session-channel-open-closeout.md.
- tasks/2026-06-22-phase12-ssh-session-shell-request-core.md.
- tasks/2026-06-03-phase10-runtime-console0-stdin-closeout.md.
- tasks/2026-06-03-phase10-userspace-stdio-triad-closeout.md.
- tasks/2026-06-04-phase10-scheduler-backed-stdin-wait-closeout.md.
- tasks/2026-06-04-phase10-terminal-ctrl-d-eof-closeout.md.
- docs/src/architecture/console.md.
- docs/src/architecture/tty-stdio.md.
- docs/src/project/phase10-local-command-stdio-bridge-closeout-checkpoint.md.
- src/ssh_service_readiness.rs.
- src/ssh_runtime_crypto.rs.
- docs/src/project/phase12-networking-ssh.md.
- docs/src/roadmap.md.

## Sufficiency Decision

The accepted local foundations are sufficient for a bounded local modeled
channel-data/stdio bridge implementation, but only below live socket delivery.
The next source task may connect decrypted SSH channel-data classifications to
the accepted shell attachment's local stdio ownership model and may construct
outbound stdout/stderr channel-data reports from the same ownership boundary.

The sufficient foundations are:

- accepted local publickey authentication success, one accepted session
  channel, one recognized shell request, and one local shell attachment;
- accepted local process/session ownership and fd0/fd1/fd2 stdio identities;
- runtime-console0/local-input stdin semantics, scheduler-backed stdin
  wait/wakeup, first-byte Ctrl-D EOF, stdout/stderr descriptor write surfaces,
  and local command-loop stdio bridge evidence;
- accepted encrypted packet state diagnostics and local decrypted-payload
  classifiers, with payload bytes kept out of durable evidence.

This remains a local modeled bridge. It does not accept encrypted socket
delivery, channel-window accounting, OpenSSH/POSIX/Linux compatibility,
hardware reachability, phase transition, or ssh-ready=true.

## Contract

The next source task may introduce a channel-data/stdio bridge only for this
exact inbound success path:

- accepted local modeled authentication success is present;
- exactly one local modeled session channel is open;
- exactly one local modeled shell request and shell attachment are active;
- local process/session ownership plus fd0/fd1/fd2 stdio ownership are present;
- the channel lifecycle is open and has not observed EOF, close, or a lifecycle
  violation;
- the decrypted payload is one SSH_MSG_CHANNEL_DATA packet with a public
  bounded field shape: message number, recipient channel, string length, and
  bounded data length;
- the data is classified only by public length category and fixed test labels,
  not by retaining command payload bytes.

On that path, the implementation may model delivery of the inbound data bytes
to the attached shell's stdin ownership boundary. It may report
channel-data-stdio-local=true and a public data length category, while
preserving live-reachability=false, channel-window-management=false, and
ssh-ready=false.

The next source task may also construct outbound local reports for stdout and
stderr only from accepted attached stdio ownership:

- stdout maps to SSH_MSG_CHANNEL_DATA with a public bounded data length;
- stderr maps to SSH_MSG_CHANNEL_EXTENDED_DATA with extended-data type
  SSH_EXTENDED_DATA_STDERR and a public bounded data length;
- outbound reports are local packet-construction evidence only and do not prove
  encrypted socket write, remote receipt, or OpenSSH interoperability.

EOF, close, exit-status, channel-window adjustment, flow control, PTY modes,
exec/subsystem request families, terminal job control, broad shell behavior,
and live transport delivery stay deferred to later explicit tasks.

## Failure Contract

All non-success paths must fail closed without claiming stdio delivery:

- missing authentication success;
- missing open session channel;
- missing shell request or shell attachment;
- missing local process/session or fd0/fd1/fd2 stdio ownership;
- wrong message family, including requests that are not SSH_MSG_CHANNEL_DATA on
  the inbound stdin path;
- unsupported SSH_MSG_CHANNEL_EXTENDED_DATA on the inbound path;
- malformed, trailing-data, over-limit, zero-capacity, or redaction-sensitive
  payloads;
- channel lifecycle violation, including EOF/close-before-data or already
  closed channel;
- outbound stdout/stderr without attached stdio ownership.

Failure evidence must use fixed labels, public message names or numbers, public
field-count and length categories, counters, paths, commands, task ids, and
classifications only. It must not retain channel identifiers, payload bytes,
key/session material, private user data, hardware data, or boot artifacts.

## Readiness Counters

On the accepted local modeled success path the next source task may report:

- authentication-success=true;
- session-count=1;
- channel-count=1;
- shell-request-count=1;
- shell-attached=true;
- channel-data-stdio-local=true.

live-reachability=false, channel-window-management=false, and ssh-ready=false
remain authoritative until later tasks prove those layers directly.

## Findings

- fixed: defined the bounded inbound SSH_MSG_CHANNEL_DATA to attached stdin
  contract after the accepted local shell attachment frontier.
- fixed: defined local outbound stdout/stderr report semantics using
  SSH_MSG_CHANNEL_DATA and SSH_MSG_CHANNEL_EXTENDED_DATA without claiming live
  socket delivery.
- fixed: retained public length/category evidence only and rejected durable
  retention of channel identifiers, payload bytes, key/session material,
  hardware data, or boot artifacts.
- fixed: kept channel-window management, EOF/close, exit-status, PTY, exec,
  subsystem, terminal job control, live sockets, hardware reachability,
  compatibility, broad expansion, phase transition, and ssh-ready=true
  deferred.
- not-an-issue: no Rust implementation or Pi 5 hardware proof is required for
  this contract-only task.

## Validation

- static task/docs/source review: pass.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check: pass.

Conditional gates not run: cargo fmt --all -- --check and
cargo -Zjson-target-spec test --quiet were not required because this task
touched no Rust source or Cargo metadata.

No Pi 5 hardware run, lab-controller API action, hardwareTestLock acquisition,
boot publication, source implementation, live reachability claim,
compatibility claim, broad expansion, phase transition, or ssh-ready=true was
performed.

## Redaction Review

Pass. Durable evidence retains only task ids, source/doc paths, fixed labels,
public SSH message names or numbers, public field-count and length categories,
validation commands, readiness counters, and classifications. It retains no
private user data, channel identifiers, request or command payload bytes,
key/session material, hardware data, or boot artifacts.

## Result

Accepted as the bounded local modeled SSH channel-data/stdio bridge contract.

selected_next_task=phase12-ssh-channel-data-stdio-core-20260623.
