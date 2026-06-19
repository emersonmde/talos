# Phase 12.3 Single-Pending ARP Retry Core

Task id: phase12-network-single-pending-arp-retry-core-20260619
Status: accepted
Owner: worker
Started: 2026-06-19T16:31:00Z
Accepted: 2026-06-19T16:52:00Z

## Goal

Implement the thinnest host-only explicit ARP retry path for an already stored
single-pending outbound ICMP echo request.

## Scope

- Keep retry caller-driven; no timers, scheduler wakeups, background polling,
  or reachability claims.
- Let a stored unresolved pending ICMP request re-emit the ARP request for its
  stored next hop, including gateway-routed next hops from the accepted
  route-aware pending path.
- Track a deterministic retry budget in the pending request.
- Preserve pending state across no-pending, output-buffer pressure, and
  transmit-error boundaries according to the retry contract.
- Cover the behavior with host/unit tests in src/network.rs.

## Non-Goals

- No multi-entry packet queue, hidden retry timer, autonomous scheduling,
  dynamic route table, DHCP, DNS, live driver adapter, live packet I/O, Pi 5
  hardware run, lab mutation, boot publication, smoltcp adoption, sockets,
  SSH, ping/network reachability claim, or phase transition.
- No RP1/BCM54213PE hardware/link readiness change.
- No replacement of the existing zero-retry direct or route-aware pending APIs.

## Implementation

src/network.rs now includes:

- PendingIcmpEchoRequest::new_with_next_hop_and_arp_retry_budget and
  arp_retries_remaining, keeping retry state in the existing single pending
  request rather than adding a queue or timer.
- PendingIcmpEchoResult::ArpRetryBudgetExhausted, which reports the stored
  final destination and next-hop IPv4 when explicit retries are exhausted.
- transmit_or_queue_routed_single_pending_ipv4_icmp_echo_request_with_arp_retry_budget,
  an opt-in route-aware queueing entrypoint. The existing route-aware function
  delegates to it with a zero retry budget, preserving previous behavior.
- retry_single_pending_ipv4_icmp_echo_arp_request, which re-emits one ARP
  request for the stored next hop only when the caller explicitly invokes it
  and the pending request has retry budget remaining.

Retry budget decrements only after a successful fake/trait-level ARP transmit.
Output-buffer pressure, device transmit errors, and budget exhaustion leave the
pending request stored with its prior deterministic state.

## Findings

- fixed: A stored unresolved pending ICMP request can now re-emit an ARP request
  for its stored next hop without recomputing routes or mutating ARP cache
  state.
- fixed: Gateway-routed pending requests retry ARP for the gateway IPv4 while
  retaining the final IPv4 destination separately.
- fixed: Retry budget exhaustion is deterministic and leaves the pending
  request stored with zero retries remaining.
- fixed: No-pending, output-buffer pressure, and transmit-error paths are
  covered and do not clear pending state or spend retry budget.
- deferred: packet queues, autonomous timers, live driver transmit, hardware
  packet I/O, sockets, SSH, smoltcp, reachability, lab mutation, boot
  publication, and phase transition remain future work.
- removed: no source API, task evidence, or docs were removed.
- not-an-issue: Existing non-budgeted direct and route-aware queueing helpers
  still default to zero retries, so callers must opt into explicit retry state.

## Validation

- fmt/lint/typecheck:
  . "$HOME/.cargo/env"; export TMPDIR=/opt/strider/openclaw/current/workspace/tmp; cargo fmt --all -- --check
  - result: initial fail before formatting; cargo fmt --all applied.
- fmt/lint/typecheck after formatting:
  . "$HOME/.cargo/env"; export TMPDIR=/opt/strider/openclaw/current/workspace/tmp; cargo fmt --all -- --check
  - result: pass.
- unit tests/full suite:
  . "$HOME/.cargo/env"; export TMPDIR=/opt/strider/openclaw/current/workspace/tmp; export PATH=/opt/strider/openclaw/current/workspace/tools/qemu-9.2.0-install/bin:$PATH; cargo -Zjson-target-spec test --quiet
  - result: pass, 613 talos no_std tests passed.
- diff whitespace check: git diff --check
  - result: pass.
- docs build: /home/node/.cargo/bin/mdbook build
  - result: pass.
- staged diff whitespace check: git diff --cached --check
  - result: pass before commit.

## Accepted Boundary

The accepted boundary is host/testable explicit ARP retry over caller-owned
buffers and fake/trait-level NetworkDevice transmit. One stored pending ICMP
echo request may re-emit ARP for its stored next-hop IPv4 when the caller calls
retry_single_pending_ipv4_icmp_echo_arp_request and retry budget remains.
Successful ARP retry transmit decrements the stored budget and keeps the
pending request stored for later ARP resolution. Budget exhaustion, output
pressure, and transmit errors preserve pending state.

## Rejected Claims

- No autonomous retry timing, multi-entry packet queue, packet scheduler,
  dynamic route table, DHCP, DNS, live driver adapter, live packet I/O, packet
  capture, ping behavior, network reachability, sockets, SSH, UDP/TCP, smoltcp
  adoption, RP1 driver readiness, Pi 5 hardware proof, boot publication, lab
  mutation, or phase transition is accepted.
- No RP1/BCM54213PE hardware/link readiness policy changes are accepted.
- No live NetworkDevice implementation is accepted from fake/mock tests.

## Evidence

- src/network.rs:
  PendingIcmpEchoResult::ArpRetryBudgetExhausted,
  PendingIcmpEchoRequest::new_with_next_hop_and_arp_retry_budget,
  PendingIcmpEchoRequest::arp_retries_remaining,
  transmit_or_queue_routed_single_pending_ipv4_icmp_echo_request_with_arp_retry_budget,
  and retry_single_pending_ipv4_icmp_echo_arp_request.
- src/network.rs tests:
  single_pending_arp_retry_reemits_stored_gateway_next_hop_and_decrements_budget,
  single_pending_arp_retry_reports_budget_exhaustion_without_clearing_pending,
  and
  single_pending_arp_retry_reports_no_pending_buffer_pressure_and_transmit_error_boundaries.

## Next Action

selected_next_task=phase12-network-single-pending-arp-retry-closeout-20260619.
Promote that closeout on a later worker wake if dependencies remain satisfied
and git status is clean. Do not promote packet queues, live driver transmit,
hardware, sockets, SSH, smoltcp adoption, ping/network reachability, lab
mutation, boot publication, or phase transition directly from this
implementation.
