# Phase 12.3 ARP Request Emission Closeout

Task id: phase12-network-arp-request-emission-closeout-20260619

Status: accepted

Classification:
phase12-network-arp-request-emission-closeout-accepted-planning-needed

Evidence level: static/source/task evidence review, task-owned JSON evidence,
docs build, and diff checks. No source implementation, Pi 5 hardware run, boot
archive publication, lab mutation, hardwareTestLock acquisition, live packet
I/O, driver transmit, sockets, SSH, smoltcp adoption, ping/network reachability
behavior, or phase transition was performed.

## Goal

Close out the accepted host-only ARP request emission task and record the exact
Phase 12.3 frontier before any follow-up task starts.

## Reviewed Evidence

- Core task record:
  tasks/2026-06-19-phase12-network-arp-request-emission-core.md.
- Core classification:
  tasks/evidence/2026-06-19-phase12-network-arp-request-emission-core/classification.json.
- Source and tests: src/network.rs.
- Phase 12 project doc: docs/src/project/phase12-networking-ssh.md.
- Roadmap: docs/src/roadmap.md.
- Accepted commit: 99b8a4cab3bb3794b29500758cf05ef6beb6b0d3.

## Closeout Result

The implementation is accepted as a local source/test boundary only:
build_outbound_arp_request constructs a complete Ethernet II ARP request frame
for a local endpoint and target IPv4 into caller-owned storage.

The accepted boundary includes deterministic broadcast destination MAC, local
source MAC, ARP EtherType, Ethernet/IPv4 hardware and protocol fields, ARP
request operation, endpoint sender MAC/IP, zero target MAC, caller-provided
target IPv4, deterministic returned frame length, too-small-output rejection,
and composition with unresolved outbound neighbor resolution without ARP cache
mutation.

The accepted boundary excludes neighbor-discovery state machines, ARP retry
timers, packet queues, routing/subnet/gateway selection, driver transmit
scheduling, live packet I/O, sockets, SSH, smoltcp adoption, ping/network
reachability behavior, RP1 Ethernet readiness, Pi 5 hardware proof, boot
publication, lab mutation, and phase transition.

## Findings

- fixed: the closeout reconciles the accepted implementation outcome,
  validation, docs, task evidence, rejected claims, and deferred work.
- fixed: the exact accepted frontier remains caller-buffered Ethernet/IPv4 ARP
  request construction for a local endpoint and target IPv4 only.
- fixed: corrected the core task record and classification JSON to include the
  final fmt, JSON, diff, docs, and staged-diff gates that were run before the
  accepted implementation commit.
- deferred: packet queues, ARP retry timers, neighbor-discovery state,
  routing, driver transmit scheduling, live packet I/O, sockets, SSH, smoltcp
  integration, ping/network reachability behavior, and hardware proof remain
  future work.
- removed: no source files, tests, dependencies, prior task evidence, or docs
  sections were removed.
- not-an-issue: no Pi 5 inconclusive-run triage was required because this task
  performed no hardware run and made no hardware-dependent claim.

## Planning Decision

planningNeeded: true

planningReason:
The closeout accepts the current caller-buffered outbound request frontier, but
no explicit queued follow-up task remains after
phase12-network-arp-request-emission-closeout-20260619 with scope, nonGoals,
dependencies, acceptanceCriteria, validationGates, docsRequired, and
evidenceRequired. Supervisor planning is required before packet queues,
driver transmit, live packet I/O, sockets, SSH, smoltcp adoption, network
reachability, hardware work, or another Phase 12.3 feature begins.

selected_next_task: null

## Rejected Claims

- No neighbor-discovery state machine, ARP retry timer, packet queue, routing
  table, subnet/gateway selection, driver transmit scheduling, live packet I/O,
  packet capture, ping behavior, network reachability, sockets, SSH, UDP/TCP,
  DHCP, DNS, smoltcp adoption, RP1 Ethernet readiness, DMA/interrupt behavior,
  Pi 5 hardware proof, boot archive publication, lab mutation, or phase
  transition is accepted by this closeout.
- No follow-up implementation task is selected by this closeout.

## Acceptance Check

- Closeout reconciles implementation outcome, validation, docs, evidence,
  rejected claims, and deferred work: satisfied by reviewed task evidence,
  classification, source/test diff, docs, and this record.
- The accepted boundary remains caller-buffered Ethernet/IPv4 ARP request
  construction only, with no queueing, retry, driver transmit, live packet I/O,
  or reachability claim: satisfied.
- If a next task is selected, dependencies are mechanical and objective;
  otherwise planningNeeded is set with a precise reason: satisfied by setting
  planningNeeded because no explicit queued follow-up task remains.
- No hardware, lab, boot publication, live packet I/O, sockets, SSH, network
  reachability, or phase transition claim is accepted: satisfied.

## Validation

- static/source/task evidence review: pass.
- jq empty on task-owned JSON evidence: pass.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check: pass before commit.

## Next Action

Supervisor planning is required before any further Phase 12.3 task. Do not
promote packet queues, driver transmit, live packet I/O, sockets, SSH, network
reachability, ping behavior, Pi 5 hardware work, boot publication, lab
mutation, smoltcp adoption, link-readiness work, or any phase transition
directly from this closeout.
