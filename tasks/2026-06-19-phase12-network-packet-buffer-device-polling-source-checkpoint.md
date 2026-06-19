# Phase 12.3 Packet Buffer and Device Polling Source Checkpoint

Task id: phase12-network-packet-buffer-device-polling-source-checkpoint-20260619

Status: accepted

Classification:
phase12-network-packet-buffer-device-polling-source-checkpoint-planning-needed

Evidence level: static source/task evidence review and task-owned JSON
classification. No Pi 5 hardware run, boot archive publication, lab mutation,
hardwareTestLock acquisition, live packet I/O, sockets, SSH, smoltcp adoption,
hardware-driver readiness, or phase transition was performed.

## Goal

Checkpoint the next smallest local source/test boundary after accepted packet
dispatch and decide whether an explicit queued follow-up can be selected.

## Reviewed Evidence

- Local packet-dispatch core:
  tasks/2026-06-19-phase12-network-local-packet-dispatch-icmp-echo-core.md.
- Local packet-dispatch closeout:
  tasks/2026-06-19-phase12-network-local-packet-dispatch-icmp-echo-closeout.md.
- Closeout classification:
  tasks/evidence/2026-06-19-phase12-network-local-packet-dispatch-icmp-echo-closeout/classification.json.
- Source boundary and tests: src/network.rs.
- Phase 12 project doc: docs/src/project/phase12-networking-ssh.md.
- Roadmap Milestone 12.3 text: docs/src/roadmap.md.
- Supervisor state taskQueue: no explicit queued implementation task follows
  this checkpoint.

## Accepted Input Frontier

- NetworkDevice currently separates device-owned raw frame movement from
  protocol parsing with receive_frame and transmit_frame over caller-provided
  slices.
- ReceivedFrame preserves immutable received bytes and exposes Ethernet parsing
  without owning buffers.
- dispatch_local_packet accepts immutable Ethernet frame input and a
  caller-provided output buffer, then produces ARP or ICMP echo replies for the
  accepted local shapes.
- The accepted dispatch path has deterministic negative behavior for malformed
  packets, unsupported protocols, nonlocal destinations, and too-small output
  buffers.

## Next Smallest Boundary

The next feature slice should be a local, host-testable packet buffer and
device polling adapter around the existing NetworkDevice and dispatch_local_packet
boundary.

Recommended shape for supervisor planning:

- define a fixed, caller-owned receive buffer and transmit buffer envelope for
  one local poll step;
- call NetworkDevice::receive_frame into the receive buffer;
- pass the received frame to dispatch_local_packet with a configured
  LocalNetworkEndpoint and the transmit buffer;
- call NetworkDevice::transmit_frame only when dispatch_local_packet produced a
  reply;
- return deterministic results for no frame, receive buffer too small, no
  reply, dispatch error, transmit error, and successful reply transmission;
- cover the behavior with host unit tests using a fake NetworkDevice, without
  hardware, allocation, DMA, interrupts, sockets, smoltcp, or live packet I/O.

This boundary would make buffer ownership and a reusable polling loop concrete
before any RP1 adapter, ARP cache, UDP/TCP, socket integration, SSH, or smoltcp
decision.

## Planning Decision

selected_next_task: null

planningNeeded: true

Rationale: the next boundary is clear, but no explicit queued task currently
exists for a packet-buffer/device-polling implementation. The worker must not
create a new task, broaden scope, or infer a phase transition. Supervisor
planning is required to add a concrete dependency-gated implementation task
with acceptance criteria, validation gates, docs requirements, evidence
requirements, scope, and non-goals.

## Findings

- fixed: the checkpoint records the accepted input frontier from NetworkDevice,
  ReceivedFrame, and dispatch_local_packet.
- fixed: the checkpoint names the next smallest source/test boundary as a
  one-step caller-owned packet buffer and device polling adapter.
- deferred: implementation of reusable buffers, poll-step result types, fake
  device tests, driver adapter integration, ARP cache, UDP/TCP, sockets,
  smoltcp, SSH, and live packet I/O remains future work.
- removed: no source, docs, dependencies, or prior task evidence were removed.
- not-an-issue: no cargo metadata or dependency feasibility check was required
  because no dependency or interface change was made in this checkpoint.
- not-an-issue: no hardware lock, lab mutation, boot publication, or Pi 5
  inconclusive-run triage was required because this is static source/task
  evidence work only.

## Rejected Claims

- No reusable packet-buffer implementation was accepted.
- No reusable device polling loop was accepted.
- No RP1 Ethernet driver readiness, DMA descriptor ownership, interrupt
  integration, packet queue, or live packet I/O was accepted.
- No smoltcp dependency or third-party network stack was adopted.
- No ARP cache, UDP/TCP behavior, socket API, SSH behavior, network
  reachability, ping response, hardware link readiness, or phase transition was
  accepted.

## Acceptance Check

- Task evidence names the next smallest local source/test boundary after packet
  dispatch, or records why supervisor planning is required: satisfied.
- No live packet I/O, driver readiness, smoltcp adoption, sockets, SSH, or phase
  transition is claimed: satisfied.
- If a follow-up implementation is selected, it is concrete, bounded, and
  dependency-gated: no follow-up task is selected; planningNeeded is set because
  no explicit queued implementation task exists.

## Validation

- static/source/task evidence review: pass.
- cargo metadata or equivalent: not run; no dependency or interface changes were
  proposed or made.
- jq empty on task-owned JSON evidence: pass.
- git diff --check: pass.
- mdbook build: not run; docs/src files were not changed by this checkpoint.
- git diff --cached --check: pass before commit.

## Next Action

Supervisor planning is required to add a bounded packet-buffer/device-polling
implementation task if this recommendation is accepted. Do not promote
hardware-driver work, live packet I/O, smoltcp adoption, sockets, SSH, RP1
Ethernet readiness, or any phase transition from this checkpoint.
