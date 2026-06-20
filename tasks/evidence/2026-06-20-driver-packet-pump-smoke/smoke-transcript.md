# Driver Packet Pump Smoke Transcript

Task: phase12-network-driver-packet-pump-smoke-20260620

Evidence level: host/QEMU-substitute smoke over crate-internal packet queue and
trait-level NetworkDevice behavior.

Command transcript:

- scripts/qemu-driver-packet-pump-smoke.sh:
  tasks/evidence/2026-06-20-driver-packet-pump-smoke/qemu-driver-packet-pump-smoke.log
- Source anchors:
  tasks/evidence/2026-06-20-driver-packet-pump-smoke/source-anchors.txt

The script records the packet-pump boundary and invokes focused test filters.
The current no_std QEMU test runner executes the full target test binary for
each invocation, so each filtered command reports the full 662-test suite while
the transcript labels the intended boundary checks.

## Lifecycle

The retained positive path is
src/syscall.rs::vfs_ping_diagnostic_svc_fixture_records_packet_queue_lifecycle.

1. ReadOnlyInitramfs resolves /bin/pingdiag before the diagnostic SVC selector
   is accepted.
2. UserMapping-backed memory provides copied input payload and caller-owned
   pump/status buffers.
3. ProcessLocalPingDispatchOperation opens a process-local descriptor and starts
   the ping-like transaction.
4. PacketQueueNetworkDevice records the outbound ARP request; pump_driver moves
   it to a trait-level NetworkDevice driver queue before the test parses it as
   Ethernet/ARP.
5. The smoke injects a matching ARP reply through the trait-level driver queue;
   pump_driver moves it into the diagnostic receive queue before descriptor
   progress advances to ICMP.
6. PacketQueueNetworkDevice records the outbound IPv4/ICMP echo request;
   pump_driver moves it to the driver queue before the test parses it as
   Ethernet/IPv4/ICMP.
7. The smoke injects a matching ICMP echo reply through the trait-level driver
   queue, pumps it back into the diagnostic receive queue, observes completed
   status/result copy-out, drains queued receive state, and closes the
   process-local descriptor.

This distinguishes the retained path from queue-only or fake-device-only
behavior: outbound ARP and ICMP records must cross pump_driver to trait-level
NetworkDevice transmit behavior, and inbound ARP and ICMP replies must cross
back through pump_driver before the descriptor observes progress.

## Deterministic Controls

src/network.rs::packet_queue_driver_pump_drains_outbound_before_polling_receive_fifo
and
src/network.rs::packet_queue_driver_pump_reports_backpressure_and_device_errors_deterministically
retain packet-pump controls for:

- transmit FIFO ordering before receive polling.
- transmit error preservation and retry.
- receive queue backpressure before consuming a driver-owned frame.
- caller receive-buffer pressure.
- oversized received frames.
- receive DeviceError::Io and no-frame behavior.

src/syscall.rs::vfs_ping_diagnostic_svc_fixture_maps_packet_queue_controls
retains packet-queue diagnostic controls for transmit queue capacity,
frame-capacity rejection, caller output-buffer pressure, malformed received
frames, retry/timeout, receive/transmit device errors, and invalid descriptor
handling.

src/syscall.rs::vfs_ping_diagnostic_svc_fixture_maps_contract_error_controls
and
src/syscall.rs::process_local_ping_user_arguments_reject_malformed_owner_descriptor_capacity_and_buffers
retain VFS/user-argument controls for missing /bin/pingdiag identity, malformed
selectors or payloads, missing or wrong owner, closed descriptor, process
descriptor capacity, invalid user memory, scratch pressure, caller buffer
pressure, and unchanged SyscallNumber/STABLE_SVC_IMMEDIATE/TALOS_* vocabulary.

## Rejected Claims

This transcript does not accept Pi 5 hardware behavior, live driver adapters,
live packet I/O, hardware reachability, lab mutation, boot publication, shell
ping, public sockets, stable/socket ABI acceptance, SSH, smoltcp, UDP/TCP,
broad socket expansion, or phase transition.
