# Phase 12.4 VFS Ping Diagnostic SVC Contract

Task: phase12-network-vfs-ping-diagnostic-svc-contract-20260620

Status: accepted

Classification: phase12-network-vfs-ping-diagnostic-svc-contract-accepted

## Scope

Define the smallest host-only contract for a VFS-backed userspace ping
diagnostic fixture that drives the accepted experimental ping SVC
user-argument bridge. This is contract work only: source/task/doc review and
frontier recording. It does not add source runtime implementation, shell ping,
kernel-backed fake command expansion, public sockets, stable syscall ABI
acceptance, socket syscall ABI acceptance, live driver adapters, live packet
I/O, hardware reachability, SSH, smoltcp, UDP/TCP, lab mutation, boot
publication, Phase 12.1 link-hardware retry, broad socket expansion, or a
phase transition.

## Findings And Dispositions

- fixed: Identified the accepted predecessor evidence. The user-argument smoke
  closeout at commit a029de8844513dec66197bd4af17ee10f83679bf retained
  host/QEMU-substitute smoke evidence for
  dispatch_process_local_ping_descriptor_user_arguments over UserMapping,
  ProcessLocalPingDispatchOperation, ProcessLocalPingDescriptorControl,
  ProcessDescriptorStore, NetworkRuntimeDevicePump, fake/trait-level
  NetworkDevice behavior, caller-owned buffers, task-owned result/status slots,
  and fixed-capacity state.
- fixed: Selected a VFS-backed userspace diagnostic as the next smallest useful
  feature step toward real user-driven networking. The accepted decoder is
  still driven by host test harnesses and typed Rust setup; the next core
  should prove that a VFS-backed executable-shaped fixture can own user memory,
  issue the experimental SVC/user-argument sequence, and observe results
  through the existing process-local descriptor path.
- fixed: Defined the diagnostic executable boundary. The future core may add a
  task-owned VFS/initramfs fixture path for an experimental ping diagnostic
  program record and host/QEMU-substitute harness glue that exercises it
  through the accepted VFS/open/read, program-loading, process ownership, and
  UserMapping patterns. The fixture remains diagnostic-only and crate-internal;
  it is not a shell command, public executable contract, libc surface, stable
  ABI, or socket API.
- fixed: Defined the accepted operation sequence for the future core:
  open process-local ping descriptor, copy idle status out, start from
  diagnostic-owned user payload, pump_or_read_result through ARP-to-ICMP
  progression, copy completed status out, and close the descriptor. The same
  future evidence must include deterministic controls for malformed selector
  or payload, missing owner, invalid or closed descriptor, capacity, user
  memory, buffer pressure, timeout/retry, and device-error behavior.
- fixed: Defined experimental SVC/user-argument responsibilities. The future
  diagnostic may use only the existing crate-internal
  dispatch_process_local_ping_descriptor_user_arguments path and experimental
  selectors for open, start, pump_or_read_result, status, retry_arp, timeout,
  and close. SyscallNumber, STABLE_SVC_IMMEDIATE, and public TALOS_* constants
  must remain unchanged.
- fixed: Defined user-memory copy-in/copy-out responsibilities. Diagnostic
  payload bytes are copied from the diagnostic's user-memory range into bounded
  kernel scratch before start. Pump/result and status records are copied back
  only through caller-owned user buffers using the existing UserMapping plus
  copy_from_user/copy_to_user style. Kernel buffers remain bounded and
  task-owned; output record layouts remain the accepted internal task records,
  not public ABI structures.
- fixed: Defined result/status placement. Open returns the process descriptor
  as the scalar success value. Start, retry_arp, timeout, and close return
  scalar success after delegated control. Pump_or_read_result copies the
  accepted pump-step/result record into a diagnostic-owned result buffer.
  Status copies the accepted idle, pending-ARP, inflight, completed, or timed
  out status record into a diagnostic-owned status buffer without consuming
  terminal state.
- fixed: Preserved the accepted error vocabulary. EBADF covers missing owner,
  invalid or closed process descriptors, wrong object lifetime, and stale
  backing ping descriptors. EMFILE covers process or backing descriptor
  capacity. EBUSY covers duplicate active start. EAGAIN covers retry exhaustion
  or no-progress retry behavior. ENOSPC covers result/status/payload storage
  pressure and bounded scratch pressure. EFAULT covers invalid or inaccessible
  user-memory ranges. EINVAL covers malformed selectors, reserved fields,
  scalar widths, invalid payload/record lengths, zero TTL, invalid route
  prefix, and contract-invalid combinations. Device/internal errors remain
  delegated through ProcessLocalPingDescriptorControl,
  DescriptorShapedPingControl, RuntimePingOperationSyscallSubstitute, and
  NetworkRuntimeDevicePump.
- fixed: Preserved fixed-capacity and lifetime expectations. Process-local
  ownership stays in ProcessDescriptorStore, backing ping ownership stays in
  ProcessLocalPingDescriptorControl, diagnostic user memory owns payload and
  output buffers, and partial open/start failures must unwind descriptors and
  backing objects without leaks.
- removed: No shell ping command, kernel-backed fake command expansion, public
  socket API, stable syscall ABI, socket syscall ABI, live driver adapter, live
  packet I/O, hardware reachability, SSH, smoltcp, UDP/TCP, lab mutation, boot
  publication, Phase 12.1 retry, broad socket expansion, or phase transition
  was added or accepted.
- deferred: The VFS/userspace diagnostic implementation, source/unit evidence,
  and any focused host/QEMU-substitute command are deferred to
  phase12-network-vfs-ping-diagnostic-svc-core-20260620.
- not-an-issue: Calling this VFS-backed userspace diagnostic work is acceptable
  because the contract ties the next feature slice to an executable-shaped
  VFS/userspace boundary while retaining host-only fake/trait-level network
  evidence and explicitly rejecting public ABI, shell, socket, live driver, and
  hardware claims.

## Selected Contract

The future core should add the thinnest diagnostic fixture that connects the
accepted VFS/userspace execution model to the accepted experimental ping
SVC/user-argument bridge. The diagnostic boundary is intentionally narrow:

- executable identity: a task-owned, VFS-backed diagnostic fixture path selected
  by the core task, loaded through the accepted initramfs/VFS executable
  patterns rather than through a kernel-backed shell ping command;
- process ownership: one current ProcessOwnerId with a process-local descriptor
  table, inherited standard descriptor behavior left unchanged, and no
  descriptor inheritance expansion;
- memory ownership: diagnostic-owned user-memory ranges for payload,
  pump/result output, and status output, mapped through UserMapping with
  explicit copy_from_user/copy_to_user behavior;
- control path: experimental selectors only, decoded by
  dispatch_process_local_ping_descriptor_user_arguments and delegated through
  dispatch_process_local_ping_descriptor_operation;
- operation sequence: open, status, start, pump_or_read_result until terminal
  completion or explicit controls, status, close;
- result vocabulary: scalar descriptor/success returns plus copied internal
  pump/result and status records in diagnostic-owned buffers;
- capacity policy: fixed ProcessDescriptorStore capacity, backing ping-control
  capacity, bounded kernel scratch, bounded caller-owned payload/result/status
  buffers, and deterministic unwinds on partial failure.

The future implementation must keep SyscallNumber, STABLE_SVC_IMMEDIATE, and
the public TALOS_* constants unchanged. Any stable syscall number, public
socket API, shell command, compatibility policy, libc surface, live driver
adapter, hardware claim, or network reachability claim requires later
supervisor planning and new acceptance criteria.

## Evidence Reviewed

- Accepted predecessor:
  tasks/2026-06-20-phase12-network-process-local-ping-svc-user-argument-smoke-closeout.md.
- Retained smoke transcript:
  tasks/evidence/2026-06-20-process-local-ping-svc-user-argument-smoke/qemu-process-local-ping-svc-user-argument-smoke.log.
- Source: src/syscall.rs stable SyscallNumber/TALOS_* vocabulary,
  SyscallArguments, and dispatch_talos_open/read user-memory patterns.
- Source: src/syscall.rs
  dispatch_process_local_ping_descriptor_user_arguments, experimental
  PROCESS_LOCAL_PING_USER_SELECTOR_* selectors, user record encoders, and
  dispatch_process_local_ping_descriptor_operation.
- Source: src/posix.rs UserMapping, copy_from_user, copy_to_user,
  ProcessDescriptorStore, DescriptorObjectKind, and fixed-capacity descriptor
  tables.
- Source: src/initramfs.rs read-only initramfs fixture and VFS-backed
  executable/file object model.
- Source: src/local_command_loop.rs VFS-backed exec harness patterns and
  inherited process descriptor surfaces.
- Source: src/network.rs NetworkRuntimeDevicePump and fake/trait-level
  NetworkDevice behavior.
- Docs: docs/src/project/phase12-networking-ssh.md and docs/src/roadmap.md.
- Durable state: supervisor selected this task after accepted user-argument
  smoke closeout commit a029de8844513dec66197bd4af17ee10f83679bf.

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

selected_next_task=phase12-network-vfs-ping-diagnostic-svc-core-20260620.

The accepted evidence level is static host-only source/task/doc contract
evidence for a VFS-backed userspace diagnostic fixture that will drive the
accepted experimental ping SVC/user-argument bridge over process-local
descriptor ownership, UserMapping copy-in/copy-out, internal dispatch-shaped
control, fake/trait-level NetworkDevice behavior, caller-owned buffers,
task-owned result/status slots, and fixed-capacity state. Shell ping,
kernel-backed fake command expansion, public sockets, stable syscall ABI
acceptance, socket syscall ABI acceptance, live driver adapters, live packet
I/O, hardware reachability, SSH, smoltcp, UDP/TCP, lab mutation, boot
publication, Phase 12.1 link-hardware retry, broad socket expansion, and phase
transition remain rejected.

Commit: recorded in durable supervisor state after commit creation.
