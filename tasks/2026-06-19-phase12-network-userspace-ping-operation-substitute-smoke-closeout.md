# Phase 12.3 Userspace Ping Operation Substitute Smoke Closeout

Task: phase12-network-userspace-ping-operation-substitute-smoke-closeout-20260619
Status: accepted
Classification: phase12-network-userspace-ping-operation-substitute-smoke-closeout-accepted

## Goal

Close out the accepted substitute evidence for the userspace/descriptor-facing
ping operation and require supervisor planning before live driver, socket,
shell, hardware, or Phase 12.4 expansion.

## Findings

- fixed: Confirmed phase12-network-userspace-ping-operation-substitute-smoke-core-20260619
  retained a reproducible QEMU/substitute transcript for the accepted
  UserspacePingOperation boundary.
- fixed: Confirmed the retained smoke evidence covers unresolved ARP pending,
  matching ARP advancement to ICMP transmit, in-flight tracking, matching
  echo-reply completion, terminal status observation, caller-driven retry
  exhaustion, explicit timeout, duplicate/active busy mapping, and
  receive/transmit IO error mapping.
- fixed: Reconciled the docs and roadmap with the exact accepted evidence
  level: host-only behavior over UserspacePingOperation,
  SinglePingPacketService, fake/trait-level NetworkDevice behavior, and
  caller-owned receive/transmit buffers.
- not-an-issue: The smoke script invokes the target cargo-test runner and the
  retained transcript reports the broader no_std QEMU/substitute suite result
  of 631 passed tests. The task-owned PASS line still names the narrower
  host-substitute userspace ping-operation smoke classification.
- deferred: Binding the operation to a real descriptor object, syscall ABI,
  shell-visible command, public socket API, live driver adapter, hardware
  packet path, smoltcp, UDP/TCP, SSH, or any Phase 12.4 transition remains
  future supervisor-planned work.

## Evidence

- Substitute smoke core task:
  tasks/2026-06-19-phase12-network-userspace-ping-operation-substitute-smoke-core.md.
- Substitute smoke core commit: 38c64be8d47560630d78f599f20e05c9693715e7.
- Retained QEMU/substitute transcript:
  tasks/evidence/2026-06-19-userspace-ping-operation-substitute-smoke/qemu-userspace-ping-operation-smoke.log.
- Smoke command:
  scripts/qemu-userspace-ping-operation-smoke.sh.
- Source boundary:
  src/network.rs UserspacePingOperation, UserspacePingOperationStatus, and
  UserspacePingOperationStep.
- Prior accepted contract closeout:
  tasks/2026-06-19-phase12-network-userspace-ping-operation-contract-closeout.md.

## Validation

- static/source/task/evidence review: pass
- diff validation: git diff --check
- docs build: /home/node/.cargo/bin/mdbook build
- staged diff validation: git diff --cached --check

## Outcome

Accepted. selected_next_task=null.
planningNeeded=true.

The accepted evidence level remains host-only: one UserspacePingOperation over
SinglePingPacketService, fake/trait-level NetworkDevice behavior, and
caller-owned receive/transmit buffers can complete the unresolved-ARP to
echo-reply lifecycle and demonstrate status, retry exhaustion, timeout, busy,
and IO-error edges through retained QEMU/substitute evidence.

Live driver adapters, live packet I/O, shell ping, public sockets, UDP/TCP,
smoltcp, hardware reachability, SSH, lab mutation, boot publication,
autonomous timers, broad packet queues, Phase 12.1 link-hardware retry, and
phase transition remain rejected. No later queued Phase 12.3 task currently
has complete objective dependencies, acceptance criteria, validation gates,
docs, and evidence requirements, so supervisor planning is required before
the next bounded feature task.

Commit: recorded in talos-supervisor-state.json after commit.
