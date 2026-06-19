# Phase 12.3 Single Ping Caller-Driven Retry/Timeout Core

Task: phase12-network-single-ping-caller-driven-retry-timeout-core-20260619

## Goal

Add deterministic caller-driven retry and timeout/status handling for one
integrated host-only ping transaction, without autonomous timers or packet
queues.

## Scope

- Extend the integrated single-ping transaction boundary so the caller can
  explicitly advance ARP retry attempts while pending and mark pending or
  in-flight transactions timed out through caller-provided policy/state.
- Preserve the accepted explicit ARP retry budget semantics and route-aware
  next-hop/final-destination identity.
- Return deterministic status for idle, pending-ARP, in-flight, completed,
  timed-out, retry-exhausted, and error paths as appropriate for future
  shell/socket integration.
- Cover no-pending/no-inflight, retry budget exhaustion, retry transmit error,
  timeout preservation/clearance, late/nonmatching replies, and completed
  transaction behavior with local/unit tests.

## Non-Goals

- No autonomous timer, scheduler wakeup, background polling loop, packet queue,
  multi-ping table, or dynamic routing.
- No shell ping command, socket API, UDP/TCP, SSH, smoltcp adoption, live driver
  adapter, live packet I/O, hardware run, lab mutation, boot publication,
  reachability claim, or phase transition.
- No Phase 12.1 hardware/link readiness change.

## Implementation

src/network.rs now adds:

- SinglePingTransactionStatus for explicit Idle, PendingArp, and Inflight
  state inspection.
- start_routed_single_ping_transaction_with_arp_retry_budget, preserving the
  existing start helper as the zero-retry-budget wrapper.
- retry_single_ping_transaction_arp_request, a caller-invoked retry wrapper over
  the accepted single-pending ARP retry primitive.
- timeout_single_ping_transaction, which deterministically clears exactly one
  pending or in-flight transaction and reports which state was timed out.

The implementation stays allocation-free and host-only. It does not add
autonomous timers, scheduler wakeups, packet queues, live driver behavior, shell
commands, sockets, hardware evidence, or reachability claims.

## Findings

- fixed: Added integrated transaction status reporting for idle, pending ARP,
  and in-flight states.
- fixed: Added caller-driven integrated pending-ARP retry while preserving final
  destination, next-hop identity, retry budget decrement, retry exhaustion, and
  retry transmit-error preservation.
- fixed: Added caller-driven timeout for pending and in-flight transactions,
  clearing exactly one transaction without reading late frames.
- fixed: Added unit coverage for no-transaction retry/timeout, pending timeout,
  in-flight timeout, completed transaction status, retry exhaustion, retry
  transmit error, and late frame behavior after timeout.
- not-an-issue: Existing integrated transaction tests already covered matching
  echo completion and nonmatching reply preservation; new tests assert the
  accepted status/timeout boundary around those states.
- deferred: shell ping, sockets, live packet I/O, autonomous retry/timeout
  scheduling, packet queues, hardware proof, smoltcp, SSH, and reachability
  remain outside this task.

## Validation Evidence

- fmt/lint/typecheck: cargo fmt --all -- --check
- unit tests: cargo -Zjson-target-spec test --quiet
- diff validation: git diff --check
- docs build: /home/node/.cargo/bin/mdbook build
- staged diff validation before commit: git diff --cached --check

All commands were run from projects/talos with:

    . "$HOME/.cargo/env"
    export TMPDIR=/opt/strider/openclaw/current/workspace/tmp
    export PATH=/opt/strider/openclaw/current/workspace/tools/qemu-9.2.0-install/bin:$PATH

Commit: recorded in talos-supervisor-state.json after acceptance.

## Accepted Boundary

The accepted boundary is a host-only, fake/trait-level NetworkDevice
transaction primitive. A caller can inspect transaction state, explicitly retry
one pending ARP request while budget remains, observe retry exhaustion or retry
transmit errors without losing state, and explicitly timeout one pending or
in-flight transaction. Completed and timed-out transactions return to Idle.

This is not a live ping implementation and does not accept shell ping, sockets,
driver adapters, live packet I/O, hardware behavior, lab mutation, boot
publication, smoltcp adoption, SSH, reachability, or a phase transition.

selected_next_task=phase12-network-single-ping-caller-driven-retry-timeout-closeout-20260619.
