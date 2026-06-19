# Phase 12.3 Host Ping Transaction Frontier Closeout

Task id: phase12-network-host-ping-transaction-frontier-closeout-20260619
Status: accepted
Owner: worker
Started: 2026-06-19T18:19:00Z
Accepted: 2026-06-19T18:19:00Z

## Goal

Checkpoint the accepted host-only ping-like transaction frontier after route
selection, ARP resolution, trait-level ICMP transmit, and single-inflight echo
reply observation, without promoting a shell ping command, sockets, live
driver adapter, or hardware/network reachability claim.

## Scope

- Review the accepted host-only Phase 12.3 packet flow from route-aware
  outbound request through ARP resolution and echo reply observation.
- Record what Talos can prove with fake/trait-level NetworkDevice tests and
  what remains unaccepted before live networking or user-facing ping.
- Set planningNeeded=true because no later queued task has complete objective
  dependencies after this checkpoint.

## Non-Goals

- No implementation work.
- No multi-entry packet queue, autonomous retry timer, scheduler wakeup,
  dynamic routing, DHCP, DNS, live driver adapter, live packet I/O, Pi 5
  hardware run, lab mutation, boot publication, smoltcp adoption, sockets,
  SSH, network reachability claim, or phase transition.
- No RP1/BCM54213PE hardware/link readiness change.
- No acceptance of live NetworkDevice implementation from fake/mock tests.
- No shell ping command or socket API.

## Reconciliation

The accepted host-only Phase 12.3 frontier now proves a ping-like transaction
as separate source/testable pieces:

- local receive dispatch can validate Ethernet/ARP/IPv4/ICMP input and build
  local ARP replies and ICMP echo replies into caller-owned buffers;
- ArpCache can learn sender addresses and resolve cached neighbors;
- outbound helpers can construct Ethernet/IPv4/ICMP echo requests and ARP
  requests into caller-owned buffers;
- route-aware request selection can choose same-subnet or gateway next-hop
  behavior while preserving the final IPv4 destination;
- one route-aware pending ICMP echo request can emit ARP requests, learn a
  matching ARP reply, transmit exactly one Ethernet/IPv4/ICMP echo request
  through the NetworkDevice trait, and clear pending only after successful
  transmit;
- one recorded in-flight ICMP echo request can complete only when an inbound
  Ethernet/IPv4/ICMP echo reply matches local endpoint addressing, remote
  source IPv4, identifier, sequence number, payload bytes, and valid IPv4/ICMP
  checksums.

This is still not one integrated user-visible ping path. The accepted pieces
do not automatically wire outbound transmit into in-flight tracking, do not
provide timeout or retry scheduling beyond the accepted caller-driven ARP
retry helper, and do not drive a live network interface.

## Findings

- fixed: The checkpoint records the host-only ping-like transaction frontier
  through route-aware outbound selection, ARP resolution, trait-level ICMP
  transmit, and single-inflight echo reply observation.
- fixed: The checkpoint preserves that the evidence is source/unit-test/task
  evidence over caller-owned buffers and fake/trait-level NetworkDevice
  behavior only.
- deferred: integrated transmit-to-in-flight wiring, shell ping, sockets,
  packet queues, timeout/retry scheduling, live driver adapters, smoltcp
  adoption, SSH, reachability, hardware, lab mutation, boot publication, and
  phase transition remain supervisor-planned future work.
- removed: no source, docs, or task evidence were removed.
- not-an-issue: Fake/trait-level NetworkDevice tests are sufficient for this
  checkpoint's host-only frontier, but they remain insufficient for accepting
  live NetworkDevice, link readiness, hardware packet I/O, or network
  reachability.

## Validation

- static/source/task evidence review:
  - src/network.rs route-aware ICMP selection, pending ARP reply poll, and
    single-inflight ICMP echo reply observation helpers and tests.
  - tasks/2026-06-19-phase12-network-pending-aware-arp-reply-poll-core.md.
  - tasks/2026-06-19-phase12-network-pending-aware-arp-reply-poll-closeout.md.
  - tasks/2026-06-19-phase12-network-single-inflight-icmp-echo-reply-observation-core.md.
  - tasks/2026-06-19-phase12-network-single-inflight-icmp-echo-reply-observation-closeout.md.
- jq evidence validation:
  - result: not applicable; this closeout created no task-owned JSON evidence.
- diff whitespace check:
  - git diff --check
  - result: pass.
- docs build:
  - /home/node/.cargo/bin/mdbook build
  - result: pass.
- staged diff whitespace check:
  - git diff --cached --check
  - result: pass before commit.

## Accepted Boundary

The accepted boundary is a host/testable ping-like transaction frontier:
Talos can select and route one ICMP echo request, resolve a next-hop neighbor
with ARP, transmit one ICMP echo request through the NetworkDevice trait after
matching ARP, and observe one matching ICMP echo reply for a separately
recorded in-flight request. This boundary is not a shell ping command, socket
API, live driver adapter, or network reachability proof.

## Rejected Claims

- No live packet I/O, driver adapter, interrupt loop, packet queue,
  autonomous polling/timer, timeout scheduler, shell ping command, socket,
  SSH, UDP/TCP, smoltcp adoption, network reachability, Pi 5 hardware proof,
  boot publication, lab mutation, or phase transition is accepted.
- No live NetworkDevice implementation is accepted from fake/mock tests.
- No automatic integration from successful outbound transmit to in-flight echo
  reply tracking is accepted by this checkpoint.

## Evidence

- Pending-aware ARP reply poll core commit:
  6aed60c46fbb627ee82ea7a9fafd63f6a7d9d3f4.
- Pending-aware ARP reply poll closeout commit:
  c1a2a302ecedf1d9872651971faca8d4f2acb2d4.
- Single-inflight ICMP echo reply observation core commit:
  a7e4ca05e60d90119a47be797f43b06e4f8037d7.
- Single-inflight ICMP echo reply observation closeout commit:
  69dc1f48a659f1c468c1e23afa25652ae78a3a6f.
- Source/test evidence: src/network.rs route-aware pending ICMP, pending ARP
  reply poll, and single-inflight ICMP echo reply observation tests.

## Next Action

selected_next_task=null.
Set planningNeeded=true after this checkpoint. Supervisor planning is required
before any integrated ping transaction task, shell command, socket API, live
driver adapter, hardware run, smoltcp adoption, SSH, reachability claim, lab
mutation, boot publication, or phase transition.
