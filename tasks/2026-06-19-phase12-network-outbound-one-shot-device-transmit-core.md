# Phase 12.3 Outbound One-Shot Device Transmit Core

Task id: phase12-network-outbound-one-shot-device-transmit-core-20260619
Status: accepted
Owner: worker
Started: 2026-06-19T13:21:27Z
Accepted: 2026-06-19T13:45:00Z

## Goal

Implement the thinnest fake/trait-level one-shot outbound transmit wrapper:
given immutable ARP cache state and caller-owned output storage, build one
resolved ICMP echo request or unresolved ARP request frame and invoke
NetworkDevice::transmit_frame exactly once when construction succeeds.

## Scope

- Add a host-testable helper in src/network.rs that composes the accepted
  select_outbound_ipv4_icmp_echo_request helper with NetworkDevice::transmit_frame.
- Return deterministic outcomes for ARP request transmit, ICMP echo request
  transmit, request build/selection error, and transmit error.
- Preserve caller-owned output storage and immutable ARP cache behavior.
- Add fake-device tests for exactly-one transmit on success, no transmit on
  build failure, and deterministic transmit-error reporting.
- Update Phase 12.3 docs and roadmap for the accepted fake/trait-level
  one-shot transmit boundary.

## Non-Goals

- No receive loop, packet queue, retry timer, neighbor-discovery state machine,
  routing/subnet/gateway selection, asynchronous scheduling, live packet I/O,
  RP1 driver adapter, DMA descriptor ownership, interrupts, sockets, SSH,
  smoltcp adoption, ping/network reachability claim, Pi 5 hardware run, boot
  publication, lab mutation, link-readiness work, or phase transition.
- No hardware or live driver readiness claim from mock NetworkDevice transmit.

## Implementation

src/network.rs now includes:

- OutboundTransmitResult, distinguishing:
  - Ipv4IcmpEchoRequestTransmitted,
  - ArpRequestTransmitted,
  - RequestError,
  - TransmitError.
- transmit_one_outbound_ipv4_icmp_echo_request, which:
  - delegates request selection and frame construction to the accepted
    select_outbound_ipv4_icmp_echo_request helper;
  - returns RequestError before any device transmit when selection/building
    fails;
  - calls NetworkDevice::transmit_frame exactly once with the caller-owned
    output frame slice after successful construction;
  - maps successful transmit to ARP or ICMP-specific outcomes;
  - reports transmit errors with request kind, frame length, and DeviceError.

The helper uses the NetworkDevice trait only. It does not schedule, queue,
retry, receive, route, perform live packet I/O, or bind to an RP1 driver.

## Findings

- fixed: Added the accepted fake/trait-level one-shot outbound transmit helper.
- fixed: Returned deterministic success, request-error, and transmit-error
  outcomes.
- fixed: Covered resolved-neighbor ICMP request transmit and unresolved-neighbor
  ARP request transmit with exactly one fake-device transmit attempt.
- fixed: Covered build failure with zero fake-device transmit attempts.
- fixed: Covered transmit-error reporting after successful caller-buffered frame
  construction.
- deferred: packet queues, retry timers, neighbor-discovery state, routing,
  live driver transmit, live packet I/O, sockets, SSH, smoltcp integration,
  ping/network reachability behavior, and hardware proof remain future work.
- removed: no existing source APIs, tests, docs, task records, or dependencies
  were removed.
- not-an-issue: the transmit-error path leaves the constructed frame in
  caller-owned storage because construction succeeded before the fake device
  rejected transmit; this is local evidence only, not a delivery claim.

## Validation

- fmt/lint/typecheck: cargo fmt --all -- --check
  - result: pass.
- unit tests/full suite: cargo -Zjson-target-spec test --quiet
  - result: pass, 591 talos no_std tests passed.
- JSON evidence validation:
  jq empty tasks/evidence/2026-06-19-phase12-network-outbound-one-shot-device-transmit-core/classification.json
  - result: pass.
- diff whitespace check: git diff --check
  - result: pass.
- docs build: /home/node/.cargo/bin/mdbook build
  - result: pass.
- staged diff whitespace check: git diff --cached --check
  - result: pass before commit.

## Accepted Boundary

The accepted boundary is source/test-only fake/trait-level one-shot outbound
transmit. It builds one outbound frame into caller-owned storage through the
accepted request selector and invokes NetworkDevice::transmit_frame exactly once
after successful construction.

## Rejected Claims

- No packet queue, retry timer, neighbor-discovery state machine, routing,
  subnet/gateway selection, asynchronous scheduling, live driver transmit,
  live packet I/O, packet capture, ping behavior, network reachability, sockets,
  SSH, UDP/TCP, DHCP, DNS, smoltcp adoption, RP1 Ethernet readiness, Pi 5
  hardware proof, boot archive publication, lab mutation, or phase transition is
  accepted.
- No RP1 driver adapter, DMA descriptor ownership, interrupt handling, or live
  NetworkDevice implementation readiness is accepted.

## Evidence

- src/network.rs: OutboundTransmitResult and
  transmit_one_outbound_ipv4_icmp_echo_request.
- src/network.rs tests:
  outbound_one_shot_transmits_icmp_request_once_for_resolved_neighbor,
  outbound_one_shot_transmits_arp_request_once_for_unresolved_neighbor,
  outbound_one_shot_does_not_transmit_when_request_building_fails, and
  outbound_one_shot_reports_transmit_error_after_successful_build.
- tasks/evidence/2026-06-19-phase12-network-outbound-one-shot-device-transmit-core/classification.json.

## Next Action

phase12-network-outbound-one-shot-device-transmit-closeout-20260619 is
mechanically unblocked for the next worker wake if dependencies remain
satisfied and git status is clean. Do not promote queues, retries, live driver
transmit, hardware, sockets, SSH, smoltcp adoption, ping/network reachability,
or any phase transition directly from this implementation.
