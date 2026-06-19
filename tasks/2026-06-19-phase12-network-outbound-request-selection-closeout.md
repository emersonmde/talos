# Phase 12.3 Outbound Request Selection Closeout

Task id: phase12-network-outbound-request-selection-closeout-20260619
Status: accepted
Owner: worker
Started: 2026-06-19T13:10:30Z
Accepted: 2026-06-19T13:18:00Z

## Goal

Close out the accepted host-only outbound request selection boundary and decide
whether the planned one-shot host/mock NetworkDevice transmit wrapper is
mechanically unblocked.

## Scope

- Review the request-selection implementation, tests, docs, task record, and
  classification evidence.
- Reconcile accepted behavior, rejected claims, deferred work, and next task
  dependencies.
- Select phase12-network-outbound-one-shot-device-transmit-core-20260619 only
  if the boundary remains host-only, caller-buffered, and free of live packet
  I/O or reachability claims.

## Non-Goals

- No source implementation beyond evidence/doc corrections for this closeout.
- No packet queue, retry timer, routing/subnet/gateway selection, live packet
  I/O, hardware run, boot publication, lab mutation, sockets, SSH, smoltcp
  adoption, ping/network reachability claim, RP1 Ethernet readiness, or phase
  transition.

## Review

The accepted request-selection core remains a pure host/testable helper in
src/network.rs. It reads immutable ArpCache state, selects a resolved
Ethernet/IPv4/ICMP echo request or unresolved Ethernet/IPv4 ARP request, writes
one frame into caller-owned storage, and returns deterministic kind and frame
length. The implementation does not call NetworkDevice, does not mutate cache
state, and does not introduce packet queues, retry state, live driver I/O, or
reachability behavior.

The task record and classification evidence for
phase12-network-outbound-request-selection-core-20260619 record full-suite
tests, docs build, JSON validation, rejected claims, and commit
ddfe5fc359dfa2a56921f27ff8c4fd2cef3ea027. The docs and roadmap describe the
accepted request-selection frontier without claiming hardware, sockets, SSH, or
network reachability.

## Findings

- fixed: Reconciled the accepted request-selection behavior, validation, docs,
  evidence, rejected claims, and deferred work in this closeout record.
- fixed: Selected phase12-network-outbound-one-shot-device-transmit-core-20260619
  as the next mechanically unblocked task because the accepted selector is
  host-only/caller-buffered, the planned transmit wrapper remains fake/trait
  level, hardwareTestLock is unlocked/restored, and supervisor intervention is
  inactive.
- deferred: packet queues, retry timers, routing/subnet/gateway selection,
  live driver adapters, live packet I/O, sockets, SSH, smoltcp adoption,
  ping/network reachability behavior, Pi 5 hardware work, boot publication,
  lab mutation, link-readiness work, and phase transition remain future work.
- removed: no source APIs, tests, task evidence, or docs were removed by this
  closeout.
- not-an-issue: Selecting the next one-shot transmit wrapper does not accept
  live network transmit because the queued task is explicitly limited to
  host/mock NetworkDevice::transmit_frame tests and rejects live driver,
  hardware, reachability, sockets, SSH, queues, and retries.

## Validation

- static/source/task evidence review:
  - result: pass.
  - reviewed src/network.rs request-selection API/tests,
    tasks/2026-06-19-phase12-network-outbound-request-selection-core.md,
    tasks/evidence/2026-06-19-phase12-network-outbound-request-selection-core/classification.json,
    docs/src/project/phase12-networking-ssh.md, and docs/src/roadmap.md.
- JSON evidence validation:
  jq empty tasks/evidence/2026-06-19-phase12-network-outbound-request-selection-closeout/classification.json
  - result: pass.
- diff whitespace check: git diff --check
  - result: pass.
- docs build: /home/node/.cargo/bin/mdbook build
  - result: pass.
- staged diff whitespace check: git diff --cached --check
  - result: pass before commit.

## Accepted Boundary

The accepted boundary remains host-only selection/construction of one outbound
request frame for a requested IPv4 ICMP echo. A resolved destination selects
caller-buffered Ethernet/IPv4/ICMP echo request construction; an unresolved
destination selects caller-buffered Ethernet/IPv4 ARP request construction. No
transmit, queue, retry, live packet I/O, reachability, socket, SSH, hardware,
boot publication, lab mutation, or phase transition claim is accepted.

## Selected Next Task

phase12-network-outbound-one-shot-device-transmit-core-20260619 is selected as
the next mechanically unblocked task, provided dependencies remain satisfied
and git status is clean on the next worker wake.

The selection is narrow: the next task may build either selected request into
caller-owned storage and invoke a mock/trait-level NetworkDevice transmit once
on successful construction. It must still reject packet queues, retries, live
driver transmit, hardware, sockets, SSH, smoltcp adoption, ping/network
reachability, and phase transition claims.

## Evidence

- Accepted core commit: ddfe5fc359dfa2a56921f27ff8c4fd2cef3ea027.
- Core task record:
  tasks/2026-06-19-phase12-network-outbound-request-selection-core.md.
- Core classification:
  tasks/evidence/2026-06-19-phase12-network-outbound-request-selection-core/classification.json.
- Closeout classification:
  tasks/evidence/2026-06-19-phase12-network-outbound-request-selection-closeout/classification.json.

## Next Action

Promote phase12-network-outbound-one-shot-device-transmit-core-20260619 on a
later worker wake if dependencies remain satisfied and git status is clean. Do
not promote packet queues, retry timers, live driver transmit, hardware,
sockets, SSH, smoltcp adoption, ping/network reachability, or any phase
transition directly from this closeout.
