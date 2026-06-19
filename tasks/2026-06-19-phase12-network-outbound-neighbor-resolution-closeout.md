# Phase 12.3 Outbound Neighbor Resolution Closeout

Task id: phase12-network-outbound-neighbor-resolution-closeout-20260619

Status: accepted

Classification:
phase12-network-outbound-neighbor-resolution-closeout-accepted

Evidence level: static source/task/evidence consistency review, task-owned
JSON classification, docs build, and diff checks. No Pi 5 hardware run, boot
archive publication, lab mutation, hardwareTestLock acquisition, live packet
I/O, ARP request emission, packet queue, driver transmit, sockets, SSH,
smoltcp adoption, ping/network reachability behavior, hardware-driver
readiness, link readiness, or phase transition was performed.

## Goal

Close out the accepted cached-only outbound neighbor-resolution core, reconcile
the implementation evidence against the Phase 12.3 host-only source/test
boundary, and select the next mechanically unblocked source checkpoint only if
it remains bounded to deterministic local outbound packet preparation.

## Reviewed Evidence

- Core task record:
  tasks/2026-06-19-phase12-network-outbound-neighbor-resolution-core.md.
- Core classification:
  tasks/evidence/2026-06-19-phase12-network-outbound-neighbor-resolution-core/classification.json.
- Source implementation and deterministic no_std tests: src/network.rs.
- Source checkpoint:
  tasks/2026-06-19-phase12-network-outbound-neighbor-resolution-source-checkpoint.md.
- Phase 12 project documentation:
  docs/src/project/phase12-networking-ssh.md.
- Roadmap Milestone 12.3 status: docs/src/roadmap.md.
- Accepted core commit:
  1eb3a7b11a33513478a9095a0ed61d7a49d18178.

## Closeout Result

The outbound neighbor-resolution core is accepted as a host-only source/test
boundary. src/network.rs exposes OutboundNeighborResolution and
resolve_outbound_neighbor as an allocation-free cached resolver over immutable
ArpCache lookup state.

Known destination IPv4 addresses return a resolved result carrying the
destination IPv4 and cached MacAddress. Cache misses return an unresolved result
carrying the destination IPv4. Updated ArpCache entries are reflected by later
resolution calls, zero-capacity caches remain deterministic misses, and
accepted cache-aware poll learning can feed later cached resolution.

The implementation intentionally does not emit ARP requests, retry, queue
packets, perform routing or subnet/gateway selection, consult a driver,
construct outbound frames, schedule transmit, perform live packet I/O, accept
ping behavior, expose sockets, adopt smoltcp, prove RP1 Ethernet readiness, or
transition phases.

## Findings

- fixed: Phase 12.3 now has accepted cached-only outbound IPv4-to-MAC
  resolution through immutable ArpCache lookup semantics.
- fixed: unresolved-neighbor classification is deterministic and carries the
  target IPv4 without ARP request emission, packet queueing, or driver
  consultation.
- fixed: deterministic no_std tests cover cached hit, unresolved miss, updated
  entry, zero-capacity miss, and cache-aware poll compatibility.
- not-an-issue: no implementation correction was required during closeout
  because static/source/task evidence matched the accepted core claims.
- deferred: outbound frame construction, ARP request emission, retry timers,
  packet queues, routing, driver transmit scheduling, live packet I/O, sockets,
  SSH, smoltcp integration, and Pi 5 packet movement evidence remain future
  bounded work.
- removed: no source, docs, dependencies, prior task evidence, or accepted APIs
  were removed by this closeout.
- rejected: ARP request emission, packet queue readiness, live packet I/O,
  hardware-driver readiness, link readiness, ping/network reachability
  behavior, sockets, SSH, smoltcp adoption, and phase transition are not
  accepted by this closeout.

## Planning Decision

selected_next_task:
phase12-network-outbound-frame-construction-source-checkpoint-20260619

planningNeeded: false

Rationale: the accepted cached-only resolver creates a deterministic local
neighbor result that can feed the next outbound packet-preparation boundary,
but it deliberately stops before frame construction or transmission. The next
smallest same-slice question is the already queued outbound frame-construction
source checkpoint. That checkpoint is explicit, dependency-gated on this
closeout, and preserves the no-hardware, no-live-I/O strategy boundary. It must
choose a bounded implementation shape or request supervisor planning; this
closeout does not authorize implementation.

## Rejected Claims

- No ARP request emission, retry timer, packet queue, routing table,
  subnet/gateway logic, driver consultation, frame construction, transmit
  scheduling, or live packet I/O was accepted.
- No Pi 5 hardware run, boot archive publication, lab mutation, or
  hardwareTestLock acquisition occurred.
- No RP1 Ethernet driver readiness, DMA descriptor ownership, interrupt
  integration, packet capture, hardware link readiness, network reachability,
  ping behavior, socket API, SSH behavior, UDP/TCP, DHCP, DNS, smoltcp
  adoption, userspace networking API, or phase transition was accepted.

## Acceptance Check

- Closeout records whether the cached-only resolver is accepted, blocked, or
  requires a corrective follow-up: satisfied by the accepted core evidence
  above.
- Closeout preserves rejected claims around ARP request emission, packet
  queues, hardware/live I/O, sockets, SSH, reachability, and phase transition:
  satisfied.
- Because implementation is accepted, closeout selects
  phase12-network-outbound-frame-construction-source-checkpoint-20260619:
  satisfied.

## Validation

- static/source/task evidence review: pass.
- jq empty on task-owned JSON evidence: pass.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check: pass before commit.

## Next Action

The queued
phase12-network-outbound-frame-construction-source-checkpoint-20260619 task is
mechanically unblocked for the next worker wake if dependencies remain
satisfied and git status is clean. Do not promote ARP request emission, packet
queues, driver transmit, live packet I/O, smoltcp adoption, sockets, SSH,
network reachability, ping behavior, hardware-driver readiness, link-readiness
work, Pi 5 hardware work, or any phase transition directly from this closeout.
