# Phase 12.3 Outbound IPv4 ICMP Echo Request Closeout

Task id: phase12-network-outbound-ipv4-icmp-echo-request-closeout-20260619

Status: accepted

Classification:
phase12-network-outbound-ipv4-icmp-echo-request-closeout-accepted

Evidence level: static/source/task evidence review, task-owned JSON evidence,
docs build, and diff checks. No source implementation, Pi 5 hardware run, boot
archive publication, lab mutation, hardwareTestLock acquisition, live packet
I/O, driver transmit, sockets, SSH, smoltcp adoption, ping/network reachability
behavior, or phase transition was performed.

## Goal

Close out the accepted host-only outbound IPv4/ICMP echo request construction
task and record the exact Phase 12.3 frontier before any follow-up task starts.

## Reviewed Evidence

- Core task record:
  tasks/2026-06-19-phase12-network-outbound-ipv4-icmp-echo-request-core.md.
- Core classification:
  tasks/evidence/2026-06-19-phase12-network-outbound-ipv4-icmp-echo-request-core/classification.json.
- Source and tests: src/network.rs.
- Phase 12 project doc: docs/src/project/phase12-networking-ssh.md.
- Roadmap: docs/src/roadmap.md.
- Accepted commit: 59db8c790d53abf255caa1c8cb181d933c16787f.

## Closeout Result

The implementation is accepted as a local source/test boundary only:
build_outbound_ipv4_icmp_echo_request constructs a complete Ethernet II IPv4
ICMP echo request frame for an already resolved outbound neighbor into
caller-owned storage.

The accepted boundary includes deterministic Ethernet destination/source MACs,
IPv4 EtherType, IPv4 version/IHL/total length/TTL/protocol/source/destination
and checksum fields, ICMP echo request type/code/identifier/sequence/payload
and checksum fields, deterministic returned frame length, cached-neighbor
composition, unresolved-neighbor rejection, output-buffer pressure rejection,
and oversized IPv4 payload rejection.

The accepted boundary excludes ARP request emission, retry timers, packet
queues, routing/subnet/gateway selection, driver transmit scheduling, live
packet I/O, sockets, SSH, smoltcp adoption, ping/network reachability behavior,
RP1 Ethernet readiness, Pi 5 hardware proof, boot publication, lab mutation,
and phase transition.

## Findings

- fixed: the closeout reconciles the accepted implementation outcome,
  validation, docs, task evidence, rejected claims, and deferred work.
- fixed: the exact accepted frontier remains caller-buffered Ethernet IPv4
  ICMP echo request construction for a resolved neighbor only.
- fixed: the queued
  phase12-network-arp-request-emission-source-checkpoint-20260619 is the next
  mechanically clear same-slice follow-up because unresolved-neighbor rejection
  is the remaining blocker below live packet I/O, and the queued checkpoint has
  explicit scope, non-goals, dependencies, acceptance criteria, validation
  gates, docs requirements, and evidence requirements.
- deferred: ARP request emission, neighbor-discovery state, retry timers,
  packet queues, routing, driver transmit scheduling, live packet I/O, sockets,
  SSH, smoltcp integration, and hardware proof remain future work.
- removed: no source files, tests, dependencies, prior task evidence, or docs
  sections were removed.
- not-an-issue: no Pi 5 inconclusive-run triage was required because this task
  performed no hardware run and made no hardware-dependent claim.

## Rejected Claims

- No ARP request emission, ARP retry timer, packet queue, routing table,
  subnet/gateway selection, driver transmit scheduling, live packet I/O,
  packet capture, ping behavior, network reachability, sockets, SSH, UDP/TCP,
  DHCP, DNS, smoltcp adoption, RP1 Ethernet readiness, DMA/interrupt behavior,
  Pi 5 hardware proof, boot archive publication, lab mutation, or phase
  transition is accepted by this closeout.
- The selected next task is a source/static checkpoint only. It does not
  authorize ARP request implementation, driver transmit, hardware work, or
  reachability claims.

## Acceptance Check

- Closeout reconciles implementation outcome, validation, docs, evidence,
  rejected claims, and deferred work: satisfied by reviewed task evidence,
  classification, source/test diff, docs, and this record.
- The accepted boundary remains caller-buffered Ethernet IPv4 ICMP echo request
  construction for a resolved neighbor, with no ARP request emission, queueing,
  driver transmit, live packet I/O, or reachability claim: satisfied.
- The selected next task is concrete, bounded, dependency-gated, and
  feature-led: satisfied by selecting the already queued
  phase12-network-arp-request-emission-source-checkpoint-20260619.
- No worker guesswork is required: satisfied; the queued checkpoint was already
  supervisor-planned and is now explicitly selected by this closeout.

## Validation

- static/source/task evidence review: pass.
- jq empty on task-owned JSON evidence: pass.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check: pass before commit.

## Next Action

Mechanically promote
phase12-network-arp-request-emission-source-checkpoint-20260619 on the next
worker wake if dependencies remain satisfied and git status is clean. Do not
promote ARP request implementation, packet queues, driver transmit, live packet
I/O, sockets, SSH, network reachability, ping behavior, Pi 5 hardware work,
boot publication, lab mutation, smoltcp adoption, link-readiness work, or any
phase transition directly from this closeout.
