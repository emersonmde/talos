# Phase 12.4 VFS Ping Diagnostic Packet Queue Core

Task: phase12-network-vfs-ping-diagnostic-packet-queue-core-20260620

Status: accepted

Classification: phase12-network-vfs-ping-diagnostic-packet-queue-core-accepted

## Scope

Implement the thinnest host-only packet-queue-backed core selected by the
accepted contract for the VFS-backed userspace ping diagnostic SVC path. The
core may add only crate-internal, fixed-capacity packet queue records and
task-local tests around fake/trait-level NetworkDevice behavior. It does not
add a shell ping command, public socket API, stable syscall ABI acceptance,
socket syscall ABI acceptance, live driver adapter, live packet I/O, hardware
reachability, SSH, lab mutation, boot publication, or phase transition.

## Findings And Dispositions

- fixed: Added PacketQueueNetworkDevice in src/network.rs. It is a
  crate-internal NetworkDevice implementation backed by fixed-capacity receive
  and transmit queues of bounded PacketQueueFrame records. It has no allocator,
  DMA, interrupt, scheduler, socket, driver-ring, or public userspace object
  semantics.
- fixed: Added deterministic queue controls. Outbound transmit records are
  copied into the transmit queue; injected receive frames are copied into the
  receive queue; full queues and oversized frames return bounded errors; caller
  receive-buffer pressure leaves the queued receive frame in place; injected IO
  controls can force deterministic receive/transmit errors.
- fixed: Wired the VFS-backed diagnostic fixture tests through the queue-backed
  NetworkDevice path. Source/unit evidence records one outbound ARP request,
  injects a matching ARP reply, records one outbound IPv4/ICMP echo request,
  injects a matching ICMP echo reply, copies completed status back to user
  memory, and closes the process-local descriptor.
- fixed: Added deterministic negative/control coverage for transmit queue
  capacity, oversized injected frames, output-buffer pressure, malformed
  injected frames, explicit ARP retry, timeout, receive IO errors, transmit IO
  errors, invalid descriptor, and unchanged SyscallNumber/TALOS_* vocabulary.
  Existing accepted tests continue to cover process-local owner rejection,
  closed descriptors, caller receive-buffer pressure, user-memory faults,
  scratch pressure, malformed arguments, and the descriptor-shaped/VFS
  diagnostic lifecycle.
- fixed: Preserved accepted boundaries. VfsPingDiagnosticSvcFixture still owns
  only a VFS/initramfs executable-shaped diagnostic identity and delegates
  payload copy-in, result/status copy-out, owner checks, descriptor checks, and
  lifecycle operations to the accepted user-argument dispatch path.
- removed: No shell ping command, kernel-backed fake command expansion, public
  socket API, stable syscall ABI acceptance, socket syscall ABI acceptance, live
  driver adapter, live packet I/O, hardware reachability, SSH, smoltcp,
  UDP/TCP, lab mutation, boot publication, Phase 12.1 link-hardware retry,
  broad socket expansion, or phase transition was added or accepted.
- deferred: Closeout reconciliation and retained smoke evidence remain
  dependency-gated follow-up tasks. The selected next task is
  phase12-network-vfs-ping-diagnostic-packet-queue-closeout-20260620.
- not-an-issue: Mapping queue capacity and oversized-frame errors to
  DeviceError::BufferTooSmall at the NetworkDevice trait boundary is acceptable
  for this crate-internal diagnostic device because the trait currently exposes
  only WouldBlock, BufferTooSmall, and Io. The more specific PacketQueueError is
  still available to task-owned injection/setup code.

## Implementation

src/network.rs now provides:

- PacketQueueError for full queue and frame-capacity failures.
- PacketQueueFrame, a bounded copy of one raw Ethernet frame.
- FixedPacketQueue, a fixed-capacity FIFO over PacketQueueFrame.
- PacketQueueNetworkDevice, a crate-internal NetworkDevice adapter with
  explicit receive injection and outbound transmit record inspection.

src/syscall.rs adds VFS diagnostic fixture tests that keep the accepted
VFS/userspace path intact while replacing immediate fake-device-only behavior
with the packet queue boundary. The lifecycle evidence remains:

- ReadOnlyInitramfs regular-file lookup for /bin/pingdiag.
- UserMapping-backed diagnostic payload copy-in.
- Process-local descriptor open/status/start/pump/status/close.
- Packet queue recording of outbound ARP and IPv4/ICMP request frames.
- Packet queue injection of ARP and ICMP reply frames.
- UserMapping-backed pump/status copy-out.

## Evidence

- Source: src/network.rs PacketQueueError, PacketQueueFrame,
  FixedPacketQueue, and PacketQueueNetworkDevice.
- Source/unit tests: src/syscall.rs
  vfs_ping_diagnostic_svc_fixture_records_packet_queue_lifecycle and
  vfs_ping_diagnostic_svc_fixture_maps_packet_queue_controls.
- Predecessor contract:
  tasks/2026-06-20-phase12-network-vfs-ping-diagnostic-packet-queue-contract.md.
- Accepted VFS diagnostic SVC smoke closeout:
  tasks/2026-06-20-phase12-network-vfs-ping-diagnostic-svc-smoke-closeout.md.

## Validation

- cargo fmt --all -- --check: passed.
- cargo -Zjson-target-spec test --quiet: passed with QEMU on PATH.
- git diff --check: passed.
- /home/node/.cargo/bin/mdbook build: passed.
- git diff --cached --check: passed.

No Pi 5 hardware run, hardwareTestLock acquisition, boot archive publication,
lab mutation, power cycle, MDIO/PHY/GPIO32 action, RP1 MMIO/DMA work, shell
ping, public socket API, stable syscall ABI acceptance, socket syscall ABI
acceptance, live packet I/O, SSH, smoltcp, UDP/TCP, or phase transition was
performed.

## Acceptance

Accepted.

selected_next_task=phase12-network-vfs-ping-diagnostic-packet-queue-closeout-20260620.

The accepted evidence level is source/unit host/QEMU-substitute evidence over a
crate-internal fixed-capacity packet queue boundary behind the VFS-backed
userspace ping diagnostic SVC path. The queue records outbound ARP request and
IPv4/ICMP echo request frames, accepts injected ARP/ICMP reply frames, and
preserves UserMapping copy-in/copy-out, process-local descriptor ownership,
fake/trait-level NetworkDevice behavior, caller-owned buffers, task-owned
diagnostic state, and unchanged stable syscall vocabulary. Shell ping,
kernel-backed fake command expansion, public sockets, stable syscall ABI
acceptance, socket syscall ABI acceptance, live driver adapters, live packet
I/O, hardware reachability, SSH, smoltcp, UDP/TCP, lab mutation, boot
publication, Phase 12.1 link-hardware retry, broad socket expansion, and phase
transition remain rejected.

Commit: recorded in durable supervisor state after commit creation.
