# Phase 12.3 ARP Cache Dispatch Integration Source Checkpoint

Task id: phase12-network-arp-cache-dispatch-integration-source-checkpoint-20260619

Status: accepted

Classification:
phase12-network-arp-cache-dispatch-integration-source-checkpoint-planning-needed

Evidence level: static source/task evidence review, task-owned JSON
classification, and diff checks. No Pi 5 hardware run, boot archive
publication, lab mutation, hardwareTestLock acquisition, live packet I/O,
sockets, SSH, smoltcp adoption, hardware-driver readiness, link readiness,
network reachability, ping behavior, or phase transition was performed.

## Goal

Checkpoint the next smallest ARP-cache integration boundary after the accepted
fixed-capacity ARP neighbor cache, without widening into hardware, live packet
I/O, sockets, SSH, or a phase transition.

## Reviewed Evidence

- ARP-cache source checkpoint:
  tasks/2026-06-19-phase12-network-arp-cache-source-checkpoint.md.
- ARP-cache core task:
  tasks/2026-06-19-phase12-network-arp-cache-core.md.
- ARP-cache core classification:
  tasks/evidence/2026-06-19-phase12-network-arp-cache-core/classification.json.
- ARP-cache closeout:
  tasks/2026-06-19-phase12-network-arp-cache-closeout.md.
- ARP-cache closeout classification:
  tasks/evidence/2026-06-19-phase12-network-arp-cache-closeout/classification.json.
- Source boundary and host tests: src/network.rs.
- Phase 12 project doc: docs/src/project/phase12-networking-ssh.md.
- Roadmap Milestone 12.3 text: docs/src/roadmap.md.
- Supervisor state taskQueue: no explicit queued ARP-cache dispatch-integration
  implementation task follows this checkpoint.

## Accepted Input Frontier

- dispatch_local_packet can generate caller-buffered ARP replies and ICMP echo
  replies for the configured LocalNetworkEndpoint.
- poll_local_network_device can receive one frame into caller-owned RX storage,
  dispatch it, and transmit from caller-owned TX storage only when a reply is
  produced.
- ArpCache records IPv4-to-MAC neighbors with fixed capacity, no heap
  allocation, deterministic lookup, insert, update, zero-capacity
  no-state-change behavior, oldest-slot replacement, and sender learning from
  valid Ethernet/IPv4 ARP requests and replies.
- Existing dispatch and poll-step behavior intentionally remains cache-unaware:
  learned neighbors do not yet affect reply policy, outbound resolution,
  packet queues, driver adapters, live packet I/O, ping behavior, sockets, SSH,
  smoltcp adoption, network reachability, link readiness, or phase transition.

## Next Smallest Boundary

The next feature slice should integrate ARP-cache learning into the local
dispatch or poll-step source path while preserving caller-owned buffers and
host-only validation. The thinnest useful implementation is cache-aware local
packet handling that observes validated inbound ARP packets and records sender
IPv4-to-MAC facts before returning the same reply/no-reply outcomes as the
accepted dispatch path.

Recommended shape for supervisor planning:

- add a cache-aware wrapper or variant around dispatch_local_packet or
  poll_local_network_device that accepts a mutable ArpCache reference;
- learn sender IPv4/MAC facts from valid Ethernet/IPv4 ARP request and reply
  frames using the accepted ArpCache learning rules;
- preserve existing cache-unaware dispatch_local_packet and
  poll_local_network_device behavior for callers that do not pass a cache;
- keep reply generation unchanged unless the implementation task explicitly
  authorizes cache-backed policy changes;
- return deterministic results when learning succeeds, learning rejects input,
  no reply is produced, dispatch rejects input, or transmit fails;
- cover ARP request learning with reply transmission, ARP reply learning with
  no local reply, unsupported/malformed ARP rejection without cache mutation,
  ICMP echo behavior unchanged, no-frame behavior unchanged, and transmit-error
  behavior with the learned or unchanged cache state documented by tests.

This boundary makes neighbor learning part of the local packet path before
outbound neighbor resolution, packet queues, driver adapters, UDP/TCP, DHCP,
DNS, routing, socket APIs, smoltcp adoption, SSH, live packet I/O, or hardware
network claims.

## Planning Decision

selected_next_task: null

planningNeeded: true

Rationale: the next ARP-cache dispatch-integration feature boundary is clear,
but no explicit queued implementation task currently exists after this
checkpoint. The worker must not create a new task, broaden scope, infer a phase
transition, or promote driver, socket, smoltcp, live packet I/O, or hardware
work. Supervisor planning is required to add a concrete dependency-gated
implementation task with acceptance criteria, validation gates, docs
requirements, evidence requirements, scope, and non-goals.

## Findings

- fixed: the checkpoint records the accepted local dispatch, poll-step, and
  ARP-cache frontiers.
- fixed: the checkpoint names the next smallest ARP-cache integration
  source/test boundary as cache-aware local packet handling that learns from
  validated inbound ARP frames while preserving current reply behavior.
- deferred: implementation of cache-aware dispatch/poll, outbound neighbor
  resolution, packet queues, driver adapter integration, UDP/TCP, DHCP, DNS,
  routing, sockets, smoltcp, SSH, live packet I/O, and Pi 5 packet movement
  evidence remain future work.
- removed: no source, docs, dependencies, or prior task evidence were removed.
- not-an-issue: no cargo metadata or dependency feasibility check was required
  because no dependency or interface change was proposed or made.
- not-an-issue: no hardware lock, lab mutation, boot publication, or Pi 5
  inconclusive-run triage was required because this is static source/task
  evidence work only.

## Rejected Claims

- No ARP-cache dispatch integration implementation was accepted.
- No outbound neighbor resolution, packet queue, driver adapter, RP1 Ethernet
  driver readiness, DMA descriptor ownership, interrupt integration, packet
  capture, or live packet I/O was accepted.
- No smoltcp dependency or third-party network stack was adopted.
- No UDP/TCP behavior, DHCP, DNS, routing, socket API, SSH behavior, network
  reachability, ping response, hardware link readiness, or phase transition was
  accepted.

## Acceptance Check

- Checkpoint evidence names the next smallest ARP-cache integration boundary,
  or records why supervisor planning is required: satisfied.
- No live packet I/O, driver readiness, smoltcp adoption, sockets, SSH, ping
  behavior, network reachability, or phase transition is claimed: satisfied.
- If a follow-up implementation is selected, it is concrete, bounded,
  dependency-gated, and preserves the no-hardware/no-live-I/O strategy
  boundary: no follow-up task is selected; planningNeeded is set because no
  explicit queued ARP-cache dispatch-integration implementation task exists.

## Validation

- static/source/task evidence review: pass.
- cargo metadata or equivalent: not run; no dependency or interface changes
  were proposed or made.
- jq empty on task-owned JSON evidence: pass.
- git diff --check: pass.
- mdbook build: not run; docs/src files were not changed by this checkpoint.
- git diff --cached --check: pass before commit.

## Next Action

Supervisor planning is required to add a bounded cache-aware local
dispatch/poll implementation task if this recommendation is accepted. Do not
promote hardware-driver work, live packet I/O, smoltcp adoption, sockets, SSH,
RP1 Ethernet readiness, network reachability, ping behavior, or any phase
transition from this checkpoint.
