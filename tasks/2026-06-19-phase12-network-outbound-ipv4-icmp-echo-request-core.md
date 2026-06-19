# Phase 12.3 Outbound IPv4 ICMP Echo Request Core

Task id: phase12-network-outbound-ipv4-icmp-echo-request-core-20260619

Status: accepted

Classification:
phase12-network-outbound-ipv4-icmp-echo-request-core-accepted

Evidence level: source implementation, no_std unit tests, docs build,
task-owned JSON evidence, and diff checks. No Pi 5 hardware run, boot archive
publication, lab mutation, hardwareTestLock acquisition, live packet I/O,
driver adapter work, sockets, SSH, smoltcp adoption, ping/network reachability
behavior, or phase transition was performed.

## Goal

Implement the smallest host-only outbound IPv4/ICMP echo request construction
boundary after cached neighbor resolution and caller-buffered Ethernet II frame
construction: produce a complete Ethernet II frame carrying an IPv4 ICMP echo
request for an already resolved neighbor into caller-owned storage.

## Scope Performed

- Added build_outbound_ipv4_icmp_echo_request, a pure caller-buffered helper
  that takes OutboundNeighborResolution, LocalNetworkEndpoint, ICMP
  identifier, sequence number, TTL, payload bytes, and caller-owned output
  storage.
- Extended OutboundFrameError with PayloadTooLarge so IPv4 total-length
  overflow is rejected before writing a wrapped length.
- Shared Ethernet header and resolved-neighbor extraction helpers with the
  accepted build_outbound_ethernet_frame path.
- Added deterministic no_std tests for resolved-neighbor success,
  cached-resolution composition, unresolved-neighbor rejection, too-small
  output rejection without partial progress, and oversized payload rejection.
- Updated Phase 12 docs and roadmap to record the accepted host-only outbound
  IPv4/ICMP request-construction boundary.

## Accepted Behavior

- A resolved outbound neighbor writes a complete Ethernet II IPv4 ICMP echo
  request frame into caller-owned output storage and returns the deterministic
  frame length.
- Ethernet destination/source MACs come from the resolved neighbor and local
  endpoint; EtherType is IPv4.
- IPv4 fields are deterministic: version/IHL 4/5, DSCP/ECN 0, identification
  0, no fragmentation flags/offset, caller-provided TTL, protocol ICMP,
  caller-provided local/destination IPv4 addresses, and a valid header
  checksum.
- ICMP fields are deterministic: echo request type/code, caller-provided
  identifier and sequence number, exact payload bytes, and a valid ICMP
  checksum.
- Unresolved neighbors, too-small output buffers, and oversized IPv4 payloads
  return deterministic errors before accepting partial frame construction as
  success.
- The helper composes with the accepted cached outbound neighbor resolver and
  remains allocation-free and host-only.

## Findings

- fixed: Phase 12.3 now has a host-only caller-buffered outbound IPv4/ICMP
  echo request construction helper below packet queues, driver transmit, and
  live packet I/O.
- fixed: success output covers deterministic Ethernet, IPv4, and ICMP fields,
  identifier/sequence/payload preservation, returned frame length, and valid
  IPv4/ICMP checksums.
- fixed: unresolved-neighbor, too-small-output, and oversized-payload paths are
  deterministic and test-covered before any partial frame is accepted as
  progress.
- fixed: cached outbound neighbor resolution composes with the request builder
  in a focused regression test without mutating ARP cache state.
- deferred: ARP request emission, retry timers, packet queues,
  routing/subnet/gateway selection, driver transmit scheduling, live packet
  I/O, sockets, SSH, smoltcp integration, ping/network reachability behavior,
  and Pi 5 packet movement evidence remain future work.
- removed: no source files, dependencies, prior task evidence, or existing
  cache-unaware/cache-aware dispatch APIs were removed.
- not-an-issue: no hardware lock, lab mutation, boot publication, or Pi 5
  inconclusive-run triage was required because this task is local source/test
  work only.

## Rejected Claims

- No ARP request emission, retry timer, packet queue, routing table, subnet or
  gateway logic, driver consultation beyond a future caller, driver transmit
  scheduling, or live packet I/O was accepted.
- No ping behavior, packet capture, RP1 Ethernet driver readiness, DMA
  descriptor ownership, interrupt integration, link readiness, network
  reachability, socket API, SSH behavior, UDP/TCP, DHCP, DNS, smoltcp
  adoption, userspace networking API, Pi 5 hardware proof, or phase transition
  was accepted.

## Evidence

- Source implementation and tests: src/network.rs.
- Prior frame construction closeout:
  tasks/2026-06-19-phase12-network-outbound-frame-construction-closeout.md.
- Task classification:
  tasks/evidence/2026-06-19-phase12-network-outbound-ipv4-icmp-echo-request-core/classification.json.
- Phase 12 project doc update: docs/src/project/phase12-networking-ssh.md.
- Roadmap update: docs/src/roadmap.md.

## Acceptance Check

- A caller can build a complete Ethernet II frame carrying an IPv4 ICMP echo
  request for a resolved destination neighbor into caller-owned output storage:
  satisfied by build_outbound_ipv4_icmp_echo_request.
- The generated frame has deterministic destination/source MACs, EtherType
  IPv4, IPv4 version/IHL/total length/TTL/protocol/source/destination/header
  checksum fields, ICMP echo type/code/identifier/sequence/payload, and valid
  ICMP checksum: satisfied by focused no_std tests.
- Unresolved neighbor input and undersized output buffers fail deterministically
  before accepting partial frame construction as success: satisfied by tests.
- The implementation remains allocation-free and host-only; it does not mutate
  ARP cache state, access a driver, queue packets, transmit frames, emit ARP
  requests, or claim network reachability: satisfied by source inspection and
  API shape.
- Tests cover the success and error boundaries and preserve accepted local
  dispatch, cache, polling, neighbor-resolution, and frame-construction
  behavior: satisfied by focused and full test runs.

## Validation

- cargo fmt --all: pass.
- cargo -Zjson-target-spec test outbound_ipv4_icmp_echo_request --quiet: pass.
- cargo -Zjson-target-spec test --quiet: pass with
  QEMU_SYSTEM_AARCH64 available through the configured QEMU PATH.
- cargo fmt --all -- --check: pass.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- jq empty on task-owned JSON evidence: pass.
- git diff --cached --check: pass before commit.

## Next Action

The queued closeout
phase12-network-outbound-ipv4-icmp-echo-request-closeout-20260619 is
mechanically unblocked after this accepted commit if dependencies remain
satisfied. Do not promote ARP request emission, packet queues, driver transmit,
live packet I/O, sockets, SSH, network reachability, ping behavior,
link-readiness work, Pi 5 hardware work, smoltcp adoption, or any phase
transition directly from this implementation.
