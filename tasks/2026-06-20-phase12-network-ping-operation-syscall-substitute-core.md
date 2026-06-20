# Phase 12.3 Ping Operation Syscall Substitute Core

Task: phase12-network-ping-operation-syscall-substitute-core-20260620

Status: accepted

Classification: phase12-network-ping-operation-syscall-substitute-core-accepted

## Scope

Implement the host-only proof adapter accepted by
phase12-network-ping-operation-syscall-substitute-contract-20260620. The slice
is bounded to NetworkPingOperationDescriptorTable, UserspacePingOperation,
SinglePingPacketService, fake/trait-level NetworkDevice behavior, caller-owned
receive/transmit/status buffers, and the accepted POSIX error vocabulary.

## Findings And Dispositions

- fixed: Added PingOperationSyscallSubstitute in src/syscall.rs. The adapter
  borrows the caller-provided NetworkPingOperationDescriptorTable plus
  caller-owned receive and transmit buffers, and routes open/start/pump/status
  /retry_arp/timeout/close through the accepted descriptor table.
- fixed: Added PingOperationSyscallSubstituteStatus and
  PingOperationSyscallSubstituteStep as proof-only scalar-shaped observation
  records. They expose operation state, destination, next-hop, retry count,
  frame length, payload length, and timeout destination without allocating a
  socket table, packet queue, or stable syscall number.
- fixed: Unit evidence covers unresolved ARP through echo-reply completion and
  terminal completed status observation through the adapter.
- fixed: Unit evidence covers invalid descriptor, closed descriptor,
  zero-capacity open, duplicate active open, retry exhaustion, explicit
  timeout and terminal timed-out status, transmit IO error, receive IO error,
  and pump-time transmit IO error through the adapter.
- removed: No direct shell ping command, kernel-backed fake command expansion,
  stable syscall ABI number, socket API, live driver adapter, live packet I/O,
  hardware path, lab mutation, boot publication, SSH, or Phase 12.4 socket
  expansion was added.
- deferred: Retained smoke transcript generation remains the selected follow-up
  task, phase12-network-ping-operation-syscall-substitute-closeout-20260620
  followed by phase12-network-ping-operation-syscall-substitute-smoke-core-20260620
  if closeout accepts this implementation.
- not-an-issue: The existing stable syscall dispatcher still rejects unsupported
  stable calls. This adapter is an explicit host-only proof substitute and does
  not alter SVC routing or userspace-visible ABI acceptance.

## Implementation

src/syscall.rs now exposes a proof-only PingOperationSyscallSubstitute that
borrows:

- NetworkPingOperationDescriptorTable;
- a caller-owned receive buffer;
- a caller-owned transmit buffer;
- a fake/trait-level NetworkDevice supplied per start/pump/retry call.

The adapter methods map directly onto the accepted descriptor table:

- open returns the table descriptor raw id;
- start begins exactly one caller-driven route-aware ping operation;
- pump advances receive/transmit progress using the borrowed buffers;
- status writes a caller-owned status record;
- retry_arp retransmits pending ARP only when the operation is pending;
- timeout explicitly moves pending or in-flight work to timed-out terminal
  state;
- close invalidates the descriptor through the table.

## Validation

- cargo fmt --all -- --check: passed.
- targeted unit test/filter:
  cargo -Zjson-target-spec test --quiet ping_operation_syscall_substitute:
  passed.
- cargo -Zjson-target-spec test --quiet with QEMU 9.2.0 on PATH: passed, 636
  no_std tests.
- git diff --check: passed.

No hardware lock, lab mutation, boot publication, live packet I/O, shell ping,
socket API, SSH, or phase transition was performed.

## Acceptance

Accepted. selected_next_task=phase12-network-ping-operation-syscall-substitute-closeout-20260620.

Commit: recorded in durable supervisor state after commit creation.

The accepted evidence level is source/unit/QEMU-substitute over a host-only
proof adapter. It does not accept shell ping, public sockets, stable syscall
ABI, socket syscall ABI, live driver adapters, live packet I/O, hardware
reachability, SSH, smoltcp, UDP/TCP, lab mutation, boot publication, Phase
12.1 link-hardware retry, or phase transition.
