# Phase 12.4 Driver Packet Pump Contract

Task: phase12-network-driver-packet-pump-contract-20260620

Status: accepted

Classification: phase12-network-driver-packet-pump-contract-accepted

## Scope

Define the next host-only feature boundary after the accepted
packet-queue-backed VFS ping diagnostic smoke closeout: a crate-internal
driver-facing packet pump that can drain accepted diagnostic outbound packet
records into trait-level NetworkDevice transmit behavior and poll trait-level
receive behavior back into diagnostic inbound packet records.

This task is contract work only. It does not add runtime source behavior, shell
ping, kernel-backed fake command expansion, public sockets, stable syscall ABI
acceptance, socket syscall ABI acceptance, live driver adapters, live packet
I/O, hardware reachability, SSH, smoltcp, UDP/TCP, lab mutation, boot
publication, Phase 12.1 link-hardware retry, broad socket expansion, or a
phase transition.

## Findings And Dispositions

- fixed: Identified why the packet pump is the next smallest useful feature
  step. The accepted packet queue smoke proves ARP and ICMP frames are retained
  in crate-internal queues behind the VFS diagnostic, but the queue is still
  the device implementation itself. The next boundary should make the
  driver-facing movement explicit while staying host-only and trait-level.
- fixed: Defined the future pump as a crate-internal adapter between
  fixed-capacity diagnostic packet records and a NetworkDevice implementation.
  The pump may drain outbound records to transmit_frame and poll receive_frame
  into inbound records, but only through fake/trait-level device behavior.
- fixed: Preserved existing ownership boundaries. Process-local descriptors
  remain owned by ProcessDescriptorStore and ProcessLocalPingDescriptorControl;
  payload, status, and result transfer remains caller-owned buffer plus
  UserMapping copy-in/copy-out; pump queues, scratch buffers, counters, and
  status slots remain task-owned and fixed-capacity.
- fixed: Defined deterministic ordering and backpressure requirements. The
  future core must preserve FIFO transmit ordering, explicit receive polling
  order, bounded queue capacity failures, caller-buffer pressure behavior, and
  deterministic propagation of transmit and receive device errors.
- fixed: Required the future core to keep the accepted VFS diagnostic flow
  intact: /bin/pingdiag lookup, diagnostic SVC argument decoding, descriptor
  open/start/pump/status/close, outbound ARP request, inbound ARP reply,
  outbound IPv4/ICMP echo request, inbound ICMP echo reply, status/result
  copy-out, and close/drop cleanup.
- fixed: Required deterministic future coverage for malformed received frames,
  missing or wrong owner/descriptor, invalid or closed descriptors,
  timeout/retry, close/drop behavior, unchanged SyscallNumber,
  STABLE_SVC_IMMEDIATE, and TALOS_* vocabulary.
- removed: No source runtime behavior, shell command, public socket surface,
  stable ABI, live driver adapter, live packet path, hardware evidence, lab
  artifact, or phase transition was added by this contract.
- deferred: The packet pump source implementation, focused source/unit tests,
  retained smoke evidence, closeout, and any later live driver adapter planning
  are deferred to explicit follow-up tasks.
- not-an-issue: Naming this boundary driver-facing is acceptable because the
  contract limits the device side to NetworkDevice trait behavior and rejects
  live RP1/GEM/MACB access, DMA descriptors, interrupts, packet reachability,
  public sockets, and stable networking ABI acceptance.

## Selected Contract

The future core should add the thinnest host-only packet pump boundary that can
sit between diagnostic packet queues and trait-level NetworkDevice behavior:

- executable identity: the task-owned VFS/initramfs /bin/pingdiag fixture
  remains the only user-visible test entry point;
- process ownership: one current ProcessOwnerId and process-local descriptor
  table drive the existing open/start/pump/status/retry/timeout/close path;
- memory ownership: payload, pump/result, and status records remain copied
  through UserMapping using caller-owned buffers and bounded kernel scratch;
- queue ownership: outbound and inbound packet records remain crate-internal,
  task-owned, fixed-capacity records with deterministic full and frame-too-large
  failures;
- transmit path: the pump drains outbound records in FIFO order and calls
  NetworkDevice::transmit_frame, preserving transmit counts, consumed records,
  and deterministic error mapping;
- receive path: the pump polls NetworkDevice::receive_frame with a caller-owned
  scratch buffer, retains well-formed inbound records, leaves state unchanged on
  receive-buffer pressure or WouldBlock, and maps malformed frames into
  deterministic diagnostic results;
- operation order: one pump step may account for explicit transmit-first then
  receive-poll ordering, with source/unit tests fixing the accepted order before
  any retained smoke evidence;
- result vocabulary: scalar descriptor/success returns plus copied internal
  pump/result/status records remain the only diagnostic outputs;
- cleanup: close/drop behavior must prove no queued or pump-owned state leaks
  across process-local descriptor closure;
- stable vocabulary: SyscallNumber, STABLE_SVC_IMMEDIATE, and public TALOS_*
  constants remain unchanged. Any pump selector, packet record, or payload
  layout stays crate-internal and task-local until a later explicit ABI task.

The core must not create or accept a public socket API, a stable syscall ABI, a
socket syscall ABI, a live driver adapter, RP1/GEM/MACB MMIO, DMA descriptor
ownership, interrupts, scheduler wakeups, shell ping behavior, live packet I/O,
hardware reachability, SSH, or a phase transition.

## Evidence Reviewed

- Accepted predecessor:
  tasks/2026-06-20-phase12-network-vfs-ping-diagnostic-packet-queue-smoke-closeout.md.
- Packet queue contract/core/closeout/smoke records:
  tasks/2026-06-20-phase12-network-vfs-ping-diagnostic-packet-queue-contract.md,
  tasks/2026-06-20-phase12-network-vfs-ping-diagnostic-packet-queue-core.md,
  tasks/2026-06-20-phase12-network-vfs-ping-diagnostic-packet-queue-closeout.md,
  and tasks/2026-06-20-phase12-network-vfs-ping-diagnostic-packet-queue-smoke.md.
- Retained packet queue smoke transcript:
  tasks/evidence/2026-06-20-vfs-ping-diagnostic-packet-queue-smoke/smoke-transcript.md.
- Source: src/network.rs NetworkDevice, DeviceError, FixedPacketQueue,
  PacketQueueFrame, PacketQueueNetworkDevice, SinglePingPacketService, and
  UserspacePingOperation.
- Source: src/syscall.rs VfsPingDiagnosticSvcFixture,
  dispatch_process_local_ping_descriptor_user_arguments,
  dispatch_process_local_ping_descriptor_operation, packet-queue diagnostic
  controls, and stable SyscallNumber/STABLE_SVC_IMMEDIATE/TALOS_* vocabulary.
- Source: src/posix.rs ProcessDescriptorStore, DescriptorObjectKind,
  UserMapping, copy_from_user, copy_to_user, and fixed-capacity descriptor
  tables.
- Source: src/initramfs.rs ReadOnlyInitramfs regular-file lookup and immutable
  VFS-backed executable/file fixture model.
- Docs: docs/src/project/phase12-networking-ssh.md and docs/src/roadmap.md.
- Durable state: supervisor selected this ready task after packet queue smoke
  closeout commit 73aeb4d8954c6febcee31678e139b38e331cbacb.

## Validation

- static source/task/doc review: passed.
- git diff --check: passed.
- /home/node/.cargo/bin/mdbook build: passed.
- git diff --cached --check: passed before commit.

No Rust source was touched, so cargo fmt --all -- --check and
cargo -Zjson-target-spec test --quiet were not required by this task's gates.
No Pi 5 hardware run, hardwareTestLock acquisition, boot archive publication,
lab mutation, power cycle, MDIO/PHY/GPIO32 action, RP1/GEM/MACB MMIO, DMA
descriptor ownership, interrupt completion, shell ping, public socket API,
stable syscall ABI acceptance, socket syscall ABI acceptance, live packet I/O,
SSH, smoltcp, UDP/TCP, or phase transition was performed.

## Acceptance

Accepted.

selected_next_task=phase12-network-driver-packet-pump-core-20260620.

The accepted evidence level is static host-only source/task/doc contract
evidence for a crate-internal driver-facing packet pump boundary between the
accepted VFS ping diagnostic packet queues and trait-level NetworkDevice
behavior. The future core is limited to draining outbound ARP and IPv4/ICMP
records to NetworkDevice::transmit_frame, polling NetworkDevice::receive_frame
into inbound diagnostic records, preserving process-local descriptor ownership,
caller-owned buffers, UserMapping copy-in/copy-out, task-owned state, fixed
capacity, deterministic error propagation, and unchanged stable syscall
vocabulary. Shell ping, public sockets, stable syscall ABI acceptance, socket
syscall ABI acceptance, live driver adapters, live packet I/O, hardware
reachability, SSH, smoltcp, UDP/TCP, lab mutation, boot publication, Phase
12.1 link-hardware retry, broad socket expansion, and phase transition remain
rejected.

Commit: recorded in durable supervisor state after commit creation.
