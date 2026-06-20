# Phase 12.3 Ping Operation Descriptor Contract Core

Task: phase12-network-ping-operation-descriptor-contract-core-20260620
Status: accepted
Classification: phase12-network-ping-operation-descriptor-contract-core-accepted

## Goal

Implement the smallest host-only descriptor-shaped contract for the accepted
UserspacePingOperation so a kernel-owned operation can be opened, driven,
observed, and closed through fd-like identity without accepting shell ping,
sockets, live packet I/O, hardware reachability, or a phase transition.

## Findings

- fixed: Added NetworkPingOperationDescriptor and
  NetworkPingOperationDescriptorTable in src/network.rs. The table opens one
  operation descriptor, drives start, pump, retry, timeout, status, and close
  through descriptor identity, and removes closed descriptors deterministically.
- fixed: Kept protocol behavior delegated to UserspacePingOperation and
  SinglePingPacketService. The descriptor layer owns identity/lifecycle only;
  it does not duplicate ARP, IPv4, ICMP, route, retry, timeout, or
  NetworkDevice logic.
- fixed: Mapped invalid and closed descriptors to EBADF, zero-capacity open to
  EMFILE, duplicate active operation open to EBUSY, retry exhaustion to
  EAGAIN, explicit timeout to terminal timed-out status, and receive/transmit
  device errors through the accepted POSIX vocabulary.
- fixed: Added focused network_ping_descriptor unit tests for unresolved ARP
  through echo-reply completion, invalid/closed descriptor lookup, capacity,
  busy, retry/timeout, and IO-error edges.
- deferred: Shell ping, public sockets, syscall ABI, live driver adapters,
  live packet I/O, hardware reachability, SSH, smoltcp, UDP/TCP, lab mutation,
  boot publication, Phase 12.1 link-hardware retry, and phase transition
  remain future supervisor-planned work.

## Evidence

- Source boundary: src/network.rs
  NetworkPingOperationDescriptor and NetworkPingOperationDescriptorTable.
- Tests: src/network.rs network_ping_descriptor_* test cases.
- Accepted prerequisite closeout:
  tasks/2026-06-19-phase12-network-userspace-ping-operation-substitute-smoke-closeout.md.
- Accepted prerequisite commit:
  9f69c09ebb5afd8c508abb6846ffdc707dc6d62a.

## Validation

- fmt/lint: cargo fmt --all -- --check
- targeted unit tests:
  cargo -Zjson-target-spec test --quiet network_ping_descriptor
- full QEMU/substitute unit suite:
  cargo -Zjson-target-spec test --quiet
- diff validation: git diff --check
- docs build: /home/node/.cargo/bin/mdbook build
- staged diff validation: git diff --cached --check

Note: the first full cargo test attempt failed because qemu-system-aarch64 was
not on PATH. The documented Talos PATH setup was applied and the full
QEMU/substitute suite then passed.

## Outcome

Accepted. selected_next_task=phase12-network-ping-operation-descriptor-contract-closeout-20260620.
planningNeeded=false.

The accepted evidence level remains host-only source/unit-test and
QEMU/substitute over a descriptor-shaped operation, UserspacePingOperation,
SinglePingPacketService, fake/trait-level NetworkDevice behavior, and
caller-owned buffers. It does not accept shell ping, public sockets, syscall
ABI, live driver adapters, live packet I/O, hardware reachability, SSH,
smoltcp, UDP/TCP, autonomous timers, broad queues, lab mutation, boot
publication, Phase 12.1 link-hardware retry, or phase transition.

Commit: recorded in talos-supervisor-state.json after commit.
