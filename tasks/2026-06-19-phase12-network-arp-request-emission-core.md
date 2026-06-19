# Phase 12.3 ARP Request Emission Core

Task id: phase12-network-arp-request-emission-core-20260619
Status: accepted
Owner: worker
Started: 2026-06-19T11:29:00Z
Accepted: 2026-06-19T11:45:00Z

## Goal

Implement the thinnest host-only ARP request emission boundary after the
accepted outbound IPv4/ICMP request-construction frontier: construct a single
Ethernet II ARP request frame into caller-owned output storage.

## Scope

- Add an allocation-free helper in src/network.rs that builds a complete
  Ethernet/IPv4 ARP request frame from a local endpoint and target IPv4.
- Keep the helper below packet queues, retry timers, driver transmit, live
  packet I/O, sockets, SSH, smoltcp adoption, and hardware readiness claims.
- Cover success, output-buffer pressure, and unresolved-neighbor composition
  with focused host/no_std tests.
- Update Phase 12.3 docs and roadmap for the accepted source boundary.

## Non-Goals

- No ARP retry state machine, packet queue, route lookup, subnet/gateway
  selection, driver adapter, transmit scheduling, live packet I/O, sockets,
  SSH, ping/network reachability behavior, RP1 Ethernet readiness, Pi 5
  hardware run, boot publication, lab mutation, or phase transition.
- No ARP cache mutation from request construction.

## Implementation

src/network.rs now includes build_outbound_arp_request. The helper writes:

- Ethernet broadcast destination MAC.
- Local endpoint source MAC.
- EtherType ARP.
- ARP hardware type Ethernet, protocol type IPv4, hardware length 6, protocol
  length 4, and operation request.
- Endpoint sender MAC/IP.
- Zero target MAC.
- Caller-provided target IPv4.

The helper returns ETHERNET_HEADER_LEN + ARP_ETHERNET_IPV4_LEN on success and
returns OutboundFrameError::OutputBufferTooSmall before writing if caller-owned
storage is too small.

## Findings

- fixed: Added the accepted caller-buffered ARP request frame construction
  helper for local endpoint plus target IPv4.
- fixed: Covered deterministic success fields, exact frame length, broadcast
  destination, zero target MAC, and parser-readable ARP request shape.
- fixed: Covered too-small output storage without partial frame construction.
- fixed: Covered composition with the unresolved result from
  resolve_outbound_neighbor without mutating ARP cache state.
- deferred: ARP request scheduling, retry timers, neighbor-discovery state,
  packet queues, routing, driver transmit, and live packet I/O remain follow-up
  work.
- not-an-issue: The helper does not take OutboundNeighborResolution directly;
  callers can pass resolution.destination_ipv4(), and the tests cover that
  composition while keeping construction independent from cache lookup.

## Validation

- fmt/lint/typecheck: cargo fmt --all
- unit tests/focused network filter: cargo -Zjson-target-spec test arp_request
  --quiet
  - result: pass, 582 talos no_std tests passed in this harness.
- unit tests/full suite: cargo -Zjson-target-spec test --quiet
  - result: pass, 582 talos no_std tests passed.
- fmt/lint/typecheck: cargo fmt --all -- --check
  - result: pass.
- JSON evidence validation:
  jq empty tasks/evidence/2026-06-19-phase12-network-arp-request-emission-core/classification.json
  - result: pass.
- diff whitespace check: git diff --check
  - result: pass.
- docs build: /home/node/.cargo/bin/mdbook build
  - result: pass.
- staged diff whitespace check: git diff --cached --check
  - result: pass before commit.

## Accepted Boundary

The accepted boundary is source/test-only caller-buffered Ethernet/IPv4 ARP
request frame construction. It is allocation-free and host-only. It does not
mutate ARP cache state, consult or transmit through a driver, queue packets,
perform live packet I/O, claim network reachability, or change the Phase 12.1
hardware frontier.

## Rejected Claims

- No packet queue, retry timer, route lookup, subnet/gateway selection, driver
  transmit, live packet I/O, sockets, SSH, smoltcp adoption, ping/network
  reachability behavior, RP1 Ethernet readiness, Pi 5 hardware readiness, boot
  publication, lab mutation, or phase transition was accepted.

## Evidence

- src/network.rs: build_outbound_arp_request implementation and tests:
  outbound_arp_request_builds_complete_broadcast_request_frame,
  outbound_arp_request_rejects_too_small_output_without_partial_frame, and
  outbound_arp_request_composes_with_unresolved_neighbor_resolution_without_cache_mutation.
- tasks/evidence/2026-06-19-phase12-network-arp-request-emission-core/classification.json.

## Next Action

Promote phase12-network-arp-request-emission-closeout-20260619 on a later wake
if dependencies remain satisfied. Do not jump directly to packet queues, retry
timers, driver transmit, live packet I/O, sockets, SSH, ping/network
reachability, hardware work, lab mutation, boot publication, or a phase
transition.
