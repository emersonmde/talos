# Phase 12.4 Process-Local Ping SVC User-Argument Closeout

Task: phase12-network-process-local-ping-svc-user-argument-closeout-20260620

Status: accepted

Classification: phase12-network-process-local-ping-svc-user-argument-closeout-accepted

Evidence level: host/QEMU-substitute source/unit evidence over fake/trait-level NetworkDevice behavior

## Scope

This closeout reconciles the accepted process-local ping SVC user-argument
contract, core implementation, source/unit evidence, task records, docs,
durable state, and rejected claims. It does not add shell ping, public sockets,
stable syscall ABI acceptance, socket syscall ABI acceptance, live driver
adapters, live packet I/O, hardware reachability, SSH, smoltcp, UDP/TCP, lab
mutation, boot publication, Phase 12.1 link-hardware retry, broad socket
expansion, or a phase transition.

## Findings And Dispositions

- not-an-issue: The accepted core remains bounded to
  dispatch_process_local_ping_descriptor_user_arguments in src/syscall.rs. It
  maps experimental scalar/user-memory inputs into the accepted
  ProcessLocalPingDispatchOperation facade and does not add a stable
  SyscallNumber variant, public TALOS_* syscall constant, public socket API, or
  stable userspace ABI.
- not-an-issue: The decoder routes open, start, pump_or_read_result, status,
  retry_arp, timeout, and close through ProcessLocalPingDispatchOperation with
  explicit current-owner, ProcessDescriptorStore, UserMapping copy-in/copy-out,
  bounded kernel scratch, caller-owned result/status buffers,
  NetworkRuntimeDevicePump, and fake/trait-level NetworkDevice context.
- not-an-issue: Source/unit evidence covers one user-argument lifecycle:
  open, idle status copy-out, start from copied user payload, ARP-to-ICMP pump
  result copy-out, echo-reply completion, completed status copy-out, and close.
- not-an-issue: Source/unit evidence covers unchanged stable syscall/TALOS_*
  vocabulary, malformed selector and reserved fields, missing current owner
  EBADF, process descriptor capacity EMFILE, invalid descriptors EBADF,
  output-buffer pressure ENOSPC, invalid user memory EFAULT, scratch pressure
  ENOSPC, zero TTL EINVAL, and invalid route prefix EINVAL.
- deferred: Retained smoke transcript evidence for the experimental
  user-argument decoder is objectively unblocked and remains the next bounded
  task,
  phase12-network-process-local-ping-svc-user-argument-smoke-20260620. The
  accepted core evidence is source/unit host/QEMU-substitute evidence, not a
  retained smoke transcript.
- removed: No shell command, public socket API, stable syscall ABI, socket
  syscall ABI, live driver adapter, live packet I/O, hardware reachability,
  SSH, smoltcp, UDP/TCP, lab mutation, boot publication, Phase 12.1 retry,
  broad socket expansion, or phase transition was added or accepted.

## Evidence Reviewed

- Contract task:
  tasks/2026-06-20-phase12-network-process-local-ping-svc-user-argument-contract.md.
- Core task:
  tasks/2026-06-20-phase12-network-process-local-ping-svc-user-argument-core.md.
- Source: src/syscall.rs
  dispatch_process_local_ping_descriptor_user_arguments,
  PROCESS_LOCAL_PING_USER_SELECTOR_* experimental selectors,
  process_local_ping_user_* scalar decoding, UserMapping copy-in/copy-out, and
  fixed record encoders.
- Source: src/syscall.rs ProcessLocalPingDispatchOperation,
  ProcessLocalPingDispatchOutputs,
  dispatch_process_local_ping_descriptor_operation,
  ProcessLocalPingDescriptorControl, DescriptorShapedPingControl,
  RuntimePingOperationSyscallSubstitute, NetworkRuntimeDevicePump, and
  ProcessDescriptorStore-facing dispatch context.
- Unit/source evidence:
  process_local_ping_user_arguments_complete_lifecycle_and_copy_outputs.
- Unit/source evidence:
  process_local_ping_user_arguments_reject_malformed_owner_descriptor_capacity_and_buffers.
- Docs: docs/src/project/phase12-networking-ssh.md and docs/src/roadmap.md.
- Durable state: currentTask
  phase12-network-process-local-ping-svc-user-argument-core-20260620 accepted
  at commit f57a36f3f92c9b227e881cb1d501cc749dfb9d16 with
  selected_next_task=phase12-network-process-local-ping-svc-user-argument-closeout-20260620.

## Accepted Evidence Boundary

The accepted evidence level remains host/QEMU-substitute source/unit evidence
over fake/trait-level NetworkDevice behavior, experimental user-argument
decoding, UserMapping copy-in/copy-out, process-local descriptor ownership,
internal dispatch-shaped control, ProcessLocalPingDescriptorControl,
DescriptorShapedPingControl, RuntimePingOperationSyscallSubstitute,
NetworkRuntimeDevicePump, caller-owned buffers, task-owned result/status slots,
and fixed-capacity state.

The closeout accepts that the crate-internal decoder can drive one
process-local ping dispatch lifecycle through experimental scalar/user-memory
arguments in source/unit tests. It also accepts that malformed selectors,
reserved fields, owner/descriptor lifetime, capacity, user-memory faults,
buffer pressure, scratch pressure, TTL, route-prefix, and stable syscall
vocabulary controls are deterministic within the host-only boundary.

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
phase12-network-process-local-ping-svc-user-argument-smoke-20260620.
