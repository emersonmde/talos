# Phase 12.3 Integrated Single Ping Transaction Core

Task id: phase12-network-integrated-single-ping-transaction-core-20260619
Status: accepted
Owner: worker
Started: 2026-06-19T19:02:16Z
Accepted: 2026-06-19T19:13:45Z

## Goal

Implement the smallest host-only integrated single-ping transaction lifecycle
by wiring the accepted route-aware outbound ICMP, pending ARP reply poll, and
single-inflight echo reply observation pieces into one allocation-free
coordinator.

## Scope

- Add source/tests in src/network.rs for a single host-only ping transaction
  coordinator over caller-owned buffers and fake/trait-level NetworkDevice
  behavior.
- On a resolved next hop, transmit exactly one IPv4 ICMP echo request and
  record the matching in-flight request only after NetworkDevice transmit
  succeeds.
- On an unresolved next hop, emit one ARP request, retain one pending
  route-aware ICMP request, then allow a matching ARP reply poll to transmit
  the ICMP request and record it in-flight only after transmit succeeds.
- Allow a matching echo reply poll to complete and clear the recorded
  in-flight request.
- Keep deterministic state preservation for route errors, buffer pressure,
  transmit errors, receive errors, nonmatching ARP replies, malformed frames,
  nonmatching echo replies, duplicate pending, and duplicate in-flight
  attempts.

## Non-Goals

- No live driver adapter, live packet I/O, RP1/BCM54213PE hardware action, Pi
  5 run, lab mutation, boot publication, or reachability claim.
- No shell ping command, socket API, UDP/TCP, SSH, smoltcp adoption,
  user-visible network behavior, or phase transition.
- No packet queue, multi-ping support, autonomous timer, scheduler wakeup,
  DHCP, DNS, dynamic routing, or multi-entry in-flight table.
- No Phase 12.1 hardware/link readiness change or same-shaped link polling
  revival.

## Implementation

Added SinglePingTransaction as a host-only coordinator with exactly one
pending ICMP echo request and exactly one in-flight ICMP echo request. The
coordinator exposes:

- start_routed_single_ping_transaction for the first caller-driven send step.
- poll_single_ping_transaction for caller-driven pending ARP reply advancement
  or in-flight echo reply observation.

The resolved-route path prechecks payload capacity before transmit, builds the
accepted routed Ethernet/IPv4/ICMP echo frame into caller-owned storage, and
records in-flight only after NetworkDevice::transmit_frame returns success.
The unresolved-route path delegates to the accepted route-aware pending ICMP
helper, stores pending only after ARP transmit succeeds, and records in-flight
only after a matching ARP reply causes a successful ICMP transmit. The in-flight
poll path delegates to the accepted echo reply observation helper, clearing the
transaction only on a matching reply.

## Findings

- fixed: Integrated successful resolved-route ICMP transmit with single
  in-flight tracking.
- fixed: Integrated unresolved-route ARP request, pending preservation,
  matching ARP reply transmit, pending clear, and in-flight tracking.
- fixed: Integrated matching echo reply completion that clears the in-flight
  transaction.
- fixed: Added host/unit tests for route errors, buffer pressure, transmit
  errors, receive errors, nonmatching ARP replies, malformed ARP frames,
  nonmatching echo replies, duplicate pending starts, and duplicate in-flight
  starts.
- deferred: caller-driven retry/timeout status, shell ping, sockets, packet
  queues, live driver adapters, smoltcp adoption, SSH, reachability, hardware,
  lab mutation, boot publication, and phase transition remain future work.
- removed: no source, docs, or task evidence was removed.
- not-an-issue: Fake/trait-level NetworkDevice tests are sufficient for this
  host-only coordinator boundary but remain insufficient for live NetworkDevice
  acceptance, hardware packet I/O, link readiness, or reachability.

## Validation

- fmt:
  - cargo fmt --all -- --check
  - result: pass.
- focused network unit tests:
  - cargo -Zjson-target-spec test --quiet network::
  - result: pass; the Talos no_std QEMU runner executed the full current test
    suite with 624 passed.
- full unit tests:
  - cargo -Zjson-target-spec test --quiet
  - result: pass; 624 talos no_std tests passed under QEMU/substitute.
- diff whitespace check:
  - git diff --check
  - result: pass.
- docs build:
  - /home/node/.cargo/bin/mdbook build
  - result: pass.
- staged diff whitespace check:
  - git diff --cached --check
  - result: pass before commit.

## Accepted Boundary

The accepted boundary is a host/testable integrated single-ping transaction:
Talos can start one route-aware ICMP echo request, either transmit it
immediately for a resolved next hop or queue one pending request after ARP
transmit for an unresolved next hop, advance a matching ARP reply into exactly
one ICMP echo request transmit and in-flight record, and complete on one
matching ICMP echo reply. All behavior is proven with caller-owned buffers and
fake/trait-level NetworkDevice tests.

## Rejected Claims

- No live packet I/O, live driver adapter, interrupt loop, packet queue,
  autonomous polling/timer, timeout scheduler, retry status, shell ping
  command, socket, SSH, UDP/TCP, smoltcp adoption, network reachability, Pi 5
  hardware proof, boot publication, lab mutation, or phase transition is
  accepted.
- No live NetworkDevice implementation is accepted from fake/mock tests.
- No user-visible ping behavior is accepted by this task.

## Evidence

- Source/test evidence: src/network.rs SinglePingTransaction,
  start_routed_single_ping_transaction, poll_single_ping_transaction, and
  integrated_single_ping_* tests.
- Validation evidence: final command results in this task record after
  acceptance.
- Commit evidence: recorded in durable supervisor state after commit.

## Next Action

selected_next_task=phase12-network-integrated-single-ping-transaction-closeout-20260619.
The closeout is mechanically unblocked after this core task is accepted and
committed. Do not promote shell ping, sockets, live driver, hardware,
smoltcp, SSH, reachability, lab mutation, boot publication, or phase
transition directly from this core.
