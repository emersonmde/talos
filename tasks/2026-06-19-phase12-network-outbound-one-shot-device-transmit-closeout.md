# Phase 12.3 Outbound One-Shot Device Transmit Closeout

Task id: phase12-network-outbound-one-shot-device-transmit-closeout-20260619
Status: accepted
Owner: worker
Started: 2026-06-19T13:39:00Z
Accepted: 2026-06-19T13:39:00Z

## Goal

Close out the accepted fake/trait-level one-shot outbound transmit boundary and
decide whether the queued Phase 12.3 host frontier checkpoint is mechanically
unblocked.

## Scope

- Review the one-shot transmit implementation, tests, docs, task record, and
  classification evidence.
- Reconcile accepted behavior, rejected claims, deferred work, validation, and
  next task dependencies.
- Select phase12-network-phase12-3-host-frontier-closeout-20260619 only if the
  accepted boundary remains host/mock-only and the checkpoint dependencies are
  objective.

## Non-Goals

- No source implementation beyond evidence/doc corrections for this closeout.
- No packet queue, retry timer, neighbor-discovery state machine, routing,
  live driver transmit, live packet I/O, hardware run, boot publication, lab
  mutation, sockets, SSH, smoltcp adoption, ping/network reachability claim,
  RP1 Ethernet readiness, or phase transition.

## Review

The accepted one-shot transmit core remains a host-testable helper in
src/network.rs. It composes the accepted outbound request selector with the
NetworkDevice trait, builds one frame into caller-owned storage, and calls
NetworkDevice::transmit_frame exactly once after successful construction.
Request-selection/build errors return before any transmit attempt. Transmit
errors report the request kind, frame length, and DeviceError after one
fake-device transmit attempt.

The task record and classification evidence for
phase12-network-outbound-one-shot-device-transmit-core-20260619 record
full-suite tests, docs build, JSON validation, rejected claims, and commit
128c8ab1c9394aced68577ab909d4a26a7402fc7. The docs and roadmap describe the
accepted fake/trait-level transmit frontier without claiming live driver
transmit, hardware, sockets, SSH, or network reachability.

## Findings

- fixed: Reconciled the accepted fake/trait-level one-shot transmit behavior,
  validation, docs, evidence, rejected claims, and deferred work in this
  closeout record.
- fixed: Selected phase12-network-phase12-3-host-frontier-closeout-20260619 as
  the next mechanically unblocked task because all selected Phase 12.3
  host-only implementation tasks through one-shot outbound transmit are
  accepted or closed out, hardwareTestLock is unlocked/restored, and supervisor
  intervention is inactive.
- deferred: packet queues, retry timers, neighbor-discovery state, routing,
  live driver adapters, live packet I/O, sockets, SSH, smoltcp adoption,
  ping/network reachability behavior, Pi 5 hardware work, boot publication,
  lab mutation, link-readiness work, and phase transition remain future work.
- removed: no source APIs, tests, task evidence, or docs were removed by this
  closeout.
- not-an-issue: Selecting the Phase 12.3 host frontier checkpoint does not
  authorize a phase transition or live networking work; the queued checkpoint is
  explicitly static/source/task evidence reconciliation and continues to reject
  hardware, live packet I/O, sockets, SSH, and reachability claims.

## Validation

- static/source/task evidence review:
  - result: pass.
  - reviewed src/network.rs one-shot transmit API/tests,
    tasks/2026-06-19-phase12-network-outbound-one-shot-device-transmit-core.md,
    tasks/evidence/2026-06-19-phase12-network-outbound-one-shot-device-transmit-core/classification.json,
    docs/src/project/phase12-networking-ssh.md, and docs/src/roadmap.md.
- JSON evidence validation:
  jq empty tasks/evidence/2026-06-19-phase12-network-outbound-one-shot-device-transmit-closeout/classification.json
  - result: pass.
- diff whitespace check: git diff --check
  - result: pass.
- docs build: /home/node/.cargo/bin/mdbook build
  - result: pass.
- staged diff whitespace check: git diff --cached --check
  - result: pass before commit.

## Accepted Boundary

The accepted boundary remains fake/trait-level host/mock one-shot transmit for
one outbound IPv4 ICMP echo request. A resolved destination builds one
Ethernet/IPv4/ICMP echo request into caller-owned storage and transmits it once
through the NetworkDevice trait. An unresolved destination builds one
Ethernet/IPv4 ARP request into caller-owned storage and transmits it once
through the NetworkDevice trait. Request build errors perform no transmit
attempt; transmit errors are reported deterministically after one fake-device
attempt. No live driver, queue, retry, live packet I/O, reachability, socket,
SSH, hardware, boot publication, lab mutation, or phase transition claim is
accepted.

## Selected Next Task

phase12-network-phase12-3-host-frontier-closeout-20260619 is selected as the
next mechanically unblocked task, provided dependencies remain satisfied and
git status is clean on the next worker wake.

The selection is narrow: the next task may reconcile the accepted host/testable
receive dispatch, ARP cache learning/resolution, outbound ICMP/ARP
construction, request selection, and one-shot trait-level transmit frontier.
It must still reject queue/retry work, routing, driver adapters, smoltcp,
socket integration, hardware packet I/O, ping/network reachability, SSH,
hardware work, lab mutation, boot publication, and phase transition claims.

## Evidence

- Accepted core commit: 128c8ab1c9394aced68577ab909d4a26a7402fc7.
- Core task record:
  tasks/2026-06-19-phase12-network-outbound-one-shot-device-transmit-core.md.
- Core classification:
  tasks/evidence/2026-06-19-phase12-network-outbound-one-shot-device-transmit-core/classification.json.
- Closeout classification:
  tasks/evidence/2026-06-19-phase12-network-outbound-one-shot-device-transmit-closeout/classification.json.

## Next Action

Promote phase12-network-phase12-3-host-frontier-closeout-20260619 on a later
worker wake if dependencies remain satisfied and git status is clean. Do not
promote packet queues, retry timers, routing, live driver transmit, hardware,
sockets, SSH, smoltcp adoption, ping/network reachability, or any phase
transition directly from this closeout.
