# Phase 12.3 Single-Ping Transaction QEMU/Substitute Smoke Closeout

Task: phase12-network-single-ping-transaction-qemu-smoke-closeout-20260619
Status: accepted

## Goal

Close out the retained QEMU/substitute single-ping transaction smoke evidence
against accepted source/unit tests without overclaiming live networking or
reachability.

## Scope

- Reconcile accepted source/unit tests, the QEMU/substitute smoke transcript,
  task record, docs, and commit evidence.
- Record the exact accepted host-only evidence level and rejected live or
  user-visible networking claims.
- Select the queued host-ping user-boundary strategy checkpoint only if the
  smoke core is accepted with committed evidence.

## Non-Goals

- No implementation work beyond docs/evidence reconciliation.
- No shell ping command, socket API, live driver adapter, hardware run, lab
  mutation, boot publication, smoltcp adoption, SSH, reachability claim, packet
  queue, autonomous timer, UDP/TCP behavior, or phase transition.

## Review

The smoke core is accepted and committed at
c421a5d705d4e78bbb781f7a4623baabab40441b. The retained transcript names the
single-ping transaction lifecycle:

- unresolved ARP pending;
- matching ARP advancement to ICMP transmit;
- in-flight recording;
- matching echo reply completion;
- final idle status;
- caller-driven ARP retry budget exhaustion followed by explicit pending
  timeout.

The source/unit-test evidence remains in src/network.rs through
SinglePingTransaction, SinglePingTransactionStatus,
retry_single_ping_transaction_arp_request, timeout_single_ping_transaction, and
qemu_substitute_single_ping_transaction_smoke_covers_lifecycle_and_retry_timeout.
The retained smoke log is a host cargo-test substitute, not a live QEMU boot or
Pi 5 network run.

## Findings

- fixed: Reconciled the retained smoke transcript with accepted source/unit
  behavior and recorded it as durable host-only feature evidence.
- fixed: Confirmed the smoke is sufficient to plan the next user-boundary
  strategy checkpoint because the integrated lifecycle, caller-driven retry,
  and explicit timeout are all accepted and committed.
- not-an-issue: The smoke log is QEMU/substitute evidence rather than live
  packet I/O; the task is explicitly scoped to fake/trait-level NetworkDevice
  behavior.
- deferred: Shell-visible ping, sockets, live driver adapters, packet queues,
  autonomous timers, UDP/TCP, smoltcp, SSH, hardware reachability, lab mutation,
  boot publication, and phase transition remain future work.

## Evidence

- Source/unit evidence: src/network.rs SinglePingTransaction,
  SinglePingTransactionStatus, retry_single_ping_transaction_arp_request,
  timeout_single_ping_transaction, and
  qemu_substitute_single_ping_transaction_smoke_covers_lifecycle_and_retry_timeout.
- Retained smoke transcript:
  tasks/evidence/2026-06-19-qemu-single-ping-transaction-smoke/qemu-single-ping-transaction-smoke.log
- Smoke core task:
  tasks/2026-06-19-phase12-network-single-ping-transaction-qemu-smoke-core.md
- Smoke core commit: c421a5d705d4e78bbb781f7a4623baabab40441b
- static/source/task/evidence review: pass
- diff validation: git diff --check
- docs build: /home/node/.cargo/bin/mdbook build
- staged diff validation before commit: git diff --cached --check

## Accepted Boundary

The accepted evidence level is QEMU/substitute plus source/unit-test evidence
for one host-only single-ping transaction over caller-owned buffers and
fake/trait-level NetworkDevice behavior. It demonstrates unresolved ARP pending,
matching ARP advancement to ICMP transmit, in-flight recording, matching echo
reply completion, final idle status, and a caller-driven retry/timeout edge.

It does not accept shell ping, sockets, live driver adapters, live packet I/O,
hardware reachability, packet queues, autonomous timers, UDP/TCP, smoltcp, SSH,
lab mutation, boot publication, or a phase transition.

selected_next_task=phase12-network-host-ping-user-boundary-strategy-checkpoint-20260619.
