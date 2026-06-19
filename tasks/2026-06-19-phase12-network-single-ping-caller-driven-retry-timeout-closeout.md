# Phase 12.3 Single Ping Caller-Driven Retry/Timeout Closeout

Task id: phase12-network-single-ping-caller-driven-retry-timeout-closeout-20260619
Status: accepted
Owner: worker
Started: 2026-06-19T19:50:00Z
Accepted: 2026-06-19T19:58:00Z

## Goal

Close out the accepted host-only integrated ping transaction plus
caller-driven retry/timeout evidence and decide whether any bounded follow-up
is objectively unblocked.

## Scope

- Reconcile the accepted integrated single-ping transaction and caller-driven
  retry/timeout source, unit-test, task, docs, and commit evidence.
- Record what the current host-only transaction boundary proves and what
  remains unaccepted before shell ping, sockets, live packet I/O, or
  reachability claims.
- Select a next task only if a bounded follow-up has objective dependencies and
  complete acceptance gates already present.

## Non-Goals

- No implementation work beyond docs/evidence reconciliation.
- No shell ping command, socket API, live driver adapter, hardware run, lab
  mutation, boot publication, smoltcp adoption, SSH, reachability claim, or
  phase transition.
- No acceptance of autonomous timers, scheduler wakeups, background polling,
  packet queues, multi-ping behavior, dynamic routing, or UDP/TCP behavior.

## Reconciliation

The accepted integrated single-ping transaction core and closeout committed one
host-only transaction coordinator over caller-owned buffers and fake/trait-level
NetworkDevice behavior. The accepted caller-driven retry/timeout core then
extended that transaction with explicit status inspection, an ARP retry-budget
start wrapper, caller-invoked pending-ARP retry, and deterministic timeout for
exactly one pending or in-flight transaction.

The reconciled boundary is deterministic, single-entry, and allocation-free. A
caller can start one route-aware ICMP echo request, either transmit immediately
for a resolved next hop or emit one ARP request for an unresolved next hop, keep
route-aware pending state, retry that pending ARP request while budget remains,
advance a matching ARP reply to one trait-level ICMP echo request transmit,
record one in-flight request after successful transmit, complete on a matching
echo reply, and timeout pending or in-flight state explicitly.

The accepted boundary is still host/unit-test evidence only. It proves the
kernel-internal state-machine and fake/trait-level transmit/receive contract for
one transaction. It does not prove live packet I/O, live driver adapters, user
visible ping behavior, sockets, UDP/TCP, SSH, smoltcp integration, hardware
behavior, network reachability, lab behavior, boot publication, or a phase
transition.

No later queued task in the current durable queue has complete objective
dependencies and explicit acceptance gates beyond this closeout. The safe
closeout result is selected_next_task=null and planningNeeded=true so the
supervisor can decompose any next host-only slice explicitly.

## Findings

- fixed: Reconciled integrated single-ping transaction and caller-driven
  retry/timeout source, tests, task records, docs, and commit evidence.
- fixed: Recorded the accepted host-only frontier as one deterministic
  transaction with explicit status, caller-driven pending-ARP retry, matching
  ARP advancement, matching echo completion, retry exhaustion/error reporting,
  and explicit pending/in-flight timeout.
- deferred: shell ping, sockets, live driver adapters, live packet I/O,
  packet queues, autonomous timers, scheduler wakeups, multi-ping behavior,
  UDP/TCP, smoltcp, SSH, hardware proof, reachability, lab mutation, boot
  publication, and phase transition remain future supervisor-planned work.
- removed: no source, docs, or task evidence was removed.
- not-an-issue: fake/trait-level NetworkDevice evidence is sufficient for this
  closeout's host-only reconciliation boundary, but it remains insufficient for
  live networking or reachability claims.

## Validation

- static/source/task evidence review:
  - reviewed src/network.rs SinglePingTransaction,
    SinglePingTransactionStatus,
    start_routed_single_ping_transaction_with_arp_retry_budget,
    retry_single_ping_transaction_arp_request,
    timeout_single_ping_transaction, the integrated/caller-driven unit tests,
    tasks/2026-06-19-phase12-network-integrated-single-ping-transaction-closeout.md,
    and tasks/2026-06-19-phase12-network-single-ping-caller-driven-retry-timeout-core.md.
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

Phase 12.3 now accepts a host-only, fake/trait-level single-ping transaction
frontier with deterministic start, unresolved ARP pending state, explicit ARP
retry budget/retry, matching ARP advancement to one ICMP echo request transmit,
single in-flight echo reply completion, state inspection, retry exhaustion and
transmit-error reporting, and explicit pending/in-flight timeout.

## Rejected Claims

- No live packet I/O, live NetworkDevice adapter, interrupt loop, packet queue,
  autonomous retry/timer scheduling, shell ping command, socket API, SSH,
  UDP/TCP, smoltcp adoption, network reachability, Pi 5 hardware proof, boot
  publication, lab mutation, or phase transition is accepted.
- No user-visible ping behavior is accepted by this closeout.

## Evidence

- Integrated transaction closeout:
  tasks/2026-06-19-phase12-network-integrated-single-ping-transaction-closeout.md.
- Caller-driven retry/timeout core task:
  tasks/2026-06-19-phase12-network-single-ping-caller-driven-retry-timeout-core.md.
- Integrated transaction core commit:
  b0a7143c34b61eaa7e3cadfd3f3b01513736eb93.
- Caller-driven retry/timeout core commit:
  8695dba394d09f6b104cc690ba9338df9a1750e6.
- Source/test evidence: src/network.rs SinglePingTransaction,
  SinglePingTransactionStatus,
  start_routed_single_ping_transaction_with_arp_retry_budget,
  retry_single_ping_transaction_arp_request,
  timeout_single_ping_transaction, and the integrated/caller-driven unit tests.
- Validation evidence: final command results in this task record after
  acceptance.
- Commit evidence: recorded in durable supervisor state after commit.

## Next Action

selected_next_task=null.
planningNeeded=true.
Supervisor planning is required before any next host-only networking slice,
shell command, socket API, live driver adapter, hardware run, smoltcp adoption,
SSH, reachability claim, lab mutation, boot publication, or phase transition.
