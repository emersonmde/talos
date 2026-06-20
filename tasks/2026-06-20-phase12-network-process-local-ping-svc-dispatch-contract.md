# Phase 12.4 Process-Local Ping SVC Dispatch Contract

Task: phase12-network-process-local-ping-svc-dispatch-contract-20260620

Status: accepted

Classification: phase12-network-process-local-ping-svc-dispatch-contract-accepted

## Scope

Define the smallest host-only internal dispatch contract for driving the
accepted ProcessLocalPingDescriptorControl through the existing process
descriptor/syscall dispatch shape. This task is limited to static
source/task/doc review and contract recording. It does not add source runtime
implementation, shell ping, public sockets, stable syscall ABI acceptance,
socket syscall ABI acceptance, live driver adapters, live packet I/O, hardware
reachability, SSH, smoltcp, UDP/TCP, lab mutation, boot publication, Phase 12.1
link-hardware retry, broad socket expansion, or a phase transition.

## Findings And Dispositions

- fixed: Selected an unstable crate-internal dispatch facade as the next
  implementation boundary. The future core task may add a host-only helper that
  resembles dispatch_process_descriptor* context plumbing, but it must not add
  new stable SyscallNumber variants, new stable syscall constants, or public SVC
  numbers.
- fixed: Preserved the existing stable syscall vocabulary. src/syscall.rs
  currently accepts stable SVC immediate zero with TalosNop, TalosWrite,
  TalosClose, TalosDup, TalosRead, and TalosOpen only; descriptor-aware dispatch
  helpers inject context for existing descriptor calls and leave unknown numbers
  as ENOSYS.
- fixed: Selected process-local ping descriptor operations for the future core
  task: open, start, pump_or_read_result, status, retry_arp, timeout, and close.
  These operations are sufficient to drive one accepted ping lifecycle and the
  deterministic edge controls already covered by ProcessLocalPingDescriptorControl
  evidence.
- fixed: Preserved scalar/status buffer ownership expectations. The dispatch
  facade may carry scalar fields for process descriptor, route policy,
  destination IPv4, identifier, sequence number, TTL, payload length, ARP retry
  budget, and operation selector, while payload/result/status bytes remain in
  caller-owned task buffers supplied by the host-only test/runtime context rather
  than public userspace socket buffers.
- fixed: Preserved the existing delegation chain:
  ProcessDescriptorStore/DescriptorTable owns the process-local handle;
  ProcessLocalPingDescriptorControl maps that handle to one
  DescriptorShapedPingControl descriptor; DescriptorShapedPingControl delegates
  network behavior to RuntimePingOperationSyscallSubstitute and
  NetworkRuntimeDevicePump.
- fixed: Preserved POSIX-shaped error mapping for the future core task. EBADF
  covers missing owner, invalid process descriptor, closed descriptor, wrong
  object kind, and stale backing ping descriptors. EMFILE covers process or
  backing ping descriptor capacity. EBUSY covers duplicate active ping start.
  EAGAIN covers retry/no-progress exhaustion. ENOSPC covers caller-owned storage
  pressure. Device/internal errors remain delegated through the accepted runtime
  pump stack.
- removed: No stable syscall number, public socket type, libc/POSIX socket API,
  shell command, kernel-backed fake command expansion, live driver adapter,
  live packet I/O, packet queue, retry scheduler, UDP/TCP path, SSH path,
  hardware path, lab mutation, boot publication, or phase transition was added
  or accepted.
- deferred: The actual dispatch facade implementation and source/unit evidence
  are deferred to
  phase12-network-process-local-ping-svc-dispatch-core-20260620.
- not-an-issue: The contract can call this path SVC-shaped because it exercises
  syscall/process-dispatch plumbing in host-only tests; it is not stable SVC ABI
  acceptance until a later supervisor-planned task explicitly accepts public ABI
  numbers, userspace memory layout, and compatibility guarantees.

## Selected Contract

The future core task should add only a crate-internal, host-only dispatch
surface for one process-local ping descriptor. The surface should be shaped like
the existing descriptor-aware dispatch helpers: it receives the current process
owner, ProcessDescriptorStore, caller-owned buffers, and a NetworkRuntimeDevicePump
through explicit context, then routes one selected operation to
ProcessLocalPingDescriptorControl.

Required operations:

- open: create one process-local descriptor backed by one
  DescriptorShapedPingControl operation and return the process descriptor. Missing
  owner is EBADF. Process table or backing ping capacity is EMFILE. Partial
  allocation must unwind the backing ping descriptor.
- start: validate the process descriptor and start one ping with explicit route
  policy, destination IPv4, identifier, sequence number, TTL, payload, ARP retry
  budget, and caller-owned transmit buffer. Duplicate active operation remains
  EBUSY.
- pump_or_read_result: validate the process descriptor and perform exactly one
  NetworkRuntimeDevicePump step through DescriptorShapedPingControl using
  caller-owned receive/transmit/result storage. The operation returns a scalar or
  task-local status/result record that distinguishes no-frame, local no-reply,
  local reply, active ping progress, completion, and delegated errors.
- status: validate the process descriptor and copy idle, pending-ARP, inflight,
  completed, or timed-out state into caller-owned status storage without
  consuming terminal status.
- retry_arp: validate the process descriptor and delegate one pending ARP retry
  step through the accepted control stack.
- timeout: validate the process descriptor and delegate the explicit terminal
  timeout transition.
- close: validate the process descriptor, close the backing
  DescriptorShapedPingControl operation, remove the process descriptor entry, and
  leave later operations on that process descriptor returning EBADF.

The contract must not modify the public SyscallNumber enum or stable
TALOS_*_SYSCALL constants. It may use a private test/runtime-only operation enum
or helper function whose evidence is explicitly labeled unstable/internal. Any
future public userspace ABI must be planned separately, with explicit syscall
numbers, user-memory copy rules, structure layout, compatibility policy, and
negative controls.

The accepted evidence level for this contract is static host-only source/task/doc
review over src/syscall.rs dispatch_process_descriptor* helpers,
ProcessDescriptorStore process-local ownership, ProcessLocalPingDescriptorControl,
DescriptorShapedPingControl, RuntimePingOperationSyscallSubstitute,
NetworkRuntimeDevicePump, fake/trait-level NetworkDevice behavior, caller-owned
buffers, and fixed-capacity state.

## Evidence Reviewed

- Source: src/syscall.rs stable syscall vocabulary and errno mapping lines 1-216.
- Source: src/syscall.rs dispatch_process_descriptor* context patterns lines
  263-520.
- Source: src/syscall.rs DescriptorShapedPingControl and
  ProcessLocalPingDescriptorControl lines 1141-1448.
- Source: src/syscall.rs dispatch_talos_read/open/close/dup descriptor helpers
  lines 1494-1765.
- Source: src/posix.rs ProcessDescriptorStore and per-process DescriptorTable
  ownership.
- Source: src/network.rs NetworkRuntimeDevicePump and ping operation descriptor
  table delegation.
- Task: tasks/2026-06-20-phase12-network-process-local-ping-descriptor-contract.md.
- Task: tasks/2026-06-20-phase12-network-process-local-ping-descriptor-core.md.
- Task: tasks/2026-06-20-phase12-network-process-local-ping-descriptor-smoke.md.
- Task:
  tasks/2026-06-20-phase12-network-process-local-ping-descriptor-smoke-closeout.md.
- Durable state: supervisor selected this task after accepted process-local ping
  descriptor smoke closeout commit fe47ae4bd80475028973a52f8208709e2d379b29.

## Validation

- static source/task/doc review: passed.
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

selected_next_task=phase12-network-process-local-ping-svc-dispatch-core-20260620.

The accepted evidence level is static host-only contract evidence. Shell ping,
public sockets, stable syscall ABI acceptance, socket syscall ABI acceptance,
live driver adapters, live packet I/O, hardware reachability, SSH, smoltcp,
UDP/TCP, lab mutation, boot publication, Phase 12.1 link-hardware retry, broad
socket expansion, and phase transition remain rejected.

Commit: recorded in durable supervisor state after commit creation.
