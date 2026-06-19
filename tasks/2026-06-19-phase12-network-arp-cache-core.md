# Phase 12.3 ARP Cache Core

Task id: phase12-network-arp-cache-core-20260619

Status: accepted

Classification:
phase12-network-arp-cache-core-accepted

Evidence level: source implementation, no_std unit tests, docs build, and diff
checks. No Pi 5 hardware run, boot archive publication, lab mutation,
hardwareTestLock acquisition, live packet I/O, sockets, SSH, smoltcp adoption,
RP1 Ethernet driver readiness, link readiness, network reachability, ping
behavior, or phase transition was performed.

## Goal

Implement the bounded ARP-cache source/test slice selected after the accepted
ARP-cache source checkpoint: fixed-capacity IPv4-to-MAC neighbor storage,
deterministic insert/update/lookup/replacement behavior, and sender learning
from validated Ethernet/IPv4 ARP packets.

## Scope Performed

- Added ArpNeighbor, ArpCacheUpdate, and ArpCache<const CAPACITY: usize> to
  src/network.rs.
- Kept the cache allocation-free with fixed storage inside the cache object.
- Implemented lookup miss/hit behavior keyed by IPv4 address.
- Implemented insert_or_update with deterministic existing-entry update,
  first-empty-slot insertion, and oldest-slot replacement when the cache is
  full.
- Added NoCapacity behavior for zero-capacity cache instances so callers get a
  deterministic no-state-change result instead of a panic.
- Added learn_ethernet_ipv4_arp over immutable Ethernet frame byte slices.
- Learning records sender IPv4-to-MAC facts from valid ARP requests and ARP
  replies after Ethernet and Ethernet/IPv4 ARP shape validation.
- Malformed, truncated, unsupported EtherType, and unsupported ARP operation
  inputs return PacketError without changing existing cache state.
- Preserved existing dispatch_local_packet and poll_local_network_device
  behavior; this task does not wire the cache into outbound resolution or
  packet-dispatch policy.

## ARP Cache Behavior Matrix

- lookup missing IPv4: returns None and does not mutate state.
- lookup present IPv4: returns the stored MacAddress.
- insert new IPv4 while space exists: stores the neighbor in the first empty
  slot and returns Inserted.
- update existing IPv4: replaces that IPv4's MAC in place and returns Updated.
- insert new IPv4 when full: replaces slots in stable oldest-slot order and
  returns Replaced(previous_neighbor).
- zero-capacity insert: returns NoCapacity and leaves lookup empty.
- learn valid ARP request: records sender_protocol_address to
  sender_hardware_address.
- learn valid ARP reply: records sender_protocol_address to
  sender_hardware_address.
- learn malformed/truncated/unsupported ARP input: returns PacketError and
  leaves existing entries unchanged.

## Findings

- fixed: Phase 12.3 now has a fixed-capacity, allocation-free ARP neighbor
  cache with deterministic lookup, insert, update, miss, full-table
  replacement, and zero-capacity behavior.
- fixed: ARP sender learning is host-testable over byte slices and accepts only
  validated Ethernet/IPv4 ARP request/reply shapes.
- fixed: malformed, truncated, unsupported EtherType, and unsupported ARP
  operation learning inputs are rejected without changing cache state.
- fixed: host tests cover insertion, existing-entry update, lookup miss,
  full-table replacement, valid ARP request learning, valid ARP reply learning,
  malformed/truncated ARP rejection, and unchanged existing dispatch and poll
  behavior through the full regression suite.
- deferred: wiring the cache into dispatch_local_packet, outbound neighbor
  resolution, packet queues, driver adapter integration, UDP/TCP, DHCP, DNS,
  routing, socket APIs, smoltcp adoption, SSH, live packet I/O, and Pi 5
  packet movement evidence remain future tasks.
- rejected: live packet I/O, RP1 Ethernet driver readiness, link readiness,
  network reachability, ping-on-lab behavior, sockets, SSH, smoltcp adoption,
  and phase transition are not accepted by this task.
- removed: no source files, dependencies, prior task evidence, or existing
  dispatch/poll APIs were removed.
- not-an-issue: no hardware lock or Pi 5 inconclusive-run triage was required
  because this task is local source/test work only.

## Accepted Behavior

- ArpCache owns a compile-time fixed number of optional neighbor entries and
  performs no dynamic allocation.
- Entries are keyed by IPv4 address and store the associated MAC address.
- Existing-entry updates do not advance the replacement cursor.
- Full-cache insertion replaces the next oldest slot in stable round-robin
  order.
- learn_ethernet_ipv4_arp parses Ethernet II ARP frames, validates the ARP
  Ethernet/IPv4 shape, accepts only ARP request and reply operations, and
  records sender facts.
- Existing packet dispatch and poll-step behavior remains unchanged by this
  cache-only implementation.

## Evidence

- Source implementation and tests: src/network.rs.
- Source checkpoint:
  tasks/2026-06-19-phase12-network-arp-cache-source-checkpoint.md.
- Task classification:
  tasks/evidence/2026-06-19-phase12-network-arp-cache-core/classification.json.
- Phase 12 project doc update: docs/src/project/phase12-networking-ssh.md.
- Roadmap update: docs/src/roadmap.md.

## Acceptance Check

- A fixed-capacity, allocation-free ARP neighbor cache API exists with
  deterministic lookup, insert/update, miss, and full-table replacement
  semantics documented in code or task evidence: satisfied by ArpCache and this
  behavior matrix.
- Learning from valid Ethernet/IPv4 ARP request and reply packets records
  sender IPv4-to-MAC facts while malformed, unsupported, or truncated ARP input
  is rejected without changing cache state: satisfied by
  learn_ethernet_ipv4_arp and tests.
- Host/unit tests cover insertion, existing-entry update, lookup miss,
  full-table replacement policy, valid ARP request learning, valid ARP reply
  learning, malformed/truncated ARP rejection, and unchanged existing ARP/ICMP
  dispatch and poll-step behavior: satisfied.
- The implementation preserves no_std/source-test scope and makes no hardware,
  live packet I/O, smoltcp, socket, SSH, driver-readiness, link-readiness,
  ping, network-reachability, or phase-transition claim: satisfied.
- Findings are recorded with fixed, removed, deferred, or not-an-issue
  disposition: satisfied.

## Validation

- cargo fmt --all -- --check: pass after formatting.
- cargo -Zjson-target-spec test --quiet: initial run failed because the QEMU
  runner could not find qemu-system-aarch64 without the Talos tool path.
- PATH=/opt/strider/openclaw/current/workspace/tools/qemu-9.2.0-install/bin:$PATH
  cargo -Zjson-target-spec test --quiet: pass, 558 no_std tests.
- focused unit-test evidence: src/network.rs tests cover ARP cache insertion,
  update, miss, full-table replacement, zero capacity, valid ARP request
  learning, valid ARP reply learning, malformed/truncated/unsupported learning
  rejection, and existing dispatch/poll regressions.
- jq empty on task-owned JSON evidence: pass.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check: pass before commit.

## Next Action

The queued closeout phase12-network-arp-cache-closeout-20260619 is
mechanically unblocked after this accepted commit if dependencies remain
satisfied. Do not promote hardware-driver work, live packet I/O, smoltcp
adoption, sockets, SSH, link-readiness work, network-reachability work, ping
behavior, or any Pi 5 hardware task directly from this implementation.
