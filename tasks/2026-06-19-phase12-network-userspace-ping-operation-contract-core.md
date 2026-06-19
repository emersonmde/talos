# Phase 12.3 Userspace Ping Operation Contract Core

Task: phase12-network-userspace-ping-operation-contract-core-20260619
Status: accepted
Classification: phase12-network-userspace-ping-operation-contract-core-accepted

## Goal

Add the smallest host-only userspace/descriptor-facing single-ping operation
contract over the accepted SinglePingPacketService without adding shell ping,
public sockets, live packet I/O, or a fake kernel command path.

## Scope

- Inspect the accepted descriptor/syscall/process and network packet-service
  boundaries.
- Reuse SinglePingPacketService and NetworkDevice instead of duplicating ARP,
  IPv4, ICMP, route, retry, or timeout protocol logic.
- Expose deterministic start, pump, status/result, retry, timeout, busy, and
  error outcomes for one ping-like operation over caller-owned buffers and
  fake/trait-level NetworkDevice behavior.
- Add focused host tests for the new userspace-facing operation contract.

## Non-Goals

- No shell ping command, kernel-only fake command path, public socket API,
  UDP/TCP, smoltcp adoption, live driver adapter, live packet I/O, hardware
  reachability, SSH, lab mutation, boot publication, autonomous timers,
  scheduler wakeups, broad packet queues, multi-ping behavior, dynamic routing,
  Phase 12.1 hardware/link retry, or phase transition.

## Findings

- fixed: Added UserspacePingOperation, UserspacePingOperationStatus, and
  UserspacePingOperationStep in src/network.rs. The operation wraps one
  SinglePingPacketService, exposes start/pump/retry/timeout/status, retains a
  terminal completed/timed-out status for observation, and maps edge outcomes
  to the accepted POSIX error vocabulary.
- fixed: Reused SinglePingPacketService for ARP cache ownership,
  SinglePingTransaction lifecycle, receive/transmit advancement, retry, and
  timeout behavior. The new contract does not duplicate Ethernet, ARP, IPv4,
  ICMP, route, retry, or timeout protocol state.
- fixed: Added userspace_ping_operation_completes_unresolved_arp_to_echo_reply
  to prove unresolved ARP -> ARP reply -> ICMP transmit/in-flight -> echo
  reply completion and terminal status observation at the new boundary.
- fixed: Added
  userspace_ping_operation_maps_busy_retry_timeout_and_io_errors to prove
  duplicate/active start maps to EBUSY, retry advancement is caller-driven,
  retry exhaustion maps to EAGAIN, explicit timeout records terminal status,
  and receive/transmit IO errors map to EIO.
- not-an-issue: The existing descriptor table has only a Socket placeholder and
  no accepted network descriptor/syscall contract. Keeping this as a local
  operation contract avoids accepting sockets or shell ping prematurely.
- deferred: Binding this operation to an actual descriptor object, syscall ABI,
  socket API, live driver adapter, or hardware packet I/O remains future
  supervisor-planned work.

## Evidence

- Source implementation: src/network.rs UserspacePingOperation and helpers.
- Targeted host/unit validation:
  cargo -Zjson-target-spec test --quiet userspace_ping_operation.
- Shared host/unit validation:
  cargo -Zjson-target-spec test --quiet.

## Validation

- fmt/lint: cargo fmt --all -- --check
- targeted unit tests: cargo -Zjson-target-spec test --quiet userspace_ping_operation
- full unit tests: cargo -Zjson-target-spec test --quiet
- diff validation: git diff --check
- docs build: /home/node/.cargo/bin/mdbook build
- staged diff validation: git diff --cached --check

## Outcome

Accepted. selected_next_task=phase12-network-userspace-ping-operation-contract-closeout-20260619.
planningNeeded=false.

The accepted boundary is host-only and source/unit-test backed: one local
userspace/descriptor-facing operation over SinglePingPacketService,
NetworkDevice, caller-owned receive/transmit buffers, and fake/trait-level
device behavior. Shell ping, public sockets, UDP/TCP, smoltcp, live driver
adapters, live packet I/O, hardware reachability, SSH, autonomous timers,
broad queues, lab mutation, boot publication, Phase 12.1 link-hardware retry,
and phase transition remain rejected.

Commit: recorded in talos-supervisor-state.json after commit.
