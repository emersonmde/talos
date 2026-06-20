# Phase 12.4 Descriptor-Shaped Ping Control Contract

Task: phase12-network-descriptor-shaped-ping-control-contract-20260620

Status: accepted

Classification: phase12-network-descriptor-shaped-ping-control-contract-accepted

## Scope

Select the narrow internal descriptor-shaped ping control contract after the
accepted Phase 12.3 host-only runtime-pump-backed ping frontier. This task is
limited to static/source/task/doc review and contract recording; it does not
add runtime behavior, public sockets, stable syscall ABI, shell ping, live
packet I/O, hardware reachability, SSH, or a phase transition.

## Findings And Dispositions

- fixed: Selected RuntimePingOperationSyscallSubstitute as the backing control
  path for the next descriptor-shaped implementation. The contract must borrow
  a caller-provided NetworkRuntimeDevicePump and caller-owned receive/transmit
  buffers, not create a public socket table, autonomous packet queue, live
  driver adapter, or stable syscall dispatch path.
- fixed: Selected one descriptor-shaped ping operation lifecycle:
  open, start, pump/read-result, status, retry_arp, timeout, and close. open
  allocates one runtime ping descriptor; start begins one route-aware ping-like
  operation; pump/read-result runs one runtime pump step so local ARP/ICMP
  responder work keeps priority before active ping progress; status writes a
  caller-owned status record; retry_arp retransmits pending ARP; timeout moves
  pending or inflight work to terminal timed-out state; close invalidates the
  descriptor.
- fixed: Preserved caller-owned receive/transmit/status buffers,
  fixed-capacity state, single-operation scope, explicit terminal status
  observation, and the accepted POSIX-shaped errors: bad or closed descriptors
  are EBADF, zero descriptor capacity is EMFILE, duplicate active open is
  EBUSY, retry exhaustion/no matching active progress is EAGAIN, receive
  buffer pressure is ENOSPC, and device/packet errors use the existing
  internal mapping.
- fixed: Mapped each operation to accepted runtime-pump-backed behavior:
  RuntimePingOperationSyscallSubstitute open/start/status/pump/retry_arp/
  timeout/close over NetworkRuntimeDevicePump, local ARP/ICMP responder
  behavior, active ping descriptor dispatch, UserspacePingOperation,
  SinglePingPacketService, fake/trait-level NetworkDevice behavior, and
  caller-owned buffers.
- removed: No shell ping command, kernel-backed fake command expansion, public
  socket API, stable syscall ABI acceptance, socket syscall ABI acceptance,
  live driver adapter, live packet I/O, hardware reachability, SSH, smoltcp,
  UDP/TCP, lab mutation, boot publication, Phase 12.1 link-hardware retry,
  broad Phase 12.4 socket expansion, or phase transition is accepted.
- deferred: The implementation of this descriptor-shaped control boundary is
  deferred to phase12-network-descriptor-shaped-ping-control-core-20260620.
  Retained smoke evidence remains deferred until the later smoke task.
- not-an-issue: No new ADR is required because this contract remains
  crate-internal, host-only, explicitly unstable, and proof/control oriented;
  it does not lock in a public socket API or stable syscall ABI.

## Selected Contract

The next core task should implement a narrow crate-internal descriptor-shaped
control layer that sits over the accepted RuntimePingOperationSyscallSubstitute.
The contract is deliberately unstable and host-only.

Inputs owned or supplied by the caller:

- a mutable NetworkRuntimeDevicePump;
- a mutable fake/trait-level NetworkDevice per start, pump/read-result, or
  retry operation;
- caller-owned receive and transmit buffers;
- caller-owned status/result records;
- explicit route policy, destination IPv4, identifier, sequence number, TTL,
  payload, and ARP retry budget for start.

Lifecycle and result vocabulary:

- open: allocate one descriptor-shaped ping control handle from the runtime
  pump and fail with EMFILE when descriptor capacity is zero.
- start: bind one descriptor to one ping-like operation and fail with EBADF
  for invalid/closed descriptors, EBUSY for a duplicate active operation, and
  the existing packet/device error mappings for transmit or packet construction
  failures.
- pump/read-result: perform exactly one runtime pump step using the selected
  active descriptor. Local ARP/ICMP replies are observable as local result
  kinds; active ping progress is observable as pending, inflight, completed,
  timed-out, or error state through the existing step/status records.
- status: copy the descriptor's current idle, pending-ARP, inflight,
  completed, or timed-out state into caller-owned storage without consuming
  terminal status.
- retry_arp: retransmit pending ARP through the caller-provided device and
  transmit buffer, preserving EAGAIN for retry exhaustion.
- timeout: move pending or inflight work to terminal timed-out status.
- close: invalidate the descriptor; later status, pump/read-result, retry,
  timeout, or close against that descriptor fails with EBADF.

The selected implementation must not duplicate ARP, IPv4, ICMP, route, retry,
timeout, or local responder logic. It should delegate those semantics to
RuntimePingOperationSyscallSubstitute, NetworkRuntimeDevicePump,
NetworkPingOperationDescriptorTable, UserspacePingOperation, and
SinglePingPacketService.

## Evidence Reviewed

- Source: src/syscall.rs RuntimePingOperationSyscallSubstitute,
  PingOperationSyscallSubstituteStatus, PingOperationSyscallSubstituteStep,
  and POSIX error mapping.
- Source: src/network.rs NetworkRuntimeDevicePump,
  NetworkPingOperationDescriptorTable, UserspacePingOperation, and
  SinglePingPacketService.
- Task: tasks/2026-06-20-phase12-network-host-ping-frontier-checkpoint.md.
- Task: tasks/2026-06-20-phase12-network-runtime-ping-syscall-substitute-core.md.
- Task: tasks/2026-06-20-phase12-network-runtime-ping-syscall-substitute-smoke-closeout.md.
- Durable state: currentTask
  phase12-network-host-ping-frontier-checkpoint-20260620 selected this task as
  the next bounded task.

## Validation

- static/source/task/evidence review: passed.
- git diff --check: passed.
- /home/node/.cargo/bin/mdbook build: passed.
- git diff --cached --check: passed.

No Rust source was touched, so cargo fmt --all -- --check and
cargo -Zjson-target-spec test --quiet were not required by this task's
conditional source gates. No hardware lock, Pi 5 boot, lab mutation, boot
publication, live packet I/O, shell ping, public socket API, stable syscall
ABI acceptance, SSH, smoltcp, UDP/TCP, Phase 12.1 retry, broad Phase 12.4
socket expansion, or phase transition was performed.

## Acceptance

Accepted.

selected_next_task=phase12-network-descriptor-shaped-ping-control-core-20260620.

The accepted evidence level is host-only contract evidence over
RuntimePingOperationSyscallSubstitute, NetworkRuntimeDevicePump, local ARP/ICMP
responder behavior, active ping descriptor dispatch, UserspacePingOperation,
SinglePingPacketService, fake/trait-level NetworkDevice behavior,
caller-owned buffers, fixed-capacity state, and task/doc review.

Shell ping, kernel-backed fake command expansion, public sockets, stable
syscall ABI acceptance, socket syscall ABI acceptance, live driver adapters,
live packet I/O, hardware reachability, SSH, smoltcp, UDP/TCP, lab mutation,
boot publication, Phase 12.1 link-hardware retry, broad Phase 12.4 socket
expansion, and phase transition remain rejected.

Commit: recorded in durable supervisor state after commit creation.
