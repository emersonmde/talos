# Phase 12.3 ARP Cache Source Checkpoint

Task id: phase12-network-arp-cache-source-checkpoint-20260619

Status: accepted

Classification:
phase12-network-arp-cache-source-checkpoint-planning-needed

Evidence level: static source/task evidence review, task-owned JSON
classification, and diff checks. No Pi 5 hardware run, boot archive
publication, lab mutation, hardwareTestLock acquisition, live packet I/O,
sockets, SSH, smoltcp adoption, hardware-driver readiness, link readiness,
network reachability, or phase transition was performed.

## Goal

Checkpoint the next smallest ARP-cache source/test boundary after accepted
local packet dispatch and packet-buffer/device polling.

## Reviewed Evidence

- Local packet-dispatch core:
  tasks/2026-06-19-phase12-network-local-packet-dispatch-icmp-echo-core.md.
- Local packet-dispatch closeout:
  tasks/2026-06-19-phase12-network-local-packet-dispatch-icmp-echo-closeout.md.
- Packet-buffer/device-polling core:
  tasks/2026-06-19-phase12-network-packet-buffer-device-polling-core.md.
- Packet-buffer/device-polling closeout:
  tasks/2026-06-19-phase12-network-packet-buffer-device-polling-closeout.md.
- Poll-step closeout classification:
  tasks/evidence/2026-06-19-phase12-network-packet-buffer-device-polling-closeout/classification.json.
- Source boundary and host tests: src/network.rs.
- Phase 12 project doc: docs/src/project/phase12-networking-ssh.md.
- Roadmap Milestone 12.3 text: docs/src/roadmap.md.
- Supervisor state taskQueue: no explicit queued ARP-cache implementation task
  follows this checkpoint.

## Accepted Input Frontier

- dispatch_local_packet can parse Ethernet II ARP and IPv4 frames and generate
  caller-buffered ARP replies or ICMP echo replies for the configured
  LocalNetworkEndpoint.
- ARP behavior is currently stateless: an ARP request targeting the local IPv4
  address can produce a reply, but no sender hardware/protocol mapping is
  retained after dispatch.
- poll_local_network_device receives one frame into caller-owned RX storage,
  routes it through dispatch_local_packet, and transmits from caller-owned TX
  storage only when a reply is produced.
- LocalPollStepResult preserves deterministic boundaries for no frame, receive
  buffer pressure, receive error, no reply, dispatch error, transmit error, and
  successful reply transmission.
- No driver adapter, packet queue, live packet I/O, ARP cache, UDP/TCP, socket,
  SSH, smoltcp, hardware link, or network reachability behavior is accepted by
  the prior tasks.

## Next Smallest Boundary

The next feature slice should be a local, host-testable ARP neighbor table that
records IPv4-to-MAC mappings without allocation and without sending live
packets.

Recommended shape for supervisor planning:

- add a fixed-capacity, caller-owned ArpCache or ArpTable type that stores a
  small number of IPv4-to-MAC entries without heap allocation;
- provide deterministic insert_or_update and lookup operations keyed by IPv4
  address;
- define replacement behavior when the table is full, preferably explicit
  oldest-entry or stable slot replacement rather than hidden allocation;
- expose a small learning helper that can learn sender IPv4/MAC from validated
  Ethernet/ARP packets without changing packet transmission semantics;
- keep dispatch_local_packet and poll_local_network_device behavior unchanged
  unless a later implementation task explicitly wires the cache into reply
  decisions;
- cover insertion, update, lookup miss, full-table replacement, malformed ARP
  rejection, and learning-from-valid-ARP behavior with host unit tests.

This boundary makes neighbor state concrete before packet queues, driver
adapters, UDP/TCP, DHCP, DNS, routing, socket APIs, smoltcp adoption, SSH,
live packet I/O, or hardware-network claims.

## Planning Decision

selected_next_task: null

planningNeeded: true

Rationale: the next ARP-cache boundary is clear, but no explicit queued
implementation task currently exists after this checkpoint. The worker must not
create a new task, broaden scope, infer a phase transition, or promote driver,
socket, smoltcp, live packet I/O, or hardware work. Supervisor planning is
required to add a concrete dependency-gated implementation task with acceptance
criteria, validation gates, docs requirements, evidence requirements, scope,
and non-goals.

## Findings

- fixed: the checkpoint records the accepted local ARP/ICMP dispatch frontier
  and the accepted one-frame poll-step frontier.
- fixed: the checkpoint names the next smallest ARP-cache source/test boundary
  as a fixed-capacity, allocation-free neighbor table with deterministic
  insertion, lookup, replacement, and ARP-learning behavior.
- deferred: ARP-cache implementation, dispatch integration, packet queues,
  driver adapter integration, UDP/TCP, DHCP, DNS, routing, sockets, smoltcp,
  SSH, live packet I/O, and Pi 5 packet movement evidence remain future work.
- removed: no source, docs, dependencies, or prior task evidence were removed.
- not-an-issue: no cargo metadata or dependency feasibility check was required
  because no dependency or interface change was proposed or made.
- not-an-issue: no hardware lock, lab mutation, boot publication, or Pi 5
  inconclusive-run triage was required because this is static source/task
  evidence work only.

## Rejected Claims

- No ARP-cache implementation was accepted.
- No dispatch integration with cached neighbors was accepted.
- No packet queue, driver adapter, RP1 Ethernet driver readiness, DMA
  descriptor ownership, interrupt integration, packet capture, or live packet
  I/O was accepted.
- No smoltcp dependency or third-party network stack was adopted.
- No UDP/TCP behavior, DHCP, DNS, routing, socket API, SSH behavior, network
  reachability, ping response, hardware link readiness, or phase transition was
  accepted.

## Acceptance Check

- Checkpoint evidence names the next smallest ARP-cache source/test boundary,
  or records why supervisor planning is required: satisfied.
- No live packet I/O, driver readiness, smoltcp adoption, sockets, SSH, network
  reachability, or phase transition is claimed: satisfied.
- If a follow-up implementation is selected, it is concrete, bounded,
  dependency-gated, and preserves the no-hardware/no-live-I/O strategy boundary:
  no follow-up task is selected; planningNeeded is set because no explicit
  queued ARP-cache implementation task exists.

## Validation

- static/source/task evidence review: pass.
- cargo metadata or equivalent: not run; no dependency or interface changes
  were proposed or made.
- jq empty on task-owned JSON evidence: pass.
- git diff --check: pass.
- mdbook build: not run; docs/src files were not changed by this checkpoint.
- git diff --cached --check: pass before commit.

## Next Action

Supervisor planning is required to add a bounded ARP-cache implementation task
if this recommendation is accepted. Do not promote hardware-driver work, live
packet I/O, smoltcp adoption, sockets, SSH, RP1 Ethernet readiness, network
reachability, or any phase transition from this checkpoint.
