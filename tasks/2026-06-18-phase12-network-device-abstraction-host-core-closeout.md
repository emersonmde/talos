# Phase 12.2 Network Device Abstraction Host Core Closeout

Task id: phase12-network-device-abstraction-host-core-closeout-20260618

Status: accepted

Classification:
phase12-network-device-abstraction-host-core-closeout-smoltcp-checkpoint-selected

Evidence level: static/task/evidence consistency review, task-owned JSON
evidence, and diff checks. No runtime implementation, Pi 5 hardware run, boot
archive publication, lab mutation, hardwareTestLock acquisition, live packet
I/O, DMA descriptor ownership, sockets, SSH, hardware-driver readiness, or
phase transition was performed.

## Goal

Close the accepted host/testable network abstraction core, record exactly what
the parser/device boundary now accepts, make the remaining Milestone 12.2 gaps
explicit, and select only the next queued local source checkpoint.

## Scope Performed

- Reconciled the accepted
  phase12-network-device-abstraction-ethernet-arp-ip-host-core-20260618 task
  record, classification JSON, source surface, and Phase 12 docs.
- Recorded accepted parser/device-boundary behavior and rejected hardware/live
  networking claims.
- Recorded remaining Milestone 12.2 gaps before any driver, packet dispatch, or
  stack integration claim can be made.
- Selected the already queued local source checkpoint
  phase12-network-device-abstraction-smoltcp-evaluation-source-checkpoint-20260618
  as the next task.

## Findings

- fixed: the host-core task is closed as a local no_std network
  device/protocol boundary with deterministic byte-slice parsing tests.
- fixed: accepted behavior is limited to NetworkDevice raw-frame movement and
  immutable Ethernet II, Ethernet/IPv4 ARP, and IPv4 parsers.
- fixed: rejected claims are explicit: hardware link readiness, live packet
  I/O, DMA descriptor ownership, RP1 Ethernet driver readiness, sockets, SSH,
  and phase transition remain unaccepted.
- deferred: packet dispatch, checksum verification, fragmentation policy,
  queueing, buffer ownership beyond caller-owned byte slices, ICMP, UDP, TCP,
  socket integration, and driver adapter work remain future tasks.
- deferred: Milestone 12.3 stack selection still needs a source checkpoint
  before adding smoltcp or choosing a local protocol-layer path.
- removed: no source, task, docs, helper, or evidence files were removed.
- not-an-issue: no hardware lock, boot publication, or inconclusive-run triage
  was required because this closeout is static/task evidence work only.

## Accepted Behavior

- NetworkDevice is the current driver-facing boundary. Implementations receive
  into caller-owned buffers and transmit caller-owned frame slices.
- ReceivedFrame separates raw received bytes from parser interpretation.
- EthernetFrame parses Ethernet II destination/source MAC addresses, EtherType,
  and payload.
- ArpPacket parses only Ethernet/IPv4 ARP packets and rejects truncated,
  unsupported hardware/protocol, and invalid hardware/protocol length inputs.
- Ipv4Packet parses version, IHL/header length, total length, protocol,
  source/destination addresses, and payload while rejecting malformed or
  truncated packets.

## Remaining Gaps

- There is no packet dispatcher from EthernetFrame into ARP/IPv4 handlers.
- IPv4 checksum validation and fragmentation policy are not implemented.
- ICMP, UDP, TCP, sockets, and SSH are not implemented.
- Packet queues, ownership rules for reusable buffers, and interrupt/DMA
  integration are not implemented.
- No RP1 Ethernet driver is connected to NetworkDevice.
- No live packet I/O, ping response, TCP connection, or lab-network behavior is
  accepted.
- smoltcp has not been selected or added; the next task only evaluates the
  source/dependency path and consequences.

## Decision

Selected next task:
phase12-network-device-abstraction-smoltcp-evaluation-source-checkpoint-20260618.

Planning needed: false.

Rationale: the host-core task deferred stack selection, and the supervisor
queue already contains a concrete source checkpoint for smoltcp-vs-local
evaluation with explicit dependencies, gates, docs, evidence, and non-hardware
constraints. This closeout does not select live networking, sockets, SSH,
hardware-driver readiness, or any hardware task.

## Evidence

- Host-core task record:
  tasks/2026-06-18-phase12-network-device-abstraction-ethernet-arp-ip-host-core.md.
- Host-core classification:
  tasks/evidence/2026-06-18-phase12-network-device-abstraction-ethernet-arp-ip-host-core/classification.json.
- Source implementation: src/network.rs.
- Module registration: src/main.rs.
- Phase 12 project doc:
  docs/src/project/phase12-networking-ssh.md.
- Roadmap record: docs/src/roadmap.md.
- Closeout classification:
  tasks/evidence/2026-06-18-phase12-network-device-abstraction-host-core-closeout/classification.json.

## Acceptance Check

- Closeout records accepted parser/device-boundary behavior and rejected
  claims: satisfied.
- Milestone 12.2 remaining gaps are explicit: satisfied.
- selected_next_task is either the smoltcp evaluation source checkpoint,
  another concrete queued task, or null with planningNeeded=true: satisfied by
  selecting
  phase12-network-device-abstraction-smoltcp-evaluation-source-checkpoint-20260618.

## Validation

- static/task/evidence consistency review: pass.
- jq empty on task-owned JSON evidence: pass.
- git diff --check: pass.
- mdbook build: not run because docs/src files were not changed by this
  closeout.
- git diff --cached --check: pass before commit.

## Next Action

On a future worker wake, mechanically promote
phase12-network-device-abstraction-smoltcp-evaluation-source-checkpoint-20260618
only if dependencies remain satisfied and git status is clean. Do not promote
hardware-driver work, live packet I/O, sockets, SSH, RP1 Ethernet readiness, or
any Pi 5 hardware task from this closeout.
