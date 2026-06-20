# Phase 12.4 Process-Local Ping SVC User-Argument Core

Task: phase12-network-process-local-ping-svc-user-argument-core-20260620

Status: accepted

Classification: phase12-network-process-local-ping-svc-user-argument-core-accepted

## Scope

Implement the bounded crate-internal, host-only user-argument decoder selected
by phase12-network-process-local-ping-svc-user-argument-contract-20260620. This
task decodes experimental scalar/user-memory inputs into the already accepted
process-local ping dispatch facade. It does not add stable syscall numbers,
public sockets, shell ping, live driver adapters, live packet I/O, hardware
reachability, SSH, smoltcp, UDP/TCP, lab mutation, boot publication, Phase 12.1
retry, broad socket expansion, or a phase transition.

## Findings And Dispositions

- fixed: Added dispatch_process_local_ping_descriptor_user_arguments in
  src/syscall.rs as the crate-internal host-only decoder over the accepted
  ProcessLocalPingDispatchOperation facade.
- fixed: Added experimental selectors for open, start, pump_or_read_result,
  status, retry_arp, timeout, and close. These are crate-internal constants and
  do not alter SyscallNumber, STABLE_SVC_IMMEDIATE, or public TALOS_* syscall
  constants.
- fixed: Added deterministic scalar decoding for process descriptor handles,
  payload/result/status user addresses and lengths, destination IPv4, TTL,
  subnet-prefix route policy, identifier, sequence number, and ARP retry budget.
  Nonzero reserved fields, unknown selectors, zero TTL, unsupported route kind,
  and invalid prefix widths fail with EINVAL.
- fixed: Routed payload copy-in through UserMapping/copy_from_user into bounded
  kernel scratch before delegated start, and routed pump/status copy-out through
  copy_to_user into caller-owned result/status buffers after delegated dispatch
  succeeds.
- fixed: Added fixed task-owned record encoders for start/ping steps, runtime
  pump steps, and status snapshots so the host-only decoder can expose accepted
  dispatch outputs without creating a stable public ABI.
- fixed: Added focused source/unit evidence for a complete user-argument
  lifecycle: open, idle status copy-out, start from copied user payload,
  ARP-to-ICMP pump copy-out, echo-reply completion copy-out, completed status
  copy-out, and close.
- fixed: Added deterministic negative/error evidence for unchanged stable
  syscall vocabulary, malformed selector/reserved-field inputs, missing owner
  EBADF, process descriptor capacity EMFILE, invalid descriptor EBADF,
  output-buffer pressure ENOSPC, invalid user memory EFAULT, scratch pressure
  ENOSPC, zero TTL EINVAL, and invalid route prefix EINVAL.
- removed: No public socket API, stable syscall ABI, socket syscall ABI, new
  TALOS_* syscall constant, shell ping command, kernel-backed fake shell command,
  live driver adapter, live packet I/O, hardware reachability, SSH, smoltcp,
  UDP/TCP, lab mutation, boot publication, Phase 12.1 retry, broad socket
  expansion, or phase transition was added or accepted.
- deferred: Closeout of this core implementation remains the dependency-gated
  follow-up task.
- not-an-issue: The record encodings are crate-internal host-only test/control
  records. They are evidence for user-shaped argument decoding, not a stable ABI
  or libc/socket surface.

## Evidence

- Source: src/syscall.rs
  dispatch_process_local_ping_descriptor_user_arguments and the
  PROCESS_LOCAL_PING_USER_SELECTOR_* experimental selector constants.
- Source: src/syscall.rs process_local_ping_user_* scalar decoding,
  UserMapping copy-in/copy-out, and fixed record encoders.
- Source: src/syscall.rs ProcessLocalPingDispatchOperation,
  ProcessLocalPingDispatchOutputs, and
  dispatch_process_local_ping_descriptor_operation.
- Unit/source evidence:
  process_local_ping_user_arguments_complete_lifecycle_and_copy_outputs.
- Unit/source evidence:
  process_local_ping_user_arguments_reject_malformed_owner_descriptor_capacity_and_buffers.
- Prior accepted contract:
  tasks/2026-06-20-phase12-network-process-local-ping-svc-user-argument-contract.md.

## Validation

- cargo fmt --all: applied formatting.
- cargo fmt --all -- --check: passed.
- focused host/QEMU-substitute unit tests:
  cargo -Zjson-target-spec test --quiet process_local_ping passed.
- full host/QEMU-substitute unit tests:
  cargo -Zjson-target-spec test --quiet passed; 656 no_std tests passed.
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

selected_next_task=phase12-network-process-local-ping-svc-user-argument-closeout-20260620.

The accepted evidence level is host/QEMU-substitute source/unit evidence over
fake/trait-level NetworkDevice behavior, experimental user-argument decoding,
UserMapping copy-in/copy-out, process-local descriptor ownership, internal
dispatch-shaped control, ProcessLocalPingDescriptorControl,
DescriptorShapedPingControl, RuntimePingOperationSyscallSubstitute,
NetworkRuntimeDevicePump, caller-owned buffers, task-owned result/status slots,
and fixed-capacity state.

Shell ping, public sockets, stable syscall ABI acceptance, socket syscall ABI
acceptance, live driver adapters, live packet I/O, hardware reachability, SSH,
smoltcp, UDP/TCP, lab mutation, boot publication, Phase 12.1 link-hardware
retry, broad socket expansion, and phase transition remain rejected.

Commit: recorded in durable supervisor state after commit creation.
