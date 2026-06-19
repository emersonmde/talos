# Phase 12.3 Outbound Neighbor Resolution Source Checkpoint

Task id: phase12-network-outbound-neighbor-resolution-source-checkpoint-20260619

Status: accepted

Classification:
phase12-network-outbound-neighbor-resolution-source-checkpoint-planning-needed

Evidence level: static source/task evidence review, task-owned JSON
classification, and diff checks. No Pi 5 hardware run, boot archive
publication, lab mutation, hardwareTestLock acquisition, live packet I/O,
sockets, SSH, smoltcp adoption, hardware-driver readiness, link readiness,
network reachability, ping behavior, or phase transition was performed.

## Goal

Checkpoint the next host-only source boundary for outbound IPv4 neighbor
resolution using the accepted ArpCache, without implementing packet queues,
sockets, live packet I/O, or hardware behavior.

## Reviewed Evidence

- ARP-cache core task:
  tasks/2026-06-19-phase12-network-arp-cache-core.md.
- ARP-cache dispatch integration source checkpoint:
  tasks/2026-06-19-phase12-network-arp-cache-dispatch-integration-source-checkpoint.md.
- ARP-cache dispatch integration core:
  tasks/2026-06-19-phase12-network-arp-cache-dispatch-integration-core.md.
- ARP-cache dispatch integration core classification:
  tasks/evidence/2026-06-19-phase12-network-arp-cache-dispatch-integration-core/classification.json.
- ARP-cache dispatch integration closeout:
  tasks/2026-06-19-phase12-network-arp-cache-dispatch-integration-closeout.md.
- ARP-cache dispatch integration closeout classification:
  tasks/evidence/2026-06-19-phase12-network-arp-cache-dispatch-integration-closeout/classification.json.
- Source implementation and host tests: src/network.rs.
- Phase 12 project doc: docs/src/project/phase12-networking-ssh.md.
- Roadmap Milestone 12.3 text: docs/src/roadmap.md.
- Supervisor state taskQueue: no explicit queued outbound neighbor-resolution
  implementation task follows this checkpoint.

## Accepted Input Frontier

- ArpCache stores fixed-capacity, allocation-free IPv4-to-MAC neighbors with
  deterministic lookup, insert, update, zero-capacity no-state-change behavior,
  and oldest-slot replacement.
- Valid Ethernet/IPv4 ARP request and reply packets can teach sender
  IPv4-to-MAC facts through ArpCache.
- dispatch_local_packet_with_arp_cache learns inbound ARP sender facts before
  preserving the existing cache-unaware local reply behavior.
- poll_local_network_device_with_arp_cache receives one caller-buffered frame,
  learns inbound ARP sender facts through the cache-aware dispatcher, and
  transmits only caller-buffered local replies.
- No accepted source path currently resolves an outbound destination IPv4
  address into an Ethernet destination MAC for future transmit preparation.

## Next Smallest Boundary

The next feature slice should be a cached-only outbound neighbor resolver. The
thinnest useful implementation is a pure source/test API that accepts a
destination IPv4 address and an immutable ArpCache reference, then returns
either the cached destination MAC or a deterministic unresolved result.

Recommended shape for supervisor planning:

- add a small outbound-neighbor result type, for example resolved cached MAC vs
  unresolved destination IPv4;
- add a cache lookup helper for direct local IPv4 neighbor resolution using the
  accepted ArpCache lookup semantics;
- keep the API allocation-free and no_std-compatible;
- do not emit ARP requests, allocate or enqueue packets, retry, schedule
  timers, route between subnets, consult a driver, or transmit frames;
- preserve existing dispatch_local_packet, dispatch_local_packet_with_arp_cache,
  poll_local_network_device, and poll_local_network_device_with_arp_cache
  behavior for all current callers;
- cover cached hit, unresolved miss, updated cache entry, zero-capacity cache,
  and compatibility with the accepted cache-aware dispatch/poll tests.

This boundary would give later packet-construction work a deterministic
neighbor lookup primitive before ARP request emission, packet queues, driver
adapters, UDP/TCP, DHCP, DNS, routing, socket APIs, smoltcp adoption, SSH,
live packet I/O, or hardware network claims.

## Planning Decision

selected_next_task: null

planningNeeded: true

Recommended task id for supervisor planning:
phase12-network-outbound-neighbor-resolution-core-20260619

Rationale: the next outbound neighbor-resolution feature boundary is clear and
feature-led, but no explicit queued implementation task currently exists after
this checkpoint. The worker must not create a new task, broaden scope, infer a
phase transition, or promote packet queues, driver work, socket work,
smoltcp, live packet I/O, or hardware work. Supervisor planning is required to
add a concrete dependency-gated implementation task with acceptance criteria,
validation gates, docs requirements, evidence requirements, scope, and
non-goals.

## Findings

- fixed: the checkpoint records the accepted ARP-cache, cache-aware dispatch,
  and cache-aware poll frontiers.
- fixed: the checkpoint names the next smallest outbound neighbor-resolution
  boundary as a cached-only IPv4-to-MAC resolver with deterministic resolved
  and unresolved outcomes.
- deferred: implementation of the cached-only resolver, ARP request emission,
  packet queues, driver adapter integration, UDP/TCP, DHCP, DNS, routing,
  sockets, smoltcp, SSH, live packet I/O, and Pi 5 packet movement evidence
  remain future work.
- removed: no source, docs, dependencies, or prior task evidence were removed.
- not-an-issue: no cargo metadata or dependency feasibility check was required
  because no dependency or interface change was proposed or made.
- not-an-issue: no hardware lock, lab mutation, boot publication, or Pi 5
  inconclusive-run triage was required because this is static source/task
  evidence work only.

## Rejected Claims

- No outbound neighbor-resolution implementation was accepted.
- No ARP request emission, retry timer, packet queue, driver adapter, RP1
  Ethernet driver readiness, DMA descriptor ownership, interrupt integration,
  packet capture, or live packet I/O was accepted.
- No smoltcp dependency or third-party network stack was adopted.
- No UDP/TCP behavior, DHCP, DNS, routing, socket API, SSH behavior, network
  reachability, ping response, hardware link readiness, or phase transition was
  accepted.

## Acceptance Check

- Checkpoint evidence names the next smallest outbound neighbor-resolution
  boundary, or records why supervisor planning is required: satisfied.
- No live packet I/O, driver readiness, smoltcp adoption, sockets, SSH, ping
  behavior, network reachability, or phase transition is claimed: satisfied.
- If a follow-up implementation is selected, it is concrete, bounded,
  dependency-gated, and preserves the no-hardware/no-live-I/O strategy
  boundary: no follow-up task is selected; planningNeeded is set because no
  explicit queued outbound neighbor-resolution implementation task exists.

## Validation

- static/source/task evidence review: pass.
- cargo metadata or equivalent: not run; no dependency or interface changes
  were proposed or made.
- jq empty on task-owned JSON evidence: pass.
- git diff --check: pass.
- mdbook build: not run; docs/src files were not changed by this checkpoint.
- git diff --cached --check: pass before commit.

## Next Action

Supervisor planning is required to add a bounded cached-only outbound
neighbor-resolution implementation task if this recommendation is accepted. Do
not promote packet queues, hardware-driver work, live packet I/O, smoltcp
adoption, sockets, SSH, RP1 Ethernet readiness, network reachability, ping
behavior, link-readiness work, or any Pi 5 hardware task directly from this
checkpoint.
