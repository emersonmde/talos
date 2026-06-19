# Phase 12.3 Local IPv4 Egress Route Policy Closeout

Task id: phase12-network-local-ipv4-egress-route-policy-closeout-20260619
Status: accepted
Owner: worker
Started: 2026-06-19T15:20:00Z
Accepted: 2026-06-19T15:20:00Z

## Goal

Close out the host-only local IPv4 egress route policy frontier and decide
whether an already queued, mechanically unblocked Phase 12.3 continuation can
be selected.

## Scope

- Review the accepted route-policy source, unit tests, docs, and task evidence.
- Reconcile the accepted route decision behavior with rejected live-networking
  claims.
- Select a bounded host-only Phase 12.3 follow-up only if it already exists
  with complete objective task definition and dependencies.

## Non-Goals

- No implementation work.
- No route-table expansion, retry timing, packet queues, live driver adapter,
  smoltcp adoption, sockets, SSH, live packet I/O, hardware run, lab mutation,
  boot publication, ping/network reachability claim, or phase transition.

## Review

The accepted core commit is 13f018847b4cc8eb2b4489f04ca9a47a34801241. Static
review of src/network.rs found that route_ipv4_egress deterministically chooses
the destination IPv4 as next hop for same-subnet destinations, chooses the
configured gateway IPv4 for off-subnet destinations, and returns
NoRouteToDestination when no gateway is configured. The routed outbound selector
uses that next-hop decision for ARP cache lookup/request emission while keeping
the final IPv4 packet destination unchanged for gateway routes.

The retained unit tests cover same-subnet, gateway, missing-gateway/no-route,
zero-mask and host-mask boundaries, immutable ARP-cache gateway selection,
unresolved-gateway ARP selection, no-route output preservation, and frame-error
wrapping without cache mutation.

## Findings

- fixed: The accepted core provided deterministic same-subnet, gateway, and
  no-route route decisions and routed request selection.
- fixed: Docs and roadmap already record the accepted host-only route-policy
  boundary and rejected live-networking claims.
- deferred: retry timing, packet queues, multi-entry neighbor discovery,
  dynamic routing, DHCP, DNS, live driver adapters, live packet I/O, smoltcp,
  sockets, SSH, ping/network reachability, hardware work, boot publication, lab
  mutation, and phase transition require explicit future tasks.
- removed: no source APIs, tests, task records, docs, or evidence were removed.
- not-an-issue: No later queued Phase 12.3 continuation currently has a
  complete task definition. The worker therefore cannot choose retry timing,
  a small packet queue, or another host-only slice without supervisor planning.

## Accepted Boundary

The Phase 12.3 host-only frontier now includes local IPv4 egress route policy:
same-subnet destinations ARP for the destination, off-subnet destinations ARP
for the configured gateway, and off-subnet destinations without a gateway fail
deterministically before touching the output buffer.

This remains local source/test behavior only. It does not accept live
networking, live driver transmit, hardware packet I/O, route tables, dynamic
routing, DHCP, DNS, retry queues, smoltcp, sockets, SSH, ping/network
reachability, lab mutation, boot publication, or phase transition.

## Validation

- static/source/task evidence review:
  - src/network.rs route-policy API, routed selector, and route-policy tests
    inspected.
  - tasks/2026-06-19-phase12-network-local-ipv4-egress-route-policy-core.md
    inspected.
  - docs/src/project/phase12-networking-ssh.md and docs/src/roadmap.md
    inspected for accepted frontier wording.
- diff whitespace check: git diff --check
  - result: pass.
- docs build: /home/node/.cargo/bin/mdbook build
  - result: pass.
- staged diff whitespace check: git diff --cached --check
  - result: pass before commit.

## Evidence

- Core task record:
  tasks/2026-06-19-phase12-network-local-ipv4-egress-route-policy-core.md.
- Source evidence:
  src/network.rs Ipv4EgressRoutePolicy, Ipv4EgressRouteDecision,
  route_ipv4_egress, build_outbound_routed_ipv4_icmp_echo_request, and
  select_routed_outbound_ipv4_icmp_echo_request.
- Test evidence:
  src/network.rs route-policy and routed outbound selection unit tests listed
  in the accepted core task record.

## Next Action

selected_next_task=null. Set planningNeeded=true because no later queued
Phase 12.3 task has complete scope, non-goals, dependencies, acceptance
criteria, validation gates, docs, and evidence requirements. The supervisor
must plan the next bounded host-only continuation before the worker promotes
retry timing, packet queues, route-table expansion, live driver integration,
hardware work, sockets, SSH, smoltcp adoption, ping/network reachability, or
any phase transition.
