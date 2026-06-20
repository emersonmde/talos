# Phase 12.3 Ping Operation Descriptor Substitute Smoke Core

Task: phase12-network-ping-operation-descriptor-substitute-smoke-core-20260620
Status: accepted
Classification: phase12-network-ping-operation-descriptor-substitute-smoke-core-accepted

## Goal

Retain durable host/QEMU-substitute evidence that the accepted
descriptor-shaped ping operation can drive one complete fake-device
transaction and descriptor lifecycle without accepting shell ping, sockets,
live packet I/O, hardware reachability, SSH, or a phase transition.

## Scope

- Add a task-owned descriptor substitute smoke command.
- Retain evidence for descriptor open/start/pump/status/retry/timeout/close
  over NetworkPingOperationDescriptorTable, UserspacePingOperation,
  SinglePingPacketService, fake NetworkDevice behavior, and caller-owned
  buffers.
- Exercise unresolved ARP pending, ARP reply advancement to ICMP transmit,
  in-flight tracking, echo-reply completion, terminal status observation,
  retry exhaustion, explicit timeout, invalid/closed descriptors, capacity,
  busy, and IO-error mapping through the descriptor-shaped surface.

## Non-Goals

- No shell ping, public sockets, syscall ABI acceptance, UDP/TCP, smoltcp,
  live driver adapter, live packet I/O, hardware reachability, SSH, lab
  mutation, boot publication, autonomous timers, broad queues, Phase 12.1
  link-hardware retry, or phase transition.
- No new protocol behavior beyond exercising the accepted descriptor contract.

## Findings

- fixed: Added scripts/qemu-ping-operation-descriptor-smoke.sh as the
  task-owned host/QEMU-substitute smoke path for the accepted descriptor-shaped
  ping operation contract.
- fixed: Retained the smoke transcript under
  tasks/evidence/2026-06-20-ping-operation-descriptor-substitute-smoke/ with
  descriptor lifecycle, boundary, retry/timeout, descriptor-edge, and IO-edge
  labels.
- fixed: The smoke invokes cargo -Zjson-target-spec test --quiet
  network_ping_descriptor. The current Talos target runner executes the full
  no_std QEMU/substitute suite for that invocation, and the retained
  transcript ends with 634 passed tests plus
  host-substitute-ping-operation-descriptor-smoke-complete.
- fixed: Confirmed the descriptor tests cover unresolved ARP pending, matching
  ARP advancement to ICMP transmit/in-flight, matching echo-reply completion,
  terminal completed status, closed and invalid descriptor EBADF mapping,
  zero-capacity EMFILE, duplicate active open EBUSY, retry exhaustion EAGAIN,
  explicit timeout, transmit IO error, and receive IO error.
- not-an-issue: The substitute remains a target cargo-test smoke through the
  QEMU runner rather than a Pi 5 hardware run. This task explicitly requires
  host/QEMU-substitute evidence and rejects live packet I/O and hardware
  reachability claims.
- deferred: Shell ping, public sockets, syscall ABI, live driver adapters,
  live packet I/O, hardware reachability, SSH, smoltcp, UDP/TCP, lab mutation,
  boot publication, Phase 12.1 link-hardware retry, and phase transition remain
  future supervisor-planned work.

## Evidence

- Smoke script:
  scripts/qemu-ping-operation-descriptor-smoke.sh.
- Retained QEMU/substitute transcript:
  tasks/evidence/2026-06-20-ping-operation-descriptor-substitute-smoke/qemu-ping-operation-descriptor-smoke.log.
- Source contract:
  src/network.rs NetworkPingOperationDescriptor and
  NetworkPingOperationDescriptorTable.
- Prior accepted core:
  tasks/2026-06-20-phase12-network-ping-operation-descriptor-contract-core.md.
- Prior accepted closeout:
  tasks/2026-06-20-phase12-network-ping-operation-descriptor-contract-closeout.md.

## Validation

- fmt/lint: cargo fmt --all -- --check.
- QEMU/substitute smoke:
  scripts/qemu-ping-operation-descriptor-smoke.sh.
- targeted QEMU/substitute unit test:
  cargo -Zjson-target-spec test --quiet network_ping_descriptor.
- diff validation: git diff --check.
- docs build: /home/node/.cargo/bin/mdbook build.
- staged diff validation: git diff --cached --check.

## Outcome

Accepted. selected_next_task=phase12-network-ping-operation-descriptor-substitute-smoke-closeout-20260620.
planningNeeded=false.

The accepted evidence level is host-only: one descriptor-shaped ping operation
over NetworkPingOperationDescriptorTable, UserspacePingOperation,
SinglePingPacketService, fake/trait-level NetworkDevice behavior, and
caller-owned receive/transmit buffers can complete the unresolved-ARP to
echo-reply lifecycle and demonstrate status, retry exhaustion, timeout,
invalid/closed descriptor, capacity, busy, and IO-error edges through a
retained substitute transcript.

Shell ping, public sockets, syscall ABI acceptance, UDP/TCP, smoltcp, live
driver adapters, live packet I/O, hardware reachability, SSH, autonomous
timers, broad queues, lab mutation, boot publication, Phase 12.1
link-hardware retry, and phase transition remain rejected.

Commit: recorded in talos-supervisor-state.json after commit.
