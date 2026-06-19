# Phase 12.3 ARP Cache Closeout

Task id: phase12-network-arp-cache-closeout-20260619

Status: accepted

Classification:
phase12-network-arp-cache-closeout-accepted

Evidence level: static source/task/evidence consistency review, task-owned
JSON classification, and diff checks. No Pi 5 hardware run, boot archive
publication, lab mutation, hardwareTestLock acquisition, live packet I/O,
sockets, SSH, smoltcp adoption, RP1 Ethernet driver readiness, link readiness,
network reachability, ping behavior, or phase transition was performed.

## Goal

Close out the accepted ARP-cache core implementation, reconcile its evidence
against the Phase 12.3 source/test boundary, and either select the queued
ARP-cache dispatch-integration source checkpoint or record a concrete blocker.

## Reviewed Evidence

- ARP-cache source checkpoint:
  tasks/2026-06-19-phase12-network-arp-cache-source-checkpoint.md.
- ARP-cache core task:
  tasks/2026-06-19-phase12-network-arp-cache-core.md.
- ARP-cache core classification:
  tasks/evidence/2026-06-19-phase12-network-arp-cache-core/classification.json.
- Source implementation and host/unit tests: src/network.rs.
- Phase 12 project documentation:
  docs/src/project/phase12-networking-ssh.md.
- Roadmap Milestone 12.3 status: docs/src/roadmap.md.
- Accepted core commit:
  616d8d85a1fb4f6415c7ebe77542756513df8671.

## Closeout Result

The ARP-cache core is accepted as a local source/test boundary. The accepted
implementation adds a fixed-capacity, allocation-free ArpCache that stores
IPv4-to-MAC neighbors with deterministic lookup, insertion, existing-entry
update, zero-capacity no-state-change behavior, and oldest-slot round-robin
replacement when the cache is full.

The accepted learning helper records sender IPv4/MAC facts from validated
Ethernet/IPv4 ARP requests and replies. Malformed, truncated, unsupported
EtherType, and unsupported ARP operation inputs return PacketError without
changing cache state.

The implementation intentionally leaves dispatch_local_packet and
poll_local_network_device behavior unchanged. Neighbor state is not yet wired
into outbound resolution, packet queues, driver adapters, live packet I/O, ping
behavior, sockets, SSH, smoltcp adoption, network reachability, link readiness,
or any phase transition.

## Findings

- fixed: Phase 12.3 now has accepted ARP neighbor state as a fixed-capacity,
  allocation-free cache with deterministic update and replacement behavior.
- fixed: ARP sender learning is bounded to valid Ethernet/IPv4 ARP request and
  reply packets and rejects unsupported or malformed input without mutating
  existing state.
- fixed: the core task recorded validation for cargo fmt, the no_std QEMU test
  suite with the Talos QEMU path, task-owned JSON, diff whitespace checks,
  mdbook build, and pre-commit cached diff checks.
- not-an-issue: no docs/src update is required in this closeout because the
  core task already updated the Phase 12 project doc and roadmap with the
  accepted ARP-cache frontier.
- deferred: ARP-cache dispatch integration, outbound neighbor resolution,
  packet queues, driver adapters, UDP/TCP, DHCP, DNS, routing, socket APIs,
  smoltcp adoption, SSH, live packet I/O, and Pi 5 packet movement evidence
  remain future bounded tasks.
- removed: no source, docs, dependencies, prior task evidence, or accepted APIs
  were removed by this closeout.
- rejected: hardware-driver readiness, link readiness, network reachability,
  ping-on-lab behavior, sockets, SSH, smoltcp adoption, live packet I/O, and
  phase transition are not accepted by the ARP-cache core or this closeout.

## Acceptance Check

- Closeout reconciles implementation evidence, findings/dispositions, rejected
  claims, docs/test status, and deferred risks: satisfied.
- If the core is accepted, closeout either selects the queued ARP-cache dispatch
  integration source checkpoint or records a concrete blocker requiring
  supervisor planning: satisfied by selected_next_task below.
- If the core is blocked, closeout preserves blocker details and does not select
  dependent integration work: not applicable because the core is accepted.
- No live packet I/O, hardware-driver readiness, smoltcp adoption, sockets,
  SSH, link readiness, network reachability, ping behavior, or phase transition
  is claimed: satisfied.

## Planning Decision

selected_next_task:
phase12-network-arp-cache-dispatch-integration-source-checkpoint-20260619

planningNeeded: false

Rationale: the queued dispatch-integration source checkpoint is mechanically
unblocked by the accepted ARP-cache core. It is the next smallest same-slice
source/test checkpoint and preserves the no-hardware/no-live-I/O strategy
boundary. The checkpoint must decide whether a later implementation should wire
cached neighbors into local dispatch/outbound resolution, or whether supervisor
planning is required before integration.

## Rejected Claims

- No Pi 5 hardware run, boot archive publication, lab mutation, or
  hardwareTestLock acquisition occurred.
- No live packet I/O, packet capture, RP1 Ethernet adapter, DMA descriptor,
  interrupt, link readiness, driver readiness, network reachability, or
  ping-on-lab behavior was accepted.
- No smoltcp dependency or third-party network stack was adopted.
- No UDP/TCP behavior, DHCP, DNS, routing, socket API, SSH behavior, or phase
  transition was accepted.

## Validation

- static/source/task evidence review: pass.
- jq empty on task-owned JSON evidence: pass.
- git diff --check: pass.
- mdbook build: not run; docs/src files were not touched by this closeout.
- git diff --cached --check: pass before commit.

## Next Action

The queued
phase12-network-arp-cache-dispatch-integration-source-checkpoint-20260619 task
is mechanically unblocked for the next worker wake if dependencies remain
satisfied and git status is clean. Do not promote hardware-driver work, live
packet I/O, smoltcp adoption, sockets, SSH, link-readiness work,
network-reachability work, ping behavior, or any Pi 5 hardware task directly
from this closeout.
