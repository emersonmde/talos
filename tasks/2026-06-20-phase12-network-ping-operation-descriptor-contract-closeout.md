# Phase 12.3 Ping Operation Descriptor Contract Closeout

Task: phase12-network-ping-operation-descriptor-contract-closeout-20260620
Status: accepted
Classification: phase12-network-ping-operation-descriptor-contract-closeout-accepted

## Goal

Close out the descriptor-shaped ping operation contract and decide whether
task-owned substitute smoke evidence is mechanically unblocked.

## Findings

- fixed: Reconciled the accepted descriptor contract core with source, tests,
  task record, docs, and commit evidence. The contract is accepted because
  src/network.rs provides NetworkPingOperationDescriptor and
  NetworkPingOperationDescriptorTable, and the core task accepted focused unit
  coverage at commit e91f7ee2b8a576eaaa620afd5193dabe1839808c.
- fixed: Preserved the exact accepted evidence level as host-only source/unit
  and QEMU/substitute evidence over the descriptor-shaped operation,
  UserspacePingOperation, SinglePingPacketService, fake/trait-level
  NetworkDevice behavior, and caller-owned buffers.
- fixed: Confirmed there is no acceptance drift from the core: descriptor
  identity owns open/start/pump/retry/timeout/status/close lifecycle only, and
  protocol behavior remains delegated to UserspacePingOperation and
  SinglePingPacketService.
- fixed: Selected
  phase12-network-ping-operation-descriptor-substitute-smoke-core-20260620 as
  the next mechanically unblocked task because the descriptor contract core is
  accepted and committed, hardwareTestLock is unlocked/restored, supervisor
  intervention is inactive, and the substitute smoke task already has explicit
  scope, non-goals, dependencies, acceptance criteria, validation gates, docs,
  and evidence requirements.
- deferred: Shell ping, public sockets, syscall ABI acceptance, live driver
  adapters, live packet I/O, hardware reachability, SSH, smoltcp, UDP/TCP, lab
  mutation, boot publication, Phase 12.1 link-hardware retry, and phase
  transition remain future supervisor-planned work.

## Evidence

- Descriptor contract source: src/network.rs
  NetworkPingOperationDescriptor and NetworkPingOperationDescriptorTable.
- Descriptor contract tests: src/network.rs network_ping_descriptor_* test
  cases.
- Accepted core task:
  tasks/2026-06-20-phase12-network-ping-operation-descriptor-contract-core.md.
- Accepted core commit: e91f7ee2b8a576eaaa620afd5193dabe1839808c.
- Updated docs: docs/src/project/phase12-networking-ssh.md and
  docs/src/roadmap.md.

## Validation

- static/source/task/evidence review: pass
- diff validation: git diff --check
- docs build: /home/node/.cargo/bin/mdbook build
- staged diff validation: git diff --cached --check

## Outcome

Accepted. selected_next_task=phase12-network-ping-operation-descriptor-substitute-smoke-core-20260620.
planningNeeded=false.

The accepted evidence level remains host-only over the descriptor-shaped ping
operation, UserspacePingOperation, SinglePingPacketService, fake/trait-level
NetworkDevice behavior, and caller-owned buffers. It does not accept shell
ping, public sockets, syscall ABI, live driver adapters, live packet I/O,
hardware reachability, SSH, smoltcp, UDP/TCP, lab mutation, boot publication,
Phase 12.1 link-hardware retry, or phase transition.

Commit: recorded in talos-supervisor-state.json after commit.
