# Phase 12.3 Pending-Aware ARP Reply Poll Core

Task id: phase12-network-pending-aware-arp-reply-poll-core-20260619
Status: accepted
Owner: worker
Started: 2026-06-19T17:30:30Z
Accepted: 2026-06-19T18:02:00Z

## Goal

Implement the thinnest host-only poll boundary that lets a received matching
ARP reply advance one stored route-aware pending ICMP echo request into exactly
one trait-level ICMP transmit.

## Scope

- Add a host-only NetworkDevice polling helper for pending ARP replies.
- Preserve the accepted single-pending route-aware ICMP request semantics:
  final IPv4 destination and ARP next-hop IPv4 stay separate.
- Learn matching ARP reply sender facts in the caller-provided ARP cache.
- Clear pending state only after successful ICMP echo request transmit.
- Cover gateway-routed pending, no-pending, no-frame, receive pressure,
  receive errors, nonmatching ARP, malformed ARP, output pressure, and transmit
  errors with unit tests.

## Non-Goals

- No live packet I/O, driver adapter, interrupt loop, packet queue,
  autonomous retry timer, socket, shell ping command, smoltcp adoption, SSH,
  Pi 5 hardware run, lab mutation, boot publication, reachability claim, or
  phase transition.
- No multi-entry neighbor-discovery state or timeout behavior.
- No change to the accepted local ARP reply or inbound ICMP echo
  reply-to-request behavior.

## Implementation

src/network.rs now includes PendingIcmpEchoPollResult and
poll_pending_arp_reply_and_transmit_single_pending_ipv4_icmp_echo_request.
The helper:

- returns NoPendingRequest before receiving when no pending ICMP echo request is
  stored;
- maps NetworkDevice::receive_frame WouldBlock, BufferTooSmall, and other
  receive errors to deterministic poll results;
- delegates received frames to the accepted
  learn_arp_reply_and_transmit_single_pending_ipv4_icmp_echo_request boundary;
- learns the ARP sender for a matching next-hop reply, transmits one
  Ethernet/IPv4/ICMP echo request through the same NetworkDevice, and clears
  pending only after that transmit succeeds.

## Findings

- fixed: Matching ARP reply polling can now advance a stored route-aware
  pending ICMP echo request to exactly one trait-level ICMP transmit.
- fixed: Gateway-routed pending requests learn and transmit through the gateway
  next-hop MAC while preserving the final IPv4 destination in the emitted IPv4
  packet.
- fixed: No-pending, no-frame, receive-buffer pressure, receive errors,
  nonmatching ARP, malformed ARP, output-buffer pressure, and transmit errors
  have deterministic outcomes and preserve pending state unless ICMP transmit
  succeeds.
- fixed: Existing local ARP reply and inbound ICMP echo reply polling tests
  remain intact; the new helper is separate from local reply dispatch.
- deferred: packet queues, live driver adapters, autonomous polling loops,
  sockets, shell ping, SSH, smoltcp adoption, reachability, hardware, lab
  mutation, boot publication, and phase transition remain future work.
- removed: no source API, task evidence, or docs were removed.
- not-an-issue: The lower-level ARP-reply learn/transmit helper already had the
  right pending-state semantics; this task only needed the receive/poll wrapper
  and receive-boundary tests.

## Validation

- source inspection:
  src/network.rs PendingIcmpEchoPollResult,
  poll_pending_arp_reply_and_transmit_single_pending_ipv4_icmp_echo_request,
  pending_arp_reply_poll_* tests, and existing local poll tests.
- fmt/lint/typecheck:
  . "$HOME/.cargo/env"; export TMPDIR=/opt/strider/openclaw/current/workspace/tmp; cargo fmt --all -- --check
  - result: initial fail before formatting; cargo fmt --all applied.
- fmt/lint/typecheck after formatting:
  . "$HOME/.cargo/env"; export TMPDIR=/opt/strider/openclaw/current/workspace/tmp; cargo fmt --all -- --check
  - result: pass.
- unit tests/full suite:
  . "$HOME/.cargo/env"; export TMPDIR=/opt/strider/openclaw/current/workspace/tmp; export PATH=/opt/strider/openclaw/current/workspace/tools/qemu-9.2.0-install/bin:$PATH; cargo -Zjson-target-spec test --quiet
  - result: pass.
- diff whitespace check: git diff --check
  - result: pass.
- docs build: /home/node/.cargo/bin/mdbook build
  - result: pass.
- staged diff whitespace check: git diff --cached --check
  - result: pass before commit.

## Accepted Boundary

The accepted boundary is host/testable pending-aware ARP reply polling over
caller-owned receive/transmit buffers and fake/trait-level NetworkDevice
receive/transmit. A matching ARP reply for the stored next-hop IPv4 can learn
that neighbor, emit one ICMP echo request for the stored final IPv4
destination, and clear pending state after successful ICMP transmit. Boundary
errors preserve pending state.

## Rejected Claims

- No live packet I/O, driver adapter, interrupt loop, packet queue,
  autonomous retry timer, timeout, shell ping command, socket, SSH, UDP/TCP,
  smoltcp adoption, network reachability, Pi 5 hardware proof, boot
  publication, lab mutation, or phase transition is accepted.
- No live NetworkDevice implementation is accepted from fake/mock tests.

## Evidence

- src/network.rs:
  PendingIcmpEchoPollResult and
  poll_pending_arp_reply_and_transmit_single_pending_ipv4_icmp_echo_request.
- src/network.rs tests:
  pending_arp_reply_poll_advances_gateway_pending_to_single_icmp_transmit,
  pending_arp_reply_poll_distinguishes_no_pending_no_frame_and_receive_errors,
  and
  pending_arp_reply_poll_preserves_pending_on_nonmatch_malformed_pressure_and_transmit_error.

## Next Action

selected_next_task=phase12-network-pending-aware-arp-reply-poll-closeout-20260619.
Promote that closeout on a later worker wake if dependencies remain satisfied
and git status is clean. Do not promote live driver, hardware, socket, SSH,
smoltcp, reachability, lab, boot publication, or phase transition directly from
this implementation.
