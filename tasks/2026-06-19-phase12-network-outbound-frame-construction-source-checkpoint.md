# Phase 12.3 Outbound Frame Construction Source Checkpoint

Task id: phase12-network-outbound-frame-construction-source-checkpoint-20260619

Status: accepted

Classification:
phase12-network-outbound-frame-construction-source-checkpoint-accepted-planning-needed

Evidence level: static source/task evidence review, task-owned JSON
classification, and diff checks. No source implementation, dependency change,
Pi 5 hardware run, boot archive publication, lab mutation,
hardwareTestLock acquisition, live packet I/O, driver transmit, socket/SSH
work, ping/network reachability behavior, smoltcp adoption, or phase
transition was performed.

## Goal

Checkpoint the next host-only outbound packet-preparation boundary after the
accepted cached-only neighbor resolver, and decide whether an implementation
task is objective or supervisor planning must pause.

## Reviewed Evidence

- Local packet dispatch and reply construction:
  tasks/2026-06-19-phase12-network-local-packet-dispatch-icmp-echo-core.md.
- Packet-buffer polling:
  tasks/2026-06-19-phase12-network-packet-buffer-device-polling-core.md.
- ARP cache and cache-aware dispatch/poll:
  tasks/2026-06-19-phase12-network-arp-cache-core.md and
  tasks/2026-06-19-phase12-network-arp-cache-dispatch-integration-core.md.
- Cached outbound neighbor resolver:
  tasks/2026-06-19-phase12-network-outbound-neighbor-resolution-core.md.
- Outbound neighbor closeout:
  tasks/2026-06-19-phase12-network-outbound-neighbor-resolution-closeout.md.
- Source implementation surface: src/network.rs.
- Phase 12 docs and roadmap: docs/src/project/phase12-networking-ssh.md and
  docs/src/roadmap.md.

## Source Frontier

The accepted Phase 12.3 host-only source frontier can parse Ethernet frames,
generate caller-buffered ARP replies and ICMP echo replies for local traffic,
poll a NetworkDevice through caller-owned receive/transmit buffers, learn
validated ARP sender facts into a fixed-capacity ArpCache, and resolve a
destination IPv4 to either a cached MacAddress or a deterministic unresolved
result.

The frontier still stops before any outbound frame construction. Existing
reply builders construct frames only as a response to inbound local ARP/ICMP
traffic. The cached neighbor resolver returns a destination MAC but does not
write Ethernet headers, copy payload bytes, choose protocols, consult a
driver, schedule transmit, emit ARP requests, or queue packets.

## Recommendation

The next smallest useful implementation boundary is:

recommended_task_id:
phase12-network-outbound-frame-construction-core-20260619

Recommended scope:

- Add a pure no_std helper that takes a resolved outbound neighbor result, the
  local endpoint MAC, an EtherType, and caller-provided payload bytes.
- Write only an Ethernet II frame into a caller-owned output buffer:
  destination MAC from the resolved neighbor, source MAC from the local
  endpoint, selected EtherType, and an exact payload copy.
- Return a deterministic frame length on success.
- Return deterministic errors for unresolved neighbors and too-small output
  buffers without mutating cache state, consulting a driver, transmitting, or
  queuing.
- Cover host/no_std tests for resolved frame construction, unresolved
  rejection, output-buffer pressure, payload preservation, and compatibility
  with the accepted cached resolver.

This is intentionally below ARP request emission, IPv4/ICMP request
construction, routing/subnet/gateway selection, driver transmit scheduling,
packet queues, live packet I/O, sockets, SSH, smoltcp, ping behavior, hardware
readiness, or phase transition.

## Planning Decision

planningNeeded: true

planningReason:
The checkpoint recommends a concrete bounded implementation task, but no
explicit queued task exists with acceptanceCriteria, validationGates,
docsRequired, evidenceRequired, scope, and nonGoals. Per worker rules, the
worker must not create or promote that implementation task from this wake.
Supervisor planning is required to add it or select a smaller boundary.

selected_next_task: null

## Findings

- fixed: the checkpoint identifies the next smallest outbound packet-preparation
  boundary as caller-buffered Ethernet II frame construction from an already
  resolved neighbor.
- fixed: the recommendation preserves the host-only, allocation-free,
  no-hardware/no-live-I/O strategy boundary.
- deferred: implementation of the outbound frame constructor, ARP request
  emission, retry timers, packet queues, routing/subnet/gateway selection,
  driver transmit scheduling, live packet I/O, sockets, SSH, smoltcp
  integration, ping behavior, and Pi 5 packet movement evidence remain future
  work.
- removed: no source files, docs, dependencies, or prior task evidence were
  removed.
- not-an-issue: no cargo metadata or dependency feasibility check was required
  because no dependency or interface change was proposed or made.
- not-an-issue: no hardware lock, lab mutation, boot publication, or Pi 5
  inconclusive-run triage was required because this is static source/task
  evidence work only.

## Rejected Claims

- No outbound frame-construction implementation was accepted.
- No ARP request emission, retry timer, packet queue, routing table,
  subnet/gateway logic, driver consultation, driver transmit scheduling,
  packet capture, or live packet I/O was accepted.
- No IPv4/ICMP outbound request construction, ping behavior, UDP/TCP, DHCP,
  DNS, socket API, SSH behavior, smoltcp adoption, RP1 Ethernet driver
  readiness, hardware link readiness, network reachability, Pi 5 hardware
  proof, or phase transition was accepted.

## Acceptance Check

- Checkpoint names the next smallest outbound packet-preparation boundary, or
  records why supervisor planning is required: satisfied by the recommendation
  and planningNeeded decision above.
- Checkpoint does not claim ARP request emission, packet queues, driver
  transmit, live packet I/O, sockets, SSH, ping/network reachability, hardware
  readiness, or phase transition: satisfied.
- If a follow-up implementation is recommended, it is concrete, bounded,
  dependency-gated, and preserves the no-hardware/no-live-I/O strategy
  boundary: satisfied as a recommendation only; supervisor planning is required
  before any implementation task can be promoted.

## Validation

- static/source/task evidence review: pass.
- cargo metadata or equivalent: not run; no dependency or interface changes
  were proposed or made.
- jq empty on task-owned JSON evidence: pass.
- git diff --check: pass.
- mdbook build: not run; docs/src files were not changed by this checkpoint.
- git diff --cached --check: pass before commit.

## Next Action

Supervisor planning is required to add
phase12-network-outbound-frame-construction-core-20260619 or select a smaller
same-slice source boundary. Do not promote ARP request emission, packet queues,
driver transmit, live packet I/O, sockets, SSH, network reachability, ping
behavior, hardware readiness, Pi 5 hardware work, smoltcp adoption, or any
phase transition directly from this checkpoint.
