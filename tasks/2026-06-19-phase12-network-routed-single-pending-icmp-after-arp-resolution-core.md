# Phase 12.3 Routed Single-Pending ICMP After ARP Resolution Core

Task id: phase12-network-routed-single-pending-icmp-after-arp-resolution-core-20260619
Status: accepted
Owner: worker
Started: 2026-06-19T16:01:50Z
Accepted: 2026-06-19T16:22:00Z

## Goal

Implement the thinnest host-only route-aware single-pending ICMP path: decide
same-subnet versus gateway next hop before ARP resolution, store exactly one
pending ICMP echo request when the next hop is unresolved, and transmit the
final-destination ICMP echo request after matching ARP resolution.

## Scope

- Add a route-aware single-pending outbound ICMP path in src/network.rs.
- Preserve the existing direct single-pending ICMP API while adapting pending
  state to retain both final destination IPv4 and ARP next-hop IPv4.
- For same-subnet destinations, ARP for the destination and later transmit the
  ICMP echo request to the destination MAC.
- For gateway routes, ARP for the configured gateway and later transmit the
  ICMP echo request to the final IPv4 destination with the gateway MAC.
- Report deterministic no-route before transmit or pending-state mutation.
- Cover resolved, unresolved, ARP-reply, nonmatching-ARP, no-route,
  output-pressure, payload-pressure, and transmit-error behavior with host/unit
  tests.

## Non-Goals

- No multi-entry packet queue, retry timer, dynamic route table, DHCP, DNS,
  live driver adapter, live packet I/O, Pi 5 hardware run, lab mutation, boot
  publication, smoltcp adoption, sockets, SSH, ping/network reachability claim,
  or phase transition.
- No RP1/BCM54213PE hardware/link readiness policy changes.
- No replacement of the existing direct single-pending API.

## Implementation

src/network.rs now includes:

- PendingIcmpEchoRequest::new_with_next_hop and next_hop_ipv4, so pending ICMP
  state records the final IPv4 destination separately from the ARP next hop.
- PendingIcmpEchoResult::RouteError, used by the route-aware pending entrypoint
  to report no-route before transmit or pending mutation.
- transmit_or_queue_routed_single_pending_ipv4_icmp_echo_request, which applies
  route_ipv4_egress before ARP lookup, emits an ARP request for an unresolved
  next hop, and stores one pending ICMP request with final destination plus
  next-hop identity.
- Route-aware pending advancement in transmit_single_pending_ipv4_icmp_echo_request
  and learn_arp_reply_and_transmit_single_pending_ipv4_icmp_echo_request: both
  resolve or match the stored next-hop IPv4, then build the ICMP echo request
  with the stored final destination.

The existing transmit_or_queue_single_pending_ipv4_icmp_echo_request API remains
available and stores direct-neighbor requests with next_hop_ipv4 equal to
destination_ipv4.

## Findings

- fixed: Pending ICMP state now carries both final destination IPv4 and ARP
  next-hop IPv4, enabling gateway-routed pending requests without changing the
  direct API.
- fixed: Route-aware pending ICMP ARPs for same-subnet destinations directly
  and for gateway-routed destinations through the gateway IPv4.
- fixed: Matching gateway ARP resolution transmits an IPv4/ICMP echo request to
  the final destination while using the gateway MAC as the Ethernet destination.
- fixed: Off-subnet no-gateway requests return RouteError(NoRouteToDestination)
  before device transmit, output mutation, or pending-state mutation.
- fixed: Nonmatching ARP replies preserve gateway pending state; existing
  malformed-ARP and transmit-error tests continue to preserve pending state.
- deferred: retry timers, packet queues, route-table expansion, live driver
  adapters, live packet I/O, smoltcp, sockets, SSH, ping/network reachability,
  hardware work, boot publication, lab mutation, and phase transition remain
  future tasks.
- removed: no source API, tests, docs, task records, or evidence were removed.
- not-an-issue: PendingIcmpEchoResult::PendingNeighborUnresolved retains its
  existing destination_ipv4 field name while route-aware paths report the
  unresolved next-hop IPv4 in that field.

## Validation

- fmt/lint/typecheck:
  . "$HOME/.cargo/env"; export TMPDIR=/opt/strider/openclaw/current/workspace/tmp; export PATH=/opt/strider/openclaw/current/workspace/tools/qemu-9.2.0-install/bin:$PATH; cargo fmt --all -- --check
  - result: pass.
- unit tests/full suite:
  . "$HOME/.cargo/env"; export TMPDIR=/opt/strider/openclaw/current/workspace/tmp; export PATH=/opt/strider/openclaw/current/workspace/tools/qemu-9.2.0-install/bin:$PATH; cargo -Zjson-target-spec test --quiet
  - result: pass, 610 talos no_std tests passed.
- JSON evidence validation:
  jq empty tasks/evidence/2026-06-19-phase12-network-routed-single-pending-icmp-after-arp-resolution-core/classification.json
  - result: pass.
- diff whitespace check: git diff --check
  - result: pass.
- docs build: /home/node/.cargo/bin/mdbook build
  - result: pass.
- staged diff whitespace check: git diff --cached --check
  - result: pass before commit.

## Accepted Boundary

The accepted boundary is host/testable route-aware single-pending ICMP over
caller-owned buffers and fake/trait-level NetworkDevice transmit. One unresolved
request can be stored after ARP request transmit. Same-subnet requests store the
destination as next hop. Gateway-routed requests store the gateway as next hop
and the final destination separately. Matching ARP resolution for the next hop
can transmit the final-destination ICMP echo request and clear pending state
after successful fake-device transmit.

## Rejected Claims

- No multi-entry packet queue, retry timer, autonomous scheduling, dynamic
  route table, DHCP, DNS, live driver adapter, live packet I/O, packet capture,
  ping behavior, network reachability, sockets, SSH, UDP/TCP, smoltcp adoption,
  RP1 driver readiness, Pi 5 hardware proof, boot publication, lab mutation, or
  phase transition is accepted.
- No RP1/BCM54213PE hardware/link readiness policy changes are accepted.
- No live NetworkDevice implementation is accepted from fake/mock tests.

## Evidence

- src/network.rs:
  PendingIcmpEchoResult::RouteError, PendingIcmpEchoRequest::new_with_next_hop,
  PendingIcmpEchoRequest::next_hop_ipv4,
  transmit_or_queue_routed_single_pending_ipv4_icmp_echo_request,
  transmit_single_pending_ipv4_icmp_echo_request, and
  learn_arp_reply_and_transmit_single_pending_ipv4_icmp_echo_request.
- src/network.rs tests:
  routed_single_pending_icmp_same_subnet_unresolved_arps_destination,
  routed_single_pending_icmp_gateway_route_arps_gateway_and_transmits_to_final_destination,
  routed_single_pending_icmp_reports_no_route_without_pending_or_transmit,
  routed_single_pending_icmp_preserves_gateway_pending_on_nonmatching_arp,
  plus the retained direct single_pending_icmp_* tests for resolved, unresolved,
  output-pressure, payload-pressure, malformed-ARP, and transmit-error
  behavior.
- tasks/evidence/2026-06-19-phase12-network-routed-single-pending-icmp-after-arp-resolution-core/classification.json.

## Next Action

selected_next_task=phase12-network-routed-single-pending-icmp-after-arp-resolution-closeout-20260619.
Promote that closeout on a later worker wake if dependencies remain satisfied
and git status is clean. Do not promote retry timing, packet queues, live
driver adapters, hardware, sockets, SSH, smoltcp adoption, ping/network
reachability, lab mutation, boot publication, or phase transition except
through the explicit queued dependency chain.
