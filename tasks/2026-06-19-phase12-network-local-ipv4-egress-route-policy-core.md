# Phase 12.3 Local IPv4 Egress Route Policy Core

Task id: phase12-network-local-ipv4-egress-route-policy-core-20260619
Status: accepted
Owner: worker
Started: 2026-06-19T15:01:00Z
Accepted: 2026-06-19T15:34:00Z

## Goal

Implement the thinnest host-only IPv4 egress route decision needed before
outbound ARP/ICMP selection can distinguish same-subnet destinations from
gateway-routed destinations.

## Scope

- Add a deterministic local IPv4 egress route policy in src/network.rs.
- Cover same-subnet next-hop, gateway next-hop, missing-gateway/no-route, and
  mask boundary behavior with unit tests.
- Let outbound request selection consume the route decision without mutating ARP
  cache state or transmitting live frames.
- Keep the route boundary host-only and source/testable over caller-owned
  buffers and fake/trait-level packet construction.
- Update Phase 12.3 docs and roadmap for the accepted host-only boundary.

## Non-Goals

- No dynamic routing, DHCP, DNS, packet queue, retry timer, multi-entry neighbor
  discovery queue, route table management, live driver adapter, live packet
  I/O, RP1 hardware action, smoltcp adoption, sockets, SSH, ping/network
  reachability claim, Pi 5 hardware run, boot publication, lab mutation, or
  phase transition.
- No live NetworkDevice implementation or hardware readiness claim from local
  source/unit tests.

## Implementation

src/network.rs now includes:

- Ipv4EgressRoutePolicy, carrying a local subnet mask and optional gateway IPv4.
- Ipv4EgressRouteDecision and Ipv4EgressRouteKind, distinguishing same-subnet
  and gateway next-hop decisions while retaining the final destination IPv4.
- route_ipv4_egress, which chooses the destination itself as next hop for
  same-subnet destinations, chooses the configured gateway as next hop for
  off-subnet destinations, and reports no-route for off-subnet destinations
  without a gateway.
- build_outbound_routed_ipv4_icmp_echo_request, which separates Ethernet
  next-hop resolution from the final IPv4 destination for gateway routes.
- select_routed_outbound_ipv4_icmp_echo_request, which applies route policy
  before immutable ARP-cache lookup and selects either a routed ICMP echo
  request or an ARP request for the next-hop IPv4.

The existing direct select_outbound_ipv4_icmp_echo_request remains available
and preserves the previously accepted direct-neighbor behavior.

## Findings

- fixed: Added deterministic same-subnet, gateway, and no-route route-policy
  decisions.
- fixed: Routed gateway ICMP construction now keeps the IPv4 packet destination
  set to the final destination while resolving Ethernet next hop through the
  gateway MAC.
- fixed: Routed outbound request selection ARPs for an unresolved gateway next
  hop rather than the final off-subnet destination.
- fixed: Unit tests cover same-subnet, gateway, no-route, zero-mask and
  host-mask boundaries, immutable-cache routed selection, unresolved-gateway
  ARP selection, and routed frame-error wrapping.
- deferred: dynamic routing, DHCP, DNS, retry queues, live driver adapters,
  live packet I/O, smoltcp, sockets, SSH, ping/network reachability, hardware
  readiness, and phase transition remain future tasks.
- removed: no existing source APIs, tests, task records, docs, or dependencies
  were removed.
- not-an-issue: Ipv4EgressRoutePolicy accepts raw masks instead of validating
  prefix contiguity; the local route decision is deterministic for every
  32-bit mask and keeps policy validation outside this thinnest host-only
  feature slice.

## Validation

- fmt/lint/typecheck:
  . "$HOME/.cargo/env"; export TMPDIR=/opt/strider/openclaw/current/workspace/tmp; export PATH=/opt/strider/openclaw/current/workspace/tools/qemu-9.2.0-install/bin:$PATH; cargo fmt --all -- --check
  - result: pass.
- unit tests/full suite:
  . "$HOME/.cargo/env"; export TMPDIR=/opt/strider/openclaw/current/workspace/tmp; export PATH=/opt/strider/openclaw/current/workspace/tools/qemu-9.2.0-install/bin:$PATH; cargo -Zjson-target-spec test --quiet
  - result: pass, 606 talos no_std tests passed.
- JSON evidence validation:
  jq empty tasks/evidence/2026-06-19-phase12-network-local-ipv4-egress-route-policy-core/classification.json
  - result: pass.
- diff whitespace check: git diff --check
  - result: pass.
- docs build: /home/node/.cargo/bin/mdbook build
  - result: pass.
- staged diff whitespace check: git diff --cached --check
  - result: pass before commit.

## Accepted Boundary

The accepted boundary is host/testable local IPv4 egress route policy over
caller-owned buffers. Same-subnet destinations use the destination IPv4 as ARP
next hop. Off-subnet destinations use the configured gateway IPv4 as ARP next
hop. Off-subnet destinations without a configured gateway report no-route. The
routed outbound selector can consume that decision without mutating ARP cache
state or transmitting live frames.

## Rejected Claims

- No dynamic routing, DHCP, DNS, packet queue, retry timer, multi-entry
  neighbor-discovery queue, route table management, asynchronous scheduling,
  live driver transmit, live packet I/O, packet capture, ping behavior, network
  reachability, sockets, SSH, UDP/TCP, smoltcp adoption, RP1 driver adapter
  readiness, DMA descriptor ownership, interrupt handling, RP1 Ethernet
  readiness, Pi 5 hardware proof, boot publication, lab mutation, or phase
  transition is accepted.
- No live NetworkDevice implementation is accepted from fake/mock tests.

## Evidence

- src/network.rs: Ipv4EgressRoutePolicy, Ipv4EgressRouteDecision,
  Ipv4EgressRouteKind, OutboundRouteError, route_ipv4_egress,
  build_outbound_routed_ipv4_icmp_echo_request, and
  select_routed_outbound_ipv4_icmp_echo_request.
- src/network.rs tests:
  ipv4_egress_route_policy_uses_destination_for_same_subnet_next_hop,
  ipv4_egress_route_policy_uses_gateway_for_off_subnet_destination,
  ipv4_egress_route_policy_reports_no_route_without_gateway,
  ipv4_egress_route_policy_handles_zero_and_host_mask_boundaries,
  routed_outbound_selection_resolves_gateway_mac_without_mutating_cache,
  routed_outbound_selection_arps_for_unresolved_gateway_next_hop,
  routed_outbound_selection_reports_no_route_before_touching_output, and
  routed_outbound_selection_wraps_frame_errors_without_cache_mutation.
- tasks/evidence/2026-06-19-phase12-network-local-ipv4-egress-route-policy-core/classification.json.

## Next Action

selected_next_task=phase12-network-local-ipv4-egress-route-policy-closeout-20260619.
Promote that closeout on a later worker wake if dependencies remain satisfied
and git status is clean. Do not promote packet queues, retry timers, live
driver transmit, hardware, sockets, SSH, smoltcp adoption, ping/network
reachability, DHCP/DNS, dynamic routing, or any phase transition directly from
this implementation.
