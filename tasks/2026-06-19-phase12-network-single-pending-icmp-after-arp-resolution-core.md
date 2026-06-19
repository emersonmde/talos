# Phase 12.3 Single-Pending ICMP After ARP Resolution Core

Task id: phase12-network-single-pending-icmp-after-arp-resolution-core-20260619
Status: accepted
Owner: worker
Started: 2026-06-19T14:29:00Z
Accepted: 2026-06-19T14:57:00Z

## Goal

Implement the thinnest host-only state needed for one outbound IPv4 ICMP echo
request to pause behind ARP resolution, then advance once the destination MAC is
known.

## Scope

- Add an allocation-free single-pending ICMP echo request holder in
  src/network.rs.
- Preserve endpoint, destination IPv4, identifier, sequence number, TTL, and
  payload bytes while waiting for ARP resolution.
- Emit one deterministic Ethernet/IPv4 ARP request for an unresolved
  destination and record exactly one pending ICMP echo request after successful
  fake/trait-level transmit.
- Advance the pending request from either a matching ARP reply or an existing
  accepted ARP cache resolution into one deterministic Ethernet/IPv4/ICMP echo
  request transmit, then clear the pending state after successful transmit.
- Cover resolved-neighbor, no-pending, duplicate-pending/backpressure, payload
  pressure, output-buffer pressure, malformed ARP, nonmatching ARP, unresolved
  cache, and transmit-error boundaries with unit tests.
- Update Phase 12.3 docs and roadmap for the accepted host/testable boundary.

## Non-Goals

- No packet queue, retry timer, multi-entry buffering, route/subnet/gateway
  selection, asynchronous scheduler integration, receive loop integration, live
  driver transmit, live packet I/O, RP1 driver adapter, smoltcp adoption,
  sockets, SSH, ping/network reachability claim, Pi 5 hardware run, boot
  publication, lab mutation, link-readiness work, or phase transition.
- No live NetworkDevice implementation or hardware readiness claim from fake
  device tests.

## Implementation

src/network.rs now includes:

- PendingIcmpEchoRequest, which stores the local endpoint, destination IPv4,
  identifier, sequence number, TTL, and payload bytes in caller-selected fixed
  storage.
- SinglePendingIcmpEcho, an Option-backed allocation-free holder for exactly one
  pending ICMP echo request.
- PendingIcmpEchoResult, distinguishing successful ICMP transmit, successful ARP
  request transmit plus pending state, no pending request, duplicate pending
  backpressure, pending payload pressure, still-unresolved neighbor, nonmatching
  ARP, request-build errors, ARP parse/shape errors, and transmit errors.
- transmit_or_queue_single_pending_ipv4_icmp_echo_request, which transmits a
  resolved ICMP echo request immediately or emits an ARP request and records one
  pending request for an unresolved destination.
- transmit_single_pending_ipv4_icmp_echo_request, which advances an existing
  pending request when immutable ARP cache state already resolves the neighbor.
- learn_arp_reply_and_transmit_single_pending_ipv4_icmp_echo_request, which
  accepts only a matching Ethernet/IPv4 ARP reply as a direct resolution source,
  learns that ARP fact into the cache, transmits the pending ICMP echo request,
  and clears pending state only after successful transmit.

The implementation remains source/test-only and allocation-free. It uses the
existing caller-owned output buffers and NetworkDevice trait fakes; it does not
add scheduling, retry timers, multi-entry queues, routing, sockets, smoltcp, or
hardware packet I/O.

## Findings

- fixed: Added the bounded single-pending ICMP echo request state and result
  surface.
- fixed: Preserved all requested ICMP echo inputs across unresolved ARP state.
- fixed: Emitted a deterministic ARP request for unresolved neighbors and stored
  exactly one pending request after successful fake-device transmit.
- fixed: Advanced pending ICMP from a matching ARP reply and from existing ARP
  cache resolution, clearing pending state only after successful ICMP transmit.
- fixed: Covered duplicate-pending/backpressure, no-pending, payload pressure,
  output-buffer pressure, unresolved cache, malformed ARP, nonmatching ARP, and
  transmit-error behavior with deterministic tests.
- deferred: packet queues, retry timers, multi-entry buffering, route policy,
  live driver adapters, live packet I/O, smoltcp, sockets, SSH, ping/network
  reachability, hardware readiness, and phase transition remain future tasks.
- removed: no existing source APIs, tests, task records, docs, or dependencies
  were removed.
- not-an-issue: matching ARP reply handling can advance the pending request even
  when cache capacity is exhausted because the reply itself carries the resolved
  MAC; cache insertion remains best-effort and does not broaden into a queue or
  retry policy.

## Validation

- fmt/lint/typecheck:
  . "$HOME/.cargo/env"; export TMPDIR=/opt/strider/openclaw/current/workspace/tmp; export PATH=/opt/strider/openclaw/current/workspace/tools/qemu-9.2.0-install/bin:$PATH; cargo fmt --all -- --check
  - result: pass.
- unit tests/full suite:
  . "$HOME/.cargo/env"; export TMPDIR=/opt/strider/openclaw/current/workspace/tmp; export PATH=/opt/strider/openclaw/current/workspace/tools/qemu-9.2.0-install/bin:$PATH; cargo -Zjson-target-spec test --quiet
  - result: pass.
- JSON evidence validation:
  jq empty tasks/evidence/2026-06-19-phase12-network-single-pending-icmp-after-arp-resolution-core/classification.json
  - result: pass.
- diff whitespace check: git diff --check
  - result: pass.
- docs build: /home/node/.cargo/bin/mdbook build
  - result: pass.
- staged diff whitespace check: git diff --cached --check
  - result: pass before commit.

## Accepted Boundary

The accepted boundary is host/testable single-pending ARP-to-ICMP progression
over caller-owned buffers and fake/trait-level NetworkDevice transmit. One
unresolved ICMP echo request can emit an ARP request, retain fixed-storage
pending state, and later transmit the exact ICMP echo request after a matching
ARP reply or accepted cache resolution.

## Rejected Claims

- No packet queue, retry timer, multi-entry neighbor-discovery queue,
  route/subnet/gateway policy, asynchronous scheduling, live driver transmit,
  live packet I/O, packet capture, ping behavior, network reachability, sockets,
  SSH, UDP/TCP, DHCP, DNS, smoltcp adoption, RP1 driver adapter readiness, DMA
  descriptor ownership, interrupt handling, RP1 Ethernet readiness, Pi 5
  hardware proof, boot publication, lab mutation, or phase transition is
  accepted.
- No live NetworkDevice implementation is accepted from fake/mock tests.

## Evidence

- src/network.rs: PendingIcmpEchoResult, PendingIcmpEchoRequest,
  SinglePendingIcmpEcho,
  transmit_or_queue_single_pending_ipv4_icmp_echo_request,
  transmit_single_pending_ipv4_icmp_echo_request, and
  learn_arp_reply_and_transmit_single_pending_ipv4_icmp_echo_request.
- src/network.rs tests:
  single_pending_icmp_unresolved_neighbor_transmits_arp_and_records_request,
  single_pending_icmp_matching_arp_reply_transmits_icmp_and_clears_pending,
  single_pending_icmp_can_advance_from_existing_cache_resolution,
  single_pending_icmp_reports_backpressure_without_replacing_pending_request,
  single_pending_icmp_reports_payload_and_output_pressure_without_state_change,
  single_pending_icmp_reports_no_pending_unresolved_and_nonmatching_arp_boundaries,
  and
  single_pending_icmp_reports_malformed_arp_and_transmit_errors_without_clearing_pending.
- tasks/evidence/2026-06-19-phase12-network-single-pending-icmp-after-arp-resolution-core/classification.json.

## Next Action

selected_next_task=phase12-network-single-pending-icmp-after-arp-resolution-closeout-20260619.
Promote that closeout on a later worker wake if dependencies remain satisfied
and git status is clean. Do not promote route policy, queues, retries, live
driver transmit, hardware, sockets, SSH, smoltcp adoption, ping/network
reachability, or any phase transition directly from this implementation.
