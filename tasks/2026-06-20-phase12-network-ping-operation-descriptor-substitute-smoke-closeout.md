# Phase 12.3 Ping Operation Descriptor Substitute Smoke Closeout

Task: phase12-network-ping-operation-descriptor-substitute-smoke-closeout-20260620
Status: accepted
Classification: phase12-network-ping-operation-descriptor-substitute-smoke-closeout-accepted

## Goal

Close out the retained descriptor substitute smoke evidence and decide whether
any later bounded Phase 12.3 task is mechanically unblocked.

## Findings

- fixed: Reconciled the retained descriptor substitute smoke transcript with
  the accepted descriptor-shaped ping operation contract. The smoke evidence is
  accepted because the transcript under
  tasks/evidence/2026-06-20-ping-operation-descriptor-substitute-smoke/ ends
  with 634 passed tests and
  host-substitute-ping-operation-descriptor-smoke-complete.
- fixed: Preserved the exact accepted evidence level as host-only
  QEMU/substitute evidence over NetworkPingOperationDescriptorTable,
  NetworkPingOperationDescriptor, UserspacePingOperation,
  SinglePingPacketService, fake/trait-level NetworkDevice behavior, and
  caller-owned receive/transmit buffers.
- fixed: Confirmed the smoke evidence covers descriptor
  open/start/pump/status/retry/timeout/close, unresolved ARP pending, matching
  ARP advancement to ICMP transmit and in-flight tracking, matching echo-reply
  completion, terminal status observation, retry exhaustion, explicit timeout,
  invalid and closed descriptors, zero-capacity open, duplicate active open,
  transmit IO error, and receive IO error.
- not-an-issue: The smoke command invokes cargo -Zjson-target-spec test
  --quiet network_ping_descriptor, but the current Talos target test runner
  executes the full no_std QEMU/substitute suite for that invocation. This is
  acceptable for retained smoke evidence and does not change the accepted
  boundary.
- deferred: Shell ping, public sockets, syscall ABI acceptance, live driver
  adapters, live packet I/O, hardware reachability, SSH, smoltcp, UDP/TCP, lab
  mutation, boot publication, Phase 12.1 link-hardware retry, and phase
  transition remain future supervisor-planned work.
- deferred: No later queued Phase 12.3 task exists with complete objective
  dependencies, acceptance criteria, validation gates, docs, and evidence
  requirements. Supervisor planning is required before the worker promotes
  further work.

## Evidence

- Descriptor smoke script:
  scripts/qemu-ping-operation-descriptor-smoke.sh.
- Retained descriptor smoke transcript:
  tasks/evidence/2026-06-20-ping-operation-descriptor-substitute-smoke/qemu-ping-operation-descriptor-smoke.log.
- Accepted descriptor smoke core:
  tasks/2026-06-20-phase12-network-ping-operation-descriptor-substitute-smoke-core.md.
- Accepted descriptor smoke core commit:
  08783c931d1e50442cb4aff709e0a9c8be5ae466.
- Accepted descriptor contract source:
  src/network.rs NetworkPingOperationDescriptor and
  NetworkPingOperationDescriptorTable.
- Accepted descriptor contract closeout:
  tasks/2026-06-20-phase12-network-ping-operation-descriptor-contract-closeout.md.

## Validation

- static/source/task/evidence review: pass.
- diff validation: git diff --check.
- docs build: /home/node/.cargo/bin/mdbook build.
- staged diff validation: git diff --cached --check.

## Outcome

Accepted. selected_next_task=null.
planningNeeded=true.

The accepted evidence level remains host-only over the descriptor-shaped ping
operation, UserspacePingOperation, SinglePingPacketService, fake/trait-level
NetworkDevice behavior, and caller-owned buffers. It does not accept shell
ping, public sockets, syscall ABI acceptance, live driver adapters, live
packet I/O, hardware reachability, SSH, smoltcp, UDP/TCP, lab mutation, boot
publication, Phase 12.1 link-hardware retry, or phase transition.

Supervisor planning is required before any next bounded Phase 12.3 feature
task is promoted.

Commit: recorded in talos-supervisor-state.json after commit.
