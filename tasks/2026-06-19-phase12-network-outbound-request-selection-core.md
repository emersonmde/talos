# Phase 12.3 Outbound Request Selection Core

Task id: phase12-network-outbound-request-selection-core-20260619
Status: accepted
Owner: worker
Started: 2026-06-19T13:00:37Z
Accepted: 2026-06-19T13:05:00Z

## Goal

Implement the thinnest host-only one-shot outbound request selection boundary:
given a requested IPv4 ICMP echo and immutable ARP cache state, build either a
resolved ICMP echo request frame or an unresolved-neighbor ARP request frame
into caller-owned output storage.

## Scope

- Add an allocation-free selector in src/network.rs that composes the accepted
  ArpCache/resolve_outbound_neighbor, build_outbound_ipv4_icmp_echo_request,
  and build_outbound_arp_request helpers.
- Return deterministic request kind plus frame length.
- Keep the selector caller-buffered and host-only, with no ArpCache mutation.
- Cover resolved ICMP, unresolved ARP, buffer pressure for both selected paths,
  oversized resolved ICMP payloads, and cache immutability with focused
  no_std/host tests.
- Update Phase 12.3 docs and roadmap for the accepted selector boundary.

## Non-Goals

- No packet queue, retry timer, neighbor-discovery state machine,
  routing/subnet/gateway selection, driver transmit scheduling, live packet I/O,
  NetworkDevice transmit wrapper, sockets, SSH, smoltcp adoption, ping/network
  reachability behavior, RP1 Ethernet readiness, Pi 5 hardware run, boot
  publication, lab mutation, link-readiness work, or phase transition.
- No Phase 12.1 hardware/link frontier changes.

## Implementation

src/network.rs now includes:

- OutboundRequestKind, distinguishing Ipv4IcmpEchoRequest from ArpRequest.
- OutboundRequestSelection, carrying request_kind and frame_len.
- select_outbound_ipv4_icmp_echo_request, which:
  - reads immutable ArpCache state with resolve_outbound_neighbor;
  - builds a full Ethernet/IPv4/ICMP echo request when the destination is
    resolved;
  - builds a full Ethernet/IPv4 ARP request when the destination is unresolved;
  - returns deterministic kind and frame length;
  - propagates deterministic OutputBufferTooSmall and PayloadTooLarge errors
    from the accepted lower-level builders.

The selector does not take or call a NetworkDevice and does not mutate ArpCache
entries.

## Findings

- fixed: Added the accepted caller-buffered request selector for resolved
  ICMP echo request construction and unresolved ARP request construction.
- fixed: Returned deterministic request kind and frame length for both selected
  paths.
- fixed: Covered resolved-neighbor ICMP request bytes/checksum, unresolved
  ARP request fields, buffer pressure for both paths, oversized resolved ICMP
  payloads, and cache immutability.
- deferred: packet queues, retry timers, neighbor-discovery state, routing,
  driver transmit, live packet I/O, sockets, SSH, smoltcp integration,
  ping/network reachability behavior, and hardware proof remain future work.
- removed: no existing source APIs, tests, docs, task records, or dependencies
  were removed.
- not-an-issue: unresolved-neighbor selection ignores ICMP payload size because
  no IPv4/ICMP packet is built on that path; oversized payload rejection is
  deterministic when the resolved ICMP path is selected.

## Validation

- fmt/lint/typecheck: cargo fmt --all -- --check
  - result: pass.
- unit tests/full suite: cargo -Zjson-target-spec test --quiet
  - result: pass, 587 talos no_std tests passed.
- JSON evidence validation:
  jq empty tasks/evidence/2026-06-19-phase12-network-outbound-request-selection-core/classification.json
  - result: pass.
- diff whitespace check: git diff --check
  - result: pass.
- docs build: /home/node/.cargo/bin/mdbook build
  - result: pass.
- staged diff whitespace check: git diff --cached --check
  - result: pass before commit.

## Accepted Boundary

The accepted boundary is source/test-only host request selection for a single
requested outbound IPv4 ICMP echo. It uses immutable ARP cache state to choose
between caller-buffered ICMP echo request frame construction for resolved
neighbors and caller-buffered ARP request frame construction for unresolved
neighbors.

## Rejected Claims

- No packet queue, retry timer, neighbor-discovery state machine, routing,
  subnet/gateway selection, driver transmit scheduling, live packet I/O,
  packet capture, ping behavior, network reachability, sockets, SSH, UDP/TCP,
  DHCP, DNS, smoltcp adoption, RP1 Ethernet readiness, Pi 5 hardware proof,
  boot archive publication, lab mutation, or phase transition is accepted.
- No NetworkDevice transmit wrapper or live driver adapter is accepted.

## Evidence

- src/network.rs: OutboundRequestKind, OutboundRequestSelection, and
  select_outbound_ipv4_icmp_echo_request.
- src/network.rs tests:
  outbound_request_selection_builds_icmp_for_resolved_neighbor_without_cache_mutation,
  outbound_request_selection_builds_arp_for_unresolved_neighbor_without_cache_mutation,
  outbound_request_selection_rejects_resolved_buffer_pressure_without_partial_frame,
  outbound_request_selection_rejects_unresolved_buffer_pressure_without_partial_frame,
  and outbound_request_selection_rejects_resolved_payloads_too_large_for_ipv4.
- tasks/evidence/2026-06-19-phase12-network-outbound-request-selection-core/classification.json.

## Next Action

phase12-network-outbound-request-selection-closeout-20260619 is mechanically
unblocked for the next worker wake if dependencies remain satisfied and git
status is clean. Do not promote packet queues, retry timers, driver/live
transmit, sockets, SSH, smoltcp adoption, ping/network reachability, hardware
work, boot publication, lab mutation, or any phase transition directly from
this implementation.
