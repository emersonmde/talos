# Phase 12.3 Route-Aware Outbound Frontier Closeout

Task id: phase12-network-phase12-3-route-aware-outbound-frontier-closeout-20260619
Status: accepted
Owner: worker
Started: 2026-06-19T16:59:30Z
Accepted: 2026-06-19T17:05:00Z

## Goal

Close out the host-only route-aware outbound Phase 12.3 frontier after accepted
route-policy, route-aware single-pending ICMP, and explicit ARP retry slices.

## Scope

- Review accepted source, tests, docs, task records, evidence, and git commits
  for the route-aware outbound frontier.
- Record the accepted frontier and any precise blockers.
- Decide whether a mechanically unblocked bounded host-only continuation is
  already queued.

## Non-Goals

- No implementation work in src/network.rs.
- No packet queue, autonomous retry timer, live driver adapter, hardware run,
  lab mutation, boot publication, smoltcp adoption, sockets, SSH, ping/network
  reachability claim, or phase transition.

## Review

Reviewed:

- src/network.rs route_ipv4_egress, select_routed_outbound_ipv4_icmp_echo_request,
  PendingIcmpEchoRequest, SinglePendingIcmpEcho,
  transmit_or_queue_routed_single_pending_ipv4_icmp_echo_request,
  transmit_or_queue_routed_single_pending_ipv4_icmp_echo_request_with_arp_retry_budget,
  retry_single_pending_ipv4_icmp_echo_arp_request,
  transmit_single_pending_ipv4_icmp_echo_request, and
  learn_arp_reply_and_transmit_single_pending_ipv4_icmp_echo_request.
- src/network.rs local route-policy, routed_single_pending_icmp_*, and
  single_pending_arp_retry_* tests.
- tasks/2026-06-19-phase12-network-local-ipv4-egress-route-policy-core.md.
- tasks/2026-06-19-phase12-network-local-ipv4-egress-route-policy-closeout.md.
- tasks/2026-06-19-phase12-network-routed-single-pending-icmp-after-arp-resolution-core.md.
- tasks/2026-06-19-phase12-network-routed-single-pending-icmp-after-arp-resolution-closeout.md.
- tasks/2026-06-19-phase12-network-single-pending-arp-retry-core.md.
- tasks/2026-06-19-phase12-network-single-pending-arp-retry-closeout.md.
- docs/src/project/phase12-networking-ssh.md.
- docs/src/roadmap.md.
- Git commits 13f01884e, 47ea5ad7, d9c0bab4, c8f8ec8b,
  a17b86e7, and 447c958b.

## Findings

- fixed: The checkpoint reconciles the accepted route-aware outbound frontier
  with source, unit-test, docs, task, evidence, and commit records.
- fixed: Docs now record the checkpoint boundary without expanding into live
  driver, hardware, socket, SSH, reachability, or phase-transition work.
- deferred: packet queues, autonomous timers, multi-entry neighbor-discovery
  state, live driver adapters, smoltcp, UDP/TCP, sockets, SSH,
  ping/network reachability, hardware packet I/O, Pi 5 hardware work, lab
  mutation, boot publication, and phase transition require supervisor-planned
  follow-up tasks.
- removed: no source APIs, tests, docs, dependencies, task records, or evidence
  were removed.
- not-an-issue: selected_next_task is null because no later queued task exists
  after this checkpoint with complete objective dependencies, acceptance
  criteria, validation gates, docs, and evidence requirements.

## Accepted Boundary

The accepted Phase 12.3 route-aware outbound frontier remains host/testable and
allocation-free. Talos can decide same-subnet versus gateway next hop for a
local IPv4 destination, select caller-buffered outbound ARP or ICMP frame
construction from that route decision, store exactly one unresolved pending
ICMP echo request with separate final destination and ARP next-hop IPv4, and
advance that pending request after matching next-hop ARP resolution.

The frontier also includes explicit caller-driven ARP retry for the stored
single pending ICMP request. Retry re-emits one ARP request for the stored
next-hop IPv4 only when the caller invokes the retry helper and retry budget
remains. Successful fake-device transmit decrements the stored retry budget and
preserves the pending ICMP request for later matching ARP resolution; budget
exhaustion and error paths are deterministic and covered by unit tests.

## Rejected Claims

- No live driver adapter, live packet I/O, packet capture, ping behavior,
  network reachability, sockets, SSH, UDP/TCP, smoltcp adoption, DHCP, DNS,
  dynamic routing, autonomous timer, scheduler wakeup, packet queue,
  multi-entry neighbor-discovery queue, RP1 Ethernet readiness, DMA descriptor
  ownership, interrupt handling, Pi 5 hardware proof, boot publication, lab
  mutation, or phase transition is accepted.
- No live NetworkDevice implementation or hardware readiness claim is accepted
  from fake/mock tests.

## Validation

- static/source/task evidence review:
  src/network.rs route-policy, route-aware pending, explicit retry source and
  tests; Phase 12 docs; roadmap; task records; task-owned evidence; and git
  commits 13f01884e, 47ea5ad7, d9c0bab4, c8f8ec8b, a17b86e7, and 447c958b
  reviewed.
  - result: pass.
- JSON evidence validation:
  jq empty tasks/evidence/2026-06-19-phase12-network-phase12-3-route-aware-outbound-frontier-closeout/classification.json
  - result: pass.
- diff whitespace check: git diff --check
  - result: pass.
- docs build: /home/node/.cargo/bin/mdbook build
  - result: pass.
- staged diff whitespace check: git diff --cached --check
  - result: pass before commit.

## Evidence

- src/network.rs route-policy, route-aware pending, and explicit retry
  implementation/tests.
- tasks/2026-06-19-phase12-network-local-ipv4-egress-route-policy-core.md.
- tasks/2026-06-19-phase12-network-local-ipv4-egress-route-policy-closeout.md.
- tasks/2026-06-19-phase12-network-routed-single-pending-icmp-after-arp-resolution-core.md.
- tasks/2026-06-19-phase12-network-routed-single-pending-icmp-after-arp-resolution-closeout.md.
- tasks/2026-06-19-phase12-network-single-pending-arp-retry-core.md.
- tasks/2026-06-19-phase12-network-single-pending-arp-retry-closeout.md.
- tasks/evidence/2026-06-19-phase12-network-phase12-3-route-aware-outbound-frontier-closeout/classification.json.
- docs/src/project/phase12-networking-ssh.md.
- docs/src/roadmap.md.

## Next Action

selected_next_task=null. Set planningNeeded=true because no later queued task
exists after this checkpoint with complete objective dependencies, acceptance
criteria, validation gates, docs, and evidence requirements.

Supervisor planning is required before packet queues, autonomous retry timers,
neighbor-discovery state expansion, live driver adapters, hardware packet I/O,
sockets, SSH, smoltcp adoption, reachability, lab mutation, boot publication,
or any phase transition.
