# Phase 12.3 Runtime Ping Syscall Substitute Core

Task: phase12-network-runtime-ping-syscall-substitute-core-20260620

Status: accepted

Classification: phase12-network-runtime-ping-syscall-substitute-core-accepted

## Scope

Implement the narrowest host-only runtime-pump-backed ping syscall
substitute/control adapter. The slice is bounded to NetworkRuntimeDevicePump,
the accepted ping operation descriptor stack, UserspacePingOperation,
SinglePingPacketService, fake/trait-level NetworkDevice behavior,
caller-owned buffers, and fixed-capacity state.

## Findings And Dispositions

- fixed: Added RuntimePingOperationSyscallSubstitute in src/syscall.rs. The
  adapter borrows a caller-provided NetworkRuntimeDevicePump plus caller-owned
  receive/transmit buffers and routes open/start/status/retry_arp/timeout/close
  through the runtime pump instead of owning a separate descriptor table.
- fixed: Added RuntimePingOperationSyscallSubstitutePumpStep and
  RuntimePingOperationSyscallSubstitutePumpKind so the control boundary can
  observe no-frame, local no-reply, local ARP/ICMP reply, and active ping
  progress without hiding the runtime pump's local-dispatch ordering.
- fixed: Reused the existing PingOperationSyscallSubstituteStatus and
  PingOperationSyscallSubstituteStep records for active ping status and step
  mapping. The runtime-backed adapter therefore preserves the accepted
  descriptor/syscall-substitute status and step vocabulary for ping work while
  adding explicit local-pump outcomes for local responder work.
- fixed: Made the existing crate-local network DeviceError and PacketError to
  PosixError mappers reusable by the syscall substitute layer. The mapping is
  unchanged and remains internal to the crate.
- fixed: Unit/QEMU-substitute evidence covers open/start/status,
  unresolved-ARP advancement to inflight through NetworkRuntimeDevicePump,
  echo-reply completion through active-ping dispatch, local ARP and ICMP reply
  dispatch while a descriptor is open, close and bad-descriptor behavior,
  zero-capacity and duplicate-open behavior, retry exhaustion, explicit
  timeout, receive IO error, local transmit IO error, and active-ping transmit
  IO error.
- removed: No shell ping command, kernel-backed fake command expansion, public
  socket API, stable syscall ABI, socket syscall ABI, UDP/TCP, smoltcp, live
  driver adapter, live packet I/O, hardware path, lab mutation, boot
  publication, SSH, Phase 12.1 retry, Phase 12.4 socket expansion, or phase
  transition was added.
- deferred: Retained smoke evidence for this runtime-backed substitute remains
  dependency-gated behind closeout acceptance.
- not-an-issue: The stable syscall dispatcher continues to reject unsupported
  stable calls. This adapter is a host-only proof/control boundary and does
  not accept a userspace-visible syscall number or ABI.

## Implementation

src/syscall.rs now exposes RuntimePingOperationSyscallSubstitute. It borrows:

- NetworkRuntimeDevicePump;
- a caller-owned receive buffer;
- a caller-owned transmit buffer;
- a fake/trait-level NetworkDevice supplied per start/pump/retry call.

The adapter methods map onto the accepted runtime pump:

- open and close wrap the runtime pump descriptor lifecycle;
- start begins one route-aware ping operation through the runtime pump's
  endpoint and descriptor table;
- pump performs one runtime pump step with the selected active descriptor,
  allowing local ARP/ICMP replies to win before active-ping processing;
- status writes the existing caller-owned ping status record;
- retry_arp and timeout preserve the existing ping operation control surface.

## Validation

- cargo fmt --all -- --check: passed.
- targeted/full no_std unit command:
  cargo -Zjson-target-spec test --quiet runtime_ping_syscall_substitute:
  passed, 647 no_std tests.
- git diff --check: passed.
- docs build: /home/node/.cargo/bin/mdbook build: passed.
- staged diff validation: git diff --cached --check: passed.

No hardware lock, lab mutation, boot publication, live packet I/O, shell ping,
public socket API, stable syscall ABI acceptance, SSH, smoltcp, UDP/TCP, Phase
12.1 retry, Phase 12.4 expansion, or phase transition was performed.

## Acceptance

Accepted. selected_next_task=phase12-network-runtime-ping-syscall-substitute-closeout-20260620.

The accepted evidence level is host-only source/unit/QEMU-substitute over
RuntimePingOperationSyscallSubstitute, NetworkRuntimeDevicePump, local ARP/ICMP
responder behavior, active ping descriptor dispatch, UserspacePingOperation,
SinglePingPacketService, fake/trait-level NetworkDevice behavior, caller-owned
receive/transmit/status buffers, and fixed-capacity state.

Shell ping, kernel-backed fake command expansion, public sockets, stable
syscall ABI acceptance, socket syscall ABI, live driver adapters, live packet
I/O, hardware reachability, SSH, smoltcp, UDP/TCP, lab mutation, boot
publication, Phase 12.1 link-hardware retry, Phase 12.4 socket expansion, and
phase transition remain rejected.

Commit: recorded in durable supervisor state after commit creation.
