# Phase 12.4 Driver Packet Adapter Core

Task: phase12-network-driver-packet-adapter-core-20260621

Status: accepted

Classification: phase12-network-driver-packet-adapter-core-accepted

## Scope

Implement the thinnest host-only source/unit driver packet adapter substrate
selected by phase12-network-driver-packet-adapter-contract-20260621. The core
represents bounded driver RX input and driver TX output through accepted
fixed-capacity PacketQueueFrame storage and the accepted
SmoltcpPacketDeviceAdapter boundary, without binding to RP1 GEM hardware.

This task does not accept shell diagnostic smoke, retained smoke evidence, live
driver programming, live packet I/O, Pi 5 hardware behavior, hardware
reachability, lab mutation, boot publication, generated-root publication, SSH,
UDP/raw sockets, libc/std socket wrappers, POSIX/Linux compatibility, public
stable ABI acceptance, broad socket expansion, or phase transition.

## Findings And Dispositions

- fixed: Added DriverPacketAdapter as a host-only wrapper over the accepted
  SmoltcpPacketDeviceAdapter and PacketQueueNetworkDevice boundaries.
- fixed: Driver RX input enters through inject_driver_rx and remains copied
  into fixed-capacity PacketQueueFrame-backed storage before smoltcp receives
  it.
- fixed: receive_one_for_smoltcp consumes at most one smoltcp RX token per
  caller-supplied smoltcp::time::Instant and records deterministic receive
  outcomes.
- fixed: transmit_one_from_smoltcp records at most one smoltcp-produced TX
  frame per caller-supplied smoltcp::time::Instant, and pop_driver_tx exposes
  the copied driver-visible TX frame.
- fixed: Focused source/unit tests prove copied RX/TX movement, TX queue
  backpressure preserving a queued RX frame, FrameTooLarge behavior, and
  deterministic DeviceError mapping.
- fixed: Full cargo source/unit regression coverage preserves accepted local
  socket, smoltcp TCP bridge, poll/poll-wait, userspace_socket_abi,
  /bin/pingdiag, and /bin/sockdiag behavior.
- deferred: Shell-visible /bin/sockdiag adapter observation and retained
  host/QEMU-substitute smoke evidence remain explicit follow-up tasks.
- removed: No fake shell command, fake network reachability, hardware action,
  lab mutation, boot publication, generated-root publication, live packet I/O
  claim, UDP/raw socket surface, SSH surface, public ABI claim, or phase
  transition was added.
- not-an-issue: The adapter remains driver-named while host-only because its
  accepted external boundary is still copied packet records and smoltcp tokens,
  not RP1 GEM MMIO, DMA descriptors, interrupts, PHY/MDIO, or hardware packet
  ownership.

## Implementation

- src/network.rs:
  - DriverPacketAdapterReceiveStep records NoFrame, Received,
    TransmitQueueFull, ReceiveBufferTooSmall, and ReceiveError outcomes.
  - DriverPacketAdapterTransmitStep records Transmitted, TransmitQueueFull,
    FrameTooLarge, and TransmitError outcomes.
  - DriverPacketAdapter owns a SmoltcpPacketDeviceAdapter and exposes
    inject_driver_rx, receive_one_for_smoltcp, transmit_one_from_smoltcp,
    pop_driver_tx, driver_rx_len, driver_tx_len, and deterministic error hooks.
- docs/src/project/phase12-networking-ssh.md and docs/src/roadmap.md record the
  accepted source/unit frontier and rejected claims.

## Evidence

- Focused source/unit tests:
  - driver_packet_adapter_moves_driver_rx_and_smoltcp_tx_with_copied_frames
  - driver_packet_adapter_preserves_rx_when_tx_backpressure_blocks_smoltcp_receive
  - driver_packet_adapter_maps_capacity_and_device_errors_deterministically
- Source anchors:
  - src/network.rs NetworkDevice, DeviceError, PacketQueueFrame,
    FixedPacketQueue, PacketQueueNetworkDevice, SmoltcpPacketDeviceAdapter,
    DriverPacketAdapter, SmoltcpSocketBridgeRecord, and associated source/unit
    tests.
  - src/userspace_socket_abi.rs SocketAbiCall and PollEntry helper surface.
  - src/syscall.rs private socket dispatch and diagnostic SVC surfaces.
  - src/local_command_loop.rs /bin/pingdiag and /bin/sockdiag controls.
- Accepted predecessor evidence:
  - phase12-network-driver-packet-adapter-contract-20260621 accepted and
    committed at 3cf10ff3b0ae2e8a86ae030c0a026170ad291e57.
  - phase12-network-shell-sockdiag-userspace-abi-closeout-20260621 accepted and
    committed at b417d9e3167cd1acb34f8c4647a503739703d496.
  - smoltcp packet-device, TCP bridge, and userspace socket ABI records listed
    in the predecessor contract remain regression anchors.

## Validation

- cargo fmt --all -- --check: passed.
- cargo -Zjson-target-spec test --quiet driver_packet_adapter: passed.
- cargo -Zjson-target-spec test --quiet: passed.
- git diff --check: passed.
- /home/node/.cargo/bin/mdbook build: passed with existing large search-index
  warning behavior.
- git diff --cached --check: pending final pre-commit gate.

No retained smoke, hardwareTestLock acquisition, lab mutation, boot
publication, generated-root publication, Pi 5 hardware run, live driver
programming, live packet I/O, hardware reachability, SSH, UDP/raw sockets,
libc/std socket wrappers, POSIX/Linux compatibility, public stable ABI
acceptance, broad socket expansion, or phase transition was performed.

## Acceptance

Accepted.

selected_next_task=phase12-network-shell-sockdiag-driver-packet-adapter-core-20260621.

The accepted evidence level is source/unit plus host/QEMU-substitute only over
a crate-internal driver packet adapter substrate. The adapter proves copied
fixed-capacity driver RX input and smoltcp-produced TX output through accepted
packet-device boundaries with deterministic capacity, backpressure, caller-time,
and DeviceError behavior while preserving accepted local socket, smoltcp TCP
bridge, userspace_socket_abi, /bin/pingdiag, and /bin/sockdiag regression
surfaces.

Commit: recorded in durable supervisor state after commit creation.
