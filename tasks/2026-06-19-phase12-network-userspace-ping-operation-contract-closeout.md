# Phase 12.3 Userspace Ping Operation Contract Closeout

Task: phase12-network-userspace-ping-operation-contract-closeout-20260619
Status: accepted
Classification: phase12-network-userspace-ping-operation-contract-closeout-accepted

## Goal

Reconcile the accepted userspace/descriptor-facing ping operation contract and
select the next bounded evidence step without accepting shell ping, sockets,
live packet I/O, hardware reachability, SSH, or a phase transition.

## Findings

- fixed: Confirmed phase12-network-userspace-ping-operation-contract-core-20260619
  accepted UserspacePingOperation in src/network.rs as a host-only local
  operation boundary over SinglePingPacketService, caller-owned buffers, and
  fake/trait-level NetworkDevice behavior.
- fixed: Confirmed the boundary exposes deterministic start, pump, status,
  retry, timeout, duplicate/active busy, retry exhaustion, and receive/transmit
  error mapping outcomes through the accepted POSIX error vocabulary.
- fixed: Confirmed the core validation evidence covers unresolved ARP through
  echo-reply completion, terminal status observation, duplicate/active start,
  caller-driven retry, retry exhaustion, explicit timeout, and receive/transmit
  IO error mapping.
- not-an-issue: The operation is not yet bound to a real descriptor object or
  syscall ABI. That is consistent with the accepted task scope because the
  existing descriptor table has only a Socket placeholder and no accepted
  network descriptor/syscall contract.
- deferred: A durable substitute transcript or equivalent task-owned evidence
  should exercise the complete operation through fake NetworkDevice behavior
  before any descriptor/syscall, shell ping, public socket, live driver, or
  hardware packet-I/O task is considered.

## Evidence

- Core task:
  tasks/2026-06-19-phase12-network-userspace-ping-operation-contract-core.md.
- Core commit: 644902ef91d740da86dbc856bb89e31844a8ed7b.
- Source boundary: src/network.rs UserspacePingOperation,
  UserspacePingOperationStatus, and UserspacePingOperationStep.
- Core validation evidence recorded in talos-supervisor-state.json:
  cargo fmt --all -- --check,
  cargo -Zjson-target-spec test --quiet userspace_ping_operation,
  cargo -Zjson-target-spec test --quiet,
  git diff --check,
  /home/node/.cargo/bin/mdbook build,
  git diff --cached --check.

## Validation

- static/source/task/evidence review: pass
- diff validation: git diff --check
- docs build: /home/node/.cargo/bin/mdbook build
- staged diff validation: git diff --cached --check

## Outcome

Accepted. selected_next_task=phase12-network-userspace-ping-operation-substitute-smoke-core-20260619.
planningNeeded=false.

The accepted boundary remains host-only source/unit-test evidence over
SinglePingPacketService, UserspacePingOperation, caller-owned buffers, and
fake/trait-level NetworkDevice behavior. Shell ping, public sockets, live
driver adapters, live packet I/O, hardware reachability, SSH, smoltcp,
UDP/TCP, autonomous timers, broad queues, lab mutation, boot publication, and
phase transition remain rejected.

The selected next task is the queued substitute smoke core because it has an
explicit host-only evidence contract and complete objective dependencies after
this closeout.
