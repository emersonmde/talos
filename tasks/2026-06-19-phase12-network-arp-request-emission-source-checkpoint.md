# Phase 12.3 ARP Request Emission Source Checkpoint

Task id: phase12-network-arp-request-emission-source-checkpoint-20260619

Status: accepted

Classification:
phase12-network-arp-request-emission-source-checkpoint-accepted-planning-needed

Evidence level: static source/task evidence review, task-owned JSON
classification, and diff checks. No source implementation, dependency change,
Pi 5 hardware run, boot archive publication, lab mutation,
hardwareTestLock acquisition, live packet I/O, driver transmit, socket/SSH
work, ping/network reachability behavior, smoltcp adoption, or phase
transition was performed.

## Goal

Checkpoint the accepted host-only outbound request-construction frontier
against remaining unresolved-neighbor behavior, and decide whether the next
ARP request or neighbor-discovery implementation boundary is objective.

## Reviewed Evidence

- Local packet dispatch and reply construction:
  tasks/2026-06-19-phase12-network-local-packet-dispatch-icmp-echo-core.md.
- Packet-buffer polling:
  tasks/2026-06-19-phase12-network-packet-buffer-device-polling-core.md.
- ARP cache and cache-aware dispatch/poll:
  tasks/2026-06-19-phase12-network-arp-cache-core.md and
  tasks/2026-06-19-phase12-network-arp-cache-dispatch-integration-core.md.
- Cached outbound neighbor resolver:
  tasks/2026-06-19-phase12-network-outbound-neighbor-resolution-core.md.
- Outbound Ethernet frame construction:
  tasks/2026-06-19-phase12-network-outbound-frame-construction-core.md.
- Outbound IPv4 ICMP echo request construction:
  tasks/2026-06-19-phase12-network-outbound-ipv4-icmp-echo-request-core.md.
- Outbound IPv4 ICMP echo request closeout:
  tasks/2026-06-19-phase12-network-outbound-ipv4-icmp-echo-request-closeout.md.
- Source implementation surface: src/network.rs.
- Phase 12 docs and roadmap: docs/src/project/phase12-networking-ssh.md and
  docs/src/roadmap.md.

## Accepted Source Frontier

The accepted Phase 12.3 host-only source frontier can parse Ethernet frames,
generate caller-buffered ARP replies and ICMP echo replies for local inbound
traffic, poll a NetworkDevice through caller-owned receive/transmit buffers,
learn validated ARP sender facts into a fixed-capacity ArpCache, resolve a
destination IPv4 to either a cached MacAddress or a deterministic unresolved
result, build caller-buffered Ethernet II frames for resolved neighbors, and
build caller-buffered Ethernet/IPv4/ICMP echo request frames for already
resolved neighbors.

The frontier still stops at deterministic unresolved-neighbor rejection.
Existing ARP construction is reply-only through local inbound dispatch. No
accepted source path builds an Ethernet/IPv4 ARP request frame for a local
endpoint to ask for an unresolved destination MAC, and no accepted path queues
that request, schedules retries, consults a driver, or transmits it.

## Recommendation

The next smallest useful implementation boundary is:

recommended_task_id:
phase12-network-arp-request-emission-core-20260619

Recommended scope:

- Add a pure no_std helper that takes a LocalNetworkEndpoint, target IPv4
  address, and caller-owned output buffer.
- Write exactly one Ethernet II ARP request frame into caller-owned storage:
  broadcast destination MAC, local source MAC, EtherType ARP, hardware type
  Ethernet, protocol type IPv4, hardware length 6, protocol length 4,
  operation request, sender MAC/IP from the local endpoint, zero target MAC,
  and target protocol address from the caller.
- Return a deterministic frame length on success.
- Return a deterministic output-buffer-too-small error without accepting a
  partial frame as progress.
- Keep the helper allocation-free, driver-independent, queue-free, and
  compatible with the accepted unresolved-neighbor result from
  resolve_outbound_neighbor.
- Cover host/no_std tests for ARP request fields, broadcast and zero target
  MACs, target IPv4 preservation, exact frame length, small-output rejection,
  and composition with unresolved cached-neighbor resolution.

This boundary reduces the real unresolved-neighbor blocker while staying below
neighbor-discovery state machines, retry timers, packet queues, routing,
driver transmit scheduling, live packet I/O, sockets, SSH, smoltcp, ping
behavior, Pi 5 packet movement evidence, hardware readiness, or phase
transition.

## Planning Decision

planningNeeded: true

planningReason:
The checkpoint recommends a concrete bounded implementation task, but no
explicit queued task exists with acceptanceCriteria, validationGates,
docsRequired, evidenceRequired, scope, and nonGoals. Per worker rules, the
worker must not create or promote that implementation task from this wake.
Supervisor planning is required to add it or select a smaller same-slice
boundary.

selected_next_task: null

## Findings

- fixed: the checkpoint reconciles the accepted outbound request-construction
  frontier against the remaining unresolved-neighbor blocker.
- fixed: the next smallest objective ARP boundary is caller-buffered Ethernet
  IPv4 ARP request frame construction for a local endpoint and target IPv4.
- fixed: the recommended boundary preserves the host-only, allocation-free,
  no-hardware/no-live-I/O strategy and composes with accepted unresolved
  outbound neighbor resolution.
- deferred: ARP request implementation, neighbor-discovery state, retry
  timers, packet queues, routing/subnet/gateway selection, driver transmit
  scheduling, live packet I/O, sockets, SSH, smoltcp integration, ping
  behavior, and Pi 5 packet movement evidence remain future work.
- removed: no source files, docs, dependencies, or prior task evidence were
  removed.
- not-an-issue: no cargo metadata or dependency feasibility check was required
  because no dependency or interface change was proposed or made.
- not-an-issue: no hardware lock, lab mutation, boot publication, or Pi 5
  inconclusive-run triage was required because this is static source/task
  evidence work only.

## Rejected Claims

- No ARP request implementation was accepted.
- No neighbor-discovery state machine, ARP retry timer, packet queue, routing
  table, subnet/gateway logic, driver consultation, driver transmit
  scheduling, packet capture, or live packet I/O was accepted.
- No socket API, SSH behavior, UDP/TCP, DHCP, DNS, smoltcp adoption, RP1
  Ethernet driver readiness, hardware link readiness, network reachability,
  ping behavior, Pi 5 hardware proof, boot archive publication, lab mutation,
  or phase transition was accepted.

## Acceptance Check

- Checkpoint reconciles the accepted outbound request-construction frontier
  against remaining unresolved-neighbor behavior: satisfied by the source
  frontier and recommendation above.
- If a next implementation is recommended, it has an exact task id and a
  bounded ARP-request/neighbor-discovery scope that does not require live
  packet I/O: satisfied as a recommendation only; supervisor planning is
  required before any implementation task can be promoted.
- If no objective ARP request emission boundary is available, planningNeeded
  is set with a precise reason: not applicable; an objective boundary is
  recommended, and planningNeeded is set because the explicit implementation
  task is not queued.
- No hardware, driver transmit, live packet I/O, sockets, SSH, or reachability
  claim is accepted: satisfied.

## Validation

- static/source/task evidence review: pass.
- jq empty on task-owned JSON evidence: pass.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: not run; docs/src files were not changed
  by this checkpoint.
- git diff --cached --check: pass before commit.

## Next Action

Supervisor planning is required to add
phase12-network-arp-request-emission-core-20260619 or select a smaller
same-slice source boundary. Do not promote ARP request implementation, packet
queues, driver transmit, live packet I/O, sockets, SSH, network reachability,
ping behavior, Pi 5 hardware work, boot publication, lab mutation, smoltcp
adoption, link-readiness work, or any phase transition directly from this
checkpoint.
