# Phase 12.4 VFS Ping Diagnostic Packet Queue Contract

Task: phase12-network-vfs-ping-diagnostic-packet-queue-contract-20260620

Status: accepted

Classification:
phase12-network-vfs-ping-diagnostic-packet-queue-contract-accepted

## Scope

Define the smallest host-only packet-queue-backed contract for the accepted
VFS-backed userspace ping diagnostic SVC path. This is contract work only:
source/task/doc review and frontier recording. It does not add source runtime
implementation, shell ping, kernel-backed fake command expansion, public
sockets, stable syscall ABI acceptance, socket syscall ABI acceptance, live
driver adapters, live packet I/O, hardware reachability, SSH, smoltcp,
UDP/TCP, lab mutation, boot publication, Phase 12.1 link-hardware retry, broad
socket expansion, or a phase transition.

## Findings And Dispositions

- fixed: Identified the accepted predecessor evidence. The VFS ping diagnostic
  SVC smoke closeout at commit
  70be0fed930c9c7581dff63b6595b8dbea8ed7b9 retained host/QEMU-substitute smoke
  evidence for VfsPingDiagnosticSvcFixture over ReadOnlyInitramfs regular-file
  lookup, dispatch_process_local_ping_descriptor_user_arguments, UserMapping
  copy-in/copy-out, ProcessDescriptorStore, ProcessLocalPingDescriptorControl,
  DescriptorShapedPingControl, RuntimePingOperationSyscallSubstitute,
  NetworkRuntimeDevicePump, fake/trait-level NetworkDevice behavior,
  caller-owned buffers, task-owned result/status slots, and fixed-capacity
  state.
- fixed: Selected a crate-internal packet queue as the next smallest useful
  feature-led step after the accepted diagnostic SVC smoke. The current smoke
  proves the diagnostic lifecycle, but the fake device behavior is still
  immediate from the diagnostic's point of view. The next core should make ARP
  and ICMP packet movement observable at a fixed-capacity queue boundary while
  preserving the same VFS/userspace/SVC lifecycle.
- fixed: Defined the future packet-queue capability. The core may add a
  host-only, crate-internal queue/adapter that records outbound ARP request and
  ICMP echo request frames emitted by the diagnostic path, exposes those
  records to task-owned test code, and accepts injected ARP/ICMP reply frames
  that advance the existing ping operation through the accepted
  NetworkRuntimeDevicePump and process-local descriptor stack.
- fixed: Kept ownership narrow. Queue storage, diagnostic result/status state,
  injected frame records, and outbound packet records remain task-owned and
  fixed-capacity. Payload/result/status transfer remains caller-owned buffer
  plus UserMapping copy-in/copy-out. Process-local descriptor ownership remains
  in ProcessDescriptorStore and ProcessLocalPingDescriptorControl.
- fixed: Preserved the accepted VFS/userspace diagnostic operation sequence.
  The future core must still prove executable lookup, open, status, start,
  pump_or_read_result through ARP-to-ICMP progression, completed status
  copy-out, and close. The packet queue is an internal observation/control
  boundary for frames, not a new public syscall object or socket.
- fixed: Required deterministic coverage for outbound packet records and
  injected reply progression. Future evidence must show ARP request recording,
  ARP reply injection that advances to ICMP echo request recording, ICMP echo
  reply injection that completes the diagnostic result/status path, and
  caller-owned buffer copy-out of the final records.
- fixed: Required negative/control coverage for queue capacity, transmit queue
  pressure, receive/injection queue pressure, output buffer pressure, malformed
  injected frames, malformed diagnostic input, wrong owner or descriptor,
  invalid or closed descriptor, timeout/retry, and device/error controls.
- fixed: Preserved stable-vocabulary boundaries. The future core must leave
  SyscallNumber, STABLE_SVC_IMMEDIATE, and public TALOS_* syscall constants
  unchanged. Any diagnostic selector, packet queue record, injected-frame
  record, or payload layout remains crate-internal and task-local until a later
  explicit ABI acceptance task.
- removed: No shell ping command, kernel-backed fake command expansion, public
  socket API, stable syscall ABI acceptance, socket syscall ABI acceptance,
  live driver adapter, live packet I/O, hardware reachability, SSH, smoltcp,
  UDP/TCP, lab mutation, boot publication, Phase 12.1 retry, broad socket
  expansion, driver ring, DMA descriptor ring, socket buffer, scheduler wakeup
  source, or phase transition was added or accepted.
- deferred: The packet queue source implementation, source/unit evidence,
  retained smoke transcript, and closeout are deferred to the explicit
  follow-up tasks selected by this contract.
- not-an-issue: Calling this a packet-queue step is acceptable because the
  contract limits the queue to a host-only crate-internal observation/control
  boundary for the VFS diagnostic. It does not claim live driver transmit,
  public sockets, a stable ABI, or hardware-backed network reachability.

## Selected Contract

The future core should add the thinnest packet queue boundary that makes the
accepted VFS-backed diagnostic's packet movement explicit without widening the
public surface:

- executable identity: the existing task-owned VFS/initramfs diagnostic
  executable fixture remains the user-facing entry point for the test;
- process ownership: one current ProcessOwnerId and process-local descriptor
  table drive the existing open/start/pump/status/retry/timeout/close path;
- memory ownership: diagnostic-owned user-memory ranges continue to hold
  payload, pump/result output, and status output, copied with UserMapping and
  bounded kernel scratch;
- queue ownership: fixed-capacity task-owned outbound and injected packet
  records are crate-internal and host-only;
- outbound records: ARP request and ICMP echo request frames must be retained
  with enough metadata for deterministic test inspection, including frame
  length and classification;
- injected records: ARP reply and ICMP echo reply frames must be injected
  through the same fake/trait-level NetworkDevice boundary used by
  NetworkRuntimeDevicePump rather than bypassing the accepted network stack;
- control path: dispatch_process_local_ping_descriptor_user_arguments and
  dispatch_process_local_ping_descriptor_operation remain the only diagnostic
  SVC bridge for the fixture;
- result vocabulary: scalar descriptor/success returns plus copied internal
  pump/result and status records remain the observable diagnostic outputs;
- capacity policy: all queues, scratch, receive/transmit buffers,
  descriptor tables, payload buffers, and status/result buffers are bounded and
  have deterministic failure evidence.

The core must not create or accept a public socket API, a stable syscall ABI, a
driver-facing transmit ring, live packet I/O, autonomous polling, scheduler
wakeups, shell ping behavior, or hardware reachability. Any such boundary
requires a later supervisor-planned task with its own acceptance criteria.

## Evidence Reviewed

- Accepted predecessor:
  tasks/2026-06-20-phase12-network-vfs-ping-diagnostic-svc-smoke-closeout.md.
- Retained smoke transcript:
  tasks/evidence/2026-06-20-vfs-ping-diagnostic-svc-smoke/qemu-vfs-ping-diagnostic-svc-smoke.log.
- Source: src/syscall.rs VfsPingDiagnosticSvcFixture,
  dispatch_process_local_ping_descriptor_user_arguments,
  ProcessLocalPingDescriptorControl, DescriptorShapedPingControl,
  RuntimePingOperationSyscallSubstitute, UserMapping copy-in/copy-out bridge,
  and stable SyscallNumber/STABLE_SVC_IMMEDIATE/TALOS_* vocabulary.
- Source: src/network.rs NetworkDevice, SinglePingPacketService,
  NetworkRuntimeDevicePump, packet parsing/building result vocabulary,
  fake/trait-level device behavior, and fixed-capacity local state.
- Source: src/posix.rs ProcessDescriptorStore, DescriptorObjectKind,
  UserMapping, copy_from_user, copy_to_user, and fixed-capacity descriptor
  tables.
- Source: src/initramfs.rs ReadOnlyInitramfs regular-file lookup and immutable
  VFS-backed executable/file fixture model.
- Docs: docs/src/project/phase12-networking-ssh.md and docs/src/roadmap.md.
- Durable state: supervisor selected this task after accepted VFS ping
  diagnostic SVC smoke closeout commit
  70be0fed930c9c7581dff63b6595b8dbea8ed7b9.

## Validation

- static source/task/doc review: passed.
- git diff --check: passed.
- /home/node/.cargo/bin/mdbook build: passed, existing large search-index
  warning only.
- git diff --cached --check: passed before commit.

No Rust source was touched, so cargo fmt --all -- --check and
cargo -Zjson-target-spec test --quiet were not required by this task's gates.
No Pi 5 hardware run, hardwareTestLock acquisition, boot archive publication,
lab mutation, power cycle, MDIO/PHY/GPIO32 action, RP1 MMIO/DMA work, shell
ping, public socket API, stable syscall ABI acceptance, socket syscall ABI
acceptance, live packet I/O, SSH, smoltcp, UDP/TCP, or phase transition was
performed.

## Acceptance

Accepted.

selected_next_task=phase12-network-vfs-ping-diagnostic-packet-queue-core-20260620.

The accepted evidence level is static host-only source/task/doc contract
evidence for a crate-internal, fixed-capacity packet queue boundary behind the
VFS-backed userspace ping diagnostic SVC path. The future core is limited to
recording outbound ARP request and ICMP echo request frames, injecting ARP/ICMP
reply frames through fake/trait-level NetworkDevice behavior, preserving
UserMapping copy-in/copy-out, process-local descriptor ownership, task-owned
diagnostic state, caller-owned buffers, and unchanged stable syscall
vocabulary. Shell ping, public sockets, stable syscall ABI acceptance, socket
syscall ABI acceptance, live driver adapters, live packet I/O, hardware
reachability, SSH, smoltcp, UDP/TCP, lab mutation, boot publication, Phase
12.1 link-hardware retry, broad socket expansion, and phase transition remain
rejected.

Commit: recorded in durable supervisor state after commit creation.
