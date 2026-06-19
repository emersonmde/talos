# Phase 12.3 Single-Pending ICMP After ARP Resolution Closeout

Task id: phase12-network-single-pending-icmp-after-arp-resolution-closeout-20260619
Status: accepted
Owner: worker
Started: 2026-06-19T14:58:00Z
Accepted: 2026-06-19T15:04:00Z

## Goal

Close out the host-only single-pending ICMP-after-ARP resolution slice and
record the accepted boundary before any route-policy, queue/retry, live packet
I/O, socket, SSH, hardware, or phase-transition work starts.

## Scope

- Review the accepted core source, tests, docs, task record, and evidence.
- Record exactly what single-pending behavior is accepted.
- Keep queue/retry timers, multi-entry buffering, routing implementation, live
  driver adapters, smoltcp, sockets, SSH, ping/network reachability, hardware
  packet I/O, and phase transition rejected.
- Select the next bounded task only if the evidence makes it mechanical.

## Non-Goals

- No implementation work in src/network.rs.
- No routing implementation, packet retry timers, multi-entry queues, live
  driver adapter, hardware run, lab mutation, boot publication, smoltcp,
  sockets, SSH, ping/network reachability claim, or phase transition.

## Review

Reviewed:

- src/network.rs SinglePendingIcmpEcho, PendingIcmpEchoRequest,
  PendingIcmpEchoResult,
  transmit_or_queue_single_pending_ipv4_icmp_echo_request,
  transmit_single_pending_ipv4_icmp_echo_request, and
  learn_arp_reply_and_transmit_single_pending_ipv4_icmp_echo_request.
- src/network.rs single_pending_icmp_* unit tests.
- tasks/2026-06-19-phase12-network-single-pending-icmp-after-arp-resolution-core.md.
- tasks/evidence/2026-06-19-phase12-network-single-pending-icmp-after-arp-resolution-core/classification.json.
- docs/src/project/phase12-networking-ssh.md.
- docs/src/roadmap.md.

## Findings

- fixed: The closeout reconciles the accepted host/testable single-pending
  ARP-to-ICMP progression with source, unit-test, docs, and task evidence.
- fixed: The core classification JSON had unescaped shell quotes in two command
  strings even though the core record claimed jq validation. The evidence file
  is corrected and covered by this closeout's JSON validation gate.
- fixed: Docs now record the closeout boundary and selected next route-policy
  task without accepting route implementation in this closeout.
- deferred: packet queues, retry timers, multi-entry buffering, live driver
  adapters, smoltcp, sockets, SSH, ping/network reachability, hardware packet
  I/O, and phase transition remain future tasks.
- removed: no source APIs, tests, docs, or dependencies were removed.
- not-an-issue: selecting route-policy core as the next task does not implement
  routing here; it only records the next queued same-slice host-only task whose
  dependency is this accepted closeout.

## Accepted Boundary

The accepted boundary remains host/testable and allocation-free. One unresolved
outbound IPv4 ICMP echo request can emit one deterministic Ethernet/IPv4 ARP
request through fake/trait-level NetworkDevice transmit, retain endpoint,
destination IPv4, identifier, sequence number, TTL, and payload bytes in fixed
storage, and later transmit the exact Ethernet/IPv4/ICMP echo request after a
matching ARP reply or accepted ARP cache resolution. Pending state clears only
after successful ICMP transmit.

## Rejected Claims

- No packet queue, retry timer, multi-entry neighbor-discovery queue,
  routing/subnet/gateway implementation, asynchronous scheduling, live driver
  transmit, live packet I/O, packet capture, ping behavior, network
  reachability, sockets, SSH, UDP/TCP, DHCP, DNS, smoltcp adoption, RP1 driver
  adapter readiness, DMA descriptor ownership, interrupt handling, RP1 Ethernet
  readiness, Pi 5 hardware proof, boot publication, lab mutation, or phase
  transition is accepted.
- No route-policy behavior is accepted by this closeout; route policy remains
  the next queued implementation task.

## Validation

- static/source/task evidence review:
  src/network.rs, core task record, core evidence JSON, Phase 12 docs, roadmap,
  and git history reviewed.
  - result: pass.
- JSON evidence validation:
  jq empty tasks/evidence/2026-06-19-phase12-network-single-pending-icmp-after-arp-resolution-core/classification.json tasks/evidence/2026-06-19-phase12-network-single-pending-icmp-after-arp-resolution-closeout/classification.json
  - result: pass.
- diff whitespace check: git diff --check
  - result: pass.
- docs build: /home/node/.cargo/bin/mdbook build
  - result: pass.
- staged diff whitespace check: git diff --cached --check
  - result: pass before commit.

## Evidence

- src/network.rs single-pending implementation and single_pending_icmp_* tests.
- tasks/2026-06-19-phase12-network-single-pending-icmp-after-arp-resolution-core.md.
- tasks/evidence/2026-06-19-phase12-network-single-pending-icmp-after-arp-resolution-core/classification.json.
- tasks/evidence/2026-06-19-phase12-network-single-pending-icmp-after-arp-resolution-closeout/classification.json.

## Next Action

selected_next_task=phase12-network-local-ipv4-egress-route-policy-core-20260619.
Promote that task on a later worker wake if dependencies remain satisfied and
git status is clean. Do not promote queues, retries, live driver transmit,
hardware, sockets, SSH, smoltcp adoption, ping/network reachability, or any
phase transition directly from this closeout.
