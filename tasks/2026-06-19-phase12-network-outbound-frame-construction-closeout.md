# Phase 12.3 Outbound Frame Construction Closeout

Task id: phase12-network-outbound-frame-construction-closeout-20260619

Status: accepted

Classification:
phase12-network-outbound-frame-construction-closeout-accepted

Evidence level: static source/task/evidence consistency review, task-owned
JSON classification, docs build, and diff checks. No Pi 5 hardware run, boot
archive publication, lab mutation, hardwareTestLock acquisition, live packet
I/O, driver transmit, ARP request emission, packet queue, sockets, SSH,
smoltcp adoption, ping/network reachability behavior, hardware-driver
readiness, link readiness, or phase transition was performed.

## Goal

Close out the accepted caller-buffered outbound Ethernet II frame-construction
core, reconcile the implementation evidence against the Phase 12.3 host-only
source/test boundary, and either select a concrete mechanically unblocked
feature-led follow-up or request supervisor planning without guessing.

## Reviewed Evidence

- Core task record:
  tasks/2026-06-19-phase12-network-outbound-frame-construction-core.md.
- Core classification:
  tasks/evidence/2026-06-19-phase12-network-outbound-frame-construction-core/classification.json.
- Source implementation and deterministic no_std tests: src/network.rs.
- Source checkpoint:
  tasks/2026-06-19-phase12-network-outbound-frame-construction-source-checkpoint.md.
- Phase 12 project documentation:
  docs/src/project/phase12-networking-ssh.md.
- Roadmap Milestone 12.3 status: docs/src/roadmap.md.
- Accepted core commit:
  f9661b41279d1fb5e34009ec14e6145e8bf1a35b.

## Closeout Result

The outbound frame-construction core is accepted as a host-only source/test
boundary. src/network.rs exposes build_outbound_ethernet_frame and
OutboundFrameError as an allocation-free helper that builds one Ethernet II
frame into caller-owned storage from an already resolved outbound neighbor.

Resolved neighbor input writes the destination MAC from the resolved neighbor,
the caller-provided source MAC, the caller-selected EtherType in network byte
order, and an exact payload copy, then returns the deterministic frame length.
Unresolved neighbor input is rejected with the destination IPv4 preserved in
the error, and too-small output buffers are rejected with required and
available lengths before partial-frame acceptance.

The helper composes with the accepted cached outbound neighbor resolver, but
it intentionally stops below ARP request emission, retry behavior, packet
queues, routing/subnet/gateway selection, outbound IPv4/ICMP request
construction, driver transmit scheduling, live packet I/O, sockets, SSH,
smoltcp adoption, ping/network reachability behavior, Pi 5 packet movement,
or a phase transition.

## Findings

- fixed: Phase 12.3 now has accepted caller-buffered Ethernet II frame
  construction from a resolved outbound neighbor into caller-owned storage.
- fixed: deterministic tests cover destination MAC, source MAC, EtherType,
  payload preservation, returned length, unresolved-neighbor rejection,
  too-small-output rejection, and resolver-to-frame composition.
- fixed: the implementation preserves the host-only/no-live-I/O boundary by
  avoiding driver access, transmit scheduling, cache mutation, packet queues,
  ARP request emission, allocation, and async behavior.
- not-an-issue: no implementation correction was required during closeout
  because static/source/task evidence matched the accepted core claims.
- deferred: ARP request emission, retry timers, packet queues, routing,
  subnet/gateway selection, outbound IPv4/ICMP request construction, driver
  transmit scheduling, live packet I/O, sockets, SSH, smoltcp integration,
  ping behavior, and Pi 5 packet movement evidence remain future bounded work.
- removed: no source, docs, dependencies, prior task evidence, or accepted APIs
  were removed by this closeout.
- rejected: live packet I/O, hardware-driver readiness, link readiness,
  ping/network reachability behavior, sockets, SSH, smoltcp adoption, and
  phase transition are not accepted by this closeout.

## Planning Decision

selected_next_task: null

planningNeeded: true

Rationale: the accepted frame-construction helper is the current explicit
Phase 12.3 frontier, and no later queued task exists with explicit scope,
non-goals, acceptance criteria, validation gates, docs requirements, evidence
requirements, and dependencies. The likely next feature-led boundary must be
supervisor-planned before a worker can promote it; plausible future work
includes outbound IPv4/ICMP request construction or ARP-request/neighbor
discovery plumbing, but this closeout does not choose or define that work.

## Rejected Claims

- No ARP request emission, retry timer, packet queue, routing table,
  subnet/gateway logic, outbound IPv4/ICMP request construction, driver
  transmit scheduling, or live packet I/O was accepted.
- No Pi 5 hardware run, boot archive publication, lab mutation, or
  hardwareTestLock acquisition occurred.
- No RP1 Ethernet driver readiness, DMA descriptor ownership, interrupt
  integration, packet capture, hardware link readiness, network reachability,
  ping behavior, socket API, SSH behavior, UDP/TCP, DHCP, DNS, smoltcp
  adoption, userspace networking API, or phase transition was accepted.

## Acceptance Check

- Closeout reconciles implementation outcome, validation, docs, evidence,
  rejected claims, and deferred work: satisfied by the reviewed evidence and
  findings above.
- Because implementation was accepted, closeout preserves the exact boundary:
  caller-buffered Ethernet II frame construction from resolved neighbor to
  output buffer, with no live transmit or reachability claims: satisfied.
- No concrete mechanically unblocked next task is available: planningNeeded is
  set true with selected_next_task null and the reason above.

## Validation

- static/source/task evidence review: pass.
- jq empty on task-owned JSON evidence: pass.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check: pass before commit.

## Next Action

Supervisor planning is required before the worker may promote another Phase
12.3 task. Do not promote ARP request emission, packet queues, driver
transmit, live packet I/O, smoltcp adoption, sockets, SSH, network
reachability, ping behavior, link-readiness work, Pi 5 hardware work, or any
phase transition directly from this closeout.
