# Phase 12.3 Integrated Single Ping Transaction Closeout

Task id: phase12-network-integrated-single-ping-transaction-closeout-20260619
Status: accepted
Owner: worker
Started: 2026-06-19T19:20:12Z
Accepted: 2026-06-19T19:23:00Z

## Goal

Close out the integrated host-only single-ping transaction core and decide
whether the caller-driven retry/timeout slice is objectively unblocked.

## Scope

- Reconcile accepted source/tests/task evidence for the integrated single-ping
  transaction coordinator.
- Record what the integrated host-only lifecycle proves and what remains
  unaccepted before shell ping, sockets, live packet I/O, or reachability.
- Select phase12-network-single-ping-caller-driven-retry-timeout-core-20260619
  only if the integrated core accepted deterministic pending/in-flight state
  that can be advanced by caller-driven retry/timeout without autonomous timers.

## Non-Goals

- No implementation work beyond docs/evidence reconciliation.
- No shell ping command, socket API, live driver adapter, hardware run, lab
  mutation, boot publication, smoltcp adoption, SSH, reachability claim, or
  phase transition.
- No acceptance of autonomous timers, packet queues, scheduler wakeups, or
  multi-ping behavior.

## Reconciliation

The accepted core task
phase12-network-integrated-single-ping-transaction-core-20260619 committed
src/network.rs SinglePingTransaction,
start_routed_single_ping_transaction, poll_single_ping_transaction, and
integrated_single_ping_* tests at
b0a7143c34b61eaa7e3cadfd3f3b01513736eb93.

The reconciled boundary is host/testable and allocation-free. A caller can
start one route-aware ICMP echo request. If the next hop is already resolved,
the coordinator emits exactly one Ethernet/IPv4/ICMP echo request through
NetworkDevice and records one in-flight request only after successful transmit.
If the next hop is unresolved, the coordinator emits exactly one ARP request
and stores one pending route-aware request. A later caller-driven poll can
learn a matching ARP reply, transmit exactly one ICMP echo request, clear
pending, and record in-flight only after successful transmit. A matching echo
reply completes the transaction and clears the in-flight record.

The accepted state machine is sufficient to unblock the next caller-driven
retry/timeout slice because pending and in-flight ownership are explicit,
single-entry, and caller-advanced. No autonomous scheduler, timer interrupt,
packet queue, live receive loop, or multi-ping table is required for that next
task.

## Findings

- fixed: Reconciled integrated single-ping source, unit-test, task, docs, and
  commit evidence.
- fixed: Confirmed the accepted coordinator has deterministic single pending
  and single in-flight state suitable for a caller-driven retry/timeout follow-up.
- deferred: caller-driven retry/timeout mechanics remain the selected next
  implementation task; shell ping, sockets, live driver adapters, smoltcp
  adoption, SSH, reachability, hardware, lab mutation, boot publication, and
  phase transition remain future work.
- removed: no source, docs, or task evidence was removed.
- not-an-issue: fake/trait-level NetworkDevice tests are sufficient for this
  closeout's host-only evidence boundary but do not prove live packet I/O or
  network reachability.

## Validation

- static/source/task evidence review:
  - reviewed src/network.rs SinglePingTransaction,
    start_routed_single_ping_transaction, poll_single_ping_transaction, the
    integrated_single_ping_* tests, and
    tasks/2026-06-19-phase12-network-integrated-single-ping-transaction-core.md.
  - result: pass.
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

Phase 12.3 now accepts one integrated host-only single-ping transaction
coordinator over caller-owned buffers and fake/trait-level NetworkDevice
behavior. The accepted boundary covers resolved-route start, unresolved-route
ARP request and pending storage, matching ARP reply advancement to one ICMP
echo request transmit and in-flight record, and matching echo reply completion.

## Rejected Claims

- No live packet I/O, live NetworkDevice adapter, interrupt loop, packet queue,
  autonomous retry/timer scheduling, shell ping command, socket API, SSH,
  UDP/TCP, smoltcp adoption, network reachability, Pi 5 hardware proof, boot
  publication, lab mutation, or phase transition is accepted.
- No user-visible ping behavior is accepted by this closeout.

## Evidence

- Core task record:
  tasks/2026-06-19-phase12-network-integrated-single-ping-transaction-core.md.
- Core commit: b0a7143c34b61eaa7e3cadfd3f3b01513736eb93.
- Source/test evidence: src/network.rs SinglePingTransaction,
  start_routed_single_ping_transaction, poll_single_ping_transaction, and
  integrated_single_ping_* tests.
- Validation evidence: final command results in this task record after
  acceptance.
- Commit evidence: recorded in durable supervisor state after commit.

## Next Action

selected_next_task=phase12-network-single-ping-caller-driven-retry-timeout-core-20260619.
The caller-driven retry/timeout core is mechanically unblocked after this
closeout is accepted and committed. Do not promote shell ping, sockets, live
driver, hardware, smoltcp, SSH, reachability, lab mutation, boot publication,
or phase transition directly from this closeout.
