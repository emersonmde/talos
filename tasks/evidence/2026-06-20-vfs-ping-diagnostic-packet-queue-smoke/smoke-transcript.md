# VFS Ping Diagnostic Packet Queue Smoke Transcript

Task: phase12-network-vfs-ping-diagnostic-packet-queue-smoke-20260620

Evidence level: host/QEMU-substitute source/unit smoke.

Command transcript:

- cargo -Zjson-target-spec test --quiet:
  tasks/evidence/2026-06-20-vfs-ping-diagnostic-packet-queue-smoke/cargo-test-quiet.log
- Source anchors:
  tasks/evidence/2026-06-20-vfs-ping-diagnostic-packet-queue-smoke/source-anchors.txt

## Lifecycle

The retained smoke evidence uses
src/syscall.rs::vfs_ping_diagnostic_svc_fixture_records_packet_queue_lifecycle
as the queue-backed positive path.

1. ReadOnlyInitramfs resolves /bin/pingdiag through the
   VfsPingDiagnosticSvcFixture identity before the diagnostic selector is
   accepted.
2. UserMapping-backed memory provides the copied diagnostic payload and
   caller-owned pump/status result buffers.
3. ProcessLocalPingDispatchOperation opens a process-local descriptor for the
   owner, starts the ping-like transaction, and reports StartedPendingArp.
4. PacketQueueNetworkDevice records the outbound ARP request in its transmit
   queue; the test parses the popped frame as Ethernet/ARP.
5. The smoke injects a matching ARP reply through the receive queue and pumps
   the descriptor; the state advances to AdvancedToInflight.
6. PacketQueueNetworkDevice records the outbound IPv4/ICMP echo request; the
   test parses the popped frame as Ethernet/IPv4 with ICMP protocol.
7. The smoke injects a matching ICMP echo reply, pumps again, copies completed
   status/result state back through UserMapping-backed memory, drains receive
   queue state, and closes the process-local descriptor.

This distinguishes the retained path from immediate fake-device-only behavior:
outbound ARP and ICMP frames must be copied into the packet queue and popped for
inspection, while inbound ARP and ICMP replies must be injected through the
receive queue before progress is observed.

## Deterministic Controls

src/syscall.rs::vfs_ping_diagnostic_svc_fixture_maps_packet_queue_controls
retains packet-queue-specific controls for:

- queue frame-capacity rejection with PacketQueueError::FrameTooLarge.
- transmit queue capacity mapped to PosixError::NoSpace.
- transmit DeviceError::Io mapped to PosixError::Io.
- caller transmit/output-buffer pressure mapped to PosixError::NoSpace.
- malformed injected receive frame mapped to PosixError::InvalidArgument.
- explicit ARP retry recording a second ARP frame.
- receive DeviceError::Io mapped to PosixError::Io.
- explicit timeout producing TimedOut.
- invalid descriptor mapped to PosixError::BadDescriptor.

src/syscall.rs::vfs_ping_diagnostic_svc_fixture_maps_contract_error_controls
retains predecessor VFS/user-argument controls for missing executable identity,
malformed selector or payload, missing owner, wrong owner, closed descriptor,
process descriptor capacity, invalid user memory, scratch pressure,
caller receive-buffer pressure, retry exhaustion, timeout, and device errors.

The lifecycle smoke also checks SyscallNumber::from_raw(6) remains Unknown(6)
and TALOS_OPEN_SYSCALL remains 5, preserving the stable syscall vocabulary.

## Rejected Claims

This transcript does not accept shell ping, kernel-backed fake command
expansion, public sockets, stable syscall ABI acceptance, socket syscall ABI
acceptance, live driver adapters, live packet I/O, hardware reachability, SSH,
smoltcp, UDP/TCP, lab mutation, boot publication, Phase 12.1 link-hardware
retry, broad socket expansion, or phase transition.
