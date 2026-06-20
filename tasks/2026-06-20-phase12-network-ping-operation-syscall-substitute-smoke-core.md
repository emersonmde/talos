# Phase 12.3 Ping Operation Syscall Substitute Smoke Core

Task: phase12-network-ping-operation-syscall-substitute-smoke-core-20260620
Status: accepted
Classification: phase12-network-ping-operation-syscall-substitute-smoke-core-accepted

## Goal

Retain durable host/QEMU-substitute evidence that the accepted
PingOperationSyscallSubstitute adapter can drive one complete fake-device
transaction and adapter lifecycle without accepting shell ping, public sockets,
stable syscall ABI, live packet I/O, hardware reachability, SSH, or a phase
transition.

## Scope

- Add a task-owned syscall-substitute smoke command.
- Retain evidence for adapter open/start/pump/status/retry_arp/timeout/close
  over PingOperationSyscallSubstitute, NetworkPingOperationDescriptorTable,
  UserspacePingOperation, SinglePingPacketService, fake NetworkDevice
  behavior, and caller-owned buffers.
- Exercise unresolved ARP pending, ARP reply advancement to ICMP transmit,
  in-flight tracking, echo-reply completion, terminal status observation,
  retry exhaustion, explicit timeout, invalid/closed descriptors, capacity,
  busy, and IO-error mapping through the proof-only adapter.

## Non-Goals

- No shell ping, kernel-backed fake command expansion, public socket API,
  stable POSIX socket ABI, socket syscall ABI, UDP/TCP, smoltcp, live driver
  adapter, live packet I/O, hardware reachability, SSH, lab mutation, boot
  publication, autonomous timers, broad packet queues, Phase 12.1
  link-hardware retry, Phase 12.4 socket expansion, or phase transition.
- No new protocol behavior beyond exercising the accepted syscall-substitute
  adapter contract.

## Findings

- fixed: Added scripts/qemu-ping-operation-syscall-substitute-smoke.sh as the
  task-owned host/QEMU-substitute smoke path for the accepted proof-only
  ping-operation syscall-substitute adapter.
- fixed: Retained the smoke transcript under
  tasks/evidence/2026-06-20-ping-operation-syscall-substitute-smoke/ with
  adapter lifecycle, boundary, retry/timeout, descriptor-edge, and IO-edge
  labels.
- fixed: The smoke invokes cargo -Zjson-target-spec test --quiet
  ping_operation_syscall_substitute. The current Talos target runner executes
  the full no_std QEMU/substitute suite for that invocation, and the retained
  transcript ends with 636 passed tests plus the
  host-substitute-ping-operation-syscall-substitute-smoke-complete
  classification.
- fixed: Confirmed the adapter tests cover unresolved ARP pending, matching
  ARP advancement to ICMP transmit/in-flight status, matching echo-reply
  completion, terminal completed status, invalid and closed descriptor EBADF
  mapping, zero-capacity EMFILE, duplicate active open EBUSY, retry exhaustion
  EAGAIN, explicit timeout with terminal timed-out status, start-time transmit
  IO error, receive IO error, and pump-time transmit IO error.
- not-an-issue: The smoke command is a target cargo-test smoke through the
  QEMU runner rather than a Pi 5 hardware run. This task explicitly requires
  host/QEMU-substitute evidence and rejects live packet I/O and hardware
  reachability claims.
- deferred: Shell ping, public sockets, stable syscall ABI acceptance, socket
  syscall ABI, live driver adapters, live packet I/O, hardware reachability,
  SSH, smoltcp, UDP/TCP, lab mutation, boot publication, Phase 12.1
  link-hardware retry, Phase 12.4 socket expansion, and phase transition
  remain future supervisor-planned work.

## Evidence

- Smoke script:
  scripts/qemu-ping-operation-syscall-substitute-smoke.sh.
- Retained QEMU/substitute transcript:
  tasks/evidence/2026-06-20-ping-operation-syscall-substitute-smoke/qemu-ping-operation-syscall-substitute-smoke.log.
- Full QEMU/substitute unit-suite transcript:
  tasks/evidence/2026-06-20-ping-operation-syscall-substitute-smoke/cargo-test-full.log.
- Source contract:
  src/syscall.rs PingOperationSyscallSubstitute,
  PingOperationSyscallSubstituteStatus, and
  PingOperationSyscallSubstituteStep.
- Accepted adapter core:
  tasks/2026-06-20-phase12-network-ping-operation-syscall-substitute-core.md.
- Accepted adapter closeout:
  tasks/2026-06-20-phase12-network-ping-operation-syscall-substitute-closeout.md.

## Validation

- QEMU/substitute smoke:
  scripts/qemu-ping-operation-syscall-substitute-smoke.sh.
- full QEMU/substitute unit suite:
  cargo -Zjson-target-spec test --quiet.
- diff validation: git diff --check.
- docs build: /home/node/.cargo/bin/mdbook build.
- staged diff validation: git diff --cached --check.

No Rust source was changed by this task, so cargo fmt was not required by the
conditional gate. No hardware lock, lab mutation, boot publication, live
packet I/O, shell ping, socket API, SSH, or phase transition was performed.

## Outcome

Accepted. selected_next_task=phase12-network-ping-operation-syscall-substitute-smoke-closeout-20260620.
planningNeeded=false.

The accepted evidence level is host-only: one proof-only
PingOperationSyscallSubstitute adapter over NetworkPingOperationDescriptorTable,
UserspacePingOperation, SinglePingPacketService, fake/trait-level
NetworkDevice behavior, and caller-owned receive/transmit/status buffers can
complete the unresolved-ARP to echo-reply lifecycle and demonstrate status,
retry exhaustion, timeout, invalid/closed descriptor, capacity, busy, and
IO-error edges through a retained substitute transcript.

Shell ping, kernel-backed fake command expansion, public sockets, stable
syscall ABI acceptance, socket syscall ABI, UDP/TCP, smoltcp, live driver
adapters, live packet I/O, hardware reachability, SSH, autonomous timers,
broad queues, lab mutation, boot publication, Phase 12.1 link-hardware retry,
Phase 12.4 socket expansion, and phase transition remain rejected.

Commit: recorded in talos-supervisor-state.json after commit.
