# Phase 12.3 Packet Buffer and Device Polling Core

Task id: phase12-network-packet-buffer-device-polling-core-20260619

Status: accepted

Classification:
phase12-network-packet-buffer-device-polling-core-accepted

Evidence level: source implementation, no_std unit tests, docs build, and diff
checks. No Pi 5 hardware run, boot archive publication, lab mutation,
hardwareTestLock acquisition, live packet I/O, sockets, SSH, smoltcp adoption,
RP1 Ethernet driver readiness, link readiness, or phase transition was
performed.

## Goal

Implement the local source/test poll-step boundary selected by the packet
buffer and device polling source checkpoint: receive one raw frame into
caller-owned storage, dispatch it through the accepted local packet-dispatch
path, and transmit from caller-owned storage only when a reply is produced.

## Scope Performed

- Extended src/network.rs with LocalPollStepResult and
  poll_local_network_device.
- Reused the existing NetworkDevice trait for caller-owned receive and transmit
  buffers.
- Routed received frames through dispatch_local_packet without allocation,
  hardware access, DMA, interrupts, sockets, smoltcp, or live packet I/O.
- Mapped receive WouldBlock to NoFrame, receive BufferTooSmall to
  ReceiveBufferTooSmall, and other receive errors to ReceiveError.
- Mapped PacketError::NotForLocalHost to NoReply and other PacketError values
  to DispatchError.
- Called NetworkDevice::transmit_frame only after a reply was produced, and
  mapped transmit failures to TransmitError.
- Added fake-device tests for ARP reply transmission, ICMP echo reply
  transmission, no-frame, receive-buffer pressure, receive error, nonlocal
  no-reply, dispatch error, and transmit error behavior.
- Updated Phase 12 docs and roadmap to record the accepted local poll-step
  boundary and rejected live-networking claims.

## Poll Result Matrix

- NoFrame: NetworkDevice::receive_frame returned DeviceError::WouldBlock; no
  dispatch or transmit was attempted.
- ReceiveBufferTooSmall: receive storage could not hold the device-provided
  frame; no dispatch or transmit was attempted.
- ReceiveError(error): receive failed for a non-WouldBlock,
  non-BufferTooSmall device error.
- NoReply: dispatch rejected the frame as not targeting the configured local
  endpoint; no transmit was attempted.
- DispatchError(error): dispatch rejected malformed or unsupported packet
  shapes other than NotForLocalHost; no transmit was attempted.
- TransmitError(error): dispatch produced a reply but transmit_frame failed.
- Replied(result): dispatch produced a reply and transmit_frame accepted the
  caller-owned reply slice.

## Findings

- fixed: Phase 12.3 now has a reusable local poll-step API that bridges
  NetworkDevice receive/transmit with dispatch_local_packet over caller-owned
  receive and transmit buffers.
- fixed: poll results distinguish no frame, receive buffer pressure, receive
  error, nonlocal no-reply, dispatch error, transmit error, and successful reply
  transmission.
- fixed: fake NetworkDevice tests cover ARP reply transmission, ICMP echo reply
  transmission, no-frame, receive-buffer pressure, receive error, no-reply,
  dispatch-error, and transmit-error cases.
- deferred: driver adapter integration, packet queues, ARP cache, UDP/TCP,
  DHCP, DNS, routing, smoltcp integration, sockets, SSH, live packet I/O, and
  Pi 5 packet movement evidence remain future tasks.
- rejected: RP1 Ethernet driver readiness, link readiness, DMA descriptor
  ownership, interrupt integration, live packet I/O, sockets, SSH, smoltcp
  adoption, and phase transition are not accepted by this task.
- removed: no source files, dependencies, or prior task evidence were removed.
- not-an-issue: no hardware lock or inconclusive Pi 5 triage was required
  because this task is local source/test work only.

## Accepted Behavior

- poll_local_network_device receives at most one frame per call into the
  caller-provided receive buffer.
- The received bytes are dispatched with the configured LocalNetworkEndpoint and
  caller-provided transmit buffer.
- The device transmit path is invoked only when dispatch_local_packet produces a
  reply.
- ARP and ICMP echo replies accepted by the prior local packet-dispatch task can
  now be transmitted through a NetworkDevice implementation in a single local
  poll step.
- Negative outcomes are deterministic and preserve the boundary between device
  movement errors and protocol dispatch errors.

## Evidence

- Source implementation and tests: src/network.rs.
- Source checkpoint:
  tasks/2026-06-19-phase12-network-packet-buffer-device-polling-source-checkpoint.md.
- Task classification:
  tasks/evidence/2026-06-19-phase12-network-packet-buffer-device-polling-core/classification.json.
- Phase 12 project doc update: docs/src/project/phase12-networking-ssh.md.
- Roadmap update: docs/src/roadmap.md.

## Acceptance Check

- A reusable local poll-step API exists that receives into caller-owned RX
  storage, dispatches using the accepted local packet-dispatch path, and
  transmits from caller-owned TX storage only when a reply is produced:
  satisfied by poll_local_network_device.
- Poll results distinguish no frame, receive buffer too small or receive error,
  no reply, dispatch rejection/error, transmit error, and successful reply
  transmission without panics or allocation: satisfied by LocalPollStepResult
  and unit tests.
- Host/unit tests with a fake NetworkDevice cover ARP reply transmission, ICMP
  echo reply transmission, nonlocal/no-reply behavior, malformed/dispatch-error
  behavior, receive-buffer pressure, no-frame, and transmit-error cases:
  satisfied.
- The implementation preserves no_std/source-test scope and makes no hardware,
  live packet I/O, smoltcp, socket, SSH, driver-readiness, link-readiness, or
  phase-transition claim: satisfied.
- Findings are recorded with fixed, removed, deferred, or not-an-issue
  disposition: satisfied.

## Validation

- cargo fmt --all -- --check: pass.
- cargo -Zjson-target-spec test --quiet: pass.
- focused unit-test evidence: src/network.rs tests cover poll-step ARP reply
  transmission, ICMP echo reply transmission, nonlocal no-reply behavior,
  dispatch error behavior, receive-buffer pressure, no-frame, receive error, and
  transmit-error behavior.
- jq empty on task-owned JSON evidence: pass.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check: pass before commit.

## Next Action

The queued closeout
phase12-network-packet-buffer-device-polling-closeout-20260619 is mechanically
unblocked after this accepted commit if dependencies remain satisfied. Do not
promote hardware-driver work, live packet I/O, smoltcp adoption, sockets, SSH,
link-readiness work, or any Pi 5 hardware task directly from this
implementation.
