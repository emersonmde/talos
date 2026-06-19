# Phase 12.3 Single-Ping Transaction QEMU/Substitute Smoke Core

Task: phase12-network-single-ping-transaction-qemu-smoke-core-20260619
Status: accepted

## Goal

Add durable QEMU/substitute evidence for the accepted host-only single-ping
transaction lifecycle before growing toward packet queues, sockets, live driver
adapters, smoltcp, or shell-visible ping.

## Scope

- Add the smallest QEMU/substitute smoke path and script that exercises the
  accepted SinglePingTransaction over fake/trait-level NetworkDevice behavior.
- Retain a transcript covering one unresolved route-aware ping lifecycle: ARP
  request emitted and pending, matching ARP reply consumed, ICMP echo request
  transmitted and recorded in-flight, matching echo reply completed, and final
  idle status.
- Cover a caller-driven retry/timeout edge without autonomous timers.

## Non-Goals

- No shell ping command, socket API, user program, UDP/TCP, smoltcp adoption,
  live driver adapter, live packet I/O, hardware run, lab mutation, boot archive
  publication, reachability claim, autonomous timer, scheduler wakeup, packet
  queue, multi-ping table, or phase transition.
- No Phase 12.1 link hardware action or retry.
- No broad network-stack refactor beyond smoke support.

## Implementation

- Added
  qemu_substitute_single_ping_transaction_smoke_covers_lifecycle_and_retry_timeout
  in src/network.rs.
- Added scripts/qemu-single-ping-transaction-smoke.sh to run the smoke as a
  host cargo-test substitute and retain the transcript under
  tasks/evidence/2026-06-19-qemu-single-ping-transaction-smoke/.

## Findings

- fixed: The accepted single-ping transaction now has a named durable smoke
  path that covers pending ARP, ARP advancement, ICMP transmit, in-flight
  recording, matching echo reply completion, and final idle status through
  fake/trait-level NetworkDevice behavior.
- fixed: The smoke includes caller-driven ARP retry budget exhaustion followed
  by explicit timeout, with no autonomous timer or scheduler behavior.
- not-an-issue: The retained evidence is a host substitute rather than a real
  QEMU boot transcript; the task explicitly allows QEMU/substitute evidence and
  the accepted boundary is still host-only.
- deferred: Packet queues, shell ping, sockets, live driver adapters, hardware,
  smoltcp, UDP/TCP, SSH, and reachability remain outside the accepted frontier.

## Evidence

- QEMU/substitute smoke transcript:
  tasks/evidence/2026-06-19-qemu-single-ping-transaction-smoke/qemu-single-ping-transaction-smoke.log
- fmt/lint/typecheck: cargo fmt --all -- --check
- unit tests: cargo -Zjson-target-spec test --quiet
- QEMU/substitute smoke script: ./scripts/qemu-single-ping-transaction-smoke.sh
- diff validation: git diff --check
- docs build: /home/node/.cargo/bin/mdbook build
- staged diff validation before commit: git diff --cached --check

All commands were run from projects/talos with:

    . "$HOME/.cargo/env"
    export TMPDIR=/opt/strider/openclaw/current/workspace/tmp
    export PATH=/opt/strider/openclaw/current/workspace/tools/qemu-9.2.0-install/bin:$PATH

Commit: recorded in talos-supervisor-state.json after acceptance.

## Accepted Boundary

The accepted boundary is host-only single-ping transaction evidence over
caller-owned buffers and fake/trait-level NetworkDevice behavior. It does not
claim live packet I/O, live driver readiness, user-visible ping, sockets,
hardware reachability, smoltcp adoption, SSH, or a phase transition.

selected_next_task=phase12-network-single-ping-transaction-qemu-smoke-closeout-20260619.
