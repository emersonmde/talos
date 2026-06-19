# Phase 12.3 Outbound Neighbor Resolution Core

Task id: phase12-network-outbound-neighbor-resolution-core-20260619

Status: accepted

Classification:
phase12-network-outbound-neighbor-resolution-core-accepted

Evidence level: source implementation, no_std unit tests, build check, docs
build, task-owned JSON evidence, and diff checks. No Pi 5 hardware run, boot
archive publication, lab mutation, hardwareTestLock acquisition, live packet
I/O, driver adapter work, sockets, SSH, smoltcp adoption, ping/network
reachability behavior, or phase transition was performed.

## Goal

Implement the smallest cached-only outbound neighbor-resolution source
boundary: resolve a destination IPv4 address through the accepted ArpCache, and
classify misses deterministically without ARP request emission or packet
transmission.

## Scope Performed

- Added OutboundNeighborResolution as an allocation-free result type that
  distinguishes a cached resolved destination MAC from an unresolved
  destination IPv4.
- Added resolve_outbound_neighbor, a pure helper that reads an immutable
  ArpCache and returns either the cached MAC or the unresolved destination.
- Kept dispatch_local_packet, dispatch_local_packet_with_arp_cache,
  poll_local_network_device, and poll_local_network_device_with_arp_cache
  behavior unchanged for current callers.
- Added deterministic no_std tests for cached hit, unresolved miss, updated
  cache entry, zero-capacity cache miss, and compatibility with cache-aware
  poll learning.
- Updated Phase 12 docs and roadmap to record the accepted host-only
  cached-resolution boundary.

## Accepted Behavior

- A known destination IPv4 returns
  OutboundNeighborResolution::Resolved with the destination IPv4 and cached
  MacAddress.
- A cache miss returns OutboundNeighborResolution::Unresolved carrying the
  destination IPv4.
- Updated ArpCache entries are reflected by later resolution calls because the
  helper uses immutable lookup semantics over current cache state.
- Zero-capacity ArpCache instances remain deterministic misses.
- Cache-aware inbound ARP learning remains compatible with the resolver: a
  learned sender can be resolved after the accepted poll path records it.

## Findings

- fixed: Phase 12.3 now has a host-only cached outbound IPv4-to-MAC resolution
  helper backed by accepted ArpCache lookup semantics.
- fixed: deterministic unresolved-neighbor classification carries the target
  IPv4 without emitting ARP requests or consulting any driver.
- fixed: cached hit, miss, updated entry, zero-capacity miss, and compatibility
  with cache-aware poll learning are covered by no_std tests.
- deferred: ARP request emission, retry timers, packet queues, routing,
  gateway/subnet selection, outbound frame construction, driver transmit
  scheduling, live packet I/O, sockets, SSH, smoltcp integration, and Pi 5
  packet movement evidence remain future work.
- removed: no source files, dependencies, prior task evidence, or existing
  cache-unaware/cache-aware dispatch APIs were removed.
- not-an-issue: no hardware lock, lab mutation, boot publication, or Pi 5
  inconclusive-run triage was required because this task is local source/test
  work only.

## Rejected Claims

- No ARP request emission, retry timer, packet queue, routing table, subnet or
  gateway logic, driver consultation, frame construction, transmit scheduling,
  or live packet I/O was accepted.
- No RP1 Ethernet driver readiness, DMA descriptor ownership, interrupt
  integration, packet capture, link readiness, network reachability, ping
  behavior, socket API, SSH behavior, UDP/TCP, DHCP, DNS, smoltcp adoption,
  userspace networking API, Pi 5 hardware proof, or phase transition was
  accepted.

## Evidence

- Source implementation and tests: src/network.rs.
- Source checkpoint:
  tasks/2026-06-19-phase12-network-outbound-neighbor-resolution-source-checkpoint.md.
- Task classification:
  tasks/evidence/2026-06-19-phase12-network-outbound-neighbor-resolution-core/classification.json.
- Phase 12 project doc update: docs/src/project/phase12-networking-ssh.md.
- Roadmap update: docs/src/roadmap.md.

## Acceptance Check

- A source-level outbound-neighbor resolver returns the cached destination MAC
  for a known destination IPv4 and a deterministic unresolved result carrying
  the destination IPv4 for a miss: satisfied by resolve_outbound_neighbor and
  OutboundNeighborResolution.
- Resolver behavior uses immutable ArpCache lookup semantics and remains
  allocation-free/no_std-compatible: satisfied.
- Updated cache entries are reflected by the resolver, and zero-capacity
  caches remain deterministic misses: satisfied by tests.
- Existing local ARP/ICMP dispatch and cache-aware polling tests continue to
  pass without broad behavior changes: satisfied.
- Task evidence explicitly rejects ARP request emission, packet queues,
  routing, driver consultation, frame transmission, live packet I/O, sockets,
  SSH, ping/network reachability, hardware readiness, and phase transition
  claims: satisfied.

## Validation

- cargo fmt --check: pass.
- cargo -Zjson-target-spec test network with configured QEMU path: pass, 570
  talos no_std tests passed.
- cargo -Zjson-target-spec check: pass.
- jq empty on task-owned JSON evidence: pass.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check: pass before commit.

## Next Action

The queued closeout
phase12-network-outbound-neighbor-resolution-closeout-20260619 is
mechanically unblocked after this accepted commit if dependencies remain
satisfied. Do not promote packet queues, ARP request emission,
hardware-driver work, live packet I/O, smoltcp adoption, sockets, SSH,
network reachability, ping behavior, link-readiness work, Pi 5 hardware work,
or any phase transition directly from this implementation.
