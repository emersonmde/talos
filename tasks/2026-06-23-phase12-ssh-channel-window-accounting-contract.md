# Phase 12.6 SSH channel-window accounting contract

Task id: phase12-ssh-channel-window-accounting-contract-20260623

Status: accepted

Classification: phase12-ssh-channel-window-accounting-contract-accepted

## Goal

Define the bounded local modeled SSH channel-window accounting contract after
the accepted channel-data/stdio closeout, without implementing source behavior
or claiming live socket reachability.

## Reviewed Inputs

- tasks/2026-06-23-phase12-ssh-channel-data-stdio-contract.md.
- tasks/2026-06-23-phase12-ssh-channel-data-stdio-core.md.
- tasks/2026-06-23-phase12-ssh-channel-data-stdio-feature-smoke.md.
- tasks/2026-06-23-phase12-ssh-channel-data-stdio-closeout.md.
- tasks/2026-06-23-phase12-ssh-session-shell-attachment-closeout.md.
- docs/src/project/phase12-networking-ssh.md.
- docs/src/roadmap.md.
- src/ssh_service_readiness.rs.

## Contract

The next source task may add local modeled per-channel receive-window and
send-window accounting on top of the accepted channel-data/stdio bridge. This
contract accepts accounting only for exactly one authenticated local modeled
session channel with one shell request, one shell attachment, local
process/session ownership, fd0/fd1/fd2 stdio ownership, and open channel
lifecycle state.

Initial ownership is split by channel direction:

- Talos receive window: a fixed local modeled receive window advertised by the
  local CHANNEL_OPEN_CONFIRMATION. The core implementation should introduce a
  named constant for this value and keep it a small multiple of the accepted
  SSH_CHANNEL_DATA_MAX_BYTES test boundary.
- Talos receive max-packet: the existing accepted SSH_CHANNEL_DATA_MAX_BYTES
  boundary unless the core task proves a narrower named constant is required.
- Remote receive window: the peer-advertised initial-window-size parsed from
  the accepted SSH_MSG_CHANNEL_OPEN shape. Talos owns this as the outbound send
  budget for stdout and stderr packet-shape reports.
- Remote max-packet: the peer-advertised maximum-packet-size parsed from the
  accepted SSH_MSG_CHANNEL_OPEN shape, capped by the accepted local
  SSH_CHANNEL_DATA_MAX_BYTES boundary before any local outbound report is
  accepted.

Inbound SSH_MSG_CHANNEL_DATA may reach the accepted stdin boundary only when:

- the message is SSH_MSG_CHANNEL_DATA with the accepted public field shape;
- the data length is nonzero;
- the data length is no larger than the local receive max-packet;
- the data length is no larger than the current Talos receive-window
  remaining count.

On accepted inbound data, Talos decrements the receive-window remaining count
by the data length before reporting channel-data-stdio-local=true. When the
remaining count is at or below a named low-water mark, the local modeled report
may emit SSH_MSG_CHANNEL_WINDOW_ADJUST with message number 93, recipient
channel, and bytes-to-add. The adjust amount must restore the receive window to
the named initial receive-window value and must be public counter-only
evidence. It must not retain channel identifiers or payload bytes.

Outbound stdout/stderr reports may be accepted only when the public data length
is nonzero, no larger than the effective outbound max-packet, and no larger
than the current remote receive-window/send-budget count. Accepted outbound
stdout reports decrement the remote receive-window count and retain
SSH_MSG_CHANNEL_DATA as the public message shape. Accepted outbound stderr
reports decrement the same remote receive-window count and retain
SSH_MSG_CHANNEL_EXTENDED_DATA with SSH_EXTENDED_DATA_STDERR as the public
message shape. If the remote receive window is exhausted or the data length is
larger than the remote budget, the classifier must fail closed or report a
local would-block/window-exhausted classification without claiming socket
delivery.

Inbound SSH_MSG_CHANNEL_WINDOW_ADJUST may only increase Talos' remote
receive-window/send-budget count. The accepted shape is message number 93,
recipient channel, bytes-to-add, and no trailing fields. bytes-to-add must be
nonzero and must not overflow the modeled u32 receive-window count; malformed,
zero, over-limit, overflow, unsupported-message, redaction-sensitive, and
lifecycle-violation cases fail closed.

Oversized or invalid channel data is not delivered to stdio and must not
mutate window counts. Oversized includes data larger than the accepted local
max-packet, data larger than the current receive-window remaining count, and
outbound data larger than the effective remote max-packet or remote
receive-window count. Inbound SSH_MSG_CHANNEL_EXTENDED_DATA remains
unsupported for stdin in this contract.

## Readiness Counters

The next source task may change channel-window-management from false to true
only on local modeled paths that satisfy this contract. The accepted capability
would then be local channel-data/stdio byte plumbing plus local channel-window
accounting only.

live-reachability=false and ssh-ready=false remain authoritative. No live
encrypted socket delivery, remote receipt, hardware reachability,
OpenSSH/POSIX/Linux compatibility, phase transition, or ssh-ready=true is
accepted by this contract.

## Findings

- fixed: defined local modeled receive-window and send-window ownership for the
  next source slice.
- fixed: defined decrement/increment rules for inbound data, outbound
  stdout/stderr reports, and inbound CHANNEL_WINDOW_ADJUST.
- fixed: defined SSH_MSG_CHANNEL_WINDOW_ADJUST encoding expectations: message
  number 93, recipient channel, bytes-to-add, and no trailing fields.
- fixed: defined oversized/invalid data handling as fail-closed with no stdio
  delivery and no window mutation.
- deferred: Rust implementation, unit coverage, live socket delivery, hardware
  reachability, OpenSSH/POSIX/Linux compatibility, EOF/close/exit-status
  behavior, broad shell behavior, phase transition, and ssh-ready=true remain
  for later explicit tasks.
- not-an-issue: this task does not require Pi 5 hardware evidence or boot
  publication.

## Evidence

- Reviewed inputs listed above: pass by static task/docs/source inspection.
- Contract text above records initial local/remote channel window ownership,
  decrement/increment rules, CHANNEL_WINDOW_ADJUST trigger and encoding,
  oversized-data behavior, and readiness-counter effects.
- Validation:
  - git diff --check: pass.
  - /home/node/.cargo/bin/mdbook build: pass.
  - git diff --cached --check: pass.

Conditional gates not run: cargo fmt --all -- --check and
cargo -Zjson-target-spec test --quiet were not run because this contract task
touched no Rust source or Cargo metadata.

No live socket delivery, lab-controller API action, hardwareTestLock
acquisition, Pi 5 hardware run, boot publication, compatibility claim, broad
expansion, phase transition, or ssh-ready=true was performed.

## Redaction Review

Pass. Retained evidence is limited to task ids, source/doc paths, public SSH
message names and numbers, fixed labels, public counters and booleans, public
length fields, validation commands, and classifications. It retains no private
user data, channel identifiers, payload bytes, command bytes, key/session
material, live peer data, hardware data, or boot artifacts.

## Result

Accepted as the bounded local modeled SSH channel-window accounting contract.

selected_next_task=phase12-ssh-channel-window-accounting-core-20260623.
