# Phase 12.4 smoltcp Packet-Device Adapter Core

Task: phase12-network-smoltcp-packet-device-adapter-core-20260621

Status: accepted

Classification: phase12-network-smoltcp-packet-device-adapter-core-accepted

## Scope

Implement the smallest host-only adapter that moves one frame at a time between
accepted Talos fixed packet/device queues and smoltcp's phy::Device token
boundary. The adapter keeps frame storage fixed-capacity, exposes deterministic
receive/transmit result mapping for tests, and leaves time, interface, socket
sets, TCP state, socket syscalls, shell diagnostics, live packet I/O, hardware,
SSH, public ABI behavior, broad socket expansion, and phase transition outside
this task.

This task preserves the accepted local ARP/ICMP responder, runtime ping,
pingdiag, and sockdiag surfaces as controls. It does not route /bin/pingdiag or
/bin/sockdiag through smoltcp.

## Findings And Dispositions

- fixed: src/network.rs now owns SmoltcpPacketDeviceAdapter as the first
  Talos-owned smoltcp phy::Device boundary over PacketQueueNetworkDevice.
- fixed: The adapter receives into a fixed scratch buffer, copies the frame into
  a fixed PacketQueueFrame-backed RxToken, and exposes a TxToken that writes
  closure-produced bytes back through the accepted packet queue transmit path.
- fixed: DeviceCapabilities are explicit and bounded for Ethernet medium,
  FRAME_CAPACITY maximum transmission unit, and single-frame burst size.
- fixed: Receive result mapping records Received, NoFrame, TransmitQueueFull,
  ReceiveBufferTooSmall, and ReceiveError outcomes deterministically.
- fixed: Transmit result mapping records Ready, Transmitted, TransmitQueueFull,
  FrameTooLarge, and TransmitError outcomes deterministically.
- fixed: Focused source/unit coverage proves receive/reply frame movement,
  no-frame behavior, transmit queue pressure without consuming receive frames,
  device receive errors, transmit errors, and frame bound failures.
- not-an-issue: No existing focused smoke script required an update because
  accepted runtime ping, pingdiag, and sockdiag behavior remains separate from
  smoltcp in this task.
- deferred: Interface polling, socket sets, TCP handshakes, Talos socket syscall
  bridging, /bin/sockdiag TCP diagnostics, live packet I/O, hardware
  reachability, SSH, public ABI acceptance, and phase transition remain future
  explicit tasks.
- removed: No fake TCP/UDP behavior, shell diagnostic expansion, hardware
  action, lab mutation, boot publication, or live packet I/O claim was added.

## Implementation

- src/network.rs adds:
  - SmoltcpPacketDeviceAdapterReceiveResult
  - SmoltcpPacketDeviceAdapterTransmitResult
  - SmoltcpPacketDeviceAdapter
  - SmoltcpPacketDeviceRxToken
  - SmoltcpPacketDeviceTxToken
  - smoltcp::phy::Device implementation for the adapter
- src/network.rs tests add:
  - smoltcp_packet_device_adapter_moves_receive_and_reply_frames
  - smoltcp_packet_device_adapter_reports_no_frame_and_transmit_queue_pressure
  - smoltcp_packet_device_adapter_maps_device_errors_and_frame_bounds

## Evidence

- source/unit plus host/QEMU-substitute:
  - command: . "$HOME/.cargo/env"; export TMPDIR=/opt/strider/openclaw/current/workspace/tmp; export PATH=/opt/strider/openclaw/current/workspace/tools/qemu-9.2.0-install/bin:$PATH; cargo fmt --all -- --check && cargo -Zjson-target-spec test --quiet
  - result: passed
  - output summary: running 690 talos no_std tests; test result: ok. 690 passed
- focused smoke scripts:
  - result: not-applicable
  - reason: no existing network/ping/socket smoke scripts were affected because
    accepted pingdiag/sockdiag/runtime-ping behavior was not routed through
    smoltcp.

## Validation

- cargo fmt --all -- --check: passed.
- cargo -Zjson-target-spec test --quiet with project QEMU path: passed.
- git diff --check: passed.
- mdbook build: passed with existing large search-index warning.
- git diff --cached --check: pending final gate.

No TCP handshake, UDP/TCP payload transport, socket syscall bridge, shell
diagnostic expansion, public stable socket ABI acceptance, live driver adapter,
live packet I/O, Pi 5 hardware run, hardwareTestLock acquisition, lab mutation,
boot publication, hardware reachability, SSH, broad socket expansion, or phase
transition was performed.

## Acceptance

Accepted.

The accepted frontier is a host-only smoltcp packet-device adapter boundary
over accepted Talos fixed packet queues. The next objective task is
phase12-network-smoltcp-loopback-tcp-handshake-core-20260621, which may use the
adapter with fake/host packet queues to prove one deterministic host-only TCP
handshake. That later task must still avoid Talos socket syscall bridging,
/bin/sockdiag TCP claims, retained smoke, live packet I/O, hardware
reachability, SSH, public ABI acceptance, broad socket expansion, and phase
transition.

Selected next task:
phase12-network-smoltcp-loopback-tcp-handshake-core-20260621.

Commit: recorded in durable supervisor state after commit creation.
