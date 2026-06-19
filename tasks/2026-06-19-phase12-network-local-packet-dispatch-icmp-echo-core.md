# Phase 12.3 Local Packet Dispatch and ICMP Echo Core

Task id: phase12-network-local-packet-dispatch-icmp-echo-core-20260619

Status: accepted

Classification:
phase12-network-local-packet-dispatch-icmp-echo-core-accepted

Evidence level: source implementation, no_std unit tests, docs build, and diff
checks. No Pi 5 hardware run, boot archive publication, lab mutation,
hardwareTestLock acquisition, live packet I/O, sockets, SSH, smoltcp adoption,
RP1 Ethernet driver readiness, or phase transition was performed.

## Goal

Implement the local packet-dispatch path recommended by the smoltcp source
checkpoint: deterministic Ethernet/ARP and Ethernet/IPv4/ICMP echo handling
over immutable input frames and caller-provided output buffers.

## Scope Performed

- Extended src/network.rs with LocalNetworkEndpoint, PacketReplyKind,
  PacketDispatchResult, and dispatch_local_packet.
- Routed Ethernet II ARP and IPv4 frames without binding to RP1 hardware, DMA,
  interrupts, sockets, smoltcp, or a live device.
- Added IPv4 header checksum validation and generation for the accepted local
  IPv4 minimum-header echo shape.
- Added ICMP echo request validation and ICMP echo reply generation.
- Added ARP request-to-local-IPv4 reply generation.
- Added deterministic errors for unsupported EtherTypes, non-ICMP IPv4
  protocols, IPv4 options, IPv4 fragments, malformed checksums, nonlocal
  Ethernet/IP destinations, and too-small output buffers.
- Updated Phase 12 docs and roadmap to record the accepted local packet
  behavior and rejected live-networking claims.

## Findings

- fixed: Phase 12.3 now has a host-testable local dispatcher that turns accepted
  ARP request and ICMP echo request shapes into reply frames without allocation.
- fixed: IPv4 header checksums are validated on accepted input and generated on
  ICMP echo replies.
- fixed: ICMP checksums are validated on echo requests and generated on echo
  replies.
- fixed: ARP replies are written into caller-provided buffers with local
  MAC/IPv4 identity and requester target fields preserved.
- fixed: unsupported protocols, malformed checksums, IPv4 fragments/options,
  nonlocal Ethernet/IP destinations, and small output buffers produce
  deterministic PacketError results.
- deferred: reusable packet buffers, device polling, a driver adapter, ARP
  cache, UDP/TCP, DHCP, DNS, routing, smoltcp integration, sockets, and SSH
  remain future tasks.
- rejected: live packet I/O, RP1 Ethernet driver readiness, DMA descriptor
  ownership, interrupt integration, smoltcp adoption, sockets, SSH, and phase
  transition are not accepted by this task.
- removed: no source files, dependencies, or prior task evidence were removed.
- not-an-issue: no hardware lock or inconclusive Pi 5 triage was required
  because this task is local source/test work only.

## Accepted Behavior

- dispatch_local_packet parses an Ethernet II frame and dispatches only ARP and
  IPv4 EtherTypes.
- Ethernet/ARP requests targeting the configured local IPv4 address can produce
  Ethernet/ARP replies into caller-provided buffers when the Ethernet
  destination is local or broadcast.
- Ethernet/IPv4/ICMP echo requests targeting the configured local IPv4 address
  can produce Ethernet/IPv4/ICMP echo replies into caller-provided buffers when
  the Ethernet destination is local.
- Accepted IPv4 echo-request input must use a minimum 20-byte header, no
  fragmentation, protocol ICMP, a valid IPv4 header checksum, and local
  destination IPv4 address.
- Accepted ICMP echo-request input must have type 8, code 0, at least the echo
  header length, and a valid ICMP checksum.
- Reply generation emits valid IPv4 and ICMP checksums for the reply frames.
- Unsupported EtherTypes, non-ICMP IPv4 protocols, unsupported IPv4 options,
  unsupported IPv4 fragments, invalid checksums, nonlocal packets, malformed
  ICMP echo shapes, nonlocal Ethernet/IP destinations, and too-small output
  buffers return deterministic errors.

## Evidence

- Source implementation and tests: src/network.rs.
- Recommendation lineage:
  tasks/2026-06-18-phase12-network-device-abstraction-smoltcp-evaluation-source-checkpoint.md.
- Task classification:
  tasks/evidence/2026-06-19-phase12-network-local-packet-dispatch-icmp-echo-core/classification.json.
- Roadmap update: docs/src/roadmap.md.
- Phase 12 project doc update: docs/src/project/phase12-networking-ssh.md.

## Acceptance Check

- A no_std local packet dispatcher accepts caller-owned input/output byte slices
  and routes Ethernet/ARP and Ethernet/IPv4/ICMP echo request shapes
  deterministically: satisfied by dispatch_local_packet.
- IPv4 header checksum validation and generation are implemented for the
  accepted local IPv4 shapes, with malformed checksum behavior covered by
  tests: satisfied.
- ARP request to the configured local IPv4/MAC identity can produce a correct
  Ethernet/ARP reply frame into a caller-provided buffer: satisfied.
- ICMP echo request to the configured local IPv4/MAC identity can produce a
  correct Ethernet/IPv4/ICMP echo reply frame into a caller-provided buffer:
  satisfied.
- Unsupported EtherTypes, non-ICMP IPv4 protocols, malformed IPv4, malformed
  ICMP, unsupported fragments/options, and too-small output buffers return
  deterministic errors without allocation: satisfied.
- Findings are recorded with dispositions: satisfied.
- Task evidence explicitly rejects live packet I/O, hardware-driver readiness,
  sockets, SSH, smoltcp adoption, and phase transition: satisfied.

## Validation

- cargo fmt --all -- --check: pass.
- cargo -Zjson-target-spec test --quiet: pass.
- focused unit-test evidence: src/network.rs tests cover packet dispatch,
  checksum validation/generation, ARP reply, ICMP echo reply, malformed IPv4
  and ICMP inputs, unsupported EtherType/protocol, unsupported options/fragments,
  nonlocal Ethernet/IP destinations, and output-buffer-too-small behavior.
- jq empty on task-owned JSON evidence: pass.
- git diff --check: pass.
- mdbook build: pass.
- git diff --cached --check: pass before commit.

## Next Action

The queued closeout
phase12-network-local-packet-dispatch-icmp-echo-closeout-20260619 is
mechanically unblocked after this accepted commit if dependencies remain
satisfied. Do not promote packet-buffer/device-polling checkpoint, smoltcp
adoption, hardware-driver work, live packet I/O, sockets, SSH, or any Pi 5
hardware task directly from this implementation.
