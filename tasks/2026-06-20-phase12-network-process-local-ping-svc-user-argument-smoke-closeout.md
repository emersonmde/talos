# Phase 12.4 Process-Local Ping SVC User-Argument Smoke Closeout

Task: phase12-network-process-local-ping-svc-user-argument-smoke-closeout-20260620

Status: accepted

Classification: phase12-network-process-local-ping-svc-user-argument-smoke-closeout-accepted

Evidence level: host/QEMU-substitute smoke evidence over fake/trait-level NetworkDevice behavior

## Scope

This closeout reconciles the accepted process-local ping SVC user-argument
contract, core implementation, source/unit evidence, retained smoke transcript,
task records, docs, durable state, and rejected claims. It does not add
implementation behavior, shell ping, public sockets, stable syscall ABI
acceptance, socket syscall ABI acceptance, live driver adapters, live packet
I/O, hardware reachability, SSH, smoltcp, UDP/TCP, lab mutation, boot
publication, Phase 12.1 link-hardware retry, broad socket expansion, or a
phase transition.

## Findings And Dispositions

- not-an-issue: The retained smoke transcript proves the accepted
  dispatch_process_local_ping_descriptor_user_arguments path through
  UserMapping, ProcessLocalPingDispatchOperation,
  ProcessLocalPingDescriptorControl, ProcessDescriptorStore,
  NetworkRuntimeDevicePump, fake NetworkDevice behavior, caller-owned buffers,
  task-owned result/status slots, and fixed-capacity state.
- not-an-issue: The smoke transcript covers one experimental user-argument
  lifecycle: open, idle status copy-out, start from copied user payload,
  ARP-to-ICMP pump result copy-out, echo-reply completion, completed status
  copy-out, and close.
- not-an-issue: The smoke transcript covers selectors for open, start,
  pump_or_read_result, status, retry_arp, timeout, and close; payload copy-in;
  result/status copy-out; bounded scratch; malformed selector and reserved
  fields; missing owner; process descriptor capacity; invalid descriptors;
  output-buffer pressure; invalid user memory; scratch pressure; zero TTL;
  invalid route prefix; and unchanged stable SyscallNumber/TALOS_* vocabulary.
- removed: No shell command, public socket API, stable syscall ABI, socket
  syscall ABI, live driver adapter, packet queue, retry timer scheduler,
  UDP/TCP path, SSH path, hardware path, lab mutation, boot publication, or
  phase transition was added or accepted.
- deferred: Public sockets, stable syscall ABI acceptance, shell ping, live
  driver adapters, live packet I/O, hardware reachability, SSH, broad Phase
  12.4 socket expansion, and any phase transition require supervisor planning
  before a later bounded task can start.

## Evidence Reviewed

- Contract task:
  tasks/2026-06-20-phase12-network-process-local-ping-svc-user-argument-contract.md.
- Core task:
  tasks/2026-06-20-phase12-network-process-local-ping-svc-user-argument-core.md.
- Core closeout:
  tasks/2026-06-20-phase12-network-process-local-ping-svc-user-argument-closeout.md.
- Smoke task:
  tasks/2026-06-20-phase12-network-process-local-ping-svc-user-argument-smoke.md.
- Smoke command:
  scripts/qemu-process-local-ping-svc-user-argument-smoke.sh.
- Retained smoke transcript:
  tasks/evidence/2026-06-20-process-local-ping-svc-user-argument-smoke/qemu-process-local-ping-svc-user-argument-smoke.log.
- Source: src/syscall.rs
  dispatch_process_local_ping_descriptor_user_arguments,
  PROCESS_LOCAL_PING_USER_SELECTOR_* experimental selectors,
  UserMapping copy-in/copy-out, process_local_ping_user_* scalar decoding, and
  fixed result/status encoders.
- Source: src/syscall.rs ProcessLocalPingDispatchOperation,
  ProcessLocalPingDispatchOutputs,
  dispatch_process_local_ping_descriptor_operation,
  ProcessLocalPingDescriptorControl, DescriptorShapedPingControl,
  RuntimePingOperationSyscallSubstitute, NetworkRuntimeDevicePump, and
  ProcessDescriptorStore-facing dispatch context.
- Docs: docs/src/project/phase12-networking-ssh.md and docs/src/roadmap.md.
- Durable state: currentTask
  phase12-network-process-local-ping-svc-user-argument-smoke-20260620 accepted
  at commit 27272c75f8a5c920d46514f7864e33addab2d099 with
  selected_next_task=phase12-network-process-local-ping-svc-user-argument-smoke-closeout-20260620.

## Accepted Evidence Boundary

The accepted evidence level remains host/QEMU-substitute smoke evidence over
fake/trait-level NetworkDevice behavior, experimental user-argument decoding,
UserMapping copy-in/copy-out, process-local descriptor ownership, internal
dispatch-shaped control, ProcessLocalPingDescriptorControl,
DescriptorShapedPingControl, RuntimePingOperationSyscallSubstitute,
NetworkRuntimeDevicePump, caller-owned buffers, task-owned result/status slots,
and fixed-capacity state.

The closeout accepts that the experimental user-argument decoder has retained
smoke coverage for one process-local ping dispatch lifecycle plus the
deterministic selector, reserved-field, owner, descriptor, capacity,
user-memory, buffer-pressure, scratch-pressure, TTL, route-prefix, and stable
syscall-vocabulary controls listed above. It does not accept public sockets,
stable syscall ABI acceptance, socket syscall ABI acceptance, shell ping, live
driver adapters, live packet I/O, hardware reachability, SSH, smoltcp, UDP/TCP,
lab mutation, boot publication, Phase 12.1 link-hardware retry, broad socket
expansion, or phase transition.

## Validation

- static source/task/evidence review: passed.
- diff validation: git diff --check passed.
- docs build: /home/node/.cargo/bin/mdbook build passed with the pre-existing
  large-search-index warning.
- staged diff validation: git diff --cached --check passed before commit.

No Rust source was touched by this closeout, so cargo fmt --all -- --check and
cargo -Zjson-target-spec test --quiet were not required by this task's gates.
The accepted smoke evidence is inherited from the committed smoke task. No Pi 5
hardware run, hardwareTestLock acquisition, boot archive publication, lab
mutation, power cycle, MDIO/PHY/GPIO32 action, RP1 MMIO/DMA work, shell ping,
public socket API, stable syscall ABI acceptance, socket syscall ABI
acceptance, live packet I/O, SSH, or phase transition was performed.

## Result

Accepted. selected_next_task is null.

planningNeeded=true because no later queued Phase 12.4 task has complete
objective dependencies, acceptance criteria, validation gates, docs
requirements, and evidence requirements. Supervisor planning is required before
public sockets, stable syscall ABI acceptance, socket syscall ABI acceptance,
shell ping, live driver adapters, live packet I/O, hardware reachability, SSH,
smoltcp, UDP/TCP, lab mutation, boot publication, Phase 12.1 link-hardware
retry, broad socket expansion, or phase transition.
