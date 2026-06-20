# Phase 12.4 Process-Local Ping SVC Dispatch Closeout

Task: phase12-network-process-local-ping-svc-dispatch-closeout-20260620

Status: accepted

Classification: phase12-network-process-local-ping-svc-dispatch-closeout-accepted

Evidence level: host/QEMU-substitute source/unit evidence over fake/trait-level NetworkDevice behavior

## Scope

This closeout reconciles the accepted process-local ping SVC dispatch contract,
core implementation, source/unit evidence, task records, docs, durable state,
and rejected claims. It does not add shell ping, public sockets, stable syscall
ABI acceptance, socket syscall ABI acceptance, live driver adapters, live packet
I/O, hardware reachability, SSH, smoltcp, UDP/TCP, lab mutation, boot
publication, Phase 12.1 link-hardware retry, broad socket expansion, or a phase
transition.

## Findings And Dispositions

- not-an-issue: The accepted core remains bounded to an unstable
  crate-internal host-only dispatch facade in src/syscall.rs. It does not add a
  stable SyscallNumber variant, TALOS_* syscall constant, public socket API, or
  stable userspace ABI.
- not-an-issue: The dispatch facade routes open, start,
  pump_or_read_result, status, retry_arp, timeout, and close through
  ProcessLocalPingDescriptorControl with explicit current-owner,
  ProcessDescriptorStore, caller-owned receive/transmit buffers, task-owned
  result/status slots, NetworkRuntimeDevicePump, and fake/trait-level
  NetworkDevice context.
- not-an-issue: Source/unit evidence covers one dispatch-shaped lifecycle:
  inherited-stdio process descriptor allocation, start to unresolved ARP,
  ARP-to-ICMP advancement, echo-reply completion, terminal completed status,
  close, and later EBADF.
- not-an-issue: Source/unit evidence covers invalid and closed descriptors,
  missing current owner EBADF, process descriptor capacity unwind EMFILE,
  duplicate active operation EBUSY, retry exhaustion EAGAIN, explicit timeout
  with terminal timed-out status, caller receive-buffer pressure ENOSPC,
  receive IO error, local transmit IO error, and active transmit IO error.
- deferred: Retained smoke transcript evidence for the dispatch-shaped
  process-local path is objectively unblocked and remains the next bounded task,
  phase12-network-process-local-ping-svc-dispatch-smoke-20260620. The accepted
  core evidence is source/unit host/QEMU-substitute evidence, not a retained
  smoke transcript.
- removed: No shell command, public socket API, stable syscall ABI, socket
  syscall ABI, live driver adapter, live packet I/O, hardware reachability, SSH,
  smoltcp, UDP/TCP, lab mutation, boot publication, Phase 12.1 retry, broad
  socket expansion, or phase transition was added or accepted.

## Evidence Reviewed

- Contract task:
  tasks/2026-06-20-phase12-network-process-local-ping-svc-dispatch-contract.md.
- Core task:
  tasks/2026-06-20-phase12-network-process-local-ping-svc-dispatch-core.md.
- Source: src/syscall.rs ProcessLocalPingDispatchOperation,
  ProcessLocalPingDispatchOutcome, ProcessLocalPingDispatchOutputs, and
  dispatch_process_local_ping_descriptor_operation.
- Source: src/syscall.rs ProcessLocalPingDescriptorControl,
  DescriptorShapedPingControl, RuntimePingOperationSyscallSubstitute,
  NetworkRuntimeDevicePump, and ProcessDescriptorStore-facing descriptor
  dispatch context.
- Unit/source evidence:
  process_local_ping_dispatch_completes_lifecycle_through_dispatch_shape.
- Unit/source evidence:
  process_local_ping_dispatch_maps_descriptor_capacity_and_runtime_errors.
- Docs: docs/src/project/phase12-networking-ssh.md and docs/src/roadmap.md.
- Durable state: currentTask
  phase12-network-process-local-ping-svc-dispatch-core-20260620 accepted at
  commit d79711005e0e46b1b88444b077fe0b262e9384e6 with
  selected_next_task=phase12-network-process-local-ping-svc-dispatch-closeout-20260620.

## Accepted Evidence Boundary

The accepted evidence level remains host/QEMU-substitute source/unit evidence
over fake/trait-level NetworkDevice behavior, process-local descriptor
ownership, internal dispatch-shaped control, ProcessLocalPingDescriptorControl,
DescriptorShapedPingControl, RuntimePingOperationSyscallSubstitute,
NetworkRuntimeDevicePump, caller-owned buffers, task-owned result/status slots,
and fixed-capacity state.

The closeout accepts that the internal dispatch facade can drive one
process-local ping descriptor through open, start, pump/read-result, status,
retry_arp, timeout, and close in source/unit tests. It also accepts that
descriptor lifetime, owner, capacity, busy, retry, timeout, receive-buffer
pressure, and device-error controls are deterministic within the
crate-internal host-only boundary.

This closeout does not accept shell ping, public sockets, stable syscall ABI
acceptance, socket syscall ABI acceptance, live driver adapters, live packet
I/O, hardware reachability, SSH, smoltcp, UDP/TCP, lab mutation, boot
publication, Phase 12.1 link-hardware retry, broad socket expansion, or phase
transition.

## Validation

- static source/task/evidence review: passed.
- diff validation: git diff --check passed.
- docs build: /home/node/.cargo/bin/mdbook build passed with the pre-existing
  large-search-index warning.
- staged diff validation: git diff --cached --check passed before commit.

No Rust source was touched by this closeout, so cargo fmt --all -- --check and
cargo -Zjson-target-spec test --quiet were not required by this task's gates.
The accepted source/unit evidence is inherited from the committed core task.
No Pi 5 hardware run, hardwareTestLock acquisition, boot archive publication,
lab mutation, power cycle, MDIO/PHY/GPIO32 action, RP1 MMIO/DMA work, shell
ping, public socket API, stable syscall ABI acceptance, socket syscall ABI
acceptance, live packet I/O, SSH, or phase transition was performed.

## Result

Accepted. selected_next_task is
phase12-network-process-local-ping-svc-dispatch-smoke-20260620.
