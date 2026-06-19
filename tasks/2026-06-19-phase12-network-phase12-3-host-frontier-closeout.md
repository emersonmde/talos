# Phase 12.3 Host Frontier Closeout

Task id: phase12-network-phase12-3-host-frontier-closeout-20260619
Status: accepted
Owner: worker
Started: 2026-06-19T13:49:30Z
Accepted: 2026-06-19T13:56:00Z

## Goal

Reconcile the accepted host/testable Phase 12.3 packet dispatch, ARP cache,
outbound request construction, and one-shot trait-level transmit frontier before
any broader network-stack, driver/hardware, socket, SSH, or reachability step.

## Scope

- Review accepted Phase 12.3 host-only source, task records, docs, evidence, and
  rejected claims through the one-shot outbound transmit closeout.
- List remaining gaps for queue/retry, routing/subnet/gateway, driver adapters,
  smoltcp adoption, sockets, hardware packet I/O, ping/network reachability, and
  SSH.
- Select a next bounded task only if it is already explicit and mechanically
  unblocked; otherwise set planningNeeded.

## Non-Goals

- No source implementation.
- No packet queue, retry timer, routing, live driver transmit, live packet I/O,
  RP1 driver adapter, DMA/interrupt work, Pi 5 hardware run, boot publication,
  lab mutation, sockets, SSH, smoltcp adoption, ping/network reachability claim,
  or phase transition.

## Review

The accepted Phase 12.3 host frontier is source/test-only. src/network.rs now
keeps packet movement behind NetworkDevice and accepts local protocol behavior
over caller-owned byte slices:

- inbound local dispatch for ARP requests and IPv4 ICMP echo requests;
- one-step fake/device polling for received local packets and replies;
- fixed-capacity ARP cache lookup, insert/update, oldest-slot replacement, and
  sender learning from valid Ethernet/IPv4 ARP requests and replies;
- cached-only outbound neighbor resolution from immutable ArpCache state;
- caller-buffered Ethernet II frame construction for resolved neighbors;
- caller-buffered Ethernet/IPv4/ICMP echo request construction for resolved
  neighbors;
- caller-buffered Ethernet/IPv4 ARP request construction for unresolved targets;
- immutable-cache outbound request selection between ICMP echo request and ARP
  request construction;
- fake/trait-level one-shot NetworkDevice transmit after successful local frame
  construction.

The accepted evidence remains local/source/test evidence. It does not prove that
any live RP1 driver can receive or transmit frames, that packets leave the Pi 5,
that ARP resolution progresses over a link, that ping/network reachability works,
or that sockets/SSH are available.

## Findings

- fixed: Reconciled the accepted host/testable receive dispatch, ARP cache
  learning/resolution, outbound ICMP/ARP construction, request selection, and
  one-shot trait-level transmit frontier in this checkpoint record.
- fixed: Recorded that all selected Phase 12.3 host-only implementation tasks
  through one-shot outbound transmit are accepted or closed out with committed
  task evidence.
- fixed: Updated the Phase 12 project doc and roadmap to name the checkpoint
  boundary and keep live networking, sockets, SSH, hardware, and phase-transition
  claims rejected.
- deferred: packet queues, retry timers, neighbor-discovery state beyond
  immutable cache lookup/request emission, routing/subnet/gateway policy, driver
  adapters, smoltcp adoption, UDP/TCP, socket integration, hardware packet I/O,
  ping/network reachability behavior, SSH, Pi 5 hardware work, boot publication,
  lab mutation, and phase transition remain future work.
- removed: no source APIs, tests, task evidence, or docs were removed.
- not-an-issue: The existing NetworkDevice trait and fake transmit tests are
  sufficient for this host/testable checkpoint; they are not live driver evidence
  and do not unblock hardware packet I/O by themselves.

## Validation

- static/source/task evidence review:
  - result: pass.
  - reviewed src/network.rs, recent Phase 12.3 task records, task-owned
    classification JSON, docs/src/project/phase12-networking-ssh.md, and
    docs/src/roadmap.md.
- JSON evidence validation:
  jq empty tasks/evidence/2026-06-19-phase12-network-phase12-3-host-frontier-closeout/classification.json
  - result: pass.
- diff whitespace check: git diff --check
  - result: pass.
- docs build: /home/node/.cargo/bin/mdbook build
  - result: pass.
- staged diff whitespace check: git diff --cached --check
  - result: pass before commit.

## Accepted Boundary

The accepted boundary is host/testable Phase 12.3 local packet handling through
one fake/trait-level outbound transmit attempt. The boundary includes local
receive dispatch, ARP cache learning/resolution, caller-buffered outbound ICMP
and ARP frame construction, immutable request selection, and fake NetworkDevice
transmit tests. It remains allocation-free and source/test-only.

## Rejected Claims

- No packet queue, retry timer, neighbor-discovery state machine, routing,
  subnet/gateway selection, asynchronous scheduler integration, live driver
  transmit, live packet I/O, packet capture, ping behavior, network
  reachability, sockets, SSH, UDP/TCP, DHCP, DNS, smoltcp adoption, RP1 driver
  adapter readiness, DMA descriptor ownership, interrupt handling, RP1 Ethernet
  readiness, Pi 5 hardware proof, boot publication, lab mutation, or phase
  transition is accepted.
- No live NetworkDevice implementation is accepted from fake/mock tests.

## Selected Next Task

selected_next_task is null. No later queued task in the current explicit Phase
12.3 host/testable slice has complete scope, non-goals, dependencies,
acceptance criteria, validation gates, docs, and evidence requirements.

planningNeeded=true: supervisor planning is required before promoting any next
task. Candidate directions such as packet queues, retry timers,
neighbor-discovery state, routing/subnet policy, driver adapters, smoltcp,
sockets, hardware packet I/O, ping/network reachability, SSH, or any phase
transition need explicit supervisor decomposition before worker execution.
