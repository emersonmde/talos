# Phase 12.3 smoltcp Evaluation Source Checkpoint

Task id: phase12-network-device-abstraction-smoltcp-evaluation-source-checkpoint-20260618

Status: accepted

Classification:
phase12-network-device-abstraction-smoltcp-deferred-local-minimal-protocol-next

Evidence level: static source/dependency review and diff checks. No protocol
implementation, dependency addition, Pi 5 hardware run, boot archive
publication, lab mutation, hardwareTestLock acquisition, live packet I/O,
sockets, SSH, or phase transition was performed.

## Goal

Evaluate whether the accepted host/testable network abstraction should next add
smoltcp or keep a small local protocol-layer path, then record the consequence
without broadening into live networking.

## Scope Performed

- Inspected the accepted host-core boundary in src/network.rs and its closeout
  evidence.
- Inspected the current Cargo manifest and metadata; Talos currently has no
  third-party dependencies.
- Checked the current smoltcp crate metadata and feature surface.
- Compared smoltcp integration requirements with the current Phase 12.2 gaps:
  packet dispatch, checksums, fragmentation policy, queue/buffer ownership,
  device polling, and a real driver adapter are still unaccepted.

## Findings

- fixed: the smoltcp-vs-local decision point is now recorded as a source
  checkpoint rather than left as an implicit future direction.
- deferred: adding smoltcp is not selected yet. smoltcp is a credible no_std
  TCP/IP stack, but selecting it now would force dependency and interface
  commitments before Talos has accepted packet queues, device polling, checksum
  policy, or a live driver boundary.
- deferred: TCP/UDP sockets and SSH remain downstream of a real packet path and
  should not be used to justify dependency churn at this frontier.
- not-an-issue: the current local Ethernet/ARP/IPv4 parsers remain useful even
  if smoltcp is later adopted because they document and test the driver/protocol
  boundary and negative packet behavior.
- not-an-issue: no ADR is required in this task because no expensive-to-reverse
  dependency is selected or added.
- removed: no source, docs, dependencies, or task evidence were removed.

## Recommendation

Do not add smoltcp in the next implementation task.

The next implementation should be a local, bounded packet-dispatch and ICMP
echo checkpoint over caller-owned byte slices:

- dispatch EthernetFrame payloads to ARP or IPv4 handlers;
- add IPv4 header checksum validation/generation;
- parse ICMP echo request/reply enough for deterministic unit tests;
- construct ARP reply and ICMP echo reply frames into caller-provided buffers;
- keep all behavior host-testable with no live packet I/O, sockets, SSH,
  driver readiness, or hardware claim.

After Talos has accepted packet dispatch, checksum policy, reusable buffer
ownership, and a driver adapter boundary, smoltcp should be reevaluated with a
concrete integration task and ADR. That later task can decide whether smoltcp
owns ARP/IP/ICMP/UDP/TCP above NetworkDevice or whether Talos keeps a smaller
local stack longer.

## Consequences

- The immediate Phase 12.3 path stays local and source-testable.
- Cargo.toml and Cargo.lock remain unchanged.
- No smoltcp API, feature set, MSRV, dependency tree, or socket model becomes a
  Talos contract yet.
- Supervisor planning is required to add the recommended local packet-dispatch
  implementation task to the queue because the worker must not create new
  queued tasks.

## Evidence

- Accepted host-core source: src/network.rs.
- Host-core task record:
  tasks/2026-06-18-phase12-network-device-abstraction-ethernet-arp-ip-host-core.md.
- Host-core closeout:
  tasks/2026-06-18-phase12-network-device-abstraction-host-core-closeout.md.
- Cargo manifest: Cargo.toml has no dependencies.
- cargo metadata --no-deps --format-version 1: pass with an empty dependency
  list for the talos package.
- cargo search smoltcp --limit 1: smoltcp 0.13.1, described as a TCP/IP stack
  for bare-metal, real-time systems without a heap.
- cargo info smoltcp -v: smoltcp 0.13.1, license 0BSD, rust-version 1.91,
  default features include std/alloc/raw-socket/tuntap/TCP/UDP/DHCP/DNS, and
  no_std-style use requires explicit feature selection.
- Task classification:
  tasks/evidence/2026-06-18-phase12-network-device-abstraction-smoltcp-evaluation-source-checkpoint/classification.json.

## Acceptance Check

- Decision record or task evidence names smoltcp vs local protocol-layer
  recommendation with consequences: satisfied by recommending local minimal
  packet-dispatch/ICMP first and deferring smoltcp.
- Any selected implementation follow-up is concrete and bounded: satisfied as a
  supervisor-planning recommendation, not a promoted worker task.
- No live networking, sockets, SSH, or hardware-driver readiness is claimed:
  satisfied.

## Validation

- static/source/dependency review: pass.
- cargo metadata or equivalent dependency feasibility check: pass with current
  metadata and smoltcp crate metadata review; no dependency added.
- jq empty on task-owned JSON evidence: pass.
- git diff --check: pass.
- mdbook build: not run because docs/src files were not changed.
- git diff --cached --check: pass before commit.

## Next Action

Set planningNeeded=true for supervisor planning. The recommended follow-up is a
new bounded local packet-dispatch and ICMP echo checkpoint; no existing queued
task is mechanically unblocked by this source checkpoint.
