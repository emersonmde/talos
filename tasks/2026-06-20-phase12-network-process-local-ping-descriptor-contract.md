# Phase 12.4 Process-Local Ping Descriptor Contract

Task: phase12-network-process-local-ping-descriptor-contract-20260620

Status: accepted

Classification: phase12-network-process-local-ping-descriptor-contract-accepted

## Scope

Define the smallest process-local descriptor-table contract that can own the
accepted DescriptorShapedPingControl lifecycle. This task is limited to
static/source/task/evidence review and contract recording. It does not add
source implementation, shell ping, public sockets, stable syscall ABI, live
packet I/O, hardware reachability, SSH, lab mutation, boot publication, or a
phase transition.

## Findings And Dispositions

- fixed: Selected the existing process-local descriptor ownership boundary as
  the next integration point. ProcessDescriptorStore owns per-ProcessOwnerId
  DescriptorTable instances; DescriptorTable entries map integer descriptors to
  DescriptorEntry values with access, flags, object kind, and object reference.
- fixed: Selected a new crate-internal kernel-object description shape for the
  future core task. A process descriptor entry should own the user-visible
  handle while its object reference indexes a fixed-capacity ping-control
  description store owned by the same host-only runtime context.
- fixed: Preserved the existing descriptor-table semantics: allocation returns
  EMFILE when the process descriptor table is full, invalid or closed process
  descriptors return EBADF, close removes the process descriptor before or with
  the backing ping-control close, and the backing description store must not
  leak live operations after close.
- fixed: Preserved the accepted DescriptorShapedPingControl lifecycle:
  open/start/pump-or-read-result/status/retry_arp/timeout/close over
  RuntimePingOperationSyscallSubstitute and NetworkRuntimeDevicePump, with
  caller-owned receive, transmit, status, and result storage.
- fixed: Kept the future implementation crate-internal and host-only. ARP,
  IPv4, ICMP, route policy, retry behavior, timeout behavior, local responder
  behavior, and fake/trait-level NetworkDevice I/O remain delegated to the
  accepted runtime pump stack rather than being duplicated in descriptor-table
  code.
- removed: No public socket kind, syscall number, libc API, POSIX socket API,
  userspace shell command, kernel-backed fake command, live driver adapter,
  packet queue, autonomous retry timer, hardware path, lab mutation, boot
  publication, SSH path, or phase transition was added or accepted.
- deferred: The implementation of the process-local descriptor handle is
  deferred to phase12-network-process-local-ping-descriptor-core-20260620.
- not-an-issue: This contract may use DescriptorObjectKind::OtherKernelObject
  or an equally narrow crate-internal object-kind refinement in the core task;
  the contract does not require accepting public sockets or changing the stable
  syscall ABI.

## Selected Contract

The future core task should add only a crate-internal, host-only process-local
descriptor handle for one ping-control operation. The process-local descriptor
is the caller-visible integer handle. The backing ping-control description is a
kernel-owned fixed-capacity entry that retains the accepted
DescriptorShapedPingControl raw descriptor and any minimal bookkeeping needed
to route operations back to the caller-provided NetworkRuntimeDevicePump.

Ownership boundary:

- ProcessDescriptorStore continues to own process tables keyed by
  ProcessOwnerId.
- DescriptorTable::allocate creates the process-local handle and returns
  EMFILE when no descriptor slot is available.
- The descriptor entry is process-local; another ProcessOwnerId cannot operate
  on it through current_descriptor_table/current_descriptor_table_mut.
- The descriptor object reference indexes only a crate-internal ping-control
  description store, not a public socket table.
- close removes the process descriptor and closes the associated
  DescriptorShapedPingControl handle. Later operations through the closed
  process descriptor return EBADF.

Lifecycle routed through the process-local handle:

- open: allocate one backing DescriptorShapedPingControl handle and one
  process-local descriptor entry. Descriptor-table capacity failure is EMFILE.
  Backing ping-control capacity failure remains EMFILE. Partial allocation must
  be unwound so no backing ping-control description leaks.
- start: validate the process-local descriptor, then delegate to
  DescriptorShapedPingControl::start with explicit route policy, destination
  IPv4, identifier, sequence number, TTL, payload, ARP retry budget, and
  caller-owned transmit buffer.
- pump/read-result: validate the process-local descriptor, then perform exactly
  one DescriptorShapedPingControl::pump_or_read_result step with the
  caller-provided NetworkDevice and caller-owned receive/transmit/result
  storage.
- status: validate the process-local descriptor and copy the current idle,
  pending-ARP, inflight, completed, or timed-out status into caller-owned
  storage without consuming terminal status.
- retry_arp: validate the process-local descriptor, then delegate pending ARP
  retransmission to DescriptorShapedPingControl::retry_arp.
- timeout: validate the process-local descriptor, then delegate terminal timeout
  transition to DescriptorShapedPingControl::timeout.
- close: validate the process-local descriptor, close the backing
  DescriptorShapedPingControl handle, remove the process descriptor entry, and
  free the backing fixed-capacity description slot.

Error and evidence boundary:

- EBADF: invalid process owner, invalid process descriptor, closed descriptor,
  wrong object kind/reference, or invalid backing ping-control descriptor.
- EMFILE: process descriptor table capacity or backing ping-control capacity.
- EBUSY: duplicate active ping operation on the same backing control handle.
- EAGAIN: accepted retry exhaustion or no matching active progress behavior.
- ENOSPC: caller receive/status/result/output storage pressure from the
  accepted runtime pump stack.
- Existing internal packet/device mappings remain delegated to
  DescriptorShapedPingControl and RuntimePingOperationSyscallSubstitute.

The accepted evidence level remains host/QEMU-substitute over fake/trait-level
NetworkDevice behavior, caller-owned buffers, fixed-capacity state, and
task/source review. The contract does not accept public sockets, stable syscall
ABI acceptance, socket syscall ABI acceptance, shell ping, live driver
adapters, live packet I/O, hardware reachability, SSH, smoltcp, UDP/TCP, lab
mutation, boot publication, Phase 12.1 link-hardware retry, broad socket
expansion, or phase transition.

## Evidence Reviewed

- Source: src/posix.rs ProcessDescriptorStore, ProcessDescriptorOwner,
  DescriptorTable, DescriptorEntry, DescriptorObject, DescriptorObjectKind,
  DescriptorAccess, DescriptorFlags, close, dup, and inherited stdio behavior.
- Source: src/syscall.rs dispatch_process_descriptor* context requirements and
  DescriptorShapedPingControl.
- Source: src/network.rs NetworkRuntimeDevicePump and
  NetworkPingOperationDescriptorTable.
- Task:
  tasks/2026-06-20-phase12-network-descriptor-shaped-ping-control-contract.md.
- Task:
  tasks/2026-06-20-phase12-network-descriptor-shaped-ping-control-core.md.
- Task:
  tasks/2026-06-20-phase12-network-descriptor-shaped-ping-control-closeout.md.
- Durable state: supervisor selected this task as the first bounded follow-up
  after the accepted descriptor-shaped ping control closeout.

## Validation

- static/source/task/evidence review: passed.
- git diff --check: passed.
- /home/node/.cargo/bin/mdbook build: passed, existing large search-index
  warning only.
- git diff --cached --check: passed.

No Rust source was touched, so cargo fmt --all -- --check and
cargo -Zjson-target-spec test --quiet were not required by this task's gates.
No Pi 5 hardware run, hardwareTestLock acquisition, boot archive publication,
lab mutation, power cycle, MDIO/PHY/GPIO32 action, RP1 MMIO/DMA work, shell
ping, public socket API, stable syscall ABI acceptance, live packet I/O, SSH,
or phase transition was performed.

## Acceptance

Accepted.

selected_next_task=phase12-network-process-local-ping-descriptor-core-20260620.

The accepted evidence level is host-only contract evidence over
ProcessDescriptorStore, per-process DescriptorTable ownership,
DescriptorShapedPingControl, RuntimePingOperationSyscallSubstitute,
NetworkRuntimeDevicePump, fake/trait-level NetworkDevice behavior,
caller-owned buffers, fixed-capacity state, and task/source review.

Shell ping, public sockets, stable syscall ABI acceptance, socket syscall ABI
acceptance, live driver adapters, live packet I/O, hardware reachability, SSH,
smoltcp, UDP/TCP, lab mutation, boot publication, Phase 12.1 link-hardware
retry, broad socket expansion, and phase transition remain rejected.

Commit: recorded in durable supervisor state after commit creation.
