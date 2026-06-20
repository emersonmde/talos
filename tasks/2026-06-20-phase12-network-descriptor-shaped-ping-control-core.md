# Phase 12.4 Descriptor-Shaped Ping Control Core

Task: phase12-network-descriptor-shaped-ping-control-core-20260620

Status: accepted

Classification: phase12-network-descriptor-shaped-ping-control-core-accepted

## Scope

Implement only the accepted descriptor-shaped ping control contract over the
existing RuntimePingOperationSyscallSubstitute and NetworkRuntimeDevicePump.
This task stays crate-internal and host-only. It does not add shell ping,
public sockets, a stable syscall ABI, live driver adapters, live packet I/O,
hardware reachability, SSH, lab mutation, boot publication, or a phase
transition.

## Findings And Dispositions

- fixed: Added DescriptorShapedPingControl as a thin crate-internal control
  wrapper around RuntimePingOperationSyscallSubstitute. open, start, status,
  pump_or_read_result, retry_arp, timeout, and close delegate to the accepted
  runtime-pump-backed path instead of duplicating ARP, IPv4, ICMP, route,
  retry, timeout, local responder, or device error logic.
- fixed: Preserved caller-owned receive/transmit/status storage and
  fixed-capacity state by borrowing the caller-provided NetworkRuntimeDevicePump
  plus caller-owned receive and transmit buffers for the lifetime of the
  control object.
- fixed: Added focused source tests for one successful fake-device lifecycle:
  open, idle status, start to pending ARP, runtime-pump ARP advancement to
  inflight, runtime-pump echo-reply completion, terminal completed status,
  close, and closed-descriptor EBADF.
- fixed: Added focused source tests for deterministic invalid descriptor,
  closed descriptor, zero descriptor capacity, duplicate active open, retry
  exhaustion, explicit timeout, caller receive-buffer pressure, receive IO
  error, local transmit IO error, and active-ping transmit IO error behavior.
- removed: No shell ping command, kernel-backed fake command expansion, public
  socket API, stable syscall ABI acceptance, socket syscall ABI acceptance,
  live driver adapter, live packet I/O, hardware reachability, SSH, smoltcp,
  UDP/TCP, packet queues, autonomous timers, multi-ping behavior, dynamic
  routing, lab mutation, boot publication, Phase 12.1 link-hardware retry,
  broad Phase 12.4 socket expansion, or phase transition is accepted.
- not-an-issue: No ADR is required because the accepted implementation remains
  crate-internal, host-only, explicitly unstable, and proof/control oriented.

## Source Changes

- src/syscall.rs: adds DescriptorShapedPingControl, delegating to
  RuntimePingOperationSyscallSubstitute for descriptor lifecycle, start,
  status, pump/read-result, retry, timeout, and close.
- src/syscall.rs: adds descriptor_shaped_ping_control_* tests covering success,
  descriptor/capacity/busy/timeout/error paths, and caller receive-buffer
  pressure.

## Validation

- cargo fmt --all -- --check: passed.
- cargo -Zjson-target-spec test --quiet: passed, 650 no_std tests.
- git diff --check: passed.
- /home/node/.cargo/bin/mdbook build: passed, existing large search-index
  warning only.
- git diff --cached --check: passed.

No Pi 5 hardware run, hardwareTestLock acquisition, boot archive publication,
lab mutation, power cycle, MDIO/PHY/GPIO32 action, RP1 MMIO/DMA work, shell
ping, public socket API, stable syscall ABI acceptance, live packet I/O, SSH,
or phase transition was performed.

## Acceptance

Accepted.

selected_next_task=phase12-network-descriptor-shaped-ping-control-smoke-20260620.

The accepted evidence level is source/unit test evidence over
DescriptorShapedPingControl delegating to RuntimePingOperationSyscallSubstitute,
NetworkRuntimeDevicePump, local ARP/ICMP responder behavior, active ping
descriptor dispatch, UserspacePingOperation, SinglePingPacketService,
fake/trait-level NetworkDevice behavior, caller-owned buffers, and
fixed-capacity state.

Shell ping, kernel-backed fake command expansion, public sockets, stable
syscall ABI acceptance, socket syscall ABI acceptance, live driver adapters,
live packet I/O, hardware reachability, SSH, smoltcp, UDP/TCP, lab mutation,
boot publication, Phase 12.1 link-hardware retry, broad Phase 12.4 socket
expansion, and phase transition remain rejected.

Commit: recorded in durable supervisor state after commit creation.
