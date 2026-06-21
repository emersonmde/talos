# Phase 12.4 smoltcp Loopback TCP Handshake Core

Task: phase12-network-smoltcp-loopback-tcp-handshake-core-20260621

Status: accepted

Classification: phase12-network-smoltcp-loopback-tcp-handshake-core-accepted

## Scope

Prove the thinnest host-only smoltcp TCP handshake path over the accepted
Talos packet-device adapter boundary. This task owns only source/unit evidence
for two fake/host Ethernet endpoints connected by fixed packet queues. It does
not bridge smoltcp into Talos socket syscalls, /bin/sockdiag, retained smoke,
live driver adapters, live packet I/O, hardware, SSH, public ABI behavior,
broad socket expansion, or a phase transition.

## Findings And Dispositions

- fixed: src/network.rs now has a host-only smoltcp TCP handshake harness that
  creates two Interface instances over SmoltcpPacketDeviceAdapter endpoints and
  shuttles transmitted frames through fixed PacketQueueFrame storage.
- fixed: The successful handshake evidence owns both endpoints' MAC addresses,
  IPv4 CIDRs, TCP socket buffers, SocketSet storage, packet queue capacities,
  and deterministic one-millisecond poll steps explicitly.
- fixed: The successful path reaches Established on both sockets after two
  poll steps, three client-to-server frames, and two server-to-client frames.
- fixed: A companion backpressure case proves a zero-capacity client transmit
  queue records TransmitQueueFull, leaves the client in SynSent and server in
  Listen, and moves zero frames instead of accepting a handshake.
- not-an-issue: Existing runtime ping, pingdiag, sockdiag, and packet pump
  surfaces remain separate from smoltcp and did not need source changes.
- deferred: Talos socket syscall bridging, /bin/sockdiag TCP diagnostics,
  retained smoke evidence, UDP/TCP payload transport beyond the handshake
  state transition, live driver adapters, live packet I/O, hardware
  reachability, SSH, public stable socket ABI acceptance, broad socket
  expansion, and phase transition remain future explicit tasks.
- removed: No fake kernel-backed TCP command, shell diagnostic expansion,
  hardware action, lab mutation, boot publication, or live packet I/O claim was
  added.

## Implementation

- src/network.rs tests add:
  - SmoltcpHandshakeOutcome
  - SmoltcpHandshakeObservation
  - make_smoltcp_interface
  - move_smoltcp_frames
  - drive_smoltcp_handshake
  - smoltcp_loopback_tcp_handshake_establishes_over_packet_device_adapters
  - smoltcp_loopback_tcp_handshake_reports_client_transmit_backpressure

## Evidence

- source/unit plus host/QEMU-substitute:
  - command: . "$HOME/.cargo/env"; export TMPDIR=/opt/strider/openclaw/current/workspace/tmp; export PATH=/opt/strider/openclaw/current/workspace/tools/qemu-9.2.0-install/bin:$PATH; cargo -Zjson-target-spec test --quiet
  - result: passed
  - output summary: running 692 talos no_std tests; test result: ok. 692 passed
- focused smoke scripts:
  - result: not-applicable
  - reason: no existing network/ping/socket smoke script was affected because
    runtime ping, /bin/pingdiag, and /bin/sockdiag remain separate from
    smoltcp in this task.

## Validation

- cargo fmt --all -- --check: passed.
- cargo -Zjson-target-spec test --quiet with project QEMU path: passed.
- git diff --check: passed.
- mdbook build: passed with existing large search-index warning.
- git diff --cached --check: pending final gate.

No Talos socket syscall bridge, /bin/sockdiag TCP diagnostic, retained smoke
evidence, UDP/TCP payload transport beyond the handshake state transition,
live driver adapter, live packet I/O, Pi 5 hardware run, hardwareTestLock
acquisition, lab mutation, boot publication, hardware reachability, SSH, public
stable socket ABI acceptance, broad socket expansion, or phase transition was
performed.

## Acceptance

Accepted.

The accepted frontier is host-only deterministic smoltcp TCP handshake evidence
over accepted Talos fixed packet queues and SmoltcpPacketDeviceAdapter. The
next objective task is phase12-network-smoltcp-tcp-frontier-closeout-20260621,
which may close out the initial smoltcp/TCP host-only frontier. That later
task must still avoid socket syscall bridging, public stable socket ABI
acceptance, live packet I/O, hardware reachability, SSH, broad expansion, and
phase transition.

Selected next task:
phase12-network-smoltcp-tcp-frontier-closeout-20260621.

Commit: recorded in durable supervisor state after commit creation.
