# Phase 12.4 Driver Packet Pump Core

Task: phase12-network-driver-packet-pump-core-20260620

Status: accepted

Classification: phase12-network-driver-packet-pump-core-accepted

## Scope

Implement the host-only crate-internal packet pump selected by
phase12-network-driver-packet-pump-contract-20260620. The pump drains accepted
diagnostic outbound packet queue records to trait-level NetworkDevice transmit
behavior and polls trait-level NetworkDevice receive behavior back into
diagnostic inbound packet records.

This task does not accept live packet I/O, public sockets, stable syscall ABI,
socket syscall ABI, shell ping, hardware reachability, SSH, lab mutation, boot
publication, broad socket expansion, or a phase transition.

## Findings And Dispositions

- fixed: Added PacketQueueDriverPumpStep in src/network.rs as the deterministic
  result vocabulary for one host-only queue-to-driver pump step.
- fixed: Added PacketQueueNetworkDevice::pump_driver. It drains one outbound
  packet record in FIFO order before polling receive, calls
  NetworkDevice::transmit_frame on the trait-level driver, and removes the
  outbound record only after transmit succeeds.
- fixed: Receive backpressure is checked before polling the driver, so a full
  diagnostic receive queue does not consume a driver-owned frame.
- fixed: Receive polling uses caller-owned scratch. WouldBlock leaves state
  unchanged, BufferTooSmall is reported separately, and oversized received
  frames map to ReceiveFrameTooLarge.
- fixed: Source/unit coverage now proves transmit ordering, transmit retry
  preservation after device errors, receive queue backpressure, caller
  receive-buffer pressure, oversized frames, receive device errors, and no-frame
  behavior.
- fixed: Updated the VFS-backed ping diagnostic packet queue lifecycle test so
  outbound ARP and ICMP records cross the packet pump into a trait-level driver
  queue, and injected ARP/ICMP replies cross back through the pump before the
  process-local descriptor pump observes them.
- fixed: Existing accepted tests continue to cover malformed received frames,
  timeout/retry, invalid and closed descriptors, wrong or missing owner,
  process descriptor capacity, user-memory faults, scratch pressure, and
  unchanged SyscallNumber/STABLE_SVC_IMMEDIATE/TALOS_* vocabulary.
- removed: No direct queue pop/inject shortcut remains in the primary VFS
  packet queue lifecycle proof; the test now routes packet movement through
  pump_driver.
- deferred: Retained smoke transcript capture and closeout remain deferred to
  explicit follow-up tasks.
- not-an-issue: The pump is named driver-facing while still host-only because
  the external side is only the NetworkDevice trait. No RP1/GEM/MACB MMIO, DMA
  descriptor ownership, interrupt completion, hardware reachability, socket
  surface, or live packet path is accepted.

## Implementation

- src/network.rs:
  - PacketQueueDriverPumpStep records Transmitted, Received, NoFrame,
    ReceiveQueueFull, ReceiveFrameTooLarge, ReceiveBufferTooSmall,
    ReceiveError, and TransmitError outcomes.
  - FixedPacketQueue exposes is_full for pre-receive backpressure.
  - PacketQueueNetworkDevice::pump_driver moves one deterministic packet pump
    step between diagnostic queues and any NetworkDevice implementation.
- src/syscall.rs:
  - vfs_ping_diagnostic_svc_fixture_records_packet_queue_lifecycle now uses a
    second PacketQueueNetworkDevice as the trait-level driver side of the pump.
    ARP request, ARP reply, ICMP echo request, and ICMP echo reply all cross
    pump_driver before the VFS diagnostic descriptor progresses.

## Evidence

- Source owners:
  - src/network.rs PacketQueueError, PacketQueueFrame, FixedPacketQueue,
    PacketQueueNetworkDevice, PacketQueueDriverPumpStep, NetworkDevice,
    NetworkRuntimeDevicePump, SinglePingPacketService, and
    UserspacePingOperation.
  - src/syscall.rs VfsPingDiagnosticSvcFixture,
    dispatch_process_local_ping_descriptor_user_arguments,
    dispatch_process_local_ping_descriptor_operation, packet queue diagnostic
    controls, and stable SyscallNumber/STABLE_SVC_IMMEDIATE/TALOS_* vocabulary.
  - src/posix.rs ProcessDescriptorStore, UserMapping copy-in/copy-out, and
    fixed-capacity process descriptor ownership.
- Accepted predecessor evidence:
  - phase12-network-driver-packet-pump-contract-20260620 accepted and committed
    at 84f7804849fa9a87672a2c54f8522180f9172831.
  - phase12-network-vfs-ping-diagnostic-packet-queue-smoke-closeout-20260620
    accepted and committed at 73aeb4d8954c6febcee31678e139b38e331cbacb.

## Validation

- cargo fmt --all -- --check: passed.
- cargo -Zjson-target-spec test --quiet: passed, 662 no_std tests.
- git diff --check: passed.
- /home/node/.cargo/bin/mdbook build: passed.
- git diff --cached --check: passed before commit.

No Pi 5 hardware run, hardwareTestLock acquisition, lab mutation, boot
publication, live packet I/O, shell ping, public socket API, stable/socket ABI
acceptance, SSH, smoltcp, UDP/TCP, or phase transition was performed.

## Acceptance

Accepted.

selected_next_task=phase12-network-driver-packet-pump-closeout-20260620.

The accepted evidence level is host/QEMU-substitute source/unit evidence over a
crate-internal packet pump boundary between fixed-capacity diagnostic packet
queues and trait-level NetworkDevice behavior. The pump preserves process-local
descriptor ownership, caller-owned buffers, UserMapping copy-in/copy-out,
task-owned state, transmit FIFO ordering, explicit receive polling,
backpressure, device-error propagation, timeout/retry controls, close/drop
coverage, and unchanged stable syscall vocabulary while rejecting live packet
I/O, sockets, shell ping, hardware reachability, SSH, lab mutation, boot
publication, broad socket expansion, and phase transition.

Commit: recorded in durable supervisor state after commit creation.
