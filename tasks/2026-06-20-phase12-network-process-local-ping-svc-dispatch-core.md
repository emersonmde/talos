# Phase 12.4 Process-Local Ping SVC Dispatch Core

Task: phase12-network-process-local-ping-svc-dispatch-core-20260620

Status: accepted

Classification: phase12-network-process-local-ping-svc-dispatch-core-accepted

## Scope

Implement the bounded host-only internal dispatch-shaped facade selected by
phase12-network-process-local-ping-svc-dispatch-contract-20260620. This task
routes one process-local ping descriptor lifecycle through explicit
process-dispatch context without adding stable syscall numbers, public sockets,
shell ping, live driver adapters, live packet I/O, hardware reachability, SSH,
smoltcp, UDP/TCP, lab mutation, boot publication, Phase 12.1 retry, broad socket
expansion, or a phase transition.

## Findings And Dispositions

- fixed: Added ProcessLocalPingDispatchOperation,
  ProcessLocalPingDispatchOutcome, ProcessLocalPingDispatchOutputs, and
  dispatch_process_local_ping_descriptor_operation in src/syscall.rs as a
  crate-internal host-only facade. The stable SyscallNumber enum and TALOS_* API
  constants were not changed.
- fixed: Routed open, start, pump_or_read_result, status, retry_arp, timeout,
  and close through ProcessLocalPingDescriptorControl with explicit current
  owner, ProcessDescriptorStore, NetworkRuntimeDevicePump, caller-owned
  receive/transmit buffers, task-owned output slots, and fake/trait-level
  NetworkDevice context.
- fixed: Added source/unit evidence for one dispatch-shaped lifecycle:
  inherited-stdio process descriptor allocation, start to unresolved ARP,
  ARP-to-ICMP advancement, echo-reply completion, terminal completed status, and
  close with later EBADF.
- fixed: Added deterministic negative/error evidence for invalid and closed
  descriptors, missing current owner EBADF, process descriptor capacity unwind
  EMFILE, duplicate active operation EBUSY, retry exhaustion EAGAIN, explicit
  timeout with terminal timed-out status, caller receive-buffer pressure ENOSPC,
  receive IO error, local transmit IO error, and active transmit IO error.
- removed: No public socket API, stable syscall ABI, socket syscall ABI, new
  TALOS_* syscall constant, shell ping command, kernel-backed fake shell command,
  live driver adapter, live packet I/O, hardware reachability, SSH, smoltcp,
  UDP/TCP, lab mutation, boot publication, Phase 12.1 retry, broad socket
  expansion, or phase transition was added or accepted.
- deferred: Closeout of this core implementation remains the dependency-gated
  follow-up task.
- not-an-issue: The facade uses a Rust enum/function boundary instead of a
  stable SVC number because this task is explicitly unstable/internal
  dispatch-shaped evidence, not public ABI acceptance.

## Evidence

- Source: src/syscall.rs ProcessLocalPingDispatchOperation,
  ProcessLocalPingDispatchOutcome, ProcessLocalPingDispatchOutputs, and
  dispatch_process_local_ping_descriptor_operation.
- Source: src/syscall.rs ProcessLocalPingDescriptorControl,
  DescriptorShapedPingControl, RuntimePingOperationSyscallSubstitute, and
  ProcessDescriptorStore-facing dispatch_process_descriptor* context patterns.
- Unit/source evidence:
  process_local_ping_dispatch_completes_lifecycle_through_dispatch_shape.
- Unit/source evidence:
  process_local_ping_dispatch_maps_descriptor_capacity_and_runtime_errors.
- Prior accepted contract:
  tasks/2026-06-20-phase12-network-process-local-ping-svc-dispatch-contract.md.

## Validation

- cargo fmt --all: applied formatting.
- cargo fmt --all -- --check: passed after formatting.
- focused host/QEMU-substitute unit tests:
  cargo -Zjson-target-spec test --quiet process_local_ping_dispatch passed.
- full host/QEMU-substitute unit tests:
  cargo -Zjson-target-spec test --quiet passed with the documented QEMU PATH;
  654 no_std tests passed. An earlier attempt without the documented QEMU PATH
  failed with qemu-system-aarch64 not found and was rerun with the correct
  environment.
- diff validation: git diff --check passed.
- docs build: /home/node/.cargo/bin/mdbook build passed, existing large
  search-index warning only.
- staged diff validation: git diff --cached --check passed before commit.

No Pi 5 hardware run, hardwareTestLock acquisition, boot archive publication,
lab mutation, power cycle, MDIO/PHY/GPIO32 action, RP1 MMIO/DMA work, shell
ping, public socket API, stable syscall ABI acceptance, socket syscall ABI
acceptance, live packet I/O, SSH, smoltcp, UDP/TCP, Phase 12.1 link-hardware
retry, broad socket expansion, or phase transition was performed.

## Acceptance

Accepted.

selected_next_task=phase12-network-process-local-ping-svc-dispatch-closeout-20260620.

The accepted evidence level is host/QEMU-substitute source/unit evidence over
fake/trait-level NetworkDevice behavior, process-local descriptor ownership,
internal dispatch-shaped control, ProcessLocalPingDescriptorControl,
DescriptorShapedPingControl, RuntimePingOperationSyscallSubstitute,
NetworkRuntimeDevicePump, caller-owned buffers, task-owned result/status slots,
and fixed-capacity state.

Shell ping, public sockets, stable syscall ABI acceptance, socket syscall ABI
acceptance, live driver adapters, live packet I/O, hardware reachability, SSH,
smoltcp, UDP/TCP, lab mutation, boot publication, Phase 12.1 link-hardware
retry, broad socket expansion, and phase transition remain rejected.

Commit: recorded in durable supervisor state after commit creation.
