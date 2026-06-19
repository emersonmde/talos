# Phase 12.3 Outbound Frame Construction Core

Task id: phase12-network-outbound-frame-construction-core-20260619

Status: accepted

Classification:
phase12-network-outbound-frame-construction-core-accepted

Evidence level: source implementation, no_std unit tests, build check, docs
build, task-owned JSON evidence, and diff checks. No Pi 5 hardware run, boot
archive publication, lab mutation, hardwareTestLock acquisition, live packet
I/O, driver adapter work, sockets, SSH, smoltcp adoption, ping/network
reachability behavior, or phase transition was performed.

## Goal

Implement the smallest outbound packet-preparation boundary after cached
neighbor resolution: construct an Ethernet II frame into caller-owned storage
from an already resolved neighbor, without queueing, ARP request emission,
driver transmit, or live packet I/O.

## Scope Performed

- Added OutboundFrameError for deterministic unresolved-neighbor and
  output-buffer-too-small outcomes.
- Added build_outbound_ethernet_frame, a pure helper that takes
  OutboundNeighborResolution, source MacAddress, EtherType, payload bytes, and
  a caller-owned output buffer.
- Kept dispatch_local_packet, dispatch_local_packet_with_arp_cache,
  poll_local_network_device, poll_local_network_device_with_arp_cache,
  ArpCache, and resolve_outbound_neighbor behavior unchanged for current
  callers.
- Added deterministic no_std tests for resolved frame construction,
  unresolved-neighbor rejection, too-small output rejection without partial
  frame acceptance, and composition with resolve_outbound_neighbor.
- Updated Phase 12 docs and roadmap to record the accepted host-only outbound
  frame-construction boundary.

## Accepted Behavior

- A resolved outbound neighbor writes an Ethernet II frame whose destination
  MAC comes from the resolved neighbor, source MAC comes from the caller,
  EtherType is caller-selected, payload bytes are copied exactly, and returned
  length is deterministic.
- An unresolved outbound neighbor returns
  OutboundFrameError::NeighborUnresolved carrying the destination IPv4.
- A too-small output buffer returns
  OutboundFrameError::OutputBufferTooSmall with required and available lengths
  before accepting any partial frame bytes as progress.
- The helper composes with the cached outbound neighbor resolver: a cached
  ArpCache entry can be resolved and then used to build a caller-buffered
  Ethernet II frame without mutating cache state.

## Findings

- fixed: Phase 12.3 now has a host-only caller-buffered Ethernet II frame
  construction helper below driver transmit/live I/O.
- fixed: resolved-neighbor output covers deterministic destination MAC, source
  MAC, EtherType, payload bytes, and returned frame length.
- fixed: unresolved-neighbor and too-small-output paths are deterministic and
  test-covered without cache mutation, driver access, transmit behavior,
  packet queueing, or ARP request emission.
- fixed: the accepted cached outbound neighbor resolver composes with the
  frame construction helper in a focused regression test.
- deferred: ARP request emission, retry timers, packet queues,
  routing/subnet/gateway selection, IPv4/ICMP outbound request construction,
  driver transmit scheduling, live packet I/O, sockets, SSH, smoltcp
  integration, ping behavior, and Pi 5 packet movement evidence remain future
  work.
- removed: no source files, dependencies, prior task evidence, or existing
  cache-unaware/cache-aware dispatch APIs were removed.
- not-an-issue: no hardware lock, lab mutation, boot publication, or Pi 5
  inconclusive-run triage was required because this task is local source/test
  work only.

## Rejected Claims

- No ARP request emission, retry timer, packet queue, routing table, subnet or
  gateway logic, driver consultation beyond a future caller, driver transmit
  scheduling, or live packet I/O was accepted.
- No IPv4/ICMP outbound request construction, ping behavior, packet capture,
  RP1 Ethernet driver readiness, DMA descriptor ownership, interrupt
  integration, link readiness, network reachability, socket API, SSH behavior,
  UDP/TCP, DHCP, DNS, smoltcp adoption, userspace networking API, Pi 5
  hardware proof, or phase transition was accepted.

## Evidence

- Source implementation and tests: src/network.rs.
- Source checkpoint:
  tasks/2026-06-19-phase12-network-outbound-frame-construction-source-checkpoint.md.
- Task classification:
  tasks/evidence/2026-06-19-phase12-network-outbound-frame-construction-core/classification.json.
- Phase 12 project doc update: docs/src/project/phase12-networking-ssh.md.
- Roadmap update: docs/src/roadmap.md.

## Acceptance Check

- A caller-buffered Ethernet II frame-construction helper exists and is
  source-owned below driver transmit/live I/O: satisfied by
  build_outbound_ethernet_frame.
- Resolved neighbor input produces a deterministic destination MAC, source
  MAC, EtherType, payload bytes, and returned length: satisfied by tests.
- Unresolved neighbor input is rejected deterministically without queueing,
  ARP request emission, driver access, cache mutation, or transmit behavior:
  satisfied by the helper signature and tests.
- Too-small output buffers are rejected deterministically without accepting a
  partial frame as progress: satisfied by tests.
- The accepted cached outbound neighbor resolver composes with the
  frame-construction helper in a focused regression test: satisfied.
- Task evidence records findings with disposition and rejects ARP request
  emission, packet queues, driver transmit, live packet I/O, sockets, SSH,
  ping/network reachability, RP1 Ethernet readiness, Pi 5 hardware readiness,
  and phase transition claims: satisfied.

## Validation

- cargo fmt --all: pass.
- cargo -Zjson-target-spec test outbound_frame_construction --quiet: pass.
- cargo fmt --all -- --check: pass.
- cargo -Zjson-target-spec test --quiet: pass.
- cargo -Zjson-target-spec check --quiet: pass.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- jq empty on task-owned JSON evidence: pass.
- git diff --cached --check: pass before commit.

## Next Action

The queued closeout
phase12-network-outbound-frame-construction-closeout-20260619 is mechanically
unblocked after this accepted commit if dependencies remain satisfied. Do not
promote ARP request emission, packet queues, driver transmit, live packet I/O,
smoltcp adoption, sockets, SSH, network reachability, ping behavior,
link-readiness work, Pi 5 hardware work, or any phase transition directly from
this implementation.
