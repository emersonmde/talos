# Phase 12.3 Single-Transaction Packet Service Core

Task: phase12-network-single-transaction-packet-service-core-20260619
Status: accepted
Classification: phase12-network-single-transaction-packet-service-core-accepted

## Goal

Implement the next feature-led host-only network boundary selected by the
host-ping user-boundary checkpoint: a caller-driven packet service/pump that
owns exactly one SinglePingTransaction while continuing to borrow
NetworkDevice and caller-owned packet buffers.

## Scope

- Add a caller-visible Rust API for one host-only packet service/pump over the
  accepted SinglePingTransaction and NetworkDevice contracts.
- Preserve the accepted single transaction lifecycle: unresolved ARP pending,
  matching ARP reply to one ICMP transmit and in-flight record, matching echo
  reply completion, status, retry, and explicit timeout.
- Cover deterministic negative cases for device receive/transmit boundaries,
  malformed/unsupported frames, nonmatching frames, active transaction start,
  retry exhaustion, timeout, and late frames after timeout.
- Keep docs and task/evidence records aligned with the accepted boundary.

## Non-Goals

- No shell ping command, sockets, UDP/TCP, smoltcp, live driver adapter, live
  packet I/O, hardware reachability, autonomous timer, broad packet queue, lab
  mutation, boot publication, SSH, Phase 12.1 link-hardware retry, or phase
  transition claim.

## Findings

- fixed: The accepted single-ping transaction free functions did not yet form
  one caller-owned service/pump boundary. Added SinglePingPacketService, which
  owns exactly one SinglePingTransaction plus a bounded ARP cache and advances
  it through start_ping, pump, retry_arp, timeout, status, and arp_cache
  inspection methods.
- fixed: The API boundary needed deterministic edge evidence rather than only
  happy-path lifecycle evidence. Added unit coverage for no-frame,
  malformed/unsupported frame, nonmatching ARP/reply, receive-buffer pressure,
  receive errors, transmit errors, retry exhaustion, duplicate/active start,
  and late frames after timeout.
- fixed: The retained QEMU/substitute smoke evidence was stale after an
  intermediate failing run. Re-ran scripts/qemu-single-ping-transaction-smoke.sh
  after the fix so the retained task evidence ends with PASS.
- removed: No dead code or obsolete network API surface was removed; the new
  API composes existing accepted functions without replacing their evidence.
- deferred: Multi-transaction queues, autonomous retry/timer scheduling,
  sockets, live driver adapters, and live packet I/O remain deferred to later
  explicitly planned tasks.
- not-an-issue: The retained QEMU/substitute smoke still targets the accepted
  SinglePingTransaction lifecycle. The service/pump is additive and preserves
  that surface, so the existing smoke remains valid after rerun.

## Evidence

- Source/unit: src/network.rs defines SinglePingPacketService and tests
  single_ping_packet_service_owns_one_transaction_lifecycle plus
  single_ping_packet_service_preserves_state_across_edge_cases.
- QEMU/substitute: tasks/evidence/2026-06-19-qemu-single-ping-transaction-smoke/qemu-single-ping-transaction-smoke.log
  ends with PASS classification=host-substitute-single-ping-transaction-smoke-complete.
- Checkpoint dependency:
  tasks/2026-06-19-phase12-network-host-ping-user-boundary-strategy-checkpoint.md
  selected a caller-driven single-transaction packet pump/service boundary and
  rejected fake/kernel-backed shell ping as feature progress.

## Validation

- fmt/lint: cargo fmt --all -- --check
- unit tests: cargo -Zjson-target-spec test --quiet single_ping_packet_service
- unit tests: cargo -Zjson-target-spec test --quiet
- QEMU/substitute: scripts/qemu-single-ping-transaction-smoke.sh
- diff validation: git diff --check
- docs build: /home/node/.cargo/bin/mdbook build
- staged diff validation: git diff --cached --check

## Outcome

Accepted. selected_next_task=phase12-network-single-transaction-packet-service-closeout-20260619.
Commit: recorded in talos-supervisor-state.json after commit.
