# Phase 12.2 Network Device Abstraction Ethernet/ARP/IPv4 Host Core

Task id: phase12-network-device-abstraction-ethernet-arp-ip-host-core-20260618

Status: accepted

Classification:
phase12-network-device-abstraction-ethernet-arp-ip-host-core-local-static

Evidence level: source implementation, no_std unit tests, docs build, and diff
checks. No Pi 5 hardware run, boot archive publication, lab mutation,
hardwareTestLock acquisition, live packet I/O, DMA ownership, sockets, SSH,
Phase 12.2 hardware-driver acceptance, or phase transition was performed.

## Goal

Start the host/testable Phase 12.2 network abstraction slice selected by the
strategy checkpoint after the BCM54213PE low-power/lifecycle pause, without
claiming hardware Ethernet readiness.

## Scope Performed

- Added src/network.rs as a no_std boundary between device-owned raw frame
  movement and protocol parsing.
- Defined a minimal NetworkDevice trait for receive/transmit frame movement
  without binding it to RP1, DMA descriptors, interrupts, sockets, or SSH.
- Added deterministic byte-slice parsers for Ethernet II, Ethernet/IPv4 ARP,
  and IPv4.
- Added positive and negative unit tests for Ethernet, ARP, and IPv4 parsing.
- Updated roadmap and Phase 12 docs to record this as host/testable protocol
  boundary progress, not live networking or hardware-driver readiness.

## Findings

- fixed: Phase 12.2 now has a local no_std packet-boundary module that can be
  tested without hardware.
- fixed: Ethernet II parsing returns destination/source MAC addresses,
  EtherType, and payload without allocation.
- fixed: ARP parsing accepts only Ethernet/IPv4 shape and rejects truncated,
  unsupported hardware/protocol, and invalid hardware/protocol length inputs
  deterministically.
- fixed: IPv4 parsing accepts minimum headers and options when total length is
  consistent, and rejects truncated, non-IPv4, invalid-IHL, invalid total
  length, and truncated total length inputs deterministically.
- deferred: checksum validation, fragmentation policy, ICMP/UDP/TCP parsing,
  packet dispatch, queueing, and smoltcp-vs-local stack selection remain future
  Phase 12 tasks.
- rejected: hardware link readiness, live packet I/O, DMA descriptor
  ownership, sockets, SSH, and Phase 12.2 hardware-driver readiness are not
  accepted by this host-core task.
- removed: no files or prior task evidence were removed.
- not-an-issue: no hardware lock or inconclusive Pi 5 triage was required
  because this task is local source/test work only.

## Accepted Behavior

- Device ownership is represented by NetworkDevice, whose implementations move
  raw frames in and out of caller-owned buffers.
- Protocol parsing is represented by immutable byte-slice types:
  ReceivedFrame, EthernetFrame, ArpPacket, and Ipv4Packet.
- Ethernet II frames shorter than 14 bytes return PacketError::Truncated.
- Ethernet/IPv4 ARP packets shorter than 28 bytes return
  PacketError::Truncated.
- ARP hardware type must be Ethernet, protocol type must be IPv4, hardware
  length must be 6, and protocol length must be 4.
- IPv4 packets must have version 4, IHL of at least 5 words, a present header,
  total length greater than or equal to header length, and a present total byte
  range.

## Evidence

- Source implementation: src/network.rs.
- Module registration: src/main.rs.
- Task classification:
  tasks/evidence/2026-06-18-phase12-network-device-abstraction-ethernet-arp-ip-host-core/classification.json.
- Roadmap update: docs/src/roadmap.md.
- Phase 12 project doc update: docs/src/project/phase12-networking-ssh.md.

## Acceptance Check

- no_std network abstraction boundary exists with clear device/protocol
  ownership: satisfied by NetworkDevice plus immutable parser types.
- Ethernet II, ARP, and IPv4 positive and negative unit tests exist:
  satisfied by src/network.rs tests.
- malformed/truncated inputs are deterministic and allocation-free: satisfied.
- findings are listed with dispositions: satisfied.
- docs explain host/testable Phase 12.2 progress without hardware readiness:
  satisfied.
- hardware, live packet I/O, sockets, SSH, and hardware-driver claims remain
  rejected: satisfied.

## Validation

- cargo fmt --all -- --check: pass.
- cargo -Zjson-target-spec test --quiet: pass.
- focused unit-test evidence: src/network.rs tests cover Ethernet header
  split/truncation, ReceivedFrame separation, ARP positive/malformed cases, and
  IPv4 positive/options/malformed cases.
- jq empty on task-owned JSON evidence: pass.
- git diff --check: pass.
- mdbook build: pass.
- git diff --cached --check: pass before commit.

## Next Action

The queued closeout
phase12-network-device-abstraction-host-core-closeout-20260618 is mechanically
unblocked after this accepted commit if dependencies remain satisfied. Do not
promote smoltcp evaluation, hardware-driver work, live packet I/O, sockets,
SSH, or any Phase 12 hardware task directly from this host-core implementation.
