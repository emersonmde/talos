# Phase 12.3 ARP Cache Dispatch Integration Core

Task id: phase12-network-arp-cache-dispatch-integration-core-20260619

Status: accepted

Classification:
phase12-network-arp-cache-dispatch-integration-core-accepted

Evidence level: source implementation, no_std unit tests, docs build, task-owned
JSON evidence, and diff checks. No Pi 5 hardware run, boot archive publication,
lab mutation, hardwareTestLock acquisition, live packet I/O, sockets, SSH,
smoltcp adoption, RP1 Ethernet driver readiness, link readiness, network
reachability, ping behavior, or phase transition was performed.

## Goal

Implement the smallest host-only cache-aware local packet handling slice:
validated inbound Ethernet/IPv4 ARP sender facts are learned through a mutable
ArpCache while existing reply/no-reply behavior remains unchanged.

## Scope Performed

- Added dispatch_local_packet_with_arp_cache as a compatibility-preserving
  wrapper around dispatch_local_packet.
- Added poll_local_network_device_with_arp_cache as a compatibility-preserving
  wrapper around poll_local_network_device's receive/dispatch/transmit result
  mapping.
- Learned valid Ethernet/IPv4 ARP request and reply sender IPv4/MAC facts
  through a caller-provided mutable ArpCache.
- Preserved cache-unaware dispatch_local_packet and poll_local_network_device
  APIs and tests for callers that do not pass an ARP cache.
- Kept ARP reply generation and ICMP echo reply generation unchanged except for
  explicit ARP-cache side effects in the new cache-aware path.
- Updated Phase 12 docs and roadmap to record the accepted host-only cache-aware
  local packet boundary.

## Accepted Behavior

- Valid ARP requests are learned before local ARP reply generation.
- Valid ARP replies are learned and produce no transmitted reply in the
  cache-aware poll path; they retain the existing dispatch rejection
  classification as UnsupportedArpOperation.
- Malformed or unsupported ARP frames return PacketError without mutating the
  ARP cache.
- Non-ARP IPv4/ICMP echo frames preserve the same output bytes and dispatch
  result as the cache-unaware path and do not mutate the ARP cache.
- NoFrame receive outcomes leave cache state unchanged.
- TransmitError outcomes retain ARP sender facts learned before reply
  transmission failed.

## Findings

- fixed: Phase 12.3 now has host-only cache-aware local dispatch and poll entry
  points that learn ARP sender facts from validated inbound ARP frames.
- fixed: existing cache-unaware dispatch_local_packet and
  poll_local_network_device callers remain source-compatible and behaviorally
  covered by tests.
- fixed: ARP request learning, ARP reply learning without transmit, malformed
  or unsupported ARP no-mutation behavior, unchanged ICMP echo behavior,
  no-frame cache behavior, and transmit-error cache behavior are covered by
  deterministic no_std tests.
- deferred: outbound neighbor resolution, packet queues, driver adapter
  integration, UDP/TCP, DHCP, DNS, routing, sockets, SSH, smoltcp integration,
  live packet I/O, and Pi 5 packet movement evidence remain future work.
- removed: no source files, dependencies, or prior task evidence were removed.
- not-an-issue: no hardware lock, lab mutation, boot publication, or Pi 5
  inconclusive-run triage was required because this task is local source/test
  work only.

## Rejected Claims

- No outbound neighbor resolution, packet queue readiness, driver adapter
  readiness, RP1 Ethernet driver readiness, DMA descriptor ownership, interrupt
  integration, packet capture, or live packet I/O was accepted.
- No smoltcp dependency or third-party network stack was adopted.
- No UDP/TCP behavior, DHCP, DNS, routing, socket API, SSH behavior, network
  reachability, ping behavior, hardware link readiness, or phase transition was
  accepted.

## Evidence

- Source implementation and tests: src/network.rs.
- Source checkpoint:
  tasks/2026-06-19-phase12-network-arp-cache-dispatch-integration-source-checkpoint.md.
- Task classification:
  tasks/evidence/2026-06-19-phase12-network-arp-cache-dispatch-integration-core/classification.json.
- Phase 12 project doc update: docs/src/project/phase12-networking-ssh.md.
- Roadmap update: docs/src/roadmap.md.

## Acceptance Check

- A cache-aware local dispatch or poll entry point learns valid ARP request and
  reply sender IPv4/MAC facts through ArpCache and returns deterministic
  results: satisfied by dispatch_local_packet_with_arp_cache and
  poll_local_network_device_with_arp_cache.
- Existing cache-unaware dispatch_local_packet and poll_local_network_device
  behavior remains covered by tests and unchanged for existing callers:
  satisfied.
- ARP reply generation and ICMP echo behavior remain semantically unchanged
  except for explicit ARP cache learning side effects in the new cache-aware
  path: satisfied.
- Malformed, unsupported, or non-ARP frames do not mutate the cache unless an
  accepted ArpCache learning rule explicitly permits it: satisfied.
- Transmit-error and no-frame cases have documented/tested cache-state behavior:
  satisfied.
- Findings are recorded with fixed, removed, deferred, or not-an-issue
  disposition: satisfied.
- selected_next_task is
  phase12-network-arp-cache-dispatch-integration-closeout-20260619:
  satisfied.

## Validation

- cargo fmt --all -- --check: pass.
- cargo -Zjson-target-spec test --quiet: pass.
- targeted network tests: src/network.rs covers cache-aware ARP request/reply
  learning, malformed/unsupported ARP without cache mutation, unchanged ICMP
  echo behavior, no-frame cache behavior, and transmit-error cache behavior.
- jq empty on task-owned JSON evidence: pass.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check: pass before commit.

## Next Action

The queued closeout
phase12-network-arp-cache-dispatch-integration-closeout-20260619 is
mechanically unblocked after this accepted commit if dependencies remain
satisfied. Do not promote outbound neighbor resolution, packet queues,
hardware-driver work, live packet I/O, smoltcp adoption, sockets, SSH, network
reachability, ping behavior, link-readiness work, or any Pi 5 hardware task
directly from this implementation.
