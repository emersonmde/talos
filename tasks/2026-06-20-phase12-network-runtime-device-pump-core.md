# Phase 12.3 Network Runtime Device Pump Core

Task: phase12-network-runtime-device-pump-core-20260620

Status: accepted

Classification: phase12-network-runtime-device-pump-core-accepted

## Scope

Implement the next host-only caller-driven runtime/service pump over
NetworkDevice. The slice is bounded to src/network.rs, caller-owned receive
and transmit buffers, fixed-capacity ARP/descriptor/payload state, local ARP
and ICMP responder behavior, and the accepted ping operation descriptor stack.

## Findings And Dispositions

- fixed: Added NetworkRuntimeDevicePump in src/network.rs. The pump owns a
  LocalNetworkEndpoint, a fixed-capacity local ARP cache, and a fixed-capacity
  NetworkPingOperationDescriptorTable. It performs exactly one caller-driven
  receive step per pump call and never allocates.
- fixed: Added NetworkRuntimeDevicePumpStepResult so the caller can observe
  no-frame, receive-buffer pressure, receive errors, local no-reply, local
  dispatch errors, local transmit errors, local ARP/ICMP replies, active ping
  steps, and active ping POSIX-error mapping.
- fixed: Added pump_received adapters for SinglePingPacketService,
  UserspacePingOperation, and NetworkPingOperationDescriptorTable so a frame
  already received by the runtime pump can advance pending ARP resolution or
  in-flight ICMP echo completion without a second device receive.
- fixed: Runtime pump ordering is deterministic: local ARP/ICMP reply
  generation gets first chance at the received frame, and non-reply traffic is
  offered to the selected active ping descriptor.
- fixed: Tests cover no-frame, nonlocal/no-reply, local ARP reply transmit,
  local ICMP echo reply transmit, active ping ARP-to-ICMP advancement, active
  ping echo-reply completion, receive-buffer pressure, receive error, local
  and active transmit errors, retry, timeout, terminal status observation, and
  ordering when inbound responder and active operation work are both possible.
- removed: No shell ping command, kernel-backed fake command expansion, public
  socket API, stable syscall ABI, socket syscall ABI, live driver adapter,
  live packet I/O, hardware path, lab mutation, boot publication, SSH, smoltcp,
  UDP/TCP, autonomous timer, scheduler wakeup, broad packet queue, or Phase
  12.4 expansion was added.
- deferred: Retained QEMU/substitute smoke evidence for the runtime pump
  remains the dependency-gated follow-up after closeout acceptance.
- not-an-issue: The accepted PingOperationSyscallSubstitute and descriptor
  tests remain source-compatible. This task adds a runtime boundary around the
  existing descriptor stack rather than changing the proof-only syscall
  substitute or stable SVC dispatch.

## Implementation

src/network.rs now exposes NetworkRuntimeDevicePump with these host-only
operations:

- open_ping_operation and close_ping_operation wrap the accepted descriptor
  table lifecycle.
- start_ping, retry_ping_arp, timeout_ping, and ping_status wrap the accepted
  UserspacePingOperation and SinglePingPacketService behavior.
- pump receives one frame into caller-owned storage, attempts local ARP/ICMP
  response generation through dispatch_local_packet_with_arp_cache, transmits
  a local reply when generated, and otherwise offers non-reply frames to the
  selected active descriptor through pump_received.

The new pump_received path reuses the existing pending ARP learning,
outbound ICMP transmit, in-flight echo-reply observation, terminal status, and
POSIX error mapping logic. It avoids duplicating protocol parsing or
performing a second NetworkDevice receive.

## Validation

- cargo fmt --all -- --check: passed.
- targeted QEMU/substitute test/filter:
  cargo -Zjson-target-spec test --quiet network_runtime_device_pump: passed,
  644 no_std tests.
- descriptor/syscall-substitute regression:
  cargo -Zjson-target-spec test --quiet ping_operation_syscall_substitute:
  passed.
- git diff --check: passed.
- docs build: /home/node/.cargo/bin/mdbook build: passed.
- staged diff validation: git diff --cached --check: passed.

No hardware lock, lab mutation, boot publication, live packet I/O, shell ping,
public socket API, stable syscall ABI acceptance, SSH, smoltcp, UDP/TCP, Phase
12.1 retry, Phase 12.4 expansion, or phase transition was performed.

## Acceptance

Accepted. selected_next_task=phase12-network-runtime-device-pump-closeout-20260620.

The accepted evidence level is host-only source/unit/QEMU-substitute over
NetworkRuntimeDevicePump, local ARP/ICMP responder behavior,
NetworkPingOperationDescriptorTable, UserspacePingOperation,
SinglePingPacketService, fake/trait-level NetworkDevice behavior,
caller-owned receive/transmit buffers, and fixed-capacity state.

Shell ping, kernel-backed fake command expansion, public sockets, stable
syscall ABI acceptance, socket syscall ABI, live driver adapters, live packet
I/O, hardware reachability, SSH, smoltcp, UDP/TCP, autonomous timers, broad
packet queues, lab mutation, boot publication, Phase 12.1 link-hardware retry,
Phase 12.4 socket expansion, and phase transition remain rejected.

Commit: recorded in durable supervisor state after commit creation.
