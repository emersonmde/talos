# Phase 12.3 Userspace Ping Operation Substitute Smoke Core

Task: phase12-network-userspace-ping-operation-substitute-smoke-core-20260619
Status: accepted
Classification: phase12-network-userspace-ping-operation-substitute-smoke-core-accepted

## Goal

Retain durable host/QEMU-substitute evidence that the accepted
userspace/descriptor-facing ping operation can drive one complete fake-device
transaction without accepting shell ping, sockets, live packet I/O, hardware
reachability, SSH, or a phase transition.

## Scope

- Add the narrowest substitute smoke command for UserspacePingOperation.
- Retain task-owned evidence showing unresolved ARP, ARP advancement, ICMP
  transmit/in-flight tracking, matching echo completion, status observation,
  retry exhaustion, timeout, and IO/error edge coverage at the operation
  boundary.
- Keep the evidence host-only over fake/trait-level NetworkDevice behavior,
  SinglePingPacketService, and caller-owned buffers.

## Non-Goals

- No Pi 5 hardware run, lab mutation, boot archive publication, live driver
  adapter, live packet I/O, hardware reachability, shell ping, public sockets,
  UDP/TCP, smoltcp, SSH, autonomous timers, broad packet queues, or phase
  transition.
- No new protocol behavior beyond exercising the accepted contract.

## Findings

- fixed: Added scripts/qemu-userspace-ping-operation-smoke.sh as a named
  host/QEMU-substitute smoke path for the accepted UserspacePingOperation
  contract.
- fixed: The initial smoke run failed before exercising Talos because
  qemu-system-aarch64 was not on PATH. The script now prepends the documented
  local QEMU 9.2.0 tool directory when present, making the smoke command
  reproducible in the OpenClaw workspace.
- fixed: Retained the smoke transcript under
  tasks/evidence/2026-06-19-userspace-ping-operation-substitute-smoke/ with
  the lifecycle, boundary, retry/timeout, and error-edge labels required by
  this task.
- fixed: The named smoke invokes cargo -Zjson-target-spec test --quiet
  userspace_ping_operation, which exercises unresolved ARP pending, matching
  ARP advancement to ICMP transmit, in-flight tracking, matching echo-reply
  completion, terminal completed status, duplicate/active busy mapping,
  caller-driven retry, retry exhaustion, explicit timeout, transmit IO error,
  and receive IO error through fake NetworkDevice behavior. The current Talos
  target runner executes the full no_std QEMU/substitute suite for that
  invocation, and the retained transcript ends with 631 passed tests.
- not-an-issue: The substitute is a target cargo-test smoke through the QEMU
  runner rather than a Pi 5 hardware run. This task explicitly requires
  host/QEMU-substitute evidence and rejects live packet I/O and hardware
  reachability claims.
- deferred: Binding UserspacePingOperation to a real descriptor object,
  syscall ABI, socket API, shell-visible command, live driver adapter, or
  hardware packet path remains future supervisor-planned work.

## Evidence

- Smoke script:
  scripts/qemu-userspace-ping-operation-smoke.sh.
- Retained QEMU/substitute transcript:
  tasks/evidence/2026-06-19-userspace-ping-operation-substitute-smoke/qemu-userspace-ping-operation-smoke.log.
- Source contract:
  src/network.rs UserspacePingOperation, UserspacePingOperationStatus, and
  UserspacePingOperationStep.
- Prior accepted core:
  tasks/2026-06-19-phase12-network-userspace-ping-operation-contract-core.md.
- Prior accepted closeout:
  tasks/2026-06-19-phase12-network-userspace-ping-operation-contract-closeout.md.

## Validation

- QEMU/substitute smoke:
  scripts/qemu-userspace-ping-operation-smoke.sh.
- diff validation: git diff --check.
- docs build: /home/node/.cargo/bin/mdbook build.
- staged diff validation: git diff --cached --check.

## Outcome

Accepted. selected_next_task=phase12-network-userspace-ping-operation-substitute-smoke-closeout-20260619.
planningNeeded=false.

The accepted evidence level is host-only: one UserspacePingOperation over
SinglePingPacketService, fake/trait-level NetworkDevice behavior, and
caller-owned receive/transmit buffers can complete the unresolved-ARP to
echo-reply lifecycle and can demonstrate status, retry exhaustion, timeout,
busy, and IO-error edges through a retained substitute transcript.

Shell ping, public sockets, UDP/TCP, smoltcp, live driver adapters, live
packet I/O, hardware reachability, SSH, autonomous timers, broad queues, lab
mutation, boot publication, Phase 12.1 link-hardware retry, and phase
transition remain rejected.

Commit: recorded in talos-supervisor-state.json after commit.
