# Phase 12.3 ARP Cache Dispatch Integration Closeout

Task id: phase12-network-arp-cache-dispatch-integration-closeout-20260619

Status: accepted

Classification:
phase12-network-arp-cache-dispatch-integration-closeout-accepted

Evidence level: static source/task/evidence consistency review, task-owned
JSON classification, docs build, and diff checks. No Pi 5 hardware run, boot
archive publication, lab mutation, hardwareTestLock acquisition, live packet
I/O, sockets, SSH, smoltcp adoption, RP1 Ethernet driver readiness, link
readiness, network reachability, ping behavior, or phase transition was
performed.

## Goal

Close out the accepted cache-aware local dispatch/poll integration, reconcile
the implementation evidence against the Phase 12.3 source/test boundary, and
select the next mechanically unblocked source checkpoint only if it remains
bounded to host-only outbound neighbor-resolution planning.

## Reviewed Evidence

- Core task record:
  tasks/2026-06-19-phase12-network-arp-cache-dispatch-integration-core.md.
- Core classification:
  tasks/evidence/2026-06-19-phase12-network-arp-cache-dispatch-integration-core/classification.json.
- Source implementation and deterministic no_std tests: src/network.rs.
- Source checkpoint:
  tasks/2026-06-19-phase12-network-arp-cache-dispatch-integration-source-checkpoint.md.
- Phase 12 project documentation:
  docs/src/project/phase12-networking-ssh.md.
- Roadmap Milestone 12.3 status: docs/src/roadmap.md.
- Accepted core commit:
  cdf24728c8c0c64db7621d595696e3f02e6fb0ef.

## Closeout Result

The cache-aware dispatch integration core is accepted as a host-only
source/test boundary. src/network.rs exposes dispatch_local_packet_with_arp_cache
and poll_local_network_device_with_arp_cache as compatibility-preserving
wrappers that learn valid Ethernet/IPv4 ARP sender facts through caller-provided
ArpCache storage before reusing the existing local dispatch and one-step poll
behavior.

ARP requests learn the sender and can still generate the existing local ARP
reply. ARP replies learn the sender and do not transmit a local reply.
Malformed or unsupported ARP does not mutate the cache. Non-ARP ICMP echo
behavior remains byte-for-byte compatible with dispatch_local_packet, no-frame
receive outcomes leave cache state unchanged, and transmit errors keep any ARP
fact learned before reply transmission failed.

The integration intentionally does not implement outbound neighbor resolution,
packet queues, driver adapters, live packet I/O, ping behavior, sockets, SSH,
smoltcp adoption, network reachability, link readiness, or any phase
transition.

## Findings

- fixed: Phase 12.3 now has accepted cache-aware local dispatch and poll entry
  points that learn ARP sender facts from validated inbound ARP request and
  reply frames.
- fixed: existing cache-unaware dispatch_local_packet and
  poll_local_network_device behavior remains source-compatible and covered by
  deterministic tests.
- fixed: core tests cover ARP request learning with reply transmission, ARP
  reply learning without transmit, malformed or unsupported ARP without cache
  mutation, unchanged ICMP echo output, no-frame cache behavior, and
  transmit-error cache behavior.
- not-an-issue: no implementation correction was required during closeout
  because static/source/task evidence matched the accepted core claims.
- deferred: outbound neighbor resolution, packet queues, driver adapters,
  UDP/TCP, DHCP, DNS, routing, sockets, SSH, smoltcp integration, live packet
  I/O, and Pi 5 packet movement evidence remain future bounded work.
- removed: no source, docs, dependencies, prior task evidence, or accepted APIs
  were removed by this closeout.
- rejected: outbound resolution implementation, packet queue readiness, live
  packet I/O, hardware-driver readiness, link readiness, ping/network
  reachability behavior, sockets, SSH, smoltcp adoption, and phase transition
  are not accepted by this closeout.

## Planning Decision

selected_next_task:
phase12-network-outbound-neighbor-resolution-source-checkpoint-20260619

planningNeeded: false

Rationale: the accepted cache-aware dispatch and poll wrappers create the first
local neighbor-state side effect while preserving reply behavior. The next
smallest same-slice question is a source checkpoint for outbound neighbor
resolution using the accepted ArpCache. That checkpoint is already queued,
explicit, dependency-gated on this closeout, and preserves the no-hardware,
no-live-I/O strategy boundary. It must choose a bounded implementation shape or
request supervisor planning; this closeout does not authorize implementation.

## Rejected Claims

- No outbound neighbor-resolution implementation, packet queue, driver adapter,
  RP1 Ethernet adapter, DMA descriptor, interrupt integration, or packet
  capture was accepted.
- No Pi 5 hardware run, boot archive publication, lab mutation, or
  hardwareTestLock acquisition occurred.
- No live packet I/O, network reachability, ping behavior, hardware-driver
  readiness, or link readiness was accepted.
- No smoltcp dependency or third-party network stack was adopted.
- No UDP/TCP behavior, DHCP, DNS, routing, socket API, SSH behavior, or phase
  transition was accepted.

## Acceptance Check

- Closeout records accepted implementation behavior or precise blocker
  classification: satisfied by the accepted core evidence above.
- Accepted and rejected claims are explicit, especially cache-aware learning,
  reply behavior, outbound resolution, live packet I/O, sockets, SSH, network
  reachability, ping behavior, hardware readiness, and phase transition:
  satisfied.
- Because implementation is accepted, selected_next_task is
  phase12-network-outbound-neighbor-resolution-source-checkpoint-20260619:
  satisfied.
- Blocked implementation handling is not applicable because the core is
  accepted and committed.

## Validation

- static/source/task evidence review: pass.
- jq empty on task-owned JSON evidence: pass.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check: pass before commit.

## Next Action

The queued
phase12-network-outbound-neighbor-resolution-source-checkpoint-20260619 task is
mechanically unblocked for the next worker wake if dependencies remain
satisfied and git status is clean. Do not promote outbound neighbor-resolution
implementation, packet queues, hardware-driver work, live packet I/O, smoltcp
adoption, sockets, SSH, network reachability, ping behavior, link-readiness
work, or any Pi 5 hardware task directly from this closeout.
