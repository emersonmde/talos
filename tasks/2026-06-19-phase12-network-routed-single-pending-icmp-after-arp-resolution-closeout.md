# Phase 12.3 Routed Single-Pending ICMP After ARP Resolution Closeout

Task id: phase12-network-routed-single-pending-icmp-after-arp-resolution-closeout-20260619
Status: accepted
Owner: worker
Started: 2026-06-19T16:19:00Z
Accepted: 2026-06-19T16:19:00Z

## Goal

Close out the host-only route-aware single-pending ICMP-after-ARP slice and
record the accepted boundary before any retry, queue, live packet I/O, socket,
SSH, hardware, or phase-transition work starts.

## Scope

- Review the accepted routed single-pending ICMP source, tests, docs, task
  record, evidence, and git commit.
- Record exactly what route-aware pending behavior is accepted.
- Keep retry timing, packet queues, multi-entry buffering, live driver
  adapters, smoltcp, sockets, SSH, ping/network reachability, hardware packet
  I/O, and phase transition rejected.
- Select the next bounded task only if the evidence makes it mechanical.

## Non-Goals

- No implementation work in src/network.rs.
- No retry timer, packet queue, autonomous scheduling, live driver adapter,
  hardware run, lab mutation, boot publication, smoltcp adoption, sockets, SSH,
  ping/network reachability claim, or phase transition.

## Review

Reviewed:

- src/network.rs PendingIcmpEchoRequest, PendingIcmpEchoResult,
  SinglePendingIcmpEcho,
  transmit_or_queue_routed_single_pending_ipv4_icmp_echo_request,
  transmit_single_pending_ipv4_icmp_echo_request, and
  learn_arp_reply_and_transmit_single_pending_ipv4_icmp_echo_request.
- src/network.rs routed_single_pending_icmp_* tests and retained
  single_pending_icmp_* regression tests.
- tasks/2026-06-19-phase12-network-routed-single-pending-icmp-after-arp-resolution-core.md.
- tasks/evidence/2026-06-19-phase12-network-routed-single-pending-icmp-after-arp-resolution-core/classification.json.
- docs/src/project/phase12-networking-ssh.md.
- docs/src/roadmap.md.
- Git commit d9c0bab44a682e96565fc153a125130db6ddfe92.

## Findings

- fixed: The closeout reconciles the accepted host/testable route-aware
  pending ARP-to-ICMP progression with source, unit-test, docs, task, and
  evidence records.
- fixed: Docs now record the closeout boundary and selected next explicit retry
  task without accepting retry behavior in this closeout.
- deferred: explicit retry state, packet queues, multi-entry neighbor
  discovery, live driver adapters, live packet I/O, smoltcp, sockets, SSH,
  ping/network reachability, hardware packet I/O, and phase transition remain
  future tasks.
- removed: no source APIs, tests, docs, dependencies, task records, or evidence
  were removed.
- not-an-issue: selecting the queued retry core as the next task does not
  implement retry behavior here; it only records the next same-slice host-only
  task whose dependency becomes satisfied by this accepted closeout.

## Accepted Boundary

The accepted boundary remains host/testable and allocation-free. A routed
outbound IPv4 ICMP echo request applies the accepted route_ipv4_egress decision
before pending-state mutation. Same-subnet unresolved destinations emit an ARP
request for the destination and store that destination as the pending ARP next
hop. Gateway-routed unresolved destinations emit ARP for the configured gateway
and store the final IPv4 destination separately from the gateway next hop.

Matching ARP resolution for the stored next hop can transmit the final
destination ICMP echo request through fake/trait-level NetworkDevice transmit
using the resolved next-hop MAC, then clear pending state after successful
transmit. Off-subnet destinations without a gateway return
NoRouteToDestination before output mutation, device transmit, or pending-state
mutation.

## Rejected Claims

- No retry timer, autonomous scheduling, packet queue, multi-entry
  neighbor-discovery queue, route-table expansion, dynamic routing, DHCP, DNS,
  live driver transmit, live packet I/O, packet capture, ping behavior, network
  reachability, sockets, SSH, UDP/TCP, smoltcp adoption, RP1 driver adapter
  readiness, DMA descriptor ownership, interrupt handling, RP1 Ethernet
  readiness, Pi 5 hardware proof, boot publication, lab mutation, or phase
  transition is accepted.
- No retry behavior is accepted by this closeout; explicit retry remains the
  next queued implementation task.

## Validation

- static/source/task evidence review:
  src/network.rs, routed pending tests, core task record, core evidence JSON,
  Phase 12 docs, roadmap, and git history reviewed.
  - result: pass.
- JSON evidence validation:
  jq empty tasks/evidence/2026-06-19-phase12-network-routed-single-pending-icmp-after-arp-resolution-core/classification.json tasks/evidence/2026-06-19-phase12-network-routed-single-pending-icmp-after-arp-resolution-closeout/classification.json
  - result: pass.
- diff whitespace check: git diff --check
  - result: pass.
- docs build: /home/node/.cargo/bin/mdbook build
  - result: pass.
- staged diff whitespace check: git diff --cached --check
  - result: pass before commit.

## Evidence

- src/network.rs routed single-pending implementation and routed_single_pending_icmp_* tests.
- src/network.rs retained direct single_pending_icmp_* regression tests.
- tasks/2026-06-19-phase12-network-routed-single-pending-icmp-after-arp-resolution-core.md.
- tasks/evidence/2026-06-19-phase12-network-routed-single-pending-icmp-after-arp-resolution-core/classification.json.
- tasks/evidence/2026-06-19-phase12-network-routed-single-pending-icmp-after-arp-resolution-closeout/classification.json.

## Next Action

selected_next_task=phase12-network-single-pending-arp-retry-core-20260619.
Promote that task on a later worker wake if dependencies remain satisfied and
git status is clean. Do not promote packet queues, live driver transmit,
hardware, sockets, SSH, smoltcp adoption, ping/network reachability, or any
phase transition directly from this closeout.
