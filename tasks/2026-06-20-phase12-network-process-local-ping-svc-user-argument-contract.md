# Phase 12.4 Process-Local Ping SVC User-Argument Contract

Task: phase12-network-process-local-ping-svc-user-argument-contract-20260620

Status: accepted

Classification: phase12-network-process-local-ping-svc-user-argument-contract-accepted

## Scope

Define the smallest host-only, unstable user-argument decoding contract for the
accepted process-local ping SVC dispatch facade. This task is limited to
source/task/doc review and contract recording. It does not add source runtime
implementation, stable syscall numbers, public sockets, shell ping, live driver
adapters, live packet I/O, hardware reachability, SSH, smoltcp, UDP/TCP, lab
mutation, boot publication, Phase 12.1 link-hardware retry, broad socket
expansion, or a phase transition.

## Findings And Dispositions

- fixed: Identified the accepted predecessor evidence. The dispatch smoke
  closeout at commit 3b55c149e86d3dbc0c84e286081d7b0d456cdb04 retained
  host/QEMU-substitute smoke evidence for
  dispatch_process_local_ping_descriptor_operation over fake/trait-level
  NetworkDevice behavior, process-local descriptor ownership, caller-owned
  buffers, task-owned result/status slots, and fixed-capacity state.
- fixed: Selected unstable user-argument decoding as the next smallest useful
  feature step toward a real process-driven ping path. The accepted dispatch
  facade currently receives typed Rust operations and direct host buffers; the
  next core should translate scalar arguments plus explicit user-memory/copy
  context into that already accepted dispatch shape.
- fixed: Preserved the stable syscall vocabulary. SyscallNumber and the public
  TALOS_* constants remain limited to TalosNop, TalosWrite, TalosClose,
  TalosDup, TalosRead, and TalosOpen; the future decoder may use only a
  crate-internal experimental selector, not a stable SVC number or public ABI.
- fixed: Defined the operation selectors for a future core task: open, start,
  pump_or_read_result, status, retry_arp, timeout, and close. Selectors outside
  that set, nonzero reserved fields, malformed scalar widths, or selector/field
  combinations outside the contract must fail deterministically.
- fixed: Defined scalar responsibilities. Scalars carry the experimental
  selector, process descriptor where required, route policy, destination IPv4,
  identifier, sequence number, TTL, payload user address and length, result
  user address and length, status user address and length, and ARP retry budget
  where the selected operation needs them.
- fixed: Defined user-memory and caller-buffer responsibilities. The future
  decoder must use the existing UserMapping plus copy_from_user/copy_to_user
  style of explicit backing storage and kernel scratch/caller buffers; payload
  bytes are copied in for start, while pump/result/status output is copied out
  only after the delegated dispatch operation succeeds far enough to produce
  owned output.
- fixed: Defined output placement. Open returns the process descriptor as the
  scalar success value. Start, retry_arp, timeout, and close return scalar
  success and may copy the accepted step/result shape only through task-owned
  output buffers selected by the core contract. Pump_or_read_result copies the
  accepted pump-step/result record to the caller result buffer. Status copies the
  accepted ping status record to the caller status buffer without consuming
  terminal state.
- fixed: Preserved the existing error vocabulary. EBADF covers missing current
  owner, invalid/closed process descriptors, wrong descriptor/object lifetime,
  and stale backing ping descriptors. EMFILE covers process or backing ping
  descriptor capacity. EBUSY covers duplicate active start. EAGAIN covers retry
  exhaustion/no progress. ENOSPC covers caller-owned result/status/payload
  storage pressure. EFAULT covers invalid or inaccessible user-memory ranges.
  EINVAL covers malformed selectors, reserved fields, scalar widths, and
  contract-invalid field combinations. Device/internal errors remain delegated
  through ProcessLocalPingDescriptorControl, DescriptorShapedPingControl,
  RuntimePingOperationSyscallSubstitute, and NetworkRuntimeDevicePump.
- fixed: Preserved fixed-capacity and ownership/lifetime expectations. The
  future core must keep process-local descriptor ownership in
  ProcessDescriptorStore, backing ping ownership in ProcessLocalPingDescriptorControl,
  payload/result/status storage caller-owned, kernel scratch bounded by the
  supplied buffers, and partial open/start failures unwound without leaking
  descriptors.
- removed: No shell command, public socket API, stable syscall ABI acceptance,
  socket syscall ABI acceptance, live driver adapter, live packet I/O, packet
  queue, retry scheduler, UDP/TCP path, SSH path, hardware path, lab mutation,
  boot publication, or phase transition was added or accepted.
- deferred: The actual decoder implementation, tests, and source/unit evidence
  are deferred to
  phase12-network-process-local-ping-svc-user-argument-core-20260620.
- not-an-issue: Calling this a user-argument contract is acceptable because it
  only defines how a host-only experimental context decodes user-shaped scalar
  and memory arguments into the already accepted dispatch facade; it is not
  stable userspace ABI acceptance.

## Selected Contract

The future core should add a crate-internal host-only decoder that accepts an
experimental selector plus scalar/user-memory context and produces exactly one
ProcessLocalPingDispatchOperation. The decoder may sit beside the existing
dispatch_process_descriptor* helpers and must call the accepted
dispatch_process_local_ping_descriptor_operation rather than duplicating
ARP/IPv4/ICMP or descriptor-control logic.

Accepted operation selector responsibilities:

- open: no process descriptor input; returns the allocated process descriptor as
  the scalar success value.
- start: requires a valid process descriptor, route policy, destination IPv4,
  identifier, sequence number, TTL, payload user address/length, and ARP retry
  budget; copies payload bytes through the accepted user-copy path into bounded
  caller/kernel storage before delegation.
- pump_or_read_result: requires a valid process descriptor plus result output
  address/length; delegates one pump/read-result step and copies the accepted
  pump-step/result record out to user memory.
- status: requires a valid process descriptor plus status output address/length;
  copies idle, pending-ARP, inflight, completed, or timed-out status to user
  memory without consuming terminal status.
- retry_arp: requires a valid process descriptor and delegates one bounded ARP
  retry through the accepted control stack.
- timeout: requires a valid process descriptor and performs the explicit
  terminal timeout transition.
- close: requires a valid process descriptor and closes the process-local
  descriptor plus backing ping descriptor.

The future implementation must keep the public SyscallNumber enum,
STABLE_SVC_IMMEDIATE, and TALOS_* constants unchanged. Any stable syscall
number, public structure layout, socket API, shell command, compatibility
policy, or userspace libc surface requires later supervisor planning and new
acceptance criteria.

## Evidence Reviewed

- Accepted predecessor:
  tasks/2026-06-20-phase12-network-process-local-ping-svc-dispatch-smoke-closeout.md.
- Retained dispatch smoke:
  tasks/evidence/2026-06-20-process-local-ping-svc-dispatch-smoke/qemu-process-local-ping-svc-dispatch-smoke.log.
- Source: src/syscall.rs stable syscall vocabulary and SyscallArguments.
- Source: src/syscall.rs dispatch_process_descriptor* helpers and
  dispatch_talos_read/open user-copy patterns.
- Source: src/syscall.rs ProcessLocalPingDispatchOperation,
  ProcessLocalPingDispatchOutputs, and
  dispatch_process_local_ping_descriptor_operation.
- Source: src/posix.rs UserMapping, copy_from_user, copy_to_user,
  ProcessDescriptorStore, and fixed-capacity descriptor tables.
- Source: src/network.rs NetworkRuntimeDevicePump and fake/trait-level
  NetworkDevice behavior.
- Docs: docs/src/project/phase12-networking-ssh.md and docs/src/roadmap.md.
- Durable state: supervisor selected this task after accepted dispatch smoke
  closeout commit 3b55c149e86d3dbc0c84e286081d7b0d456cdb04.

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

selected_next_task=phase12-network-process-local-ping-svc-user-argument-core-20260620.

The accepted evidence level is static host-only source/task/doc contract
evidence for an unstable experimental user-argument decoder over the accepted
process-local ping SVC dispatch facade. Shell ping, public sockets, stable
syscall ABI acceptance, socket syscall ABI acceptance, live driver adapters,
live packet I/O, hardware reachability, SSH, smoltcp, UDP/TCP, lab mutation,
boot publication, Phase 12.1 link-hardware retry, broad socket expansion, and
phase transition remain rejected.

Commit: recorded in durable supervisor state after commit creation.
